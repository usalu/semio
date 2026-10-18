//! 🔚️ Canvas host staging and close: the event acknowledgement, the cursor, clipboard and request staging and the close step. See [`super`].

use super::*;

impl<P: BrowserHostPort, D: BrowserWindowDelegate> CanvasHost<P, D> {
    pub(super) fn acknowledge_event(&mut self, event: &AbiEvent) -> Result<(), BrowserHostError> {
        if self.outbound.is_some() {
            return Err(BrowserHostError::Busy);
        }
        self.outbound = Some(AbiMessage::Reply(AbiReply { request_id: event.request_id, generation: event.generation, status: semio_framework_abi::AbiStatus::OK, bytes: AbiBytes::default() }));
        self.outbound_inspected_bytes = 0;
        Ok(())
    }

    pub(super) fn stage_cursor(&mut self, cursor: CursorRequest) -> Result<(), BrowserHostError> {
        if self.last_cursor == Some(cursor) {
            return Ok(());
        }
        self.last_cursor = Some(cursor);
        let request_id = self.next_request();
        let mut bytes = canvas_payload(self.canvas);
        bytes.push(cursor_tag(cursor));
        self.stage_request(request(BROWSER_HOST_OPERATION_CURSOR, request_id, self.generation, bytes)?)
    }

    pub(super) fn stage_clipboard(&mut self, operation: ClipboardOperation, text: Vec<u8>) -> Result<AbiRequestId, BrowserHostError> {
        if self.closing {
            return Err(BrowserHostError::Closed);
        }
        if self.clipboard.is_some() || self.outbound.is_some() {
            return Err(BrowserHostError::Busy);
        }
        let request_id = self.next_request();
        let mut bytes = canvas_payload(self.canvas);
        bytes.extend_from_slice(&text);
        let code = if operation == ClipboardOperation::Read { BROWSER_HOST_OPERATION_CLIPBOARD_READ } else { BROWSER_HOST_OPERATION_CLIPBOARD_WRITE };
        let request = request(code, request_id, self.generation, bytes)?;
        self.replies.admit(request_id, self.generation)?;
        self.outbound = Some(AbiMessage::Request(request));
        self.outbound_inspected_bytes = 0;
        self.clipboard = Some(ClipboardRequest { request_id, generation: self.generation, operation, text });
        Ok(request_id)
    }

    pub(super) fn stage_request(&mut self, request: AbiRequest) -> Result<(), BrowserHostError> {
        if self.outbound.is_some() {
            return Err(BrowserHostError::Busy);
        }
        self.replies.admit(request.request_id, request.generation)?;
        self.outbound = Some(AbiMessage::Request(request));
        self.outbound_inspected_bytes = 0;
        Ok(())
    }

    pub(super) fn next_request(&mut self) -> AbiRequestId {
        let request = AbiRequestId(self.next_request_id);
        self.next_request_id = self.next_request_id.checked_add(1).unwrap_or(1);
        request
    }

