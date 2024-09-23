wit_bindgen::generate!({
    generate_all,
    type_section_suffix: "wasi-passthrough",
});

type Component = ();

#[cfg(not(target_os = "linux"))]
export!(Component);
