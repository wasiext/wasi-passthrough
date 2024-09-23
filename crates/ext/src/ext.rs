use crate::bindings::{exports, wasiext};

impl exports::wasiext::http::ext::Guest for () {
    fn new_response_outparam() -> (
        wasi_passthrough::bindings::exports::wasi::http::types::ResponseOutparam,
        wasi_passthrough::bindings::exports::wasi::http::types::FutureIncomingResponse,
    ) {
        let (out, res) = wasiext::http::ext::new_response_outparam();
        (
            wasi_passthrough::bindings::exports::wasi::http::types::ResponseOutparam::new(out),
            wasi_passthrough::bindings::exports::wasi::http::types::FutureIncomingResponse::new(
                res,
            ),
        )
    }

    fn new_incoming_request(
        req: wasi_passthrough::bindings::exports::wasi::http::types::OutgoingRequest,
    ) -> wasi_passthrough::bindings::exports::wasi::http::types::IncomingRequest {
        wasi_passthrough::bindings::exports::wasi::http::types::IncomingRequest::new(
            wasiext::http::ext::new_incoming_request(req.into_inner()),
        )
    }
}
