mod dusa;
mod mailing;

use dusa_collection_utils::{
    errors::{ErrorArray, WarningArray},
    functions::truncate,
    stringy::Stringy,
};
use dusa_common::{
    prefix::{receive_message, send_message},
    MessageType, RequestPayload, RequestRecsPlainText, SOCKET_PATH,
};
use mailing::{Email, EmailSecure};
use serde::Serialize;
use std::os::unix::net::UnixStream;
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
    let throw_away_warning = WarningArray::new_container();
    let throw_away_errors = ErrorArray::new_container();

    match SOCKET_PATH(false, throw_away_errors.clone(), throw_away_warning.clone()).uf_unwrap() {
        Ok(_d) => {
            _d.warning.display();
            let socket_path = _d.data;

            let mut stream = if let Ok(d) = UnixStream::connect(socket_path.clone()) {
                d
            } else {
                // panic!("invalid stream given")
                return Ok(json(&GenericResponse {
                    status: FAILURE.into(),
                    message: "Invalid stream given".to_owned(),
                }));
            };

            let request_data = RequestRecsPlainText {
                command: dusa_common::Commands::EncryptRawText,
                data: "This is data".to_owned(),
                uid: 1000,
            };

            let dusa_message = dusa_common::Message {
                version: dusa_common::VERSION.to_owned(),
                msg_type: MessageType::Request,
                payload: serde_json::to_value(RequestPayload::PlainText(request_data)).unwrap(),
                error: None,
            };

            if let Err(err) =
                send_message(&mut stream, &dusa_message, throw_away_errors.clone()).uf_unwrap()
            {
                err.display(false);
            }

            let data = match receive_message(&mut stream, throw_away_errors.clone()).uf_unwrap() {
                Ok(d) => d,
                Err(e) => {
                    e.display(false);
                    return Ok(json(&GenericResponse {
                        status: FAILURE.into(),
                        message: "Invalid stream given".to_owned(),
                    }));
                }
            };

            match data.msg_type {
                MessageType::Response => {
                    let string_data = data
                        .payload
                        .get("value")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Invalid data received");

                    return Ok(json(&GenericResponse {
                        status: SUCCESS.into(),
                        message: truncate(string_data, 6).to_owned(),
                    }));
                }
                MessageType::ErrorResponse => {
                    return Ok(json(&GenericResponse {
                        status: FAILURE.into(),
                        message: "Dusa encountered an error".to_owned(),
                    }));
                }
                _ => {
                    return Ok(json(&GenericResponse {
                        status: FAILURE.into(),
                        message: "Invalid response given".to_owned(),
                    }))
                }
            }
        }
        Err(e) => {
            e.display(false);
            return Ok(json(&GenericResponse {
                status: FAILURE.into(),
                message: "Dusa not running".to_owned(),
            }));
        }
    }
}

pub async fn send_mail(email: Email) -> WebResult<impl Reply> {
    let encrypted_mail: EmailSecure = match EmailSecure::new(email).await {
        Ok(d) => d,
        Err(e) => {
            ErrorArray::new(vec![e]).display(false);
            return Ok(json(&GenericResponse {
                status: FAILURE.into(),
                message: "Error creating secure email object".to_owned(),
            }));
        }
    };

    match encrypted_mail.send() {
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
        .allow_methods(&[Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origins(vec!["http://localhost:3000/", "http://localhost:8000/"])
        .allow_headers(vec!["content-type"])
        .allow_credentials(true);

    let health_checker = warp::path!("api" / "healthcheck")
        .and(warp::get())
        .and_then(health_checker_handler);

    let receive_mail = warp::path!("api" / "sendmail")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(send_mail);

    let routes = health_checker
        .with(warp::log("api"))
        .with(cors)
        .with(warp::log("api"))
        .or(receive_mail);

    println!("🚀 Server started successfully");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
