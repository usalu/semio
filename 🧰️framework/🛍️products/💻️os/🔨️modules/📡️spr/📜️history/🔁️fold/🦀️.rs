//! ⏳️ Retained normalization and folding of immutable persisted history.

use super::HistoryLog;
use protocol::{self as protocol, ProtocolError};
use std::sync::Arc;

/// 🧮️ Derived facts and exact normalized transition owners of one immutable history.
pub type RetainedHistoryFold = (protocol::HistoryFold, Vec<protocol::MutationEnvelope>, Vec<String>, Vec<protocol::Conflict>);

impl HistoryLog {
    /// 🌱️ Captures one immutable history alias without inspecting its members.
    pub fn fold_job(source: Arc<Self>, shape: protocol::HistoryShape) -> protocol::HistoryFoldJob<'static, RetainedHistoryFold> {
        protocol::HistoryFoldJob::new(move |control| async move { source.fold_controlled(shape, &control).await })
    }

    /// ⏳️ Normalizes each field and repeated member through granted work and byte pages.
    pub async fn fold_controlled(&self, shape: protocol::HistoryShape, control: &protocol::HistoryFoldControl) -> Result<RetainedHistoryFold, ProtocolError> {
        let mut edits = control.vector(self.edits.len()).await?;
        let mut owners = control.track(::protocol::HistoryFoldIndex::<String, usize>::new()).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
        for (position, edit) in self.edits.iter().enumerate() {
            control.pulse().await?;
            let metadata = edit.meta.as_deref().unwrap_or_default();
            let timestamp = match metadata.first().and_then(|metadata| metadata.hlt) {
                Some((actor, physical_ms, logical)) => protocol::HybridLogicalTimestamp { actor, physical_ms: match u64::try_from(physical_ms){Ok(value)=>value,Err(_)=>return Err(semio_framework_replication_history::history_fold_refusal_controlled("negative history clock",control).await?)}, logical },
                None => protocol::HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 },
            };
            let id = control.track(protocol::copy_history_text(&edit.id, control).await?).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            let actor = control.track(match &edit.actor { Some(actor) => Some(control.lease_actor(actor).await), None => None }).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            let line = control.track(match &edit.line { Some(line) => Some(protocol::copy_history_text(line, control).await?), None => None }).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            let mut mutations = control.vector(edit.ops.len()).await?;
            for index in 0..edit.ops.len() {
                let identity = control.track(match metadata.get(index).and_then(|metadata| metadata.op_id.as_deref()) {
                    Some(identity) => protocol::copy_history_text(identity, control).await?,
                    None => semio_framework_replication_history::copy_history_operation_identity(&edit.id,index as u64,control).await?,
                }).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
                control.insert_text(&mut owners, protocol::copy_history_text(&identity, control).await?, position).await?;
                control.push_vec(&mut mutations,protocol::MutationId(identity.take())).await?;
            }
            control.push_vec(&mut edits,protocol::FoldEdit { id: id.take(), actor: actor.take(), timestamp, mutation_ids: mutations.take(), line: line.take() }).await?;
        }
        let mut excluded = control.track(::protocol::HistoryFoldIndex::<String, ()>::new()).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
        for conflict in &self.conflicts {
            control.pulse().await?;
            if conflict.kind != 0 || conflict.status == 1 { continue; }
            for bytes in &conflict.envelopes {
                let identity = control.track(protocol::history_envelope_id_controlled(bytes, control).await?).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
                if let Some(position) = control.get_text(&owners, &identity.0).await? { control.insert_text(&mut excluded, protocol::copy_history_text(&self.edits[*position].id, control).await?, ()).await?; }
            }
        }
        let mut transitions = control.vector(self.transitions.len()).await?;
        for source in &self.transitions {
            let decoded = control.track(protocol::decode_history_transition_controlled(&source.payload, control).await?).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            if !shape.admits(decoded.kind()) { return Err(semio_framework_replication_history::history_fold_refusal_controlled("transition is refused by the history shape",control).await?); }
            let mutation_id = control.track(protocol::MutationId(protocol::copy_history_text(&source.id, control).await?)).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            let document_id = control.track(protocol::ArtifactId(protocol::copy_history_text(&self.doc_id, control).await?)).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            let actor=control.track(protocol::ActorId(control.lease_actor(&source.actor).await)).unwrap_or_else(|_|unreachable!("history actor shared ownership")).await;
            let mut dependencies = control.vector(source.dependencies.len()).await?;
            for dependency in &source.dependencies { control.push_vec(&mut dependencies,protocol::MutationId(protocol::copy_history_text(dependency, control).await?)).await?; }
            let observed = control.track(match &source.observed { Some(observed) => Some(protocol::MutationId(protocol::copy_history_text(observed, control).await?)), None => None }).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            let payload = control.track(protocol::copy_history_payload(&source.payload, control).await?).unwrap_or_else(|_|unreachable!("history payload declares controlled ownership")).await;
            let schema = control.track(protocol::SchemaId(protocol::copy_history_text(protocol::HISTORY_TRANSITION_SCHEMA,control).await?)).unwrap_or_else(|_|unreachable!("history schema declares controlled ownership")).await;
            let inverse_schema=protocol::SchemaId(protocol::copy_history_text(protocol::HISTORY_TRANSITION_SCHEMA,control).await?);
            control.push_vec(&mut transitions,protocol::MutationEnvelope { mutation_id: mutation_id.take(), document_id: document_id.take(), actor: actor.take(), dependencies: dependencies.take(), observed: observed.take(), target: Vec::new(), diff: protocol::ArtifactDiff { schema: schema.take(), payload:payload.take() }, inverse: protocol::InverseMutation { schema:inverse_schema, payload: Default::default() }, timestamp: protocol::HybridLogicalTimestamp { actor: source.hlt.0, physical_ms: source.hlt.1, logical: source.hlt.2 }, transaction: None, verb: None, line: None }).await?;
        }
        let document_id = control.track(protocol::ArtifactId(protocol::copy_history_text(&self.doc_id, control).await?)).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
        let trunk = control.track(semio_framework_replication_history::copy_history_trunk(&document_id, control).await?).unwrap_or_else(|_|unreachable!("history trunk declares controlled local ownership")).await;
        let line_id = control.track(match self.viewer_line.as_ref().filter(|line| line.as_str() != trunk.as_str()) { Some(line) => protocol::copy_history_text(line, control).await?, None => trunk.take() }).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
        let checkpoint_id = match &self.viewer_checkpoint { Some(checkpoint) => Some(protocol::copy_history_text(checkpoint, control).await?), None => None };
        let head = control.track(protocol::ViewerHead { line_id: line_id.take(), checkpoint_id }).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
        let excluded_index: &::protocol::HistoryFoldIndex<String, ()> = &excluded;
        let folded = protocol::fold_history_for_controlled(&document_id, &edits, &*transitions, excluded_index, &head, control).await?;
        let folded = control.track(folded).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
        let mut replay_order = control.vector(if shape == protocol::HistoryShape::Document { folded.applied.len() } else { 0 }).await?;
        if shape == protocol::HistoryShape::Document { for id in &folded.applied { control.push_vec(&mut replay_order,protocol::copy_history_text(id, control).await?).await?; } }
        let mut conflicts = control.vector(self.conflicts.len()).await?;
        for source in &self.conflicts {
            let identity = control.track(protocol::ConflictId(protocol::copy_history_text(&source.id, control).await?)).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            let status = match source.status { 0 => protocol::ConflictStatus::Open, 1 => protocol::ConflictStatus::Accepted, 2 => protocol::ConflictStatus::Discarded, _ => return Err(semio_framework_replication_history::history_fold_refusal_controlled("unknown conflict status",control).await?) };
            let mut actors = control.vector(source.actors.len()).await?;
            for actor in &source.actors {control.push_vec(&mut actors,protocol::ActorId(control.lease_actor(actor).await)).await?;}
            let mut messages = control.vector(source.messages.len()).await?;
            for source in &source.messages {
                let level = match semio_framework_diagnostic::Severity::from_u8(source.level){Some(level)=>level,None=>return Err(semio_framework_replication_history::history_fold_refusal_controlled("unknown conflict severity",control).await?)};
                let code = control.track(protocol::copy_history_text(&source.code, control).await?).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
                let message = control.track(protocol::copy_history_text(&source.message, control).await?).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
                let mut target = control.vector(source.target.len()).await?;
                for segment in &source.target { control.push_vec(&mut target,protocol::copy_history_text(segment, control).await?).await?; }
                control.push_vec(&mut messages,protocol::MutationMessage { level, code: semio_framework_diagnostic::FaultCode(code.take()), message: message.take(), target: target.take(), op_index: source.op_index }).await?;
            }
            let kind = control.track(match source.kind {
                0 => { let mut envelopes = control.vector(source.envelopes.len()).await?; for bytes in &source.envelopes { control.push_vec(&mut envelopes,protocol::decode_history_envelope_controlled(bytes, control).await?).await?; } protocol::ConflictKind::Quarantined { envelopes: envelopes.take() } },
                1 => { let mut edit_ids = control.vector(source.edit_ids.len()).await?; for id in &source.edit_ids { control.push_vec(&mut edit_ids,protocol::copy_history_text(id, control).await?).await?; } protocol::ConflictKind::Degraded { edit_ids: edit_ids.take() } },
                _ => return Err(semio_framework_replication_history::history_fold_refusal_controlled("unknown conflict kind",control).await?),
            }).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            control.push_vec(&mut conflicts,protocol::Conflict { id: identity.take(), kind: kind.take(), status, messages: messages.take(), actors: actors.take(), timestamp: protocol::HybridLogicalTimestamp { actor: source.hlt.0, physical_ms: source.hlt.1, logical: source.hlt.2 } }).await?;
        }
        Ok((folded.take(), transitions.take(), replay_order.take(), conflicts.take()))
    }
}
