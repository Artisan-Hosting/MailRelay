use std::{os::unix::net::UnixStream, thread, time::Duration};

use dusa_collection_utils::{
    errors::{ErrorArray, ErrorArrayItem, WarningArray},
    stringy::Stringy,
    types::PathType,
};
use dusa_common::{
    check_version,
    prefix::{receive_message, send_message},
    MessageType, RequestPayload, RequestRecsPlainText, SOCKET_PATH,
};

pub async fn encrypt_text(data: String) -> Result<Stringy, ErrorArrayItem> {
    let throw_away_warning: WarningArray = WarningArray::new_container();
    let throw_away_errors: ErrorArray = ErrorArray::new_container();

    let socket_path: PathType =
        match SOCKET_PATH(false, throw_away_errors.clone(), throw_away_warning.clone()).uf_unwrap()
        {
            Ok(d) => {
                d.warning.display();
                d.data
            }
            Err(mut e) => return Err(e.pop()),
        };

    let mut stream: UnixStream = match UnixStream::connect(socket_path.clone()) {
        Ok(d) => d,
        Err(e) => return Err(ErrorArrayItem::from(e)),
    };

    let request_data = RequestRecsPlainText {
        command: dusa_common::Commands::EncryptRawText,
        data,
        uid: 1000,
    };

    let dusa_message = dusa_common::Message {
        version: dusa_common::VERSION.to_owned(),
        msg_type: MessageType::Request,
        payload: serde_json::to_value(RequestPayload::PlainText(request_data)).unwrap(),
        error: None,
    };

    if let Err(mut err) =
        send_message(&mut stream, &dusa_message, throw_away_errors.clone()).uf_unwrap()
    {
        return Err(err.pop());
    }

    thread::sleep(Duration::from_millis(100));

    let response: dusa_common::prefix::GeneralMessage =
        match receive_message(&mut stream, throw_away_errors.clone()).uf_unwrap() {
            Ok(d) => d,
            Err(mut e) => return Err(e.pop()),
        };

    // sanity check
    check_version(&response.version);

    let data: Stringy = response
        .payload
        .get("value")
        .and_then(|v| v.as_str())
        .unwrap_or("Invalid data received")
        .into();

    drop(throw_away_errors);
    drop(throw_away_warning);

    return Ok(data);
}
