//! 🔌️ The local bootstrap transport port over the inherited pipe and the request and transport controls.

use super::*;

impl LocalBootstrapTransport for InheritedLocalBootstrapTransport {
    fn run_id(&self) -> &str {
        &self.run_id
    }

    fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Acquire) && !self.shutdown.load(Ordering::Acquire)
    }

    fn request_cancelled(&self, request_id: &str) -> bool {
        self.shutdown.load(Ordering::Acquire) || self.cancelled.lock().map_or(true, |cancelled| cancelled.contains(request_id))
    }

    fn accept<'a>(&'a self, control: &'a dyn IdentityVerificationControl) -> LocalBootstrapAcceptFuture<'a> {
        Box::pin(async move { self.accept_next(control).await })
    }

    fn issue<'a>(&'a self, request: &'a VerifiedLocalBootstrapRequest, session: &'a IssuedAuthSession, context: &'a IdentityVerificationContext<'a>) -> LocalBootstrapIssueFuture<'a> {
        Box::pin(async move { self.issue_response(request, session, context).await })
    }

    fn reject<'a>(&'a self, request_id: &'a str, code: LocalBootstrapRejectCode, context: &'a IdentityVerificationContext<'a>) -> LocalBootstrapTerminalFuture<'a> {
        Box::pin(async move { self.reject_response(request_id, code, context).await })
    }

    fn cancel<'a>(&'a self, request_id: &'a str) -> LocalBootstrapTerminalFuture<'a> {
        Box::pin(async move {
            self.cancel_pending(request_id);
            Ok(())
        })
    }

    fn closed<'a>(&'a self) -> LocalBootstrapTerminalFuture<'a> {
        Box::pin(async move {
            self.closed.wait().await;
            Ok(())
        })
    }

    fn shutdown<'a>(&'a self) -> LocalBootstrapTerminalFuture<'a> {
        Box::pin(async move {
            self.shutdown.store(true, Ordering::Release);
            self.ready.store(false, Ordering::Release);
            Ok(())
        })
    }
}

pub(super) struct RequestControl {
    pub(super) base: Arc<dyn IdentityVerificationControl>,
    pub(super) transport: Arc<dyn LocalBootstrapTransport>,
    pub(super) request_id: String,
}

pub(super) struct TransportControl<'a> {
    pub(super) base: &'a dyn IdentityVerificationControl,
    pub(super) shutdown: &'a AtomicBool,
}

impl IdentityVerificationControl for TransportControl<'_> {
    fn now_ms(&self) -> i64 {
        self.base.now_ms()
    }

    fn is_cancelled(&self) -> bool {
        self.base.is_cancelled() || self.shutdown.load(Ordering::Acquire)
    }

    fn report(&self, progress: semio_hub_directory_contract::IdentityVerificationProgress) {
        self.base.report(progress);
    }
}

impl IdentityVerificationControl for RequestControl {
    fn now_ms(&self) -> i64 {
        self.base.now_ms()
    }

    fn is_cancelled(&self) -> bool {
        self.base.is_cancelled() || self.transport.request_cancelled(&self.request_id)
    }

    fn report(&self, progress: semio_hub_directory_contract::IdentityVerificationProgress) {
        self.base.report(progress);
    }
}
