//! 🎞️ Kernel media and job programs: spawned job drive, media export encoding, video render program and ledger, file import staging.

use semio_framework_value::DslValue;
use semio_framework_value::Number;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use semio_framework_kernel_identity::RequestOutcome;

//#region 🔖️Effect
//#region 🧵️SpawnedJobDrive
/// 🧰️ The `Effect::SpawnJob` kind of every framework reserved tool verb (undo, redo, copy/paste,
/// selection and interaction verbs): a live job each host starts on the spawning instance, steps to its
/// end and answers with `Event::JobCompleted`, never a replayable product job. Twin of TypeScript
/// `FRAMEWORK_RESERVED_JOB_KIND` in `🎠️kernel/🟦️.ts`; both are read from `🧫️fixtures/🧵️spawned-job-drive`.
pub const FRAMEWORK_RESERVED_JOB_KIND: &str = "framework.reserved.tool";

/// 🧵 How many `step-job` observations ONE host admission of a spawned job may take before it gives
/// up. A framework reserved tool job (`interactionSelect`/`interactionHover`/`clearSelection`) reaches
/// `Done` in two; the ceiling is the host's patience, not the job's expected cost.
pub const SPAWNED_JOB_STEP_CEILING: u32 = 32;

/// 🧵 The fuel one `step-job` is granted. Deliberately the host's STATIC budget rather than a lane
/// grant: a reserved tool job is an interaction the user is waiting on, and a budget that varied per
/// call would make "did the selection apply?" depend on scheduler weather.
pub const SPAWNED_JOB_FUEL: u64 = 50_000_000;

/// 🧵 The wall deadline one `step-job` is granted, in milliseconds.
pub const SPAWNED_JOB_DEADLINE_MS: u32 = 100;

/// 🧵 One observation of the guest's `jobs::step-job` export, in the WIT `job-step` vocabulary.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "status")]
pub enum SpawnedJobStep {
    Running,
    Done { value: Vec<u8> },
    Failed { value: Vec<u8> },
}

/// 🧵 What the host owes the guest once a spawned job reached a terminal step: the `Event::JobCompleted`
/// payload, plus how many steps it took to get there (the number a budget law reads).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpawnedJobCompletion {
    pub steps: u32,
    pub outcome: RequestOutcome,
}

/// 🧵 Why a spawned job produced no completion. Both are loud: a host that silently stopped pumping
/// is exactly the defect this contract closes — every `interactionSelect`/`interactionHover` on the
/// wgpu target admitted its reserved tool job and then never ran it, so hover and selection were
/// published perfectly and applied never (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-world3d-interaction-2026-09-13.md` §7).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpawnedJobDriveError {
    /// ⏳️ [`SPAWNED_JOB_STEP_CEILING`] observations passed with no terminal step.
    Stalled { steps: u32 },
    /// 📏️ More observations than the ceiling admits — a driver bug, never a guest one.
    Overrun { steps: u32 },
}

impl std::fmt::Display for SpawnedJobDriveError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stalled { steps } => write!(formatter, "spawned job did not reach a terminal step within {steps} host steps"),
            Self::Overrun { steps } => write!(formatter, "spawned job drive took {steps} steps, past the {SPAWNED_JOB_STEP_CEILING}-step ceiling"),
        }
    }
}

impl std::error::Error for SpawnedJobDriveError {}

/// 🧵 The ONE rule that turns a transcript of `step-job` observations into the `Event::JobCompleted`
/// the host owes the guest. Every host on every renderer answers through this, and its TypeScript
/// twin `spawnedJobCompletion` in `🎠️kernel/🟦️.ts` answers identically; both drive
/// `🧫️fixtures/🧵️spawned-job-drive/🔣️.json`.
///
/// Observations AFTER the first terminal step are ignored rather than refused: a driver that stops
/// stepping the instant it sees `Done` and one that read a batch both hand over the same completion.
pub fn spawned_job_completion(steps: &[SpawnedJobStep]) -> Result<SpawnedJobCompletion, SpawnedJobDriveError> {
    if steps.len() as u32 > SPAWNED_JOB_STEP_CEILING {
        return Err(SpawnedJobDriveError::Overrun { steps: steps.len() as u32 });
    }
    for (index, step) in steps.iter().enumerate() {
        let outcome = match step {
            SpawnedJobStep::Running => continue,
            SpawnedJobStep::Done { value } => RequestOutcome::Ok(value.clone()),
            SpawnedJobStep::Failed { value } => RequestOutcome::Err(value.clone()),
        };
        return Ok(SpawnedJobCompletion { steps: index as u32 + 1, outcome });
    }
    Err(SpawnedJobDriveError::Stalled { steps: steps.len() as u32 })
}

//#endregion 🧵️SpawnedJobDrive

/// 🖼️ One icon-render export request: the destination filename plus the opaque icon-scene
/// render request forwarded to the shell's `iconRenderPort`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_value::RetireOwned)]
pub struct IconRenderExportItem {
    pub filename: String,
    pub request: DslValue,
}

