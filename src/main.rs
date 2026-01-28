use crate::config::load::{ConfigInterface, ImplConfigInterface};
use crate::handlers::ai::endpoints;
use custom_logger as log;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use mimalloc::MiMalloc;
use std::collections::HashMap;
use std::fs;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Mutex;
use tokio::net::TcpListener;

mod api;
mod config;
mod handlers;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

// used for lookup in read mode only
static MAP_LOOKUP: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);

fn main() {
    // read and parse config
    // for unikernels it will allways be '/etc/config/application-config.json'
    let impl_config = ImplConfigInterface {};
    #[cfg(feature = "local")]
    let res_params = impl_config.read("config/application-config.json".to_string());

    #[cfg(not(feature = "local"))]
    let res_params = impl_config.read("/etc/config/application-config.json".to_string());

    let parameters = match res_params {
        Ok(params) => params,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let level = match parameters.log_level.as_str() {
        "debug" => log::LevelFilter::Debug,
        "trace" => log::LevelFilter::Trace,
        &_ => log::LevelFilter::Info,
    };

    // setup logging
    if let Err(e) = log::Logging::new().with_level(level).init() {
        // log is broken so use eprintln!
        eprintln!("[main] error {}", e);
        std::process::exit(1);
    }

    // parameters used in service
    let mut hm: HashMap<String, String> = HashMap::new();
    // read api key from disk
    let res_content = fs::read_to_string(parameters.api_key_path);
    match res_content {
        Ok(content) => {
            hm.insert(
                "api_key".to_string(),
                content.trim().to_string().replace("\n", ""),
            );
        }
        Err(e) => {
            log::error!("[main] openai api key {}", e);
            std::process::exit(1);
        }
    }
    hm.insert("base_url".to_owned(), parameters.base_url.to_owned());
    *MAP_LOOKUP.lock().unwrap() = Some(hm.clone());

    log::info!("application : {}", env!("CARGO_PKG_NAME"));
    log::info!("author      : {}", env!("CARGO_PKG_AUTHORS"));
    log::info!("version     : {}", env!("CARGO_PKG_VERSION"));

    let result = run_server(parameters.server_port);
    match result {
        Ok(_) => log::info!("[main] unikernel shutdown successfully"),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}

#[tokio::main]
pub async fn run_server(port: usize) -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::new(Ipv4Addr::new(0, 0, 0, 0).into(), port as u16);
    log::info!("[run_server] starting to serve on http://{}", addr);
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(endpoints))
                .await
            {
                log::error!("[run_server] error serving connection: {:?}", err);
            }
        });
    }
}
