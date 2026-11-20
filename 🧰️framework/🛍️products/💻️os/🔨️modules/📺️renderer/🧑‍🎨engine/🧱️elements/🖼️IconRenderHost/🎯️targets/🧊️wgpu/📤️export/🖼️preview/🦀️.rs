//! 🖼️ Preview raster derived from the same bounded vector export as the downloaded SVG.

use std::sync::mpsc::{self, Receiver, TryRecvError};

use infinite_world::world::World3dMeshAsset;
use semio_framework_async::CancelToken;
use semio_framework_pixels::RasterImage;
use semio_renderer_wgpu_host_base::RuntimeHostWaker;

use super::{IconSvgExport, IconSvgRejected};

#[path = "📐️extent/🦀️.rs"]
mod extent;
use extent::preview_extent;

pub struct IconSvgPreview {
    vector: Option<IconSvgExport>,
    contents: Option<Vec<u8>>,
    raster: Option<Receiver<Result<RasterImage, String>>>,
    cancel: CancelToken,
    wake: Option<RuntimeHostWaker>,
    byte_budget: usize,
    #[cfg(target_arch = "wasm32")]
    dimensions: (u32, u32),
}

impl IconSvgPreview {
    #[expect(clippy::result_large_err, reason = "Preview refusal preserves the exact SVG asset owner.")]
    pub fn new(request: &str, asset: World3dMeshAsset, cancel: CancelToken, byte_budget: usize) -> Result<Self, IconSvgRejected> {
        let vector = IconSvgExport::new(request, asset)?;
        #[cfg(target_arch = "wasm32")]
        let dimensions = vector.dimensions();
        Ok(Self {
            vector: Some(vector), contents: None, raster: None, cancel, wake: None, byte_budget,
            #[cfg(target_arch = "wasm32")]
            dimensions,
        })
    }

    pub fn with_waker(mut self, wake: Option<RuntimeHostWaker>) -> Self {
        self.wake = wake;
        self
    }

    pub fn waits_for_raster(&self) -> bool {
        self.raster.is_some()
    }

    pub fn vector_progress(&self) -> Option<(usize, usize)> {
        let (_, completed, total) = self.vector.as_ref()?.progress();
        Some((completed, total))
    }

    pub fn advance(&mut self) -> Result<Option<RasterImage>, String> {
        if self.cancel.is_cancelled_now() {
            return Err("icon SVG preview was cancelled".into());
        }
        if let Some(vector) = self.vector.as_mut() {
            if self.contents.is_none() {
                if vector.advance()? {
                    self.contents = Some(vector.take_svg().ok_or("icon SVG preview had no markup")?);
                }
            } else if vector.close_step() {
                self.vector = None;
            }
            return Ok(None);
        }
        if let Some(contents) = self.contents.take() {
            let (send, receive) = mpsc::channel();
            let cancel = self.cancel.clone();
            let wake = self.wake.take();
            let byte_budget = self.byte_budget;
            #[cfg(target_arch = "wasm32")]
            let dimensions = self.dimensions;
            semio_renderer_wgpu_host_base::spawn_app_task(async move {
                #[cfg(not(target_arch = "wasm32"))]
                let image = rasterize(&contents, &cancel, byte_budget);
                #[cfg(target_arch = "wasm32")]
                let image = rasterize_browser(contents, dimensions, byte_budget, cancel).await;
                if send.send(image).is_ok() {
                    if let Some(wake) = wake {
                        wake();
                    }
                }
            });
            self.raster = Some(receive);
            return Ok(None);
        }
        let Some(raster) = self.raster.as_ref() else { return Ok(None) };
        match raster.try_recv() {
            Ok(image) => {
                self.raster = None;
                image.map(Some)
            }
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => {
                self.raster = None;
                Err("icon SVG preview raster worker closed unexpectedly".into())
            }
        }
    }

    pub fn close_step(&mut self) -> bool {
        self.cancel.cancel_now();
        self.wake = None;
        if let Some(vector) = self.vector.as_mut() {
            if !vector.close_step() {
                return false;
            }
            self.vector = None;
        }
        if let Some(contents) = self.contents.as_mut() {
            contents.truncate(contents.len().saturating_sub(4096));
            if !contents.is_empty() {
                return false;
            }
            self.contents = None;
        }
        if let Some(raster) = self.raster.as_ref() {
            if matches!(raster.try_recv(), Err(TryRecvError::Empty)) {
                return false;
            }
            self.raster = None;
        }
        true
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn rasterize(contents: &[u8], cancel: &CancelToken, byte_budget: usize) -> Result<RasterImage, String> {
    let check = || (!cancel.is_cancelled_now()).then_some(()).ok_or("icon SVG preview was cancelled");
    check()?;
    let tree = usvg::Tree::from_data(contents, &usvg::Options::default()).map_err(|error| error.to_string())?;
    check()?;
    let natural = tree.size();
    let (width, height) = preview_extent(natural.width(), natural.height(), byte_budget)
        .ok_or("icon SVG preview raster budget was too small")?;
    let mut pixmap = tiny_skia::Pixmap::new(width, height).ok_or("icon SVG preview raster allocation failed")?;
    let transform = tiny_skia::Transform::from_scale(width as f32 / natural.width(), height as f32 / natural.height());
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    let mut pixels = pixmap.take();
    for rows in pixels.chunks_mut(16384) {
        check()?;
        super::png::unpremultiply_rgba(rows);
    }
    Ok(RasterImage { width, height, pixels })
}

#[cfg(target_arch = "wasm32")]
async fn rasterize_browser(contents: Vec<u8>, dimensions: (u32, u32), budget: usize, cancel: CancelToken) -> Result<RasterImage, String> {
    let (width, height) = preview_extent(dimensions.0 as f32, dimensions.1 as f32, budget)
        .ok_or("icon SVG preview raster budget was too small")?;
    let pixels = semio_renderer_wgpu_host_base::rasterize_browser_svg(contents, width, height, cancel).await?;
    Ok(RasterImage { width, height, pixels })
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../../🧪️tests/🖼️svg-preview/🦀️.rs"]
mod tests;