//#region ⬇️MediaExportEncoding
/// ⬇️ The only `Effect::DownloadMediaExport::encoding` value that means "`data` is not text".
/// Declared here, beside the effect it qualifies, so no shell can invent a second spelling.
pub const MEDIA_EXPORT_BASE64_ENCODING: &str = "base64";

/// ⬇️ The textual encoding a producer may state EXPLICITLY — `puzzle3d`'s `exportSnapshot` does
/// (`✏️s/🔌️plugins/🧩️puzzle/…/📤️export-snapshot/🦀️.rs`). It means exactly what an absent `encoding`
/// means: `data` IS the file. Named rather than left to fall through an "anything that is not
/// base64 is text" branch, which is how a genuinely unknown encoding used to be saved as text.
pub const MEDIA_EXPORT_UTF8_ENCODING: &str = "utf-8";

/// ⬇️ Why a `DownloadMediaExport` envelope could not become bytes. Both are loud on purpose: a shell
/// that quietly saved the envelope's `data` as text is exactly the defect this contract closes —
/// every binary export in the repo reached disk as base64 TEXT under a binary file name
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️io-surface-2026-09-13.md` §7.3).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MediaExportEncodingError {
    /// 🗣️ An encoding no shell knows how to read. `None` and [`MEDIA_EXPORT_BASE64_ENCODING`] are the
    /// whole vocabulary; a segmented-download handle is consumed by the segmented lane before it
    /// ever reaches here.
    Unsupported { encoding: String },
    /// 🔤️ `encoding` said base64 and `data` is not canonical RFC 4648 standard base64.
    Malformed { detail: String },
}

impl std::fmt::Display for MediaExportEncodingError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsupported { encoding } => write!(formatter, "unsupported media-export encoding {encoding:?}"),
            Self::Malformed { detail } => write!(formatter, "malformed base64 media export ({detail})"),
        }
    }
}

impl std::error::Error for MediaExportEncodingError {}

/// ⬇️ The ONE rule that turns a [`Effect::DownloadMediaExport`] envelope into the bytes the user
/// saves — no `encoding` (or [`MEDIA_EXPORT_UTF8_ENCODING`]) means `data` IS the file, and
/// [`MEDIA_EXPORT_BASE64_ENCODING`] means `data` carries the bytes. Every shell on every renderer answers through this, and its
/// TypeScript twin `mediaExportBytes` in `🎠️kernel/🟦️.ts` answers identically; both drive
/// `🧫️fixtures/⬇️media-export-encoding/🔣️.json`.
pub fn media_export_bytes(data: &str, encoding: Option<&str>) -> Result<Vec<u8>, MediaExportEncodingError> {
    match encoding {
        None | Some(MEDIA_EXPORT_UTF8_ENCODING) => Ok(data.as_bytes().to_vec()),
        Some(MEDIA_EXPORT_BASE64_ENCODING) => semio_framework_io_base64::base64_standard_decode(data).map_err(|error| MediaExportEncodingError::Malformed { detail: error.to_string() }),
        Some(other) => Err(MediaExportEncodingError::Unsupported { encoding: other.to_string() }),
    }
}

//#endregion ⬇️MediaExportEncoding
//#region 🎞️VideoRenderProgram

/// 🎟️ The host capability a plugin requests (`CapabilityRequest.id` in its package descriptor) before any host renders
/// its [`Effect::VideoRenderExport`]: the host lends its canvas and its encoder only to plugins that declared the ask, and
/// answers every other plugin with the `capability` refusal of `🧫️fixtures/🎞️video-render-job/🔣️.json`.
pub const MEDIA_VIDEO_RENDER_CAPABILITY: &str = "media.video-render";

/// 🎞️ The schema id every [`VideoRenderProgram`] states; `🧫️fixtures/🎞️video-render-program/🔣️.json` is its
/// language-agnostic law, `videoRenderProgramProblem` in `🎠️kernel/🟦️.ts` its TypeScript twin.
pub const VIDEO_RENDER_PROGRAM_SCHEMA: &str = "semio.video-render.program.v1";

/// 📏️ The largest picture edge a program may ask for (the raster video tier's own ceiling).
pub const VIDEO_RENDER_PROGRAM_MAXIMUM_EDGE: u32 = 4096;

/// 🎞️ The fastest frame rate a program may ask for.
pub const VIDEO_RENDER_PROGRAM_MAXIMUM_FPS: u32 = 120;

/// ⏱️ The most frames one program may render (ten minutes at 60 fps).
pub const VIDEO_RENDER_PROGRAM_MAXIMUM_FRAMES: u64 = 36_000;

/// ✏️ One path in a program's shared path table: `verbs` is one letter per segment (`M` move, `L` line,
/// `Q` quadratic, `C` cubic, `Z` close), `points` the flat `x, y` pairs they consume (1, 1, 2, 3, 0).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_value::RetireOwned)]
pub struct VideoRenderPath {
    pub verbs: String,
    pub points: Vec<f64>,
}

/// 🖼️ One picture a program draws from: a same-origin absolute path (`/🖼️assets/…`) or a `data:image/…` URL, never a
/// foreign origin — a host fetches it itself, and a cross-origin picture would taint the canvas it encodes from.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_value::RetireOwned)]
pub struct VideoRenderImage {
    pub url: String,
}

/// 🎨️ One paint operation of a frame, composited in list order over the program background. Every `transform` is
/// `[a, b, c, d, e, f]` (`x' = a x + c y + e`, `y' = b x + d y + f`) into device pixels, y down; every colour is straight
/// RGBA in `0..=1`. `Fill`/`Stroke` paint path `path` of the table (a stroke `width` path units wide); `Image` draws the
/// `crop` (`[x, y, width, height]`, normalised to the picture) of image `image` onto the unit square that `transform`
/// places, at `opacity`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[derive(semio_framework_value::RetireOwned)]
pub enum VideoRenderOp {
    Fill { path: u32, transform: [f64; 6], color: [f64; 4] },
    Stroke { path: u32, transform: [f64; 6], color: [f64; 4], width: f64 },
    Image { image: u32, crop: [f64; 4], transform: [f64; 6], opacity: f64 },
}

/// 🖼️ One distinct picture: its paint operations over the program background.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_value::RetireOwned)]
pub struct VideoRenderScene {
    pub ops: Vec<VideoRenderOp>,
}

/// ⏯️ Scene `scene` shown for `frames` consecutive frames.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_value::RetireOwned)]
pub struct VideoRenderRun {
    pub scene: u32,
    pub frames: u32,
}

/// 🎞️ Everything a host needs to render a video without asking the guest again: picture size, frame rate,
/// background, a shared path table, the pictures it draws from, the distinct scenes and the run-length timeline that
/// plays them. Identical consecutive frames are one run, so a still slide costs one scene however long it plays.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_value::RetireOwned)]
pub struct VideoRenderProgram {
    pub schema: String,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub background: [f64; 4],
    pub paths: Vec<VideoRenderPath>,
    pub images: Vec<VideoRenderImage>,
    pub scenes: Vec<VideoRenderScene>,
    pub timeline: Vec<VideoRenderRun>,
}

/// 🚨️ Why a host refuses a [`VideoRenderProgram`]; `code()` is the fixture's language-agnostic vocabulary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VideoRenderProgramError {
    Schema { schema: String },
    Dimensions { width: u32, height: u32 },
    FrameRate { fps: u32 },
    Empty,
    TooLong { frames: u64 },
    Timeline { run: usize },
    PathVerbs { path: usize },
    ImageUrl { image: usize },
    PathIndex { scene: usize, op: usize },
    ImageIndex { scene: usize, op: usize },
    Paint { scene: usize, op: usize },
}

impl VideoRenderProgramError {
    /// 🔤️ The fixture's error code for this refusal.
    pub fn code(&self) -> &'static str {
        match self {
            Self::Schema { .. } => "schema",
            Self::Dimensions { .. } => "dimensions",
            Self::FrameRate { .. } => "frameRate",
            Self::Empty => "empty",
            Self::TooLong { .. } => "tooLong",
            Self::Timeline { .. } => "timeline",
            Self::PathVerbs { .. } => "pathVerbs",
            Self::ImageUrl { .. } => "imageUrl",
            Self::PathIndex { .. } => "pathIndex",
            Self::ImageIndex { .. } => "imageIndex",
            Self::Paint { .. } => "paint",
        }
    }
}

impl std::fmt::Display for VideoRenderProgramError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Schema { schema } => write!(formatter, "video program schema {schema:?} is not {VIDEO_RENDER_PROGRAM_SCHEMA}"),
            Self::Dimensions { width, height } => write!(formatter, "video program picture {width}x{height} is not an even size within 2..={VIDEO_RENDER_PROGRAM_MAXIMUM_EDGE}"),
            Self::FrameRate { fps } => write!(formatter, "video program frame rate {fps} is outside 1..={VIDEO_RENDER_PROGRAM_MAXIMUM_FPS}"),
            Self::Empty => formatter.write_str("video program has no frames"),
            Self::TooLong { frames } => write!(formatter, "video program has {frames} frames, more than {VIDEO_RENDER_PROGRAM_MAXIMUM_FRAMES}"),
            Self::Timeline { run } => write!(formatter, "video program timeline run {run} names no scene or shows it for zero frames"),
            Self::PathVerbs { path } => write!(formatter, "video program path {path} has verbs its points do not match"),
            Self::ImageUrl { image } => write!(formatter, "video program image {image} is neither a same-origin path nor a data:image URL"),
            Self::PathIndex { scene, op } => write!(formatter, "video program scene {scene} op {op} names no path"),
            Self::ImageIndex { scene, op } => write!(formatter, "video program scene {scene} op {op} names no image"),
            Self::Paint { scene, op } => write!(formatter, "video program scene {scene} op {op} has a colour, transform, width, crop or opacity out of range"),
        }
    }
}

impl std::error::Error for VideoRenderProgramError {}

impl VideoRenderProgram {
    /// 🎞️ Frames the timeline plays.
    pub fn frame_count(&self) -> u64 {
        self.timeline.iter().map(|run| u64::from(run.frames)).sum()
    }

    /// ⏱️ Playing time in milliseconds, rounded half up — what the encoded MP4 states.
    pub fn duration_milliseconds(&self) -> u64 {
        (self.frame_count() * 1000 + u64::from(self.fps) / 2) / u64::from(self.fps.max(1))
    }

    /// 🚦️ The ONE admission rule every host applies before it renders a frame.
    pub fn validate(&self) -> Result<(), VideoRenderProgramError> {
        if self.schema != VIDEO_RENDER_PROGRAM_SCHEMA {
            return Err(VideoRenderProgramError::Schema { schema: self.schema.clone() });
        }
        let edge = |value: u32| (2..=VIDEO_RENDER_PROGRAM_MAXIMUM_EDGE).contains(&value) && value % 2 == 0;
        if !edge(self.width) || !edge(self.height) {
            return Err(VideoRenderProgramError::Dimensions { width: self.width, height: self.height });
        }
        if self.fps == 0 || self.fps > VIDEO_RENDER_PROGRAM_MAXIMUM_FPS {
            return Err(VideoRenderProgramError::FrameRate { fps: self.fps });
        }
        let frames = self.frame_count();
        if frames == 0 {
            return Err(VideoRenderProgramError::Empty);
        }
        if frames > VIDEO_RENDER_PROGRAM_MAXIMUM_FRAMES {
            return Err(VideoRenderProgramError::TooLong { frames });
        }
        if let Some(run) = self.timeline.iter().position(|run| run.frames == 0 || run.scene as usize >= self.scenes.len()) {
            return Err(VideoRenderProgramError::Timeline { run });
        }
        if let Some(path) = self.paths.iter().position(|path| !video_render_path_is_well_formed(path)) {
            return Err(VideoRenderProgramError::PathVerbs { path });
        }
        if let Some(image) = self.images.iter().position(|image| !video_render_image_url_is_admitted(&image.url)) {
            return Err(VideoRenderProgramError::ImageUrl { image });
        }
        if !self.background.iter().all(|value| video_render_unit(*value)) {
            return Err(VideoRenderProgramError::Paint { scene: 0, op: 0 });
        }
        let finite = |values: &[f64]| values.iter().all(|value| value.is_finite());
        for (scene_index, scene) in self.scenes.iter().enumerate() {
            for (op_index, op) in scene.ops.iter().enumerate() {
                let (reference, painted) = match op {
                    VideoRenderOp::Fill { path, transform, color } => ((*path as usize) < self.paths.len(), finite(transform) && color.iter().all(|value| video_render_unit(*value))),
                    VideoRenderOp::Stroke { path, transform, color, width } => ((*path as usize) < self.paths.len(), finite(transform) && color.iter().all(|value| video_render_unit(*value)) && width.is_finite() && *width >= 0.0),
                    VideoRenderOp::Image { image, crop, transform, opacity } => ((*image as usize) < self.images.len(), finite(transform) && video_render_crop_is_admitted(crop) && video_render_unit(*opacity)),
                };
                if !reference {
                    return Err(match op {
                        VideoRenderOp::Image { .. } => VideoRenderProgramError::ImageIndex { scene: scene_index, op: op_index },
                        _ => VideoRenderProgramError::PathIndex { scene: scene_index, op: op_index },
                    });
                }
                if !painted {
                    return Err(VideoRenderProgramError::Paint { scene: scene_index, op: op_index });
                }
            }
        }
        Ok(())
    }
}

fn video_render_unit(value: f64) -> bool {
    value.is_finite() && (0.0..=1.0).contains(&value)
}

/// ✏️ A path's verbs consume exactly its points, start with a move and every coordinate is finite.
pub fn video_render_path_is_well_formed(path: &VideoRenderPath) -> bool {
    let mut needed = 0usize;
    for (index, verb) in path.verbs.chars().enumerate() {
        needed += match verb {
            'M' => 2,
            'L' => 2,
            'Q' => 4,
            'C' => 6,
            'Z' => 0,
            _ => return false,
        };
        if index == 0 && verb != 'M' {
            return false;
        }
    }
    needed == path.points.len() && path.points.iter().all(|value| value.is_finite())
}

/// 🖼️ An image URL a host may fetch for a program: a same-origin absolute path (`/…`, never the protocol-relative
/// `//…`) or an inline `data:image/…` URL.
pub fn video_render_image_url_is_admitted(url: &str) -> bool {
    (url.starts_with('/') && !url.starts_with("//")) || url.starts_with("data:image/")
}

