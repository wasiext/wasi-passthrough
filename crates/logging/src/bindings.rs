wit_bindgen::generate!({
    generate_all,
});

type Component = ();

#[cfg(not(target_os = "linux"))]
export!(Component);
