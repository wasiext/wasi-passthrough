wit_bindgen::generate!({
    generate_all,
    type_section_suffix: "wasi-passthrough-keyvalue",
});

type Component = ();

#[cfg(not(target_os = "linux"))]
export!(Component);