/// ✂️ A crop `[x, y, width, height]` lies inside the unit picture and covers a non-empty area.
pub fn video_render_crop_is_admitted(crop: &[f64; 4]) -> bool {
    let [x, y, width, height] = *crop;
    crop.iter().all(|value| value.is_finite()) && x >= 0.0 && y >= 0.0 && width > 0.0 && height > 0.0 && x + width <= 1.0 + 1e-9 && y + height <= 1.0 + 1e-9
}

//#endregion 🎞️VideoRenderProgram
//#region 🧵️VideoRenderJob

/// 🧩️ Which encoder produced a finished render: the host's platform encoder (the OS or browser H.264 encoder plugged in
/// behind the raster video tier's port) or the tier's own first-party encoder.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "kebab-case")]
#[value(rename_all = "kebab-case")]
pub enum VideoRenderEncoderTier {
    Platform,
    FirstParty,
}

/// 🏁️ How one host video render job ended.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "status", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "status", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum VideoRenderJobOutcome {
    Done { bytes: u64, tier: VideoRenderEncoderTier },
    Cancelled { completed: u64 },
    Refused { code: String },
    Failed { reason: String },
}

/// 🧵️ One fact of a host video render job's life — ephemeral, local-only state of the host that renders it. A host
/// never mutates its task list: it appends one of these and the task list is the fold of the log
/// ([`VideoRenderJobLedger`]); a cancel is a command that appends [`Self::CancelRequested`], which the render loop reads
/// back from the ledger before its next frame. `🧫️fixtures/🎞️video-render-job/🔣️.json` is the law both twins fold.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum VideoRenderJobEvent {
    Started { job: u64, owner: String, filename: String, frames: u64, at_ms: u64 },
    Progressed { job: u64, completed: u64 },
    CancelRequested { job: u64 },
    Finished { job: u64, outcome: VideoRenderJobOutcome },
}

