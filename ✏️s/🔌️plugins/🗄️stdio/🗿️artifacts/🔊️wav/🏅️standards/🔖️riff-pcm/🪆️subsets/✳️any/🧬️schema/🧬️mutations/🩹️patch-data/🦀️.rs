//! 🩹️ Bounded WAV sample-range patch with exact inverse data.

use super::*;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct PatchData {
    pub index: u64,
    pub remove_count: u64,
    pub data: WavData,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub move_to: Option<u64>,
}

fn len(data: &WavData) -> usize {
    match data {
        WavData::Pcm16(values) => values.len(),
        WavData::Pcm8(values) | WavData::Raw(values) => values.len(),
        WavData::Float32(values) => values.len(),
    }
}

fn empty(data: &WavData) -> bool {
    len(data) == 0
}

fn slice(data: &WavData, start: usize, end: usize) -> WavData {
    match data {
        WavData::Pcm16(values) => WavData::Pcm16(values[start..end].to_vec()),
        WavData::Pcm8(values) => WavData::Pcm8(values[start..end].to_vec()),
        WavData::Float32(values) => WavData::Float32(values[start..end].to_vec()),
        WavData::Raw(values) => WavData::Raw(values[start..end].to_vec()),
    }
}

fn apply(data: &WavData, patch: &PatchData) -> Result<WavData, String> {
    let index = usize::try_from(patch.index).map_err(|_| "WAV sample index exceeds this platform".to_string())?;
    if let Some(move_to) = patch.move_to {
        let move_to = usize::try_from(move_to).map_err(|_| "WAV move destination exceeds this platform".to_string())?;
        if patch.remove_count != 0 || !empty(&patch.data) || index >= len(data) || move_to >= len(data) {
            return Err("WAV sample move is outside the data range or carries replacement data".into());
        }
        let mut next = data.clone();
        match &mut next {
            WavData::Pcm16(values) => { let value = values.remove(index); values.insert(move_to, value); }
            WavData::Pcm8(values) | WavData::Raw(values) => { let value = values.remove(index); values.insert(move_to, value); }
            WavData::Float32(values) => { let value = values.remove(index); values.insert(move_to, value); }
        }
        return Ok(next);
    }
    let remove_count = usize::try_from(patch.remove_count).map_err(|_| "WAV removal count exceeds this platform".to_string())?;
    let end = index.checked_add(remove_count).ok_or_else(|| "WAV sample range overflows".to_string())?;
    if index > len(data) || end > len(data) {
        return Err("WAV sample patch is outside the data range".into());
    }
    let mut next = data.clone();
    match (&mut next, &patch.data) {
        (WavData::Pcm16(values), WavData::Pcm16(insert)) => { values.splice(index..end, insert.iter().copied()); }
        (WavData::Pcm8(values), WavData::Pcm8(insert)) | (WavData::Raw(values), WavData::Raw(insert)) => { values.splice(index..end, insert.iter().copied()); }
        (WavData::Float32(values), WavData::Float32(insert)) => { values.splice(index..end, insert.iter().copied()); }
        _ => return Err("WAV sample patch data kind differs from the snapshot data kind".into()),
    }
    Ok(next)
}

pub(crate) fn diff(payload: &PatchData, base: &WavSnapshot) -> protocol::MutationOutcome<WavDiff> {
    match apply(&base.data, payload) {
        Ok(data) => protocol::MutationOutcome::new(diff_set_data(data)),
        Err(message) => protocol::MutationOutcome::error("stdio.wav.patch-data.invalid-range", message, ["data".into(), "value".into(), payload.index.to_string()]),
    }
}

pub(crate) fn inverse(payload: &PatchData, base: &WavSnapshot) -> Vec<WavMutation> {
    let Ok(index) = usize::try_from(payload.index) else { return Vec::new() };
    if let Some(move_to) = payload.move_to {
        if apply(&base.data, payload).is_err() { return Vec::new(); }
        return vec![WavMutation::PatchData(PatchData { index: move_to, remove_count: 0, data: slice(&base.data, 0, 0), move_to: Some(payload.index) })];
    }
    let Ok(remove_count) = usize::try_from(payload.remove_count) else { return Vec::new() };
    let Some(end) = index.checked_add(remove_count) else { return Vec::new() };
    if end > len(&base.data) || apply(&base.data, payload).is_err() { return Vec::new(); }
    vec![WavMutation::PatchData(PatchData { index: payload.index, remove_count: len(&payload.data) as u64, data: slice(&base.data, index, end), move_to: None })]
}

impl protocol::MutationKind<WavSnapshot, WavMutation> for PatchData {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "data", kind: "patch-data", record: "PatchData" };

    fn diff(&self, base: &WavSnapshot) -> protocol::MutationOutcome<<WavMutation as Mutation<WavSnapshot>>::Diff> {
        diff(self, base)
    }
    fn inverse(&self, base: &WavSnapshot) -> Vec<WavMutation> {
        inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Patch samples", "Samples bearbeiten")
    }
    fn target(&self) -> Vec<String> {
        vec!["data".into(), "value".into(), self.index.to_string()]
    }
}
