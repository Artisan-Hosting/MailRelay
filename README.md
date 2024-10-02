# Website Relay API

This is a Rust-based web application that provides two main API endpoints:

1. **Health Check Endpoint**: `GET /api/healthcheck` - Checks the health status of a service called Dusa.
2. **Send Mail Endpoint**: `POST /api/sendmail` - Receives email data in JSON format and sends an email using secure encryption.

The application uses the `warp` web framework and runs over HTTPS using TLS with Rustls. It is designed to run on `relay.artisanhosting.net`.

## Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Running the Application](#running-the-application)
- [API Endpoints](#api-endpoints)
  - [Health Check Endpoint](#health-check-endpoint)
  - [Send Mail Endpoint](#send-mail-endpoint)
- [TLS Configuration](#tls-configuration)
- [Logging](#logging)
- [CORS Configuration](#cors-configuration)
- [License](#license)

## Features

- **Secure HTTPS Server**: Uses TLS with Rustls for encrypted communication.
- **Health Check API**: Verifies the status of the Dusa service via Unix sockets.
- **Email Sending API**: Accepts email data and sends encrypted emails.
- **CORS Support**: Configured to allow cross-origin requests from specific domains.
- **Logging**: Provides detailed logging for requests and errors.

## Prerequisites

- **Rust**: Ensure you have Rust and Cargo installed. You can install Rust using [rustup](https://rustup.rs/).
- **OpenSSL**: Required for generating SSL certificates (if not using Let's Encrypt).
- **Let's Encrypt Certificate**: For production, obtain SSL certificates for your domain.
- **Unix Socket Support**: The application communicates with the Dusa service via Unix sockets.
- **Dependencies**: The application relies on several Rust crates, which will be fetched automatically by Cargo.

## Installation

1. **Clone the Repository**

   ```bash
   git clone https://github.com/yourusername/website-relay-api.git
   cd website-relay-api
   ```

2. **Build the Application**

   ```bash
   cargo build --release
   ```

## Configuration

### TLS Certificates

Place your TLS certificate and private key in the appropriate directory. The application expects the certificates at:

- Certificate: `/etc/letsencrypt/live/relay.artisanhosting.net/fullchain.pem`
- Private Key: `/etc/letsencrypt/live/relay.artisanhosting.net/privkey.pem`

If your certificates are located elsewhere, update the paths in the `load_rustls_config` function in `main.rs`.

### Environment Variables

Set the `RUST_LOG` environment variable to configure logging levels:

```bash
export RUST_LOG=api=info
```

## Running the Application

Start the server using Cargo:

```bash
cargo run --release
```

The server will start listening on port `8000` and is accessible via `https://relay.artisanhosting.net`.

## API Endpoints

### Health Check Endpoint

- **URL**: `GET /api/healthcheck`
- **Description**: Checks the health status of the Dusa service.
- **Response**:
  - **Success** (`200 OK`):

    ```json
    {
      "status": "success",
      "message": "Encrypted message truncated"
    }
    ```

  - **Failure** (`200 OK` with failure status):

    ```json
    {
      "status": "failed",
      "message": "Error message"
    }
    ```

- **Example Request**:

  ```bash
  curl -X GET https://relay.artisanhosting.net/api/healthcheck
  ```

### Send Mail Endpoint

- **URL**: `POST /api/sendmail`
- **Description**: Accepts JSON data to send an email securely.
- **Request Headers**:

  ```
  Content-Type: application/json
  ```

- **Request Body**:

  ```json
  {
    "to": "recipient@example.com",
    "from": "sender@example.com",
    "subject": "Test Email",
    "body": "This is a test email."
  }
  ```

- **Response**:
  - **Success** (`200 OK`):

    ```json
    {
      "status": "success",
      "message": "Email relayed!"
    }
    ```

  - **Failure** (`200 OK` with failure status):

    ```json
    {
      "status": "failed",
      "message": "Error message"
    }
    ```

- **Example Request**:

  ```bash
  curl -X POST https://relay.artisanhosting.net/api/sendmail \
    -H "Content-Type: application/json" \
    -d '{
          "to": "recipient@example.com",
          "from": "sender@example.com",
          "subject": "Test Email",
          "body": "This is a test email."
        }'
  ```

## TLS Configuration

The application uses Rustls for TLS support. Ensure that your SSL certificates are correctly configured:

- Update the certificate and key file paths in the `load_rustls_config` function if they differ from the default.
- The application is configured to use certificates from Let's Encrypt.

```rust
fn load_rustls_config() -> Result<ServerConfig, Box<dyn std::error::Error>> {
    // Load certificates and private key
    let cert_file = &mut BufReader::new(File::open("/path/to/fullchain.pem")?);
    let key_file = &mut BufReader::new(File::open("/path/to/privkey.pem")?);

    // Rest of the code...
}
```

## Logging

The application uses `pretty_env_logger` for logging. Logs include request information and errors.

- Logs are printed to the console.
- Set the `RUST_LOG` environment variable to control logging levels.

## CORS Configuration

Cross-Origin Resource Sharing (CORS) is configured to allow requests from:

- `https://www.artisanhosting.net`
- `https://artisanhosting.net`

Headers and methods are specified to ensure the frontend can communicate with the API.

```rust
let cors = warp::cors()
    .allow_origin("https://www.artisanhosting.net")
    .allow_origin("https://artisanhosting.net")
    .allow_methods(&[Method::POST, Method::GET, Method::OPTIONS])
    .allow_headers(vec![
        "Content-Type",
        "Authorization",
        // Other headers...
    ])
    .build();
```

## Dependencies

Key dependencies used in this application:

- **warp**: Web framework for building the API.
- **tokio**: Asynchronous runtime for Rust.
- **tokio-rustls**: TLS support for tokio using Rustls.
- **rustls**: Modern TLS library for Rust.
- **serde** and **serde_json**: Serialization and deserialization of JSON data.
- **hyper**: HTTP library used under the hood by warp.
- **dusa_common** and **dusa_collection_utils**: Custom crates for communication with the Dusa service.
- **mailing**: Custom module handling email encryption and sending.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