/// 🧵️ One running job as a task list shows it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoRenderJobRow {
    pub job: u64,
    pub owner: String,
    pub filename: String,
    pub frames: u64,
    pub completed: u64,
    pub cancelling: bool,
    pub started_at_ms: u64,
}

/// 🚨️ Why a [`VideoRenderJobLedger`] refuses an event; `code()` is the fixture's vocabulary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VideoRenderJobEventError {
    StaleJob { job: u64, last: u64 },
    UnknownJob { job: u64 },
    ProgressRegressed { job: u64, completed: u64, previous: u64 },
    ProgressOverrun { job: u64, completed: u64, frames: u64 },
    AlreadyCancelling { job: u64 },
}

impl VideoRenderJobEventError {
    /// 🔤️ The fixture's error code for this refusal.
    pub fn code(&self) -> &'static str {
        match self {
            Self::StaleJob { .. } => "staleJob",
            Self::UnknownJob { .. } => "unknownJob",
            Self::ProgressRegressed { .. } => "progressRegressed",
            Self::ProgressOverrun { .. } => "progressOverrun",
            Self::AlreadyCancelling { .. } => "alreadyCancelling",
        }
    }
}

impl std::fmt::Display for VideoRenderJobEventError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StaleJob { job, last } => write!(formatter, "video render job {job} does not follow job {last}"),
            Self::UnknownJob { job } => write!(formatter, "video render job {job} is not running"),
            Self::ProgressRegressed { job, completed, previous } => write!(formatter, "video render job {job} went back from frame {previous} to {completed}"),
            Self::ProgressOverrun { job, completed, frames } => write!(formatter, "video render job {job} reports frame {completed} of {frames}"),
            Self::AlreadyCancelling { job } => write!(formatter, "video render job {job} is already cancelling"),
        }
    }
}

