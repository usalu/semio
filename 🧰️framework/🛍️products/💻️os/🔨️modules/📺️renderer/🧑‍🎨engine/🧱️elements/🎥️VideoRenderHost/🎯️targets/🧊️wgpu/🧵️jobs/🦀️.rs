//! 🧵️ The event-sourced job state of the native video export host: the Rust twin of the `🧵️Jobs` region of `🎥️VideoRenderHost/🟦️.ts`.
//! The host appends [`VideoRenderJobEvent`]s to one log and the task list is the kernel ledger's fold of it; a cancel appends
//! `CancelRequested` and flips the shared flag the render loop reads before its next frame. The shell owns the future that renders.

use super::labels::{video_render_text, VideoRenderText};
use super::render::{VideoRenderFailure, VideoRenderOutput, VideoRenderShared};
use semio_framework_kernel_media_jobs::{VideoRenderEncoderTier, VideoRenderJobEvent, VideoRenderJobLedger, VideoRenderJobOutcome, VideoRenderJobRow, VideoRenderProgram, MEDIA_VIDEO_RENDER_CAPABILITY};
use std::collections::VecDeque;
use std::sync::Arc;

/// 🏷️ The prefix of every Task Manager id this host owns.
pub const VIDEO_RENDER_EXPORT_TASK_PREFIX: &str = "export:video#";

/// ⏱️ The shortest gap between two `Progressed` events of one job; the last frame always reports.
pub const VIDEO_RENDER_PROGRESS_EVENT_MS: u64 = 100;

/// 🧵️ How many job events the host keeps readable after the ledger folded them.
pub const VIDEO_RENDER_JOB_LOG_TAIL: usize = 256;

/// 🏷️ The Task Manager id of job `job`.
pub fn video_render_export_task_id(job: u64) -> String {
    format!("{VIDEO_RENDER_EXPORT_TASK_PREFIX}{job}")
}

/// 🏁️ How an export ended, with the sentence the shell shows for it and, when it succeeded, the file to deliver.
#[derive(Debug)]
pub struct VideoRenderExportOutcome {
    pub job: u64,
    pub outcome: VideoRenderJobOutcome,
    pub text: String,
    pub filename: String,
    pub bytes: Option<Vec<u8>>,
}

/// 📮️ What the host asks of its shell for one request.
#[derive(Debug)]
pub enum VideoRenderStart {
    Refused(VideoRenderExportOutcome),
    Started { job: u64, program: VideoRenderProgram, shared: Arc<VideoRenderShared>, announce: String },
}

struct Running {
    job: u64,
    filename: String,
    shared: Arc<VideoRenderShared>,
    reported_ms: Option<u64>,
}

/// 📒️ Every export of this shell: the ledger, the readable log tail and the shared state of each running render.
#[derive(Default)]
pub struct VideoRenderExportHost {
    ledger: VideoRenderJobLedger,
    log: VecDeque<VideoRenderJobEvent>,
    running: Vec<Running>,
}

impl VideoRenderExportHost {
    fn record(&mut self, event: VideoRenderJobEvent) {
        if self.ledger.apply(&event).is_err() {
            return;
        }
        self.log.push_back(event);
        while self.log.len() > VIDEO_RENDER_JOB_LOG_TAIL {
            self.log.pop_front();
        }
    }

    fn finish(&mut self, job: u64, outcome: VideoRenderJobOutcome, text: String, filename: String, bytes: Option<Vec<u8>>) -> VideoRenderExportOutcome {
        self.record(VideoRenderJobEvent::Finished { job, outcome: outcome.clone() });
        self.running.retain(|running| running.job != job);
        VideoRenderExportOutcome { job, outcome, text, filename, bytes }
    }

    /// 🚦️ Starts a job for `filename` asked by `owner`: admits it (the plugin's declared [`MEDIA_VIDEO_RENDER_CAPABILITY`], then the
    /// kernel's program rule) or finishes it refused.
    pub fn start(&mut self, filename: &str, owner: &str, program: VideoRenderProgram, capabilities: &[String], now_ms: u64, is_de: bool) -> VideoRenderStart {
        let job = self.ledger.last_job() + 1;
        let frames = program.frame_count();
        self.record(VideoRenderJobEvent::Started { job, owner: owner.into(), filename: filename.into(), frames, at_ms: now_ms });
        let filename_text = filename.to_string();
        if !capabilities.iter().any(|capability| capability == MEDIA_VIDEO_RENDER_CAPABILITY) {
            let text = video_render_text(VideoRenderText::RefusedCapability, is_de, &[("filename", filename), ("owner", owner), ("capability", MEDIA_VIDEO_RENDER_CAPABILITY)]);
            return VideoRenderStart::Refused(self.finish(job, VideoRenderJobOutcome::Refused { code: "capability".into() }, text, filename_text, None));
        }
        if let Err(problem) = program.validate() {
            let text = video_render_text(VideoRenderText::Refused, is_de, &[("filename", filename), ("code", problem.code())]);
            return VideoRenderStart::Refused(self.finish(job, VideoRenderJobOutcome::Refused { code: problem.code().into() }, text, filename_text, None));
        }
        let shared = VideoRenderShared::new(frames as u32);
        self.running.push(Running { job, filename: filename_text, shared: shared.clone(), reported_ms: None });
        VideoRenderStart::Started { job, program, shared, announce: video_render_text(VideoRenderText::Started, is_de, &[("filename", filename)]) }
    }

