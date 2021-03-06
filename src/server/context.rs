use std::time::Duration;

use super::parser::ErrorCode;
use super::Response;

pub trait Context {
    /// A bad request occured.
    ///
    /// You should send a complete response. If there is already a `Server`
    /// instance to handle the request the `bad_request` method over there
    /// is called to allow stopping database requests and similiar.
    fn emit_error_page(&self, code: ErrorCode, response: &mut Response) {
        use super::parser::ErrorCode::*;
        let (status, reason) = match code {
            BadRequest => (400, "Bad Request"),
            PayloadTooLarge => (413, "Payload Too Large"),
            RequestTimeout => (408, "Request Timeout"),
            RequestHeaderFieldsTooLarge => (431, "Request Header Fields Too Large"),
        };
        response.status(status, reason);
        let data = format!("<h1>{} {}</h1>\n\
            <p><small>Served for you by rotor-http</small></p>\n",
            status, reason);
        let bytes = data.as_bytes();
        response.add_length(bytes.len() as u64).unwrap();
        response.add_header("Content-Type", b"text/html").unwrap();
        response.done_headers().unwrap();
        response.write_body(bytes);
        response.done();
    }
    /// The time the server waits for new input from the client.
    ///
    /// The default is ten seconds.
    fn byte_timeout(&self) -> Duration {
        Duration::new(10, 0)
    }
}
