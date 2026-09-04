//! Typst WASM plugin entry point for dgs.
//!
//! This crate owns the wire-format (`CanvasRequest`, CBOR-encoded by the
//! Typst side via `cbor.encode`) and delegates all rendering to the [`dgs`]
//! crate from crates.io.

use wasm_minimal_protocol::*;
use ciborium::de::from_reader;
use dgs::{Color, Object, Scene, Theme, Viewport};
use serde::Deserialize;

initiate_protocol!();

/// Wire format sent from the Typst side.
#[derive(Deserialize)]
pub struct CanvasRequest {
    pub viewport: Viewport,
    pub theme: Theme,
    pub grid: bool,
    pub grid_color: Option<Color>,
    pub grid_width: Option<f64>,
    pub grid_spacing: Option<f64>,
    pub axes: bool,
    pub axis_color: Option<Color>,
    pub axis_width: Option<f64>,
    pub axis_label_size: Option<f64>,
    pub objects: Vec<Object>,
}

impl From<CanvasRequest> for Scene {
    fn from(req: CanvasRequest) -> Self {
        let mut scene = Scene::new(req.viewport).theme(req.theme);
        if req.grid {
            scene = scene.grid(true);
        }
        if let Some(c) = req.grid_color {
            scene = scene.grid_color(c);
        }
        if let Some(w) = req.grid_width {
            scene = scene.grid_width(w);
        }
        if let Some(s) = req.grid_spacing {
            scene = scene.grid_spacing(s);
        }
        if req.axes {
            scene = scene.axes(true);
        }
        if let Some(c) = req.axis_color {
            scene = scene.axis_color(c);
        }
        if let Some(w) = req.axis_width {
            scene = scene.axis_width(w);
        }
        if let Some(s) = req.axis_label_size {
            scene = scene.axis_label_size(s);
        }
        scene.extend(req.objects)
    }
}

#[wasm_func]
pub fn render_dgs(arg: &[u8]) -> Result<Vec<u8>, String> {
    let req: CanvasRequest =
        from_reader(arg).map_err(|e| format!("CBOR deserialization failed: {}", e))?;
    Ok(Scene::from(req).to_svg().into_bytes())
}