impl std::error::Error for VideoRenderJobEventError {}

/// 📒️ The fold of a host's [`VideoRenderJobEvent`] log: the running jobs in start order and the last job id issued.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VideoRenderJobLedger {
    last_job: u64,
    running: Vec<VideoRenderJobRow>,
}

impl VideoRenderJobLedger {
    /// 📒️ Folds a whole log; the error names the index of the first event the ledger refused.
    pub fn fold<'a>(events: impl IntoIterator<Item = &'a VideoRenderJobEvent>) -> Result<Self, (usize, VideoRenderJobEventError)> {
        let mut ledger = Self::default();
        for (index, event) in events.into_iter().enumerate() {
            ledger.apply(event).map_err(|error| (index, error))?;
        }
        Ok(ledger)
    }

    /// ➕️ Applies one event, or refuses it and leaves the ledger as it was.
    pub fn apply(&mut self, event: &VideoRenderJobEvent) -> Result<(), VideoRenderJobEventError> {
        match event {
            VideoRenderJobEvent::Started { job, owner, filename, frames, at_ms } => {
                if *job <= self.last_job {
                    return Err(VideoRenderJobEventError::StaleJob { job: *job, last: self.last_job });
                }
                self.last_job = *job;
                self.running.push(VideoRenderJobRow { job: *job, owner: owner.clone(), filename: filename.clone(), frames: *frames, completed: 0, cancelling: false, started_at_ms: *at_ms });
            }
            VideoRenderJobEvent::Progressed { job, completed } => {
                let row = self.running.iter_mut().find(|row| row.job == *job).ok_or(VideoRenderJobEventError::UnknownJob { job: *job })?;
                if *completed < row.completed {
                    return Err(VideoRenderJobEventError::ProgressRegressed { job: *job, completed: *completed, previous: row.completed });
                }
                if *completed > row.frames {
                    return Err(VideoRenderJobEventError::ProgressOverrun { job: *job, completed: *completed, frames: row.frames });
                }
                row.completed = *completed;
            }
            VideoRenderJobEvent::CancelRequested { job } => {
                let row = self.running.iter_mut().find(|row| row.job == *job).ok_or(VideoRenderJobEventError::UnknownJob { job: *job })?;
                if row.cancelling {
                    return Err(VideoRenderJobEventError::AlreadyCancelling { job: *job });
                }
                row.cancelling = true;
            }
            VideoRenderJobEvent::Finished { job, .. } => {
                let index = self.running.iter().position(|row| row.job == *job).ok_or(VideoRenderJobEventError::UnknownJob { job: *job })?;
                self.running.remove(index);
            }
        }
        Ok(())
    }

    /// 🧵️ Every running job, in start order.
    pub fn running(&self) -> &[VideoRenderJobRow] {
        &self.running
    }

    /// 🔎️ Running job `job`, if any.
    pub fn row(&self, job: u64) -> Option<&VideoRenderJobRow> {
        self.running.iter().find(|row| row.job == job)
    }

    /// 🔢️ The last job id issued (`0` before the first).
    pub fn last_job(&self) -> u64 {
        self.last_job
    }
}

