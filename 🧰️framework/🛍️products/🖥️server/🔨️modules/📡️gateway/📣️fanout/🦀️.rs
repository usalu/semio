//! 📣️ The event fanout, lane names and the kick map.

use super::*;

//#region 🔖️Fanout
/// 📻️ How many frames a lane buffers before a slow subscriber is lagged out of them.
const FANOUT_CAPACITY: usize = 256;

/// 📡️ Per-lane broadcast fan-out. One [`broadcast`] sender per lane key, created on the first
/// subscribe and dropped when the last subscriber goes, so an idle instance holds no lanes at all.
#[derive(Clone, Default)]
pub struct Fanout {
    channels: Arc<ShardedMap<String, broadcast::Sender<Vec<u8>>>>,
}

impl Fanout {
    /// 🌱️ A registry holding no lanes.
    pub fn new() -> Self {
        Self::default()
    }

    /// 🔔️ Subscribe to `lane`, creating it if nobody holds it yet. The returned [`Subscription`] is
    /// the lane's lifetime: dropping the last one removes the sender.
    pub fn subscribe(&self, lane: &str) -> Subscription {
        let sender = self.channels.get_or_insert_with_cloned(lane.to_string(), || broadcast::channel(FANOUT_CAPACITY).0);
        Subscription { lane: lane.to_string(), channels: Arc::clone(&self.channels), receiver: sender.subscribe() }
    }

    /// 📢️ Publish `bytes` on `lane` and report how many subscribers received it. Publishing to a
    /// lane nobody holds is a no-op — a fan-out never creates a lane, only a subscriber does.
    pub fn publish(&self, lane: &str, bytes: Vec<u8>) -> usize {
        match self.channels.get_cloned(lane) {
            Some(sender) => sender.send(bytes).unwrap_or(0),
            None => 0,
        }
    }

    /// 🔢️ How many lanes currently have at least one subscriber.
    pub fn lanes(&self) -> usize {
        self.channels.len()
    }
}

/// 🎧️ One live subscription. Holding it keeps its lane alive; dropping it releases the lane once no
/// other subscriber remains.
pub struct Subscription {
    pub(super) lane: String,
    channels: Arc<ShardedMap<String, broadcast::Sender<Vec<u8>>>>,
    pub(super) receiver: broadcast::Receiver<Vec<u8>>,
}

impl Subscription {
    /// 📥️ The next frame, or `None` once the lane is closed. A lagged subscriber silently skips the
    /// frames it missed rather than erroring: an ephemeral lane is lossy by construction, and the
    /// durable lane recovers by replaying from its sequence number.
    pub async fn recv(&mut self) -> Option<Vec<u8>> {
        loop {
            match self.receiver.recv().await {
                Ok(bytes) => return Some(bytes),
                Err(broadcast::error::RecvError::Lagged(_)) => continue,
                Err(broadcast::error::RecvError::Closed) => return None,
            }
        }
    }

    /// 📬️ The next frame, or the fact that frames were missed, or the end of the lane — for a
    /// subscriber whose protocol is a delta stream and must resynchronize rather than silently skip.
    pub async fn next(&mut self) -> Delivery {
        match self.receiver.recv().await {
            Ok(bytes) => Delivery::Frame(bytes),
            Err(broadcast::error::RecvError::Lagged(_)) => Delivery::Lagged,
            Err(broadcast::error::RecvError::Closed) => Delivery::Closed,
        }
    }

    /// 🏷️ The lane this subscription listens on.
    pub fn lane(&self) -> &str {
        &self.lane
    }
}

/// 📬️ What [`Subscription::next`] delivers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Delivery {
    /// 📦️ One published frame.
    Frame(Vec<u8>),
    /// ⏭️ The subscriber fell behind and frames were dropped for it.
    Lagged,
    /// 🔚️ The lane has no sender any more.
    Closed,
}

impl Drop for Subscription {
    fn drop(&mut self) {
        self.channels.remove_if(&self.lane, |sender| sender.receiver_count() <= 1);
    }
}

/// 📚️ The durable lane key of one actor's event stream.
pub fn stream_lane(actor: &ActorKey) -> String {
    format!("stream:{}/{}/{}", actor.tenant.0, actor.kind, actor.id)
}

/// 💨️ The ephemeral lane key of one scope — cursors, selections, typing.
pub fn ephemeral_lane(scope: &Scope) -> String {
    format!("ephemeral:{}", scope.0)
}

/// 📄️ The document lane key of one scope, carrying opaque engine frames between sessions.
pub fn document_lane(scope: &Scope) -> String {
    format!("document:{}", scope.0)
}

/// 🧍️ The presence lane key of one scope: its room, its colour namespace and its batch lane.
pub fn presence_lane(scope: &Scope) -> String {
    format!("presence:{}", scope.0)
}
//#endregion 🔖️Fanout

//#region 🔖️Kick
/// 🦵️ Per-session close signals. The administration plane fires one; the socket loop owning that
/// session observes it and closes itself. Nothing here ever touches a socket.
#[derive(Default)]
pub struct KickMap {
    signals: ShardedMap<String, Arc<Notify>>,
}

impl KickMap {
    /// 🌱️ An empty map.
    pub fn new() -> Self {
        Self::default()
    }

    /// 🔔️ Signal the session to close. The permit is stored, so a kick that arrives before the loop
    /// starts waiting is still observed.
    pub fn kick(&self, session: &str) {
        self.signal(session).notify_one();
    }

    /// ⏳️ Wait until this session is kicked.
    pub async fn kicked(&self, session: &str) {
        let signal = self.signal(session);
        signal.notified().await;
    }

    /// 🧹️ Forget a session's signal once its socket is gone.
    pub fn forget(&self, session: &str) {
        self.signals.remove(session);
    }

    /// 🔢️ How many sessions currently carry a signal.
    pub fn tracked(&self) -> usize {
        self.signals.len()
    }

    fn signal(&self, session: &str) -> Arc<Notify> {
        self.signals.get_or_insert_with_cloned(session.to_string(), || Arc::new(Notify::new()))
    }
}
//#endregion 🔖️Kick