    /// 📈️ Folds the frames the render loops reported since the last call into `Progressed` events, at most one per
    /// [`VIDEO_RENDER_PROGRESS_EVENT_MS`] per job and always for the last frame.
    pub fn sync_progress(&mut self, now_ms: u64) {
        let mut events = Vec::new();
        for running in &mut self.running {
            let (completed, total) = running.shared.progress();
            let known = self.ledger.row(running.job).map_or(0, |row| row.completed);
            let due = running.reported_ms.is_none_or(|at| now_ms.saturating_sub(at) >= VIDEO_RENDER_PROGRESS_EVENT_MS);
            if u64::from(completed) > known && (completed == total || due) {
                running.reported_ms = Some(now_ms);
                events.push(VideoRenderJobEvent::Progressed { job: running.job, completed: u64::from(completed) });
            }
        }
        for event in events {
            self.record(event);
        }
    }

    /// 🛑️ The cancel command: appends `CancelRequested` for the job behind Task Manager id `task_id` and flips its shared flag;
    /// `false` when no such job runs or it is already cancelling.
    pub fn cancel(&mut self, task_id: &str) -> bool {
        let Some(job) = task_id.strip_prefix(VIDEO_RENDER_EXPORT_TASK_PREFIX).and_then(|job| job.parse::<u64>().ok()) else { return false };
        if self.ledger.row(job).is_none_or(|row| row.cancelling) {
            return false;
        }
        self.record(VideoRenderJobEvent::CancelRequested { job });
        if let Some(running) = self.running.iter().find(|running| running.job == job) {
            running.shared.cancel();
        }
        true
    }

    /// 🛑️ Cancels every running job; how many were asked.
    pub fn cancel_all(&mut self) -> usize {
        let ids: Vec<String> = self.ledger.running().iter().map(|row| video_render_export_task_id(row.job)).collect();
        ids.iter().filter(|id| self.cancel(id)).count()
    }

    /// 🏁️ Settles job `job` with the render loop's answer; never fails — every failure is a finished job with a stated outcome.
    pub fn settle(&mut self, job: u64, result: Result<VideoRenderOutput, VideoRenderFailure>, is_de: bool) -> VideoRenderExportOutcome {
        let filename = self.running.iter().find(|running| running.job == job).map(|running| running.filename.clone()).unwrap_or_default();
        match result {
            Ok(output) => {
                let text = video_render_text(VideoRenderText::Done, is_de, &[("filename", &filename), ("frames", &output.frames.to_string())]);
                let outcome = VideoRenderJobOutcome::Done { bytes: output.bytes.len() as u64, tier: VideoRenderEncoderTier::FirstParty };
                self.finish(job, outcome, text, filename, Some(output.bytes))
            }
            Err(VideoRenderFailure::Cancelled { completed }) => {
                let text = video_render_text(VideoRenderText::Cancelled, is_de, &[("filename", &filename)]);
                self.finish(job, VideoRenderJobOutcome::Cancelled { completed: u64::from(completed) }, text, filename, None)
            }
            Err(failure) => {
                let reason = failure.reason();
                let text = video_render_text(VideoRenderText::Failed, is_de, &[("filename", &filename), ("reason", &reason)]);
                self.finish(job, VideoRenderJobOutcome::Failed { reason }, text, filename, None)
            }
        }
    }

    /// 🧵️ Every running job, in start order (the ledger's fold).
    pub fn rows(&self) -> &[VideoRenderJobRow] {
        self.ledger.running()
    }

    /// 📜️ The most recent job events, oldest first.
    pub fn log(&self) -> impl Iterator<Item = &VideoRenderJobEvent> {
        self.log.iter()
    }

    /// ▶️ Whether any export is running.
    pub fn is_running(&self) -> bool {
        !self.ledger.running().is_empty()
    }

    /// 📈️ The frames the render loop of `job` has reported so far.
    pub fn shared_progress(&self, job: u64) -> Option<(u32, u32)> {
        self.running.iter().find(|running| running.job == job).map(|running| running.shared.progress())
    }
}