//#endregion 🧵️VideoRenderJob
//#region 📤️FileOpenImport

/// 📥️ Bytes ONE import chunk may carry to the guest.
///
/// 🧊️ Derived from the guest's per-request contiguous ceiling, never a literal: an import's
/// `payload` crosses as one string, and every guest hop that carries it asks for one contiguous
/// block, so a whole document sent as a single invocation asks the fixed guest heap for a block
/// several times that ceiling. Half the ceiling leaves the other half for the invocation envelope
/// the chunk rides in.
pub const IMPORT_CHUNK_BYTES: usize = semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES / 2;

/// 📥️ The argument names one import chunk is dispatched with. Declared here, beside
/// [`Effect::RequestFileOpen`], so no shell can invent a second spelling of the same envelope —
/// the wgpu shell used to send `{json, payload}` and React `{payload, name, chunk, chunkCount}`,
/// and a plugin could satisfy only one of them.
pub const IMPORT_ARGUMENT_PAYLOAD: &str = "payload";

pub const IMPORT_ARGUMENT_NAME: &str = "name";

pub const IMPORT_ARGUMENT_CHUNK: &str = "chunk";

pub const IMPORT_ARGUMENT_CHUNK_COUNT: &str = "chunkCount";

pub const IMPORT_ARGUMENT_INDEX: &str = "index";

pub const IMPORT_ARGUMENT_TOTAL: &str = "total";

/// 📥️ One chunk of one opened file, positioned in its own run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportChunk {
    pub payload: String,
    pub chunk: usize,
    pub chunk_count: usize,
}

/// 📥️ Slices one opened file's contents into the chunks a shell dispatches, so no single import
/// invocation asks the guest for a contiguous block above its own per-request ceiling.
///
/// 🔤️ Sliced by UTF-8 EXTENT, not by code units: the guest measures `text.len()` in bytes, so a
/// slice counted in UTF-16 units would overrun the cap by up to 3× on non-ASCII text. No slice ever
/// splits a code point, and an empty payload still yields exactly one chunk — a picked empty file is
/// a real pick the guest must be told about.
///
/// TypeScript twin: `importPayloadChunks` in `🎠️kernel/🟦️.ts`; both drive
/// `🧫️fixtures/📤️file-open-import/🔣️.json`.
pub fn import_payload_chunks(payload: &str) -> Vec<ImportChunk> {
    let mut pages: Vec<String> = Vec::new();
    let mut page = String::new();
    for character in payload.chars() {
        if page.len() + character.len_utf8() > IMPORT_CHUNK_BYTES {
            pages.push(core::mem::take(&mut page));
        }
        page.push(character);
    }
    if !page.is_empty() || pages.is_empty() {
        pages.push(page);
    }
    let chunk_count = pages.len();
    pages.into_iter().enumerate().map(|(chunk, payload)| ImportChunk { payload, chunk, chunk_count }).collect()
}

/// 📥️ The arguments ONE import chunk is dispatched with — `fan_out` is `Some((index, total))` only
/// when the picker was opened with `multiple`, exactly as React's `dispatchOpenedFiles` extends a
/// multi-file pick. Integers are minted as [`Number::UInt`] carriers, never floats: the guest's
/// [`ImportStaging`] reads `chunk`/`chunkCount` with `DslValue::as_u64`, which a `Float` never answers.
pub fn import_chunk_arguments(name: &str, chunk: &ImportChunk, fan_out: Option<(usize, usize)>) -> DslValue {
    let uint = |value: usize| DslValue::Number(semio_framework_value::Number::UInt(value as u64));
    let mut entries = vec![
        (IMPORT_ARGUMENT_PAYLOAD.to_string(), DslValue::String(chunk.payload.clone())),
        (IMPORT_ARGUMENT_NAME.to_string(), DslValue::String(name.to_string())),
        (IMPORT_ARGUMENT_CHUNK.to_string(), uint(chunk.chunk)),
        (IMPORT_ARGUMENT_CHUNK_COUNT.to_string(), uint(chunk.chunk_count)),
    ];
    if let Some((index, total)) = fan_out {
        entries.push((IMPORT_ARGUMENT_INDEX.to_string(), uint(index)));
        entries.push((IMPORT_ARGUMENT_TOTAL.to_string(), uint(total)));
    }
    DslValue::Object(entries)
}

