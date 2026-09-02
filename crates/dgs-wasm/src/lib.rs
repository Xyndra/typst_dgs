use wasm_minimal_protocol::*;
use ciborium::de::from_reader;
use dgs_core::{render, CanvasRequest};

initiate_protocol!();

#[wasm_func]
pub fn render_dgs(arg: &[u8]) -> Result<Vec<u8>, String> {
    let req: CanvasRequest = from_reader(arg)
        .map_err(|e| format!("CBOR deserialization failed: {}", e))?;
    let svg = render(req);
    Ok(svg.into_bytes())
}
