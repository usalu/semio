//! 🎛️ Actor observation port: the read-only actor snapshot the kernel pool publishes and the suspend/resume/cancel controls the shell sends back — the wgpu twin of what React's TaskManager reads from `ActivationRegistry.runtimeMetricsSnapshot()` (actor id, plugin, residency, shard) and sends through `suspend`/`resume`/`cancel`.

use std::sync::Mutex;

/// 👁️ One actor as React's `RuntimeMetricsSnapshot.actors` row states it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObservedActor {
    pub actor_id: String,
    pub plugin_id: String,
    pub resident: bool,
    pub shard: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActorControlKind {
    Suspend,
    Resume,
    Cancel,
}

/// 🎛️ One control press addressed at an actor id of the latest snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActorControl {
    pub kind: ActorControlKind,
    pub actor_id: String,
}

static OBSERVED: Mutex<Option<Vec<ObservedActor>>> = Mutex::new(None);
static CONTROLS: Mutex<Vec<ActorControl>> = Mutex::new(Vec::new());

/// 📡️ Replaces the published snapshot; the kernel pool is the only writer.
pub fn publish_observed_actors(actors: Vec<ObservedActor>) {
    if let Ok(mut slot) = OBSERVED.lock() {
        *slot = Some(actors);
    }
}

/// 📡️ The latest snapshot, `None` while no kernel has published one (React's absent registry).
pub fn observed_actors() -> Option<Vec<ObservedActor>> {
    OBSERVED.lock().ok().and_then(|slot| slot.clone())
}

/// 🎛️ Queues a control for the kernel pool, which applies it on its next loop turn.
pub fn request_actor_control(control: ActorControl) {
    if let Ok(mut queue) = CONTROLS.lock() {
        queue.push(control);
    }
}

/// 🎛️ Takes every queued control, oldest first; the kernel pool is the only reader.
pub fn take_actor_controls() -> Vec<ActorControl> {
    CONTROLS.lock().map(|mut queue| core::mem::take(&mut *queue)).unwrap_or_default()
}
