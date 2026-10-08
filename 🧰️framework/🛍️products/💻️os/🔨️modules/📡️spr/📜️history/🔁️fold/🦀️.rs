//! ⏳️ Retained normalization and folding of immutable persisted history.

use super::HistoryLog;
use crate::os_spr::{self as protocol, wire::ProtocolError};
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
        let mut edits = control.track(Vec::with_capacity(self.edits.len())).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
        let mut owners = control.track(::protocol::HistoryFoldIndex::<String, usize>::new()).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
        for (position, edit) in self.edits.iter().enumerate() {
            control.pulse().await?;
            let metadata = edit.meta.as_deref().unwrap_or_default();
            let timestamp = match metadata.first().and_then(|metadata| metadata.hlt) {
                Some((actor, physical_ms, logical)) => protocol::HybridLogicalTimestamp { actor, physical_ms: u64::try_from(physical_ms).map_err(|_| ProtocolError::Malformed { what: "history fold", offset: 0, detail: "negative history clock".into() })?, logical },
                None => protocol::HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 },
            };
            let id = control.track(protocol::copy_history_text(&edit.id, control).await?).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            let actor = control.track(match &edit.actor { Some(actor) => Some(protocol::copy_history_text(actor, control).await?), None => None }).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            let line = control.track(match &edit.line { Some(line) => Some(protocol::copy_history_text(line, control).await?), None => None }).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            let mut mutations = control.track(Vec::with_capacity(edit.ops.len())).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            for index in 0..edit.ops.len() {
                let identity = control.track(match metadata.get(index).and_then(|metadata| metadata.op_id.as_deref()) {
                    Some(identity) => protocol::copy_history_text(identity, control).await?,
                    None => protocol::copy_history_text_parts(&[&edit.id, &format!("#{index}")], control).await?,
                }).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
                owners.insert(protocol::copy_history_text(&identity, control).await?, position);
                mutations.push(protocol::MutationId(identity.take()));
            }
            edits.push(protocol::FoldEdit { id: id.take(), actor: actor.take(), timestamp, mutation_ids: mutations.take(), line: line.take() });
        }
        let mut excluded = control.track(::protocol::HistoryFoldIndex::<String, ()>::new()).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
        for conflict in &self.conflicts {
            control.pulse().await?;
            if conflict.kind != 0 || conflict.status == 1 { continue; }
            for bytes in &conflict.envelopes {
                let identity = control.track(protocol::history_envelope_id_controlled(bytes, control).await?).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
                if let Some(position) = owners.get(&identity.0) { excluded.insert(protocol::copy_history_text(&self.edits[*position].id, control).await?, ()); }
            }
        }
        let mut transitions = control.track(Vec::with_capacity(self.transitions.len())).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
        for source in &self.transitions {
            let decoded = control.track(protocol::decode_history_transition_controlled(&source.payload, control).await?).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            if !shape.admits(decoded.kind()) { return Err(ProtocolError::Malformed { what: "history fold", offset: 0, detail: "transition is refused by the history shape".into() }); }
            let mutation_id = control.track(protocol::MutationId(protocol::copy_history_text(&source.id, control).await?)).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            let document_id = control.track(protocol::ArtifactId(protocol::copy_history_text(&self.doc_id, control).await?)).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            let actor = control.track(protocol::ActorId(protocol::copy_history_text(&source.actor, control).await?)).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            let mut dependencies = control.track(Vec::with_capacity(source.dependencies.len())).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            for dependency in &source.dependencies { dependencies.push(protocol::MutationId(protocol::copy_history_text(dependency, control).await?)); }
            let observed = control.track(match &source.observed { Some(observed) => Some(protocol::MutationId(protocol::copy_history_text(observed, control).await?)), None => None }).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            let payload = protocol::copy_history_bytes(&source.payload, control).await?;
            let schema = protocol::SchemaId(protocol::HISTORY_TRANSITION_SCHEMA.into());
            transitions.push(protocol::MutationEnvelope { mutation_id: mutation_id.take(), document_id: document_id.take(), actor: actor.take(), dependencies: dependencies.take(), observed: observed.take(), target: Vec::new(), diff: protocol::ArtifactDiff { schema: schema.clone(), payload }, inverse: protocol::InverseMutation { schema, payload: Vec::new() }, timestamp: protocol::HybridLogicalTimestamp { actor: source.hlt.0, physical_ms: source.hlt.1, logical: source.hlt.2 }, transaction: None, verb: None, line: None });
        }
        let document_id = control.track(protocol::ArtifactId(protocol::copy_history_text(&self.doc_id, control).await?)).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
        let trunk = protocol::trunk_alternative_id(&document_id);
        let line_id = control.track(match self.viewer_line.as_ref().filter(|line| *line != &trunk) { Some(line) => protocol::copy_history_text(line, control).await?, None => trunk }).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
        let checkpoint_id = match &self.viewer_checkpoint { Some(checkpoint) => Some(protocol::copy_history_text(checkpoint, control).await?), None => None };
        let head = control.track(protocol::ViewerHead { line_id: line_id.take(), checkpoint_id }).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
        let excluded_index: &::protocol::HistoryFoldIndex<String, ()> = &excluded;
        let folded = protocol::fold_history_for_controlled(&document_id, &edits, &transitions, &|id| excluded_index.contains_key(id), &head, control).await?;
        let folded = control.track(folded).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
        let mut replay_order = control.track(Vec::with_capacity(if shape == protocol::HistoryShape::Document { folded.applied.len() } else { 0 })).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
        if shape == protocol::HistoryShape::Document { for id in &folded.applied { replay_order.push(protocol::copy_history_text(id, control).await?); } }
        let mut conflicts = control.track(Vec::with_capacity(self.conflicts.len())).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
        for source in &self.conflicts {
            let identity = control.track(protocol::ConflictId(protocol::copy_history_text(&source.id, control).await?)).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            let status = match source.status { 0 => protocol::ConflictStatus::Open, 1 => protocol::ConflictStatus::Accepted, 2 => protocol::ConflictStatus::Discarded, _ => return Err(ProtocolError::Malformed { what: "history fold", offset: 0, detail: "unknown conflict status".into() }) };
            let mut actors = control.track(Vec::with_capacity(source.actors.len())).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            for actor in &source.actors { actors.push(protocol::ActorId(protocol::copy_history_text(actor, control).await?)); }
            let mut messages = control.track(Vec::with_capacity(source.messages.len())).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            for source in &source.messages {
                let level = semio_framework_diagnostic::Severity::from_u8(source.level).ok_or_else(|| ProtocolError::Malformed { what: "history fold", offset: 0, detail: "unknown conflict severity".into() })?;
                let code = control.track(protocol::copy_history_text(&source.code, control).await?).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
                let message = control.track(protocol::copy_history_text(&source.message, control).await?).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
                let mut target = control.track(Vec::with_capacity(source.target.len())).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
                for segment in &source.target { target.push(protocol::copy_history_text(segment, control).await?); }
                messages.push(protocol::MutationMessage { level, code: semio_framework_diagnostic::FaultCode(code.take()), message: message.take(), target: target.take(), op_index: source.op_index });
            }
            let kind = control.track(match source.kind {
                0 => { let mut envelopes = control.track(Vec::with_capacity(source.envelopes.len())).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await; for bytes in &source.envelopes { envelopes.push(protocol::decode_history_envelope_controlled(bytes, control).await?); } protocol::ConflictKind::Quarantined { envelopes: envelopes.take() } },
                1 => { let mut edit_ids = control.track(Vec::with_capacity(source.edit_ids.len())).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await; for id in &source.edit_ids { edit_ids.push(protocol::copy_history_text(id, control).await?); } protocol::ConflictKind::Degraded { edit_ids: edit_ids.take() } },
                _ => return Err(ProtocolError::Malformed { what: "history fold", offset: 0, detail: "unknown conflict kind".into() }),
            }).unwrap_or_else(|_| unreachable!("history schema declares controlled local ownership")).await;
            conflicts.push(protocol::Conflict { id: identity.take(), kind: kind.take(), status, messages: messages.take(), actors: actors.take(), timestamp: protocol::HybridLogicalTimestamp { actor: source.hlt.0, physical_ms: source.hlt.1, logical: source.hlt.2 } });
        }
        Ok((folded.take(), transitions.take(), replay_order.take(), conflicts.take()))
    }
}
