use crate::bindings::{exports, wasiext};

impl exports::wasiext::http::ext::Guest for () {
    fn new_response_outparam() -> (
        exports::wasi::http::types::ResponseOutparam,
        exports::wasi::http::types::FutureIncomingResponse,
    ) {
        let (out, res) = wasiext::http::ext::new_response_outparam();
        (
            exports::wasi::http::types::ResponseOutparam::new(out),
            exports::wasi::http::types::FutureIncomingResponse::new(res),
        )
    }

    fn new_incoming_request(
        req: exports::wasi::http::types::OutgoingRequest,
    ) -> exports::wasi::http::types::IncomingRequest {
        exports::wasi::http::types::IncomingRequest::new(wasiext::http::ext::new_incoming_request(
            req.into_inner(),
        ))
    }
}
