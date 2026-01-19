use crate::MAP_LOOKUP;
use crate::api::schema::ChatCompletionResponse;
use custom_logger as log;
use http::{Method, Request, Response, StatusCode};
use http_body_util::BodyExt;
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use reqwest::Client;

pub async fn endpoints(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let mut response = Response::new(Full::default());
    let request = req.uri().path();
    log::debug!("{}", request);
    match *req.method() {
        Method::POST => match request {
            x if x.contains("/v1/chat/completions") => {
                let data = req.into_body().collect().await?.to_bytes();
                let result = process_post_call(data).await;
                match result {
                    Ok(content) => {
                        *response.body_mut() = Full::from(content);
                    }
                    Err(err) => {
                        log::error!("[endpoints] {}", err);
                        *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                        *response.body_mut() = Full::from(format!("error : {:?}\n", err.source()));
                    }
                }
            }
            &_ => {}
        },
        Method::GET => match request {
            x if x.contains("/v1/health") => {
                let content = format!(
                    r##"{{ "status":"ok", "appplication": "{}", "version": "{}" }}"##,
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION"),
                );
                *response.body_mut() = Full::from(content);
            }
            &_ => {}
        },
        _ => {
            log::error!("[endpoints] method/endpoint not implemented");
            *response.body_mut() = Full::from("[endpoints] method/endpoint not implmented\n");
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };
    Ok(response)
}

#[allow(unused)]
async fn process_get_call() -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    let url = get_item("base_url")?;
    let token = get_item("api_key")?;
    log::debug!("[process_get_call] {}", url);
    let client_response = client.get(url).bearer_auth(token).send().await?;
    log::debug!("[process_get_call] status {}", client_response.status());
    let response = client_response.bytes().await?;
    let result = str::from_utf8(&response)?;
    Ok(result.to_string())
}

async fn process_post_call(data: Bytes) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    let url = get_item("base_url")?;
    let token = get_item("api_key")?;
    log::debug!("[process_post_call] url {}", url);
    log::info!(
        "[process_post_call] data {}",
        String::from_utf8(data.to_vec()).unwrap()
    );
    let client_response = client
        .post(url)
        .bearer_auth(token)
        .header("Content-Type", "application/json")
        .body(data)
        .send()
        .await?;
    log::debug!("[process_post_call] status {}", client_response.status());
    let response = client_response.bytes().await?;
    let chat_response: ChatCompletionResponse = serde_json::from_slice(&response)?;
    log::debug!(
        "[process_post_call] avg token/s {}",
        chat_response.usage.avg_tok_per_sec
    );
    log::debug!(
        "[process_post_call] completion time {}",
        chat_response.usage.total_completion_time_sec
    );
    log::debug!(
        "[process_post_call] total tokens {}",
        chat_response.usage.total_tokens
    );
    Ok(format!(
        "\n{:?}\n",
        chat_response.choices[0].message.content.clone()
    ))
}

fn get_item(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let hm_guard = MAP_LOOKUP.lock().map_err(|_| "mutex lock failed")?;
    let value = match hm_guard.as_ref() {
        Some(res) => {
            let item_value = res.get(name);
            match item_value {
                Some(final_value) => final_value,
                None => {
                    return Err(Box::from(format!(
                        "[get_item] hashmap lookup {} not found",
                        name
                    )));
                }
            }
        }
        None => {
            return Err(Box::from("[get_item] error validating hashmap lookup"));
        }
    };
    Ok(value.to_string())
}
