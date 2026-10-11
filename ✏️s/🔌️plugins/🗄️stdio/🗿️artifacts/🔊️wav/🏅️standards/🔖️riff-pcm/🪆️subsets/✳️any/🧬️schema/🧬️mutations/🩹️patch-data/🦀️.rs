//! 🩹️ Bounded WAV sample-range patch: one splice of the sample lane (a take-out plus a put-back for a move), with the exact inverse patch built from the replaced elements.

use super::*;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct PatchData {
    pub index: u64,
    pub remove_count: u64,
    pub data: WavData,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub move_to: Option<u64>,
}

impl PatchData {
    /// ✂️ The splices this patch asks of `base`'s sample lane — none for a patch that changes nothing — or why the patch is outside the lane.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn splices(&self, base: &WavSnapshot) -> Result<Vec<WavSplice>, String> {
        let lane = &base.data;
        let len = data_len(lane);
        let index = usize::try_from(self.index).map_err(|_| "WAV sample index exceeds this platform".to_string())?;
        if let Some(move_to) = self.move_to {
            let move_to = usize::try_from(move_to).map_err(|_| "WAV move destination exceeds this platform".to_string())?;
            if self.remove_count != 0 || data_len(&self.data) != 0 || index >= len || move_to >= len {
                return Err("WAV sample move is outside the data range or carries replacement data".into());
            }
            return Ok(match index == move_to {
                true => Vec::new(),
                false => vec![WavSplice { index: self.index, remove: 1, insert: data_slice(lane, 0, 0) }, WavSplice { index: move_to as u64, remove: 0, insert: data_slice(lane, index, index + 1) }],
            });
        }
        let remove = usize::try_from(self.remove_count).map_err(|_| "WAV removal count exceeds this platform".to_string())?;
        let end = index.checked_add(remove).ok_or_else(|| "WAV sample range overflows".to_string())?;
        if index > len || end > len {
            return Err("WAV sample patch is outside the data range".into());
        }
        if std::mem::discriminant(&self.data) != std::mem::discriminant(lane) {
            return Err("WAV sample patch data kind differs from the snapshot data kind".into());
        }
        let unchanged = (remove == 0 && data_len(&self.data) == 0) || data_slice(lane, index, end) == self.data;
        Ok(match unchanged {
            true => Vec::new(),
            false => vec![WavSplice { index: self.index, remove: self.remove_count, insert: self.data.clone() }],
        })
    }
}

impl protocol::MutationKind<WavSnapshot, WavMutation> for PatchData {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "data", kind: "patch-data", record: "PatchData" };

    fn diff(&self, base: &WavSnapshot) -> protocol::MutationOutcome<<WavMutation as Mutation<WavSnapshot>>::Diff> {
        match self.splices(base) {
            Ok(data_splices) => protocol::MutationOutcome::new(WavDiff { data_splices, ..WavDiff::default() }),
            Err(message) => protocol::MutationOutcome::error("mutation.target-mismatch", message, ["data".into(), "value".into(), self.index.to_string()]),
        }
    }
    fn inverse(&self, base: &WavSnapshot) -> Result<Vec<WavMutation>, semio_framework_value::ValueError> {
        let lane = &base.data;
        let Ok(splices) = self.splices(base) else { return Ok(Vec::new()) };
        Ok(match (splices.is_empty(), self.move_to) {
            (true, _) => Vec::new(),
            (false, Some(move_to)) => vec![WavMutation::PatchData(PatchData { index: move_to, remove_count: 0, data: data_slice(lane, 0, 0), move_to: Some(self.index) })],
            (false, None) => {
                let start = self.index as usize;
                vec![WavMutation::PatchData(PatchData { index: self.index, remove_count: data_len(&self.data) as u64, data: data_slice(lane, start, start + self.remove_count as usize), move_to: None })]
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Patch samples", "Samples bearbeiten")
    }
    fn target(&self) -> Vec<String> {
        vec!["data".into(), "value".into(), self.index.to_string()]
    }
}