    pub(super) fn close_step(&mut self, budget: AbiWorkBudget) -> Result<BrowserHostStep, BrowserHostError> {
        if let Some(request) = self.clipboard.take() {
            self.replies.lose(request.request_id, request.generation)?;
            self.outbound = Some(AbiMessage::Control(AbiControl::Cancel { request_id: request.request_id, generation: request.generation }));
            self.outbound_inspected_bytes = 0;
            return Ok(progress(1, 3));
        }
        if self.outbound.is_some() {
            let total = message_body_bytes(self.outbound.as_ref().expect("checked")).max(1);
            self.outbound_inspected_bytes += 1;
            if self.outbound_inspected_bytes < total {
                return Ok(progress(self.outbound_inspected_bytes as u32, total as u32));
            }
            let message = self.outbound.take().expect("checked");
            self.port.try_send(message, one_credit(budget)).map_err(|rejection| {
                self.outbound = Some(rejection.message);
                self.outbound_inspected_bytes = total.saturating_sub(1);
                BrowserHostError::Abi(rejection.code)
            })?;
            self.outbound_inspected_bytes = 0;
            return Ok(progress(1, 2));
        }
        if let Some(pending) = self.pending_reply.as_mut() {
            pending.inspected_bytes += 1;
            let total = pending.reply.bytes.len().max(1);
            if pending.inspected_bytes < total {
                return Ok(progress(pending.inspected_bytes as u32, total as u32));
            }
            let pending = self.pending_reply.take().expect("checked");
            return self.accept_reply(pending.reply);
        }
        if let Some(pending) = self.pending_event.as_mut() {
            pending.inspected_bytes += 1;
            let total = pending.event.bytes.len().max(1);
            if pending.inspected_bytes < total {
                return Ok(progress(pending.inspected_bytes as u32, total as u32));
            }
            let event = self.pending_event.take().expect("checked").event;
            self.acknowledge_event(&event)?;
            return Ok(progress(total as u32, total as u32));
        }
        if let Some(pending) = self.pending_page.as_mut() {
            pending.inspected_bytes += 1;
            let total = pending.page.bytes.len().max(1);
            if pending.inspected_bytes < total {
                return Ok(progress(pending.inspected_bytes as u32, total as u32));
            }
            let page = self.pending_page.take().expect("checked").page;
            self.outbound = Some(AbiMessage::Control(AbiControl::Acknowledge { handle: page.handle, index: page.index }));
            self.outbound_inspected_bytes = 0;
            return Ok(progress(total as u32, total as u32));
        }
        if let Some((event, _)) = self.pending_dispatch.take() {
            self.acknowledge_event(&event)?;
            return Ok(progress(1, 1));
        }
        if let Some((event, _)) = self.latest_metrics.take() {
            self.acknowledge_event(&event)?;
            return Ok(progress(1, 1));
        }
        if let Some((event, _)) = self.latest_pointer.take() {
            self.acknowledge_event(&event)?;
            return Ok(progress(1, 1));
        }
        if let Some((event, _)) = self.pending_frame.take() {
            self.acknowledge_event(&event)?;
            return Ok(progress(1, 1));
        }
        if let Some(listener) = self.listener.take() {
            self.outbound = Some(AbiMessage::Control(AbiControl::Close { handle: listener.handle() }));
            self.outbound_inspected_bytes = 0;
            return Ok(progress(1, 2));
        }
        if !self.detach_sent {
            let request_id = self.next_request();
            let detach = request(BROWSER_HOST_OPERATION_DETACH, request_id, self.generation, canvas_payload(self.canvas))?;
            self.replies.admit(request_id, self.generation)?;
            self.outbound = Some(AbiMessage::Request(detach));
            self.outbound_inspected_bytes = 0;
            self.detach_sent = true;
            self.detach_request = Some(request_id);
            self.pending_cursor = None;
            return Ok(progress(1, 1));
        }
        if self.detach_acknowledged {
            return Ok(BrowserHostStep::Closed);
        }
        match self.port.poll(one_credit(budget))? {
            AbiPortPoll::Pending => Ok(BrowserHostStep::AwaitingHost),
            AbiPortPoll::Closed => Err(BrowserHostError::Closed),
            AbiPortPoll::Message(AbiMessage::Event(event)) => {
                if event.generation != self.generation || event.bytes.len() > BROWSER_HOST_MAX_EVENT_BODY_BYTES {
                    let code = if event.generation < self.generation { AbiErrorCode::AbaHandle } else { AbiErrorCode::StaleGeneration };
                    return Err(BrowserHostError::Abi(code));
                }
                self.pending_event = Some(PendingEvent { event, inspected_bytes: 0 });
                Ok(progress(1, 1))
            }
            AbiPortPoll::Message(AbiMessage::Reply(reply)) => {
                self.pending_reply = Some(PendingReply { reply, inspected_bytes: 0 });
                Ok(progress(1, 1))
            }
            AbiPortPoll::Message(AbiMessage::Page(page)) => {
                if page.bytes.len() > BROWSER_HOST_MAX_PAGE_BODY_BYTES {
                    return Err(BrowserHostError::Abi(AbiErrorCode::LimitExceeded));
                }
                self.pending_page = Some(PendingPage { page, inspected_bytes: 0 });
                Ok(progress(1, 1))
            }
            AbiPortPoll::Message(AbiMessage::Control(AbiControl::Close { .. })) => Ok(progress(1, 1)),
            AbiPortPoll::Message(_) => Err(BrowserHostError::Abi(AbiErrorCode::MalformedTag)),
        }
    }
}