//#region 📥️ImportStaging
/// 📦️ Chunks one staged import may reassemble: 128 × [`IMPORT_CHUNK_BYTES`] = 4 MiB, the byte authority of one
/// document archive — the largest thing a person opens from a file — so no staged run outgrows one document.
pub const IMPORT_STAGING_MAXIMUM_CHUNKS: usize = 128;

/// 📦️ The bytes one staged run may hold, implied by [`IMPORT_STAGING_MAXIMUM_CHUNKS`] chunks of at most
/// [`IMPORT_CHUNK_BYTES`] each.
pub const IMPORT_STAGING_MAXIMUM_BYTES: usize = IMPORT_STAGING_MAXIMUM_CHUNKS * IMPORT_CHUNK_BYTES;

/// 🧵️ Runs one app instance stages at once — one per file a person is picking into this document. A new pick takes
/// the slot of the least recently advanced run, so an abandoned or cancelled pick holds memory only until then:
/// at most `IMPORT_STAGING_RUNS × IMPORT_STAGING_MAXIMUM_BYTES` per instance.
pub const IMPORT_STAGING_RUNS: usize = 2;

/// 🚫️ Why the staging refused one chunk — a typed code the dispatch turns into a fault, never a silent drop.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImportStagingRefusal {
    Envelope,
    Chunk,
    Gap,
}

impl ImportStagingRefusal {
    /// 🏷️ The stable fault code.
    pub fn code(self) -> &'static str {
        match self {
            Self::Envelope => "file-import.envelope",
            Self::Chunk => "file-import.chunk",
            Self::Gap => "file-import.gap",
        }
    }

    /// 🗣️ What a person is told.
    pub fn message(self) -> &'static str {
        match self {
            Self::Envelope => "The file arrived with an invalid chunk envelope, or is larger than one document may be.",
            Self::Chunk => "One part of the file is larger than one import chunk.",
            Self::Gap => "A part of the file arrived out of order; pick the file again.",
        }
    }
}

/// 📥️ What one action's arguments are to the staging: not an import chunk at all, a chunk of a run that is still open
/// (`next_chunk` of `chunk_count` is its progress), or the whole file — as the arguments the app's action decodes:
/// `payload` and `name` (plus `index`/`total` of a multi-file pick and every other argument the invocation carried),
/// never the chunk envelope.
#[derive(Clone, Debug, PartialEq)]
pub enum ImportArguments {
    NotAnImport,
    Staged { next_chunk: usize, chunk_count: usize },
    Whole(DslValue),
}

struct ImportRun {
    name: String,
    fan_out: Option<(usize, usize)>,
    chunk_count: usize,
    next_chunk: usize,
    pages: Vec<String>,
    touched: u64,
}

/// 🧵️ One app instance's open file-import runs: the framework half of every [`Effect::RequestFileOpen`] import, so no
/// app stages chunks itself. Instance-scoped, never process-global — two people importing into two documents of the
/// same component never see each other's bytes, and an instance that closes takes its runs with it. Pages stay
/// chunk-sized until the run closes.
#[derive(Default)]
pub struct ImportStaging {
    runs: Vec<ImportRun>,
    sequence: u64,
}

