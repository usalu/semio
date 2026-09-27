//! 📷️ Renderer-neutral fitting of measured canvas viewports to world bounds.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Canvas2dFraming {
    pub revision: u32,
    pub bounds: [f64; 4],
    pub padding: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Canvas2dFrameCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Canvas2dFraming {
    /// 🖼️ Fits a nonempty measured viewport; zero-extent artwork stays centered without infinite zoom.
    pub fn fit(&self, width: f64, height: f64) -> Option<Canvas2dFrameCamera> {
        let [x0,y0,x1,y1] = self.bounds;
        if ![x0,y0,x1,y1,width,height,self.padding].iter().all(|value| value.is_finite()) || width <= 0.0 || height <= 0.0 || self.padding < 0.0 || x1 < x0 || y1 < y0 { return None; }
        let (dx,dy) = (x1-x0,y1-y0);
        if !dx.is_finite() || !dy.is_finite() { return None; }
        let zx = if dx > 0.0 { (width-2.0*self.padding).max(1.0)/dx } else { f64::INFINITY };
        let zy = if dy > 0.0 { (height-2.0*self.padding).max(1.0)/dy } else { f64::INFINITY };
        let zoom = if dx == 0.0 && dy == 0.0 { 1.0 } else { zx.min(zy).clamp(0.05,32.0) };
        Some(Canvas2dFrameCamera { x: x0*0.5+x1*0.5,y: y0*0.5+y1*0.5,zoom })
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
