//! 🔐️ Bounded process-wide publication admission, specified by `🧬️schema/🔣️.json`.

use crate::{CancelState, CancelToken};
use std::sync::atomic::Ordering;
use std::sync::{Mutex, MutexGuard, TryLockError};

static PUBLICATION_GATE: Mutex<()> = Mutex::new(());
const TOKEN_NODE_CEILING: u32 = 4096;

/// 📏️ Explicit finite authority for scheduler-owned admission attempts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PublicationLimits {
    pub max_attempts: u32,
    pub max_token_nodes: u32,
}

/// 📊️ One admission turn observed before the gate is touched.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PublicationProgress {
    pub attempt: u32,
    pub max_attempts: u32,
}

/// 🔔️ Caller-owned progress reporting before any publication lock survives.
pub trait PublicationObserver {
    fn on_attempt(&mut self, progress: PublicationProgress);
}

/// 🧮️ The exact finite authority exhausted by an admission turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PublicationBound {
    Attempts,
    TokenNodes,
}

/// 🚫️ Typed admission refusal without inferred retry or cancellation policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PublicationError {
    InvalidLimits,
    Busy,
    Cancelled,
    Parked,
    BoundExceeded(PublicationBound),
    Unavailable,
}

impl std::fmt::Display for PublicationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::InvalidLimits => "publication admission limits are invalid",
            Self::Busy => "publication admission is busy",
            Self::Cancelled => "publication admission is cancelled",
            Self::Parked => "publication admission is parked",
            Self::BoundExceeded(PublicationBound::Attempts) => "publication admission attempt bound exceeded",
            Self::BoundExceeded(PublicationBound::TokenNodes) => "publication cancellation ancestry bound exceeded",
            Self::Unavailable => "publication admission is unavailable after writer panic",
        })
    }
}

impl std::error::Error for PublicationError {}

/// 🧷️ The opaque non-Send owner of the single process-wide publication barrier.
///
/// ```compile_fail,E0277
/// use semio_framework_async::publication::PublicationTransaction;
/// fn require_send<T: Send>() {}
/// require_send::<PublicationTransaction<'_>>();
/// ```
///
/// ```compile_fail,E0616
/// use semio_framework_async::publication::PublicationTransaction;
/// fn expose(transaction: PublicationTransaction<'_>) { let _ = transaction._guard; }
/// ```
///
/// ```
/// use semio_framework_async::{CancelToken, publication::{PublicationControl, PublicationLimits, PublicationObserver, PublicationProgress, PublicationTransaction}};
/// struct Observer;
/// impl PublicationObserver for Observer { fn on_attempt(&mut self, _: PublicationProgress) {} }
/// fn begin(token: &CancelToken) -> PublicationTransaction<'_> {
///     let mut observer = Observer;
///     let mut control = PublicationControl::new(PublicationLimits { max_attempts: 1, max_token_nodes: 1 }, token, &mut observer).unwrap();
///     control.try_begin().unwrap()
/// }
/// let token = CancelToken::root_now();
/// let transaction = begin(&token);
/// transaction.checkpoint().unwrap();
/// drop(transaction);
/// println!("[DEBUG] Publication transaction retains token after its progress observer ends");
/// ```
///
/// ```compile_fail,E0515
/// use semio_framework_async::{CancelToken, publication::{PublicationControl, PublicationLimits, PublicationObserver, PublicationProgress, PublicationTransaction}};
/// struct Observer;
/// impl PublicationObserver for Observer { fn on_attempt(&mut self, _: PublicationProgress) {} }
/// fn escape<'a>() -> PublicationTransaction<'a> {
///     let token = CancelToken::root_now();
///     let mut observer = Observer;
///     let mut control = PublicationControl::new(PublicationLimits { max_attempts: 1, max_token_nodes: 1 }, &token, &mut observer).unwrap();
///     control.try_begin().unwrap()
/// }
/// ```
#[must_use]
pub struct PublicationTransaction<'token> {
    _guard: MutexGuard<'static, ()>,
    token: &'token CancelToken,
    max_token_nodes: u32,
}

impl PublicationTransaction<'_> {
    /// 🛂️ Checks the admitted token without consuming a turn or releasing the publication barrier.
    pub fn checkpoint(&self) -> Result<(), PublicationError> {
        checkpoint(self.token, self.max_token_nodes)
    }
}

