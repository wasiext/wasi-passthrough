use crate::bindings::{exports, wasiext};

impl exports::wasiext::http::ext::Guest for () {
    fn new_response_outparam() -> (
        wasi_passthrough::bindings::exports::wasi::http::types::ResponseOutparam,
        wasi_passthrough::bindings::exports::wasi::http::types::FutureIncomingResponse,
    ) {
        wasiext::http::ext::new_response_outparam()
    }

    fn new_incoming_request(
        req: wasi_passthrough::bindings::exports::wasi::http::types::OutgoingRequest,
    ) -> wasi_passthrough::bindings::exports::wasi::http::types::IncomingRequest {
        wasiext::http::ext::new_incoming_request(req)
    }
}
