use std::fs;
use std::process::Command;

use anyhow::Context as _;
use wasi_preview1_component_adapter_provider::{
    WASI_SNAPSHOT_PREVIEW1_ADAPTER_NAME, WASI_SNAPSHOT_PREVIEW1_REACTOR_ADAPTER,
};

#[test]
fn componentize() -> anyhow::Result<()> {
    let status = Command::new(env!("CARGO"))
        .args([
            "build",
            "-p",
            "test-all",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .status()
        .context("failed to build component")?;
    assert!(status.success());
    let wasm = fs::read("target/wasm32-unknown-unknown/debug/test_all.wasm")
        .context("failed to read Wasm")?;
    wit_component::ComponentEncoder::default()
        .validate(true)
        .module(&wasm)
        .context("failed to set core component module")?
        .adapter(
            WASI_SNAPSHOT_PREVIEW1_ADAPTER_NAME,
            WASI_SNAPSHOT_PREVIEW1_REACTOR_ADAPTER,
        )
        .context("failed to add WASI preview1 adapter")?
        .encode()
        .context("failed to encode a component from module")?;
    Ok(())
}
