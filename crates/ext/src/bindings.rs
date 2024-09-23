wit_bindgen::generate!({
    with: {
        "wasi:clocks/monotonic-clock@0.2.1": wasi_passthrough::bindings::exports::wasi::clocks::monotonic_clock,
        "wasi:http/types@0.2.1": wasi_passthrough::bindings::exports::wasi::http::types,
        "wasi:io/error@0.2.1": wasi_passthrough::bindings::exports::wasi::io::error,
        "wasi:io/poll@0.2.1": wasi_passthrough::bindings::exports::wasi::io::poll,
        "wasi:io/streams@0.2.1": wasi_passthrough::bindings::exports::wasi::io::streams,
        "wasiext:http/ext@0.1.0": generate,
    },
    type_section_suffix: "wasi-passthrough-ext",
});

type Component = ();

#[cfg(not(target_os = "linux"))]
export!(Component);
