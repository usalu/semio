//! 🎞️ The render loop of one video export: every distinct scene painted ONCE offscreen through the wgpu rasterizer, encoded by the
//! framework's own AVC encoder (the first picture of a run exactly, a `P_Skip` repeat per further frame) and muxed as MP4 — the
//! same container and codec React's first-party tier writes. Progress and cancellation are shared atomics the shell reads and sets
//! between frames.

use super::program::{decode_video_render_picture, video_render_path, video_render_scene};
use semio_framework_kernel_media_jobs::VideoRenderProgram;
use semio_framework_raster::video::{write_avc_mp4, AvcPcmEncoder, VideoEncodeError, VideoEncoder, VideoStreamParameters};
use semio_framework_raster::SceneRasterizer;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

/// 🤝️ What the shell and the render loop share: frames done, frames in all, and the cancel request.
#[derive(Debug, Default)]
pub struct VideoRenderShared {
    completed: AtomicU32,
    total: AtomicU32,
    cancelled: AtomicBool,
}

impl VideoRenderShared {
    /// 🏗️ A fresh state for a job of `total` frames.
    pub fn new(total: u32) -> Arc<Self> {
        Arc::new(Self { completed: AtomicU32::new(0), total: AtomicU32::new(total), cancelled: AtomicBool::new(false) })
    }

    /// 🛑️ Asks the loop to stop before its next frame.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    /// 🛑️ Whether a cancel was asked.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    /// 📈️ Stores the frames done so far.
    pub fn report(&self, completed: u32) {
        self.completed.store(completed, Ordering::SeqCst);
    }

    /// 📈️ `(frames done, frames in all)`.
    pub fn progress(&self) -> (u32, u32) {
        (self.completed.load(Ordering::SeqCst), self.total.load(Ordering::SeqCst))
    }
}

/// 📦️ A finished render: the MP4 and the frames it holds.
#[derive(Debug)]
pub struct VideoRenderOutput {
    pub bytes: Vec<u8>,
    pub frames: u32,
}

/// 🚨️ Why a render stopped short of a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VideoRenderFailure {
    Cancelled { completed: u32 },
    Picture { index: usize, reason: String },
    Gpu { reason: String },
    Encode { reason: String },
}

impl VideoRenderFailure {
    /// 🗒️ The reason a finished job states.
    pub fn reason(&self) -> String {
        match self {
            Self::Cancelled { completed } => format!("cancelled at frame {completed}"),
            Self::Picture { index, reason } => format!("image {index}: {reason}"),
            Self::Gpu { reason } => format!("no wgpu rasterizer: {reason}"),
            Self::Encode { reason } => reason.clone(),
        }
    }
}

struct YieldOnce(bool);

impl std::future::Future for YieldOnce {
    type Output = ();

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        if self.0 {
            return std::task::Poll::Ready(());
        }
        self.0 = true;
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

fn encode_error(error: VideoEncodeError) -> VideoRenderFailure {
    VideoRenderFailure::Encode { reason: error.to_string() }
}

/// 🎬️ Renders and encodes an ADMITTED `program` into MP4 bytes: each timeline run painted once, `shared`'s cancel read before every
/// frame, its progress stored after every frame, the executor yielded to after every frame so the shell keeps running.
pub async fn render_video_program(program: &VideoRenderProgram, resolve: &(dyn Fn(&str) -> Result<Vec<u8>, String> + Sync), shared: &VideoRenderShared) -> Result<VideoRenderOutput, VideoRenderFailure> {
    let cancelled = || shared.is_cancelled();
    let total = program.frame_count() as u32;
    shared.total.store(total, Ordering::SeqCst);
    let mut pictures = Vec::with_capacity(program.images.len());
    for (index, image) in program.images.iter().enumerate() {
        if cancelled() {
            return Err(VideoRenderFailure::Cancelled { completed: 0 });
        }
        pictures.push(decode_video_render_picture(&image.url, resolve, &cancelled).map_err(|reason| VideoRenderFailure::Picture { index, reason })?);
    }
    let paths: Vec<_> = program.paths.iter().map(video_render_path).collect();
    let parameters = VideoStreamParameters { width: program.width, height: program.height, fps: program.fps };
    let mut encoder = AvcPcmEncoder::new(parameters).map_err(encode_error)?;
    let mut rasterizer = SceneRasterizer::new(program.width, program.height).await.map_err(|error| VideoRenderFailure::Gpu { reason: error.to_string() })?;
    let background = program.background.map(|value| value as f32);
    let mut samples = Vec::with_capacity(total as usize);
    for run in &program.timeline {
        if cancelled() {
            return Err(VideoRenderFailure::Cancelled { completed: samples.len() as u32 });
        }
        let rgba = rasterizer.render(&video_render_scene(program, run.scene as usize, &paths, &pictures), background).map_err(|error| VideoRenderFailure::Gpu { reason: error.to_string() })?;
        for repeat in 0..run.frames {
            if cancelled() {
                return Err(VideoRenderFailure::Cancelled { completed: samples.len() as u32 });
            }
            samples.push(if repeat == 0 { encoder.encode(&rgba) } else { encoder.repeat() }.map_err(encode_error)?);
            shared.report(samples.len() as u32);
            YieldOnce(false).await;
        }
    }
    let bytes = write_avc_mp4(&parameters, encoder.configuration(), &samples).map_err(encode_error)?;
    Ok(VideoRenderOutput { bytes, frames: total })
}