impl ImportStaging {
    /// 📥️ Admits the arguments one import action was dispatched with ([`import_chunk_arguments`]). A run is keyed by
    /// `(name, fan-out, chunk_count)`: a re-pick restarts its run at chunk 0, a chunk the run already admitted is a
    /// retransmission acknowledged at the cursor, any other out-of-order chunk drops the run as a gap.
    pub fn admit_args(&mut self, args: Option<&DslValue>) -> Result<ImportArguments, ImportStagingRefusal> {
        let Some(DslValue::Object(entries)) = args else {
            return Ok(ImportArguments::NotAnImport);
        };
        let field = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, value)| value);
        let Some(count) = field(IMPORT_ARGUMENT_CHUNK_COUNT) else {
            return Ok(ImportArguments::NotAnImport);
        };
        let position = |value: Option<&DslValue>| value.and_then(DslValue::as_u64).and_then(|value| usize::try_from(value).ok());
        let chunk_count = position(Some(count)).ok_or(ImportStagingRefusal::Envelope)?;
        let chunk = position(field(IMPORT_ARGUMENT_CHUNK)).ok_or(ImportStagingRefusal::Envelope)?;
        let payload = field(IMPORT_ARGUMENT_PAYLOAD).and_then(DslValue::as_str).ok_or(ImportStagingRefusal::Envelope)?;
        let name = field(IMPORT_ARGUMENT_NAME).and_then(DslValue::as_str).ok_or(ImportStagingRefusal::Envelope)?;
        let fan_out = match (field(IMPORT_ARGUMENT_INDEX), field(IMPORT_ARGUMENT_TOTAL)) {
            (None, None) => None,
            (index, total) => match (position(index), position(total)) {
                (Some(index), Some(total)) if index < total => Some((index, total)),
                _ => return Err(ImportStagingRefusal::Envelope),
            },
        };
        if chunk_count == 0 || chunk_count > IMPORT_STAGING_MAXIMUM_CHUNKS || chunk >= chunk_count {
            return Err(ImportStagingRefusal::Envelope);
        }
        if payload.len() > IMPORT_CHUNK_BYTES {
            return Err(ImportStagingRefusal::Chunk);
        }
        let envelope = [IMPORT_ARGUMENT_PAYLOAD, IMPORT_ARGUMENT_NAME, IMPORT_ARGUMENT_CHUNK, IMPORT_ARGUMENT_CHUNK_COUNT, IMPORT_ARGUMENT_INDEX, IMPORT_ARGUMENT_TOTAL];
        let whole = |text: String| {
            let uint = |value: usize| DslValue::Number(semio_framework_value::Number::UInt(value as u64));
            let mut arguments = vec![(IMPORT_ARGUMENT_PAYLOAD.to_string(), DslValue::String(text)), (IMPORT_ARGUMENT_NAME.to_string(), DslValue::String(name.to_string()))];
            if let Some((index, total)) = fan_out {
                arguments.push((IMPORT_ARGUMENT_INDEX.to_string(), uint(index)));
                arguments.push((IMPORT_ARGUMENT_TOTAL.to_string(), uint(total)));
            }
            arguments.extend(entries.iter().filter(|(key, _)| !envelope.contains(&key.as_str())).cloned());
            ImportArguments::Whole(DslValue::Object(arguments))
        };
        if chunk_count == 1 {
            return Ok(whole(payload.to_string()));
        }
        self.sequence = self.sequence.saturating_add(1);
        let sequence = self.sequence;
        let held = self.runs.iter().position(|run| run.name == name && run.fan_out == fan_out && run.chunk_count == chunk_count);
        let index = match held {
            Some(index) if self.runs[index].next_chunk == chunk => index,
            Some(index) if chunk != 0 && chunk < self.runs[index].next_chunk => {
                let run = &mut self.runs[index];
                run.touched = sequence;
                return Ok(ImportArguments::Staged { next_chunk: run.next_chunk, chunk_count: run.chunk_count });
            }
            Some(index) if chunk != 0 => {
                self.runs.swap_remove(index);
                return Err(ImportStagingRefusal::Gap);
            }
            Some(index) => {
                self.runs[index] = ImportRun { name: name.to_string(), fan_out, chunk_count, next_chunk: 0, pages: Vec::new(), touched: sequence };
                index
            }
            None if chunk != 0 => return Err(ImportStagingRefusal::Gap),
            None => {
                if self.runs.len() >= IMPORT_STAGING_RUNS {
                    if let Some(stalest) = self.runs.iter().enumerate().min_by_key(|(_, run)| run.touched).map(|(index, _)| index) {
                        self.runs.swap_remove(stalest);
                    }
                }
                self.runs.push(ImportRun { name: name.to_string(), fan_out, chunk_count, next_chunk: 0, pages: Vec::new(), touched: sequence });
                self.runs.len() - 1
            }
        };
        let run = &mut self.runs[index];
        run.pages.push(payload.to_string());
        run.next_chunk = chunk.saturating_add(1);
        run.touched = sequence;
        if run.next_chunk < run.chunk_count {
            return Ok(ImportArguments::Staged { next_chunk: run.next_chunk, chunk_count: run.chunk_count });
        }
        let closed = self.runs.swap_remove(index);
        Ok(whole(closed.pages.concat()))
    }

    /// 🛑️ Drops every open run — what closing the instance or a person's cancel does to picks still arriving.
    pub fn cancel_all(&mut self) {
        self.runs.clear();
    }

    /// 🔎️ Open runs as `(name, next_chunk, chunk_count, bytes)` — the progress census the laws read.
    pub fn open_runs(&self) -> Vec<(String, usize, usize, usize)> {
        self.runs.iter().map(|run| (run.name.clone(), run.next_chunk, run.chunk_count, run.pages.iter().map(String::len).sum())).collect()
    }
}

//#endregion 📥️ImportStaging
//#endregion 📤️FileOpenImport
//#endregion 🔖️Effect

//#region 🔖️Effect
//#region 🧵️VideoRenderJob
#[cfg(test)]
#[path = "../🧪️tests/🧵️video-render-job/🦀️.rs"]
mod video_render_job_tests;
//#endregion 🧵️VideoRenderJob
//#endregion 🔖️Effect