fn checkpoint(token: &CancelToken, max_token_nodes: u32) -> Result<(), PublicationError> {
        let mut node = Some(token);
        let mut visited = 0;
        let mut effective = CancelState::Live;
        while let Some(current) = node {
            if visited == max_token_nodes {
                return Err(PublicationError::BoundExceeded(PublicationBound::TokenNodes));
            }
            visited += 1;
            let local = match current.0.local.load(Ordering::SeqCst) {
                0 => CancelState::Live,
                1 => CancelState::Park,
                _ => return Err(PublicationError::Cancelled),
            };
            effective = effective.max(local);
            node = current.0.parent.as_ref();
        }
        if effective == CancelState::Park {
            return Err(PublicationError::Parked);
        }
        Ok(())
    }

/// 🎛️ Mandatory cancellation, progress and cumulative finite admission authority.
pub struct PublicationControl<'token, 'observer, Observer: PublicationObserver + ?Sized> {
    limits: PublicationLimits,
    token: &'token CancelToken,
    observer: &'observer mut Observer,
    attempts: u32,
}

impl<'token, 'observer, Observer: PublicationObserver + ?Sized> PublicationControl<'token, 'observer, Observer> {
    /// 🪪️ Admits only explicit positive bounds and a bounded cancellation ancestry.
    ///
    /// ```
    /// use semio_framework_async::{CancelToken, publication::{PublicationControl, PublicationLimits, PublicationObserver, PublicationProgress}};
    /// struct Observer;
    /// impl PublicationObserver for Observer {
    ///     fn on_attempt(&mut self, progress: PublicationProgress) { assert_eq!(progress.attempt, 1); }
    /// }
    /// let token = CancelToken::root_now();
    /// let mut observer = Observer;
    /// let mut control = PublicationControl::new(PublicationLimits { max_attempts: 1, max_token_nodes: 1 }, &token, &mut observer).unwrap();
    /// let transaction = control.try_begin().unwrap();
    /// assert_eq!(control.attempts(), 1);
    /// drop(transaction);
    /// println!("[DEBUG] Publication external owned caller acquired and released one transaction");
    /// ```
    ///
    /// ```compile_fail,E0515
    /// use semio_framework_async::{CancelToken, publication::{PublicationControl, PublicationLimits, PublicationObserver}};
    /// fn escape<'a, O: PublicationObserver>(observer: &'a mut O) -> PublicationControl<'a, 'a, O> {
    ///     let token = CancelToken::root_now();
    ///     PublicationControl::new(PublicationLimits { max_attempts: 1, max_token_nodes: 1 }, &token, observer).unwrap()
    /// }
    /// ```
    pub fn new(limits: PublicationLimits, token: &'token CancelToken, observer: &'observer mut Observer) -> Result<Self, PublicationError> {
        if limits.max_attempts == 0 || limits.max_token_nodes == 0 || limits.max_token_nodes > TOKEN_NODE_CEILING {
            return Err(PublicationError::InvalidLimits);
        }
        Ok(Self { limits, token, observer, attempts: 0 })
    }

    /// 🔢️ Reports the authority consumed by this exact control owner.
    pub fn attempts(&self) -> u32 {
        self.attempts
    }

    fn checkpoint(&self) -> Result<(), PublicationError> {
        checkpoint(self.token, self.limits.max_token_nodes)
    }

    /// 🕰️ Attempts admission once and returns the scheduler turn on contention.
    pub fn try_begin(&mut self) -> Result<PublicationTransaction<'token>, PublicationError> {
        self.checkpoint()?;
        if self.attempts == self.limits.max_attempts {
            return Err(PublicationError::BoundExceeded(PublicationBound::Attempts));
        }
        self.attempts += 1;
        self.observer.on_attempt(PublicationProgress { attempt: self.attempts, max_attempts: self.limits.max_attempts });
        self.checkpoint()?;
        let admission = PUBLICATION_GATE.try_lock().map_err(|error| match error {
            TryLockError::WouldBlock => PublicationError::Busy,
            TryLockError::Poisoned(_) => PublicationError::Unavailable,
        });
        self.checkpoint()?;
        admission.map(|guard| PublicationTransaction { _guard: guard, token: self.token, max_token_nodes: self.limits.max_token_nodes })
    }
}
