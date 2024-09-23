wit_bindgen::generate!({
    world: "passthrough-imports",
    with: {
        "wasi:clocks/monotonic-clock@0.2.1": wasi_passthrough::bindings::wasi::clocks::monotonic_clock,
        "wasi:http/types@0.2.1": wasi_passthrough::bindings::wasi::http::types,
        "wasi:io/error@0.2.1": wasi_passthrough::bindings::wasi::io::error,
        "wasi:io/poll@0.2.1": wasi_passthrough::bindings::wasi::io::poll,
        "wasi:io/streams@0.2.1": wasi_passthrough::bindings::wasi::io::streams,
        "wasiext:http/ext@0.1.0": generate,
    },
    type_section_suffix: "wasi-passthrough-ext-imports",
});

pub mod exports {
    mod bindings {
        wit_bindgen::generate!({
            world: "passthrough-exports",
            with: {
                "wasi:clocks/monotonic-clock@0.2.1": wasi_passthrough::bindings::exports::wasi::clocks::monotonic_clock,
                "wasi:http/types@0.2.1": wasi_passthrough::bindings::exports::wasi::http::types,
                "wasi:io/error@0.2.1": wasi_passthrough::bindings::exports::wasi::io::error,
                "wasi:io/poll@0.2.1": wasi_passthrough::bindings::exports::wasi::io::poll,
                "wasi:io/streams@0.2.1": wasi_passthrough::bindings::exports::wasi::io::streams,
                "wasiext:http/ext@0.1.0": generate,
            },
            type_section_suffix: "wasi-passthrough-ext-exports",
        });

        type Component = ();

        #[cfg(not(target_os = "linux"))]
        export!(Component);
    }
    pub use bindings::exports::*;
}
