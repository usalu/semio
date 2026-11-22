//! 🎥️ `shell-doc-open` staging of React's `🎥️VideoRenderHost` job surface: the `Effect::VideoRenderExport` door of the wgpu shell. A plugin that
//! requested `media.video-render` hands the shell a scene program; the shell renders it offscreen through the wgpu rasterizer, encodes and
//! muxes an MP4 under an event-sourced job (progress, cancellation), shows the same notices as React and hands the file to the save door.
//! Native only: the wasm32 shell has no blocking GPU readback and keeps its export to the TypeScript host.

use super::*;
use semio_framework::kernel::VideoRenderProgram;
use semio_framework::Severity;

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::*;
    use super::{present_media_export_bytes_cancellable, ShellDetached};
    use semio_framework_async::CancelToken;
    use semio_framework::kernel::VideoRenderJobOutcome;
    use semio_renderer_wgpu_video_render_host::{render_video_program, video_render_export_task_id, video_render_text, VideoRenderExportHost, VideoRenderExportOutcome, VideoRenderFailure, VideoRenderOutput, VideoRenderStart, VideoRenderText, VIDEO_RENDER_EXPORT_TASK_PREFIX};

    struct VideoRenderSave {
        outcome: VideoRenderExportOutcome,
        answer: ShellDetached<Result<bool, String>>,
        cancel: CancelToken,
    }

    #[derive(Default)]
    pub struct VideoRenderJobs {
        host: VideoRenderExportHost,
        rendering: Vec<(u64, ShellDetached<Result<VideoRenderOutput, VideoRenderFailure>>)>,
        saving: Vec<VideoRenderSave>,
    }

    fn video_render_asset_door(url: &str) -> Result<Vec<u8>, String> {
        Err(format!("the native renderer has no same-origin asset door for {url}"))
    }

    fn outcome_notice(chrome: &mut ChromeState, outcome: &VideoRenderExportOutcome) {
        let (status, severity) = match &outcome.outcome {
            VideoRenderJobOutcome::Done { .. } => ("done", Severity::Info),
            VideoRenderJobOutcome::Cancelled { .. } => ("cancelled", Severity::Info),
            VideoRenderJobOutcome::Refused { .. } => ("refused", Severity::Warning),
            VideoRenderJobOutcome::Failed { .. } => ("failed", Severity::Warning),
        };
        let code = format!("video-render-export.{status}");
        chrome.show_transient_notice(outcome.text.clone(), severity, Some(code.as_str()));
    }

    impl OpeningState {
        /// 🎥️ The whole shell answer to one `videoRenderExport` effect: starts a job, admits it (the plugin's declared capability, then the kernel's program rule) and renders it on a detached task, or refuses it with a notice.
        pub fn enqueue_video_render(&mut self, filename: String, owner: &str, capabilities: &[String], program: VideoRenderProgram, chrome: &mut ChromeState, is_de: bool) {
            match self.video_render.host.start(&filename, owner, program, capabilities, chrome_now_ms() as u64, is_de) {
                VideoRenderStart::Refused(outcome) => outcome_notice(chrome, &outcome),
                VideoRenderStart::Started { job, program, shared, announce } => {
                    chrome.show_transient_notice(announce, Severity::Info, Some("video-render-export.started"));
                    let render = async move { render_video_program(&program, &video_render_asset_door, &shared).await };
                    self.video_render.rendering.push((job, ShellDetached::spawn(render)));
                }
            }
        }

        /// 🫀️ One frame-pump step: folds the render loops' progress into the ledger, settles every finished render and hands its file to the save door, and shows the outcome notices.
        pub fn advance_video_render(&mut self, chrome: &mut ChromeState, is_de: bool) {
            let jobs = &mut self.video_render;
            jobs.host.sync_progress(chrome_now_ms() as u64);
            let mut index = 0;
            while index < jobs.rendering.len() {
                let Some(answer) = jobs.rendering[index].1.take() else {
                    index += 1;
                    continue;
                };
                let (job, _) = jobs.rendering.remove(index);
                let mut outcome = jobs.host.settle(job, answer, is_de);
                match outcome.bytes.take() {
                    Some(bytes) => {
                        let cancel = CancelToken::root_now();
                        let answer = ShellDetached::spawn(present_media_export_bytes_cancellable(outcome.filename.clone(), "video/mp4".into(), bytes, cancel.clone()));
                        jobs.saving.push(VideoRenderSave { outcome, answer, cancel });
                    }
                    None => outcome_notice(chrome, &outcome),
                }
            }
            let mut index = 0;
            while index < jobs.saving.len() {
                let Some(answer) = jobs.saving[index].answer.take() else {
                    index += 1;
                    continue;
                };
                let save = jobs.saving.remove(index);
                match answer {
                    Ok(true) => outcome_notice(chrome, &save.outcome),
                    Ok(false) => chrome.show_transient_notice(video_render_text(VideoRenderText::Cancelled, is_de, &[("filename", &save.outcome.filename)]), Severity::Info, Some("video-render-export.cancelled")),
                    Err(reason) => chrome.show_transient_notice(video_render_text(VideoRenderText::Failed, is_de, &[("filename", &save.outcome.filename), ("reason", &reason)]), Severity::Warning, Some("video-render-export.failed")),
                }
            }
        }

        /// 🛑️ Cancels every running export (the band's Cancel control and the accessibility Activate); how many were asked.
        pub fn cancel_video_render(&mut self) -> usize {
            for save in &self.video_render.saving {
                save.cancel.cancel_now();
            }
            self.video_render.host.cancel_all()
        }

        /// 🛑️ Cancels the export behind Task Manager id `task_id`.
        pub fn cancel_video_render_task(&mut self, task_id: &str) -> bool {
            self.video_render.host.cancel(task_id)
        }

        /// 🎛️ Takes the queued Cancel presses of video exports out of the Task Manager request queue and cancels them; the other presses stay queued.
        pub fn take_video_render_cancel_requests(&mut self) -> usize {
            let (mine, kept): (Vec<_>, Vec<_>) = core::mem::take(&mut self.runtime_requests).into_iter().partition(|request| request.kind == TaskManagerActionKind::Cancel && request.task_id.as_deref().is_some_and(|id| id.starts_with(VIDEO_RENDER_EXPORT_TASK_PREFIX)));
            self.runtime_requests = kept;
            mine.iter().filter(|request| request.task_id.as_deref().is_some_and(|id| self.video_render.host.cancel(id))).count()
        }

        /// ▶️ Whether a video export renders or waits on its save dialog.
        pub fn video_render_running(&self) -> bool {
            self.video_render.host.is_running() || !self.video_render.rendering.is_empty() || !self.video_render.saving.is_empty()
        }

        /// 🚪️ One close step: cancels every export, drops the answers that arrived and reports whether nothing is left to wait for.
        pub fn close_video_render_step(&mut self) -> bool {
            self.cancel_video_render();
            self.video_render.rendering.retain(|(_, answer)| answer.take().is_none());
            self.video_render.saving.retain(|save| save.answer.take().is_none());
            self.video_render.rendering.is_empty() && self.video_render.saving.is_empty()
        }

        /// 🗒️ The status line of the running exports for the progress band, `None` when none runs.
        pub fn video_render_message(&self, is_de: bool) -> Option<String> {
            let row = self.video_render.host.rows().first()?;
            let (completed, total) = self.video_render.host.shared_progress(row.job).unwrap_or((row.completed as u32, row.frames as u32));
            let progress = video_render_text(VideoRenderText::Progress, is_de, &[("completed", &completed.to_string()), ("total", &total.to_string())]);
            Some(format!("{progress} · {}", row.filename))
        }

        /// 📊️ `(frames done, frames in all)` of the first running export.
        pub fn video_render_counts(&self) -> (u64, u64) {
            self.video_render.host.rows().first().map_or((0, 0), |row| {
                let (completed, total) = self.video_render.host.shared_progress(row.job).unwrap_or((row.completed as u32, row.frames as u32));
                (u64::from(completed), u64::from(total))
            })
        }

        /// 🧵️ The running exports as Task Manager tasks (the ledger's fold): file, owner, frames and cancel.
        pub fn video_render_tasks(&self, is_de: bool) -> Vec<TaskManagerTask> {
            self.video_render
                .host
                .rows()
                .iter()
                .map(|row| TaskManagerTask {
                    id: video_render_export_task_id(row.job),
                    lane: TaskManagerTaskLane::Export,
                    title: row.filename.clone(),
                    owner: row.owner.clone(),
                    started_at_ms: row.started_at_ms,
                    steps: None,
                    progress: Some(TaskManagerTaskProgress { completed: row.completed, total: Some(row.frames), text: video_render_text(VideoRenderText::Progress, is_de, &[("completed", &row.completed.to_string()), ("total", &row.frames.to_string())]) }),
                    suspendable: false,
                    state: if row.cancelling { TaskManagerTaskState::Cancelling } else { TaskManagerTaskState::Running },
                })
                .collect()
        }

        /// 🏷️ The Cancel control's name.
        pub fn video_render_cancel_label(&self, is_de: bool) -> String {
            video_render_text(VideoRenderText::Cancel, is_de, &[])
        }

        /// 🧵️ Appends the running exports to the runtime tasks the Task Manager lists.
        pub fn sync_runtime_export_tasks(&mut self, is_de: bool) {
            let tasks = self.video_render_tasks(is_de);
            self.runtime_tasks.extend(tasks);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::VideoRenderJobs;

#[cfg(target_arch = "wasm32")]
#[derive(Default)]
pub struct VideoRenderJobs;

#[cfg(target_arch = "wasm32")]
impl OpeningState {
    pub fn enqueue_video_render(&mut self, filename: String, _owner: &str, _capabilities: &[String], _program: VideoRenderProgram, chrome: &mut ChromeState, _is_de: bool) {
        chrome.show_transient_notice(format!("Video export {filename} is not available in the browser renderer."), Severity::Warning, Some("video-render-export.failed"));
    }
    pub fn advance_video_render(&mut self, _chrome: &mut ChromeState, _is_de: bool) {}
    pub fn cancel_video_render(&mut self) -> usize { 0 }
    pub fn take_video_render_cancel_requests(&mut self) -> usize { 0 }
    pub fn video_render_running(&self) -> bool { false }
    pub fn close_video_render_step(&mut self) -> bool { true }
    pub fn video_render_message(&self, _is_de: bool) -> Option<String> { None }
    pub fn video_render_counts(&self) -> (u64, u64) { (0, 0) }
    pub fn video_render_cancel_label(&self, _is_de: bool) -> String { String::new() }
    pub fn sync_runtime_export_tasks(&mut self, _is_de: bool) {}
}
