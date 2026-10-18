//! 🎞️ Canvas host lifecycle: the `CanvasHost` state, its construction, wake and clipboard requests, the close start and the delegate access. See [`super`].

use super::*;

/// 🌐️ Browser state machine over an owned A1 port.
pub struct CanvasHost<P: BrowserHostPort, D: BrowserWindowDelegate> {
    pub(super) canvas: CanvasId,
    pub(super) generation: u32,
    pub(super) port: P,
    pub(super) delegate: D,
    pub(super) replies: AbiReplyLedger,
    pub(super) listener: Option<ListenerId>,
    pub(super) outbound: Option<AbiMessage>,
    pub(super) outbound_inspected_bytes: usize,
    pub(super) pending_event: Option<PendingEvent>,
    pub(super) pending_reply: Option<PendingReply>,
    pub(super) pending_page: Option<PendingPage>,
    pub(super) latest_metrics: Option<(AbiEvent, WindowMetrics)>,
    pub(super) latest_pointer: Option<(AbiEvent, DispatchEvent)>,
    pub(super) pending_dispatch: Option<(AbiEvent, DispatchEvent)>,
    pub(super) pending_frame: Option<(AbiEvent, FrameSchedulerEnvelope)>,
    pub(super) clipboard: Option<ClipboardRequest>,
    pub(super) clipboard_mailbox: BrowserClipboard,
    pub(super) last_cursor: Option<CursorRequest>,
    pub(super) pending_cursor: Option<CursorRequest>,
    pub(super) frame_pending: bool,
    pub(super) next_request_id: u64,
    pub(super) closing: bool,
    pub(super) detach_sent: bool,
    pub(super) detach_request: Option<AbiRequestId>,
    pub(super) detach_acknowledged: bool,
    _ui_token: crate::enqueue::UiThreadToken,
}

impl<P: BrowserHostPort, D: BrowserWindowDelegate> CanvasHost<P, D> {
    pub fn new(canvas: CanvasId, port: P, delegate: D) -> Result<Self, BrowserHostError> {
        let request_id = AbiRequestId(canvas.get() as u64);
        let attach = request(BROWSER_HOST_OPERATION_ATTACH, request_id, 1, canvas_payload(canvas))?;
        let mut replies = AbiReplyLedger::new();
        replies.admit(request_id, 1)?;
        Ok(Self {
            canvas,
            generation: 1,
            port,
            delegate,
            replies,
            listener: None,
            outbound: Some(AbiMessage::Request(attach)),
            outbound_inspected_bytes: 0,
            pending_event: None,
            pending_reply: None,
            pending_page: None,
            latest_metrics: None,
            latest_pointer: None,
            pending_dispatch: None,
            pending_frame: None,
            clipboard: None,
            clipboard_mailbox: BrowserClipboard::default(),
            last_cursor: None,
            pending_cursor: None,
            frame_pending: false,
            next_request_id: canvas.get() as u64 + 1,
            closing: false,
            detach_sent: false,
            detach_request: None,
            detach_acknowledged: false,
            _ui_token: crate::enqueue::UiThreadToken::mint(),
        })
    }

    pub fn request_wake(&mut self) -> Result<bool, BrowserHostError> {
        if self.closing {
            return Err(BrowserHostError::Closed);
        }
        if self.frame_pending {
            return Ok(false);
        }
        let request_id = self.next_request();
        let frame = request(BROWSER_HOST_OPERATION_FRAME, request_id, self.generation, canvas_payload(self.canvas))?;
        self.stage_request(frame)?;
        self.frame_pending = true;
        Ok(true)
    }

    pub fn request_clipboard_read(&mut self) -> Result<AbiRequestId, BrowserHostError> {
        self.stage_clipboard(ClipboardOperation::Read, Vec::new())
    }

    pub fn request_clipboard_write(&mut self, text: String) -> Result<AbiRequestId, BrowserHostError> {
        self.clipboard_mailbox.write_text(&text);
        self.stage_clipboard(ClipboardOperation::Write, text.into_bytes())
    }

    pub fn cancel_clipboard(&mut self) -> Result<bool, BrowserHostError> {
        let Some(request) = self.clipboard.take() else { return Ok(false) };
        if let Some(message) = self.outbound.take() {
            if !matches!(&message, AbiMessage::Request(value) if value.request_id == request.request_id) {
                self.outbound = Some(message);
                self.clipboard = Some(request);
                return Err(BrowserHostError::Busy);
            }
            self.replies.lose(request.request_id, request.generation)?;
            return Ok(true);
        }
        self.replies.lose(request.request_id, request.generation)?;
        self.outbound = Some(AbiMessage::Control(AbiControl::Cancel { request_id: request.request_id, generation: request.generation }));
        self.outbound_inspected_bytes = 0;
        Ok(true)
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.detach_sent
            && self.detach_acknowledged
            && self.listener.is_none()
            && self.outbound.is_none()
            && self.pending_event.is_none()
            && self.pending_reply.is_none()
            && self.pending_page.is_none()
            && self.latest_metrics.is_none()
            && self.latest_pointer.is_none()
            && self.pending_dispatch.is_none()
            && self.pending_frame.is_none()
            && self.clipboard.is_none()
            && self.pending_cursor.is_none()
    }

    pub fn delegate(&self) -> &D {
        &self.delegate
    }

    pub fn delegate_mut(&mut self) -> &mut D {
        &mut self.delegate
    }

}
