use artisan_middleware::notifications::Email;
use dusa_collection_utils::{
    errors::ErrorArray,
    functions::truncate,
    stringy::Stringy,
};
use hyper::server::conn::Http;
use rustls_pemfile::{certs, read_one, Item};
use serde::Serialize;
use tokio::net::TcpListener;
use std::{fs::File, io::BufReader, net::SocketAddr, sync::Arc};
use tokio_rustls::{rustls::{Certificate, PrivateKey, ServerConfig
}, TlsAcceptor};
use warp::{http::Method, reply::json, Filter, Rejection, Reply};
// filters::body::json,

type WebResult<T> = std::result::Result<T, Rejection>;

const SUCCESS: &str = "success";
const FAILURE: &str = "failed";

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: Stringy,
    pub message: String,
}

pub struct SimpleResponse {
    pub status: Stringy,
}

pub async fn health_checker_handler() -> WebResult<impl Reply> {

    return Ok(json(&GenericResponse {
        status: SUCCESS.into(),
        message: truncate("functional", 6).to_string(),
    }));

}

pub async fn send_mail(email: Email) -> WebResult<impl Reply> {
    match email.send(None).await.uf_unwrap() {
        Ok(_) => {
            return Ok(json(&GenericResponse {
                status: SUCCESS.into(),
                message: "Email relayed!".to_owned(),
            }));
        }
        Err(e) => {
            ErrorArray::new(vec![e.clone()]).display(false);
            return Ok(json(&GenericResponse {
                status: FAILURE.into(),
                message: format!("{}", e),
            }));
        }
    }
}

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "api=info");
    }

    pretty_env_logger::init();

    let cors = warp::cors()
        // .allow_any_origin() // Allow requests from any origin
        .allow_origin("https://www.artisanhosting.net") // Alternatively, specify your React app's domain
        .allow_origin("https://artisanhosting.net") // Alternatively, specify your React app's domain
        .allow_methods(&[Method::POST, Method::GET, Method::OPTIONS])
        .allow_headers(vec![
            "Content-Type",
            "Authorization",
            "Accept",
            "Origin",
            "User-Agent",
            "DNT",
            "Cache-Control",
            "X-Mx-ReqToken",
            "Keep-Alive",
            "X-Requested-With",
            "Access-Control-Allow-Origin",
            "If-Modified-Since",
            "X-CSRF-Token",
        ])
        .build();

    let health_checker = warp::path!("api" / "healthcheck")
        .and(warp::get())
        .and_then(health_checker_handler);

    let receive_mail = warp::path!("api" / "sendmail")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(send_mail)
        .with(cors.clone());

    let routes = health_checker
        .with(warp::log("api"))
        .with(cors.clone())
        .or(receive_mail);

    // Build the warp service
    let svc = warp::service(routes);

    // Set up TLS
    let tls_cfg = load_rustls_config().unwrap();

    let acceptor = TlsAcceptor::from(Arc::new(tls_cfg));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    let listener = TcpListener::bind(&addr).await.unwrap();

    println!("Server running on https://relay.artisanhosting.net");

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let acceptor = acceptor.clone();
        let svc = svc.clone();

        tokio::spawn(async move {
            // Perform the TLS handshake
            let tls_stream = match acceptor.accept(stream).await {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("TLS accept error: {:?}", e);
                    return;
                }
            };

            // Serve the connection using Hyper and Warp
            if let Err(err) = Http::new()
                .serve_connection(tls_stream, svc)
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}


// Setup TLS with Rustls
fn load_rustls_config() -> Result<ServerConfig, Box<dyn std::error::Error>> {
    // Load certificates and private key
    let cert_file = &mut BufReader::new(File::open("/etc/letsencrypt/live/relay.artisanhosting.net/fullchain.pem")?);
    let key_file = &mut BufReader::new(File::open("/etc/letsencrypt/live/relay.artisanhosting.net/privkey.pem")?);

    // Load certificate chain
    let cert_chain: Vec<Certificate> = certs(cert_file)
        .map_err(|_| "Failed to read certificate")?
        .into_iter()
        .map(Certificate)
        .collect();

    // Load private key
    let key = load_private_key(key_file)?;

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth() // Disable client certificate authentication
        .with_single_cert(cert_chain, key)
        .map_err(|_| "Failed to create TLS config")?;

    Ok(config)
}


// Function to load the private key from a PEM file
fn load_private_key<R: std::io::Read + std::io::BufRead>(mut reader: R) -> Result<PrivateKey, Box<dyn std::error::Error>> {
    let mut keys = Vec::new();

    // Read and process the PEM file items
    while let Some(item) = read_one(&mut reader)? {
        match item {
            Item::PKCS8Key(key) => keys.push(PrivateKey(key)),
            Item::RSAKey(key) => keys.push(PrivateKey(key)),
            _ => continue, // Skip over other PEM items
        }
    }

    if keys.is_empty() {
        return Err("No valid private key found in the file".into());
    }

    // Use the first valid private key found
    Ok(keys.remove(0))
}
