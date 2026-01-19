# Overview

A simple unikernel service that calls grok api endpoints written in Rust

## Requirements

This service uses (openai type schema) as its base, it can be configured to use the specific schema enabled service, along with the relevant api token

## Usage

**NB** The unikernel launch process requires a statically linked elf binary, the following make recipe will build a static binary

Clone this repo

cd rust-ai-unikernel-grok-service

```
make fmt
make verify
make build
```

## Signing

The unikernel launch process checks to see if the binary has been signed.

To sign the binary use the "rust-microservice-package-manager" project.

Execute the following commands

Create a key-pair (ignore this step if you have already created a key-pair) for signing

```
./target/release/microservice-package-manager keypair
```

Sign the binary

```
./target/release/rust-microservice-package-manager sign --artifact <path-to-binary>
```

The signed artifact will be stored in the .ssh folder of rust-microservice-package-manager project 

## Configration

The config file should be left as is, the field 'api-key' should be left blank (for obvious security reasons), the token will be injected at unikernel build time (via scripts)

## Local Testing

Build as follows

```
make fmt
make verify
make build-local
```

Execute locally 

```
./target/release/ai-unikernel-grok-service
```







