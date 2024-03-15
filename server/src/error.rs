// use database::InMemoryDatabase::{ParseError,ReqErr};
// use std::io;
// use std::str;
// use std::string::ParseError;
// pub fn handle_parse_error(err: ParseError) -> ServerError {
//     match err {
//         ParseError::UrlHasNoPath => ServerError::UrlHasNoPath,
//         ParseError::UrlIsNotHTTPS => ServerError::UrlIsNotHTTPS,
//         ParseError::Parser(err) => ServerError::Parser(err),
//     }
// }

// /// Returns errors stemming from requests to the Firebase.
// pub fn handle_req_error(err: ReqErr) -> ServerError {
//     match err {
//         ReqErr::ReqNotJSON => ServerError::ReqNotJSON,
//         ReqErr::RespNotUTF8(err) => ServerError::RespNotUTF8(err),
//         ReqErr::NetworkErr(err) => ServerError::NetworkErr(err),
//         ReqErr::OtherErr(err) => ServerError::OtherErr(err),
//     }
// }

// #[derive(Debug)]
// /// Possible server errors, including ones inherited from dependecies such as hyper
// pub enum ServerError {
//     BadRequest,
//     ReqNotJSON,
//     RespNotUTF8(str::Utf8Error),
//     NetworkErr(hyper::error::Error),
//     FirebaseIoJsonParseErr(rustc_serialize::json::DecoderError),
//     OtherErr(io::Error),
//     UrlHasNoPath,
//     UrlIsNotHTTPS,
//     Parser(url::ParseError),
//     InvalidUserId,
//     // InvalidThreadId,
//     DatabaseFormatErr,
//     // SendMessageErr(websocket::WebSocketError),
// }
