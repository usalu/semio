//! 📦️ Export: the entity and event export records.

use super::{Input, Interrupt, Payload, Sha256, StoreError};

//#region 📦️Export

/// 🏷️ One entity of a repository snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportEntity {
    pub kind: String,
    pub id: String,
    pub value: Payload,
}

/// 🧭️ The port an export reads its entities through.
pub trait ExportSource {
    /// 📋️ Enumerates every snapshot entity in taxonomy order.
    fn export_entities(&self) -> Vec<ExportEntity>;
}

impl ExportSource for Vec<ExportEntity> {
    fn export_entities(&self) -> Vec<ExportEntity> {
        self.clone()
    }
}

/// 🧾️ The deterministic identity, per-kind counts and namespaced inputs of one export batch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportSnapshot {
    pub snapshot: String,
    pub counts: Vec<(String, usize)>,
    pub inputs: Vec<Input>,
}

impl ExportSnapshot {
    /// 🔢️ The count recorded for one entity kind.
    pub fn count(&self, kind: &str) -> usize {
        self.counts
            .iter()
            .find(|(name, _)| name == kind)
            .map(|(_, count)| *count)
            .unwrap_or_default()
    }
}

/// 🧮️ Sorts entities by id, hashes them and namespaces every input id with the snapshot.
pub fn build_export_snapshot(
    source: &dyn ExportSource,
    interrupt: &dyn Interrupt,
) -> Result<ExportSnapshot, StoreError> {
    let entities = source.export_entities();
    let mut counts: Vec<(String, usize)> = Vec::new();
    let mut inputs: Vec<Input> = Vec::with_capacity(entities.len());
    for entity in entities {
        if interrupt.cancelled() {
            return Err(StoreError::Cancelled);
        }
        match counts.iter_mut().find(|(name, _)| name == &entity.kind) {
            Some(slot) => slot.1 += 1,
            None => counts.push((entity.kind.clone(), 1)),
        }
        inputs.push(Input {
            id: format!("{}:{}", entity.kind, entity.id),
            kind: format!("{}.recorded", entity.kind),
            data: entity.value,
        });
    }
    inputs.sort_by(|left, right| left.id.cmp(&right.id));
    let mut hasher = Sha256::new();
    for input in &inputs {
        if interrupt.cancelled() {
            return Err(StoreError::Cancelled);
        }
        hasher.update(format!("{}\u{0}{}\u{0}", input.id, input.kind).as_bytes());
        hasher.update(input.data.text().as_bytes());
    }
    let snapshot = hasher.finish();
    for input in inputs.iter_mut() {
        input.id = format!("snapshot:{snapshot}:{}", input.id);
    }
    Ok(ExportSnapshot { snapshot, counts, inputs })
}

//#endregion 📦️Export
