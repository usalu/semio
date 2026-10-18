//! ⏩️ Canvas host step and events: the host step, the message and reply acceptance and the event classification. See [`super`].

use super::*;

impl<P: BrowserHostPort, D: BrowserWindowDelegate> CanvasHost<P, D> {
    pub fn step(&mut self, budget: AbiWorkBudget) -> Result<BrowserHostStep, BrowserHostError> {
        validate_budget(budget)?;
        if self.closing {
            return self.close_step(budget);
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
            return Ok(progress(1, 1));
        }
        if let Some(cursor) = self.pending_cursor.take() {
            self.stage_cursor(cursor)?;
            return Ok(progress(1, 1));
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
        if let Some((event, dispatch)) = self.pending_dispatch.take() {
            self.delegate.handle_event(dispatch);
            self.acknowledge_event(&event)?;
            return Ok(BrowserHostStep::Event);
        }
        if self.pending_frame.is_some() && self.latest_metrics.is_some() {
            let (event, metrics) = self.latest_metrics.take().expect("checked");
            self.delegate.handle_metrics(metrics);
            self.acknowledge_event(&event)?;
            return Ok(BrowserHostStep::Event);
        }
        if self.pending_frame.is_some() && self.latest_pointer.is_some() {
            let (event, dispatch) = self.latest_pointer.take().expect("checked");
            self.delegate.handle_event(dispatch);
            self.acknowledge_event(&event)?;
            return Ok(BrowserHostStep::Event);
        }
        if let Some((event, frame)) = self.pending_frame.take() {
            self.frame_pending = false;
            if let Some(reason) = should_request_redraw(self.delegate.scheduler_mut(), frame.timestamp_ms / 1_000.0) {
                self.delegate.enqueue_browser_frame(reason);
                let outcome = self.delegate.present_browser_frame();
                if self.last_cursor != Some(outcome.cursor) {
                    self.pending_cursor = Some(outcome.cursor);
                }
            }
            self.acknowledge_event(&event)?;
            return Ok(BrowserHostStep::Frame(frame));
        }
        if let Some(pending) = self.pending_event.as_mut() {
            pending.inspected_bytes += 1;
            let total = pending.event.bytes.len().max(1);
            if pending.inspected_bytes < total {
                return Ok(progress(pending.inspected_bytes as u32, total as u32));
            }
            let pending = self.pending_event.take().expect("checked");
            self.classify_event(pending.event)?;
            return Ok(progress(total as u32, total as u32));
        }
        match self.port.poll(one_credit(budget))? {
            AbiPortPoll::Pending => {
                if let Some((event, metrics)) = self.latest_metrics.take() {
                    self.delegate.handle_metrics(metrics);
                    self.acknowledge_event(&event)?;
                    Ok(BrowserHostStep::Event)
                } else if let Some((event, dispatch)) = self.latest_pointer.take() {
                    self.delegate.handle_event(dispatch);
                    self.acknowledge_event(&event)?;
                    Ok(BrowserHostStep::Event)
                } else {
                    Ok(BrowserHostStep::AwaitingHost)
                }
            }
            AbiPortPoll::Closed => {
                self.begin_close();
                Ok(BrowserHostStep::Closed)
            }
            AbiPortPoll::Message(message) => self.accept_message(message),
        }
    }

    fn accept_message(&mut self, message: AbiMessage) -> Result<BrowserHostStep, BrowserHostError> {
        match message {
            AbiMessage::Event(event) => {
                if event.generation != self.generation || event.bytes.len() > BROWSER_HOST_MAX_EVENT_BODY_BYTES {
                    let code = if event.generation < self.generation { AbiErrorCode::AbaHandle } else { AbiErrorCode::StaleGeneration };
                    return Err(BrowserHostError::Abi(code));
                }
                self.pending_event = Some(PendingEvent { event, inspected_bytes: 0 });
                Ok(progress(1, 1))
            }
            AbiMessage::Reply(reply) => {
                self.pending_reply = Some(PendingReply { reply, inspected_bytes: 0 });
                Ok(progress(1, 1))
            }
            AbiMessage::Page(page) => {
                if page.bytes.len() > BROWSER_HOST_MAX_PAGE_BODY_BYTES {
                    return Err(BrowserHostError::Abi(AbiErrorCode::LimitExceeded));
                }
                self.pending_page = Some(PendingPage { page, inspected_bytes: 0 });
                Ok(progress(1, 1))
            }
            AbiMessage::Control(AbiControl::Close { handle }) => {
                let listener = self.listener.ok_or(BrowserHostError::Unavailable(BrowserHostUnavailable::Listener))?;
                validate_listener_handle(listener, handle)?;
                self.begin_close();
                Ok(BrowserHostStep::Closed)
            }
            _ => Err(BrowserHostError::Abi(AbiErrorCode::MalformedTag)),
        }
    }

    pub(super) fn accept_reply(&mut self, reply: AbiReply) -> Result<BrowserHostStep, BrowserHostError> {
        self.replies.accept(&reply)?;
        if self.detach_request == Some(reply.request_id) {
            self.detach_request = None;
            self.detach_acknowledged = true;
            return if reply.status.code == AbiStatusCode::Ok { Ok(BrowserHostStep::Closed) } else { Ok(BrowserHostStep::Unavailable(unavailable_from_reply(&reply))) };
        }
        if reply.status.code != AbiStatusCode::Ok {
            let unavailable = unavailable_from_reply(&reply);
            if self.clipboard.as_ref().is_some_and(|request| request.request_id == reply.request_id) {
                self.clipboard = None;
            }
            return Ok(BrowserHostStep::Unavailable(unavailable));
        }
        if self.listener.is_none() && reply.request_id == AbiRequestId(self.canvas.get() as u64) {
            self.listener = Some(parse_listener(reply.bytes.as_slice())?);
            return Ok(BrowserHostStep::Event);
        }
        if self.clipboard.as_ref().is_some_and(|request| request.request_id == reply.request_id) {
            let request = self.clipboard.take().expect("checked");
            let text = if request.operation == ClipboardOperation::Read {
                let text = String::from_utf8(reply.bytes.into_vec()).map_err(|_| BrowserHostError::Abi(AbiErrorCode::InvalidUtf8))?;
                self.clipboard_mailbox.settle_read(text.clone());
                self.delegate.handle_event(DispatchEvent::Paste { text: text.clone() });
                Some(text)
            } else {
                None
            };
            return Ok(BrowserHostStep::Clipboard { request_id: request.request_id, text });
        }
        Ok(BrowserHostStep::Event)
    }

    fn classify_event(&mut self, event: AbiEvent) -> Result<(), BrowserHostError> {
        let listener = self.listener.ok_or(BrowserHostError::Unavailable(BrowserHostUnavailable::Listener))?;
        match decode_browser_host_event(&event, self.canvas, listener)? {
            BrowserHostEvent::Metrics(metrics) => {
                if let Some((replaced, _)) = self.latest_metrics.replace((event, WindowMetrics { physical: PhysicalSize::new(metrics.width, metrics.height), scale_factor: metrics.scale_factor })) {
                    self.acknowledge_event(&replaced)?;
                }
            }
            BrowserHostEvent::Visibility { visible } => {
                self.delegate.scheduler_mut().set_visible(visible);
                self.acknowledge_event(&event)?;
            }
            BrowserHostEvent::Frame { timestamp_ms } => {
                self.pending_frame = Some((event, FrameSchedulerEnvelope { canvas: self.canvas, generation: self.generation, timestamp_ms }));
            }
            BrowserHostEvent::Dispatch(dispatch) => {
                if matches!(dispatch, DispatchEvent::PointerMove { .. }) {
                    if let Some((replaced, _)) = self.latest_pointer.replace((event, dispatch)) {
                        self.acknowledge_event(&replaced)?;
                    }
                } else {
                    self.pending_dispatch = Some((event, dispatch));
                }
            }
            BrowserHostEvent::Closed => self.begin_close(),
        }
        Ok(())
    }

}
