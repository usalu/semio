//! 🎛️ Runtime observation: the kernel pool publishes a read-only actor snapshot and applies the controls the shell queued, through the host-base actor observation port, between two requests.

use super::*;
use semio_renderer_wgpu_host_base::{publish_observed_actors, take_actor_controls, ActorControlKind, ObservedActor};

const OBSERVATION_PERIOD_MS: u128 = 500;

impl KernelPoolState {
    /// 🎛️ Applies the queued suspend/resume/cancel controls (cancel is the shard-loop's `Payload::Cancel { seq: 0 }`, whose value is ignored), then republishes the actor snapshot when `OBSERVATION_PERIOD_MS` passed since `last`.
    pub(super) async fn serve_runtime_observation(&mut self, started: std::time::Instant, last: &mut Option<u128>) {
        for control in take_actor_controls() {
            let Ok(raw) = control.actor_id.parse::<u64>() else { continue };
            let actor = ActorId(raw);
            let applied = match control.kind {
                ActorControlKind::Suspend => self.runtime.kernel_mut().suspend(actor, None).await.map_err(|_| "refused by the kernel".to_string()),
                ActorControlKind::Resume => self.runtime.kernel_mut().resume(actor).await.map_err(|_| "refused by the kernel".to_string()),
                ActorControlKind::Cancel => match next_seq() {
                    Ok(seq) => {
                        let cancel = Envelope { to: actor, from: Origin::Kernel, lane: Lane::Interactive, seq, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Cancel { seq: 0 } };
                        match self.runtime.submit(&cancel).await {
                            Backpressure::Accept => Ok(()),
                            _ => Err("mailbox refused the cancel".to_string()),
                        }
                    }
                    Err(_) => Err("no sequence number".to_string()),
                },
            };
            if let Err(error) = applied {
                log_debug(&format!("kernel: task manager control on actor {raw} refused: {error}"));
            }
            *last = None;
        }
        let elapsed = started.elapsed().as_millis();
        if last.is_some_and(|at| elapsed.saturating_sub(at) < OBSERVATION_PERIOD_MS) {
            return;
        }
        *last = Some(elapsed);
        let snapshot = self.runtime.kernel().runtime_metrics_snapshot(elapsed as u64).await;
        let mut actors = Vec::with_capacity(snapshot.actors.len());
        for sample in &snapshot.actors {
            let shard = self.runtime.kernel().actor_record(sample.id).await.map_or(0, |record| u64::from(record.shard.0));
            actors.push(ObservedActor { actor_id: sample.id.0.to_string(), plugin_id: sample.package.0.clone(), resident: !matches!(sample.status, semio_framework_actor::ActorStatus::Suspended { .. }), shard });
        }
        publish_observed_actors(actors);
    }
}
