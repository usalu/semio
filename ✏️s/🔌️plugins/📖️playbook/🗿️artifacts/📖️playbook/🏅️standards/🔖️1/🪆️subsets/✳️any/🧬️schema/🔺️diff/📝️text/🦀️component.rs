//! 🔺️ Playbook artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::playbook::schema::diff::PlaybookDiff;
use crate::artifacts::playbook::schema::PlaybookArtifact;
use crate::artifacts::playbook::schema::snapshot::PlaybookSnapshot;
use crate::playbook::PlaybookStep;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


//#region 🔖️Apply
impl PlaybookDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &PlaybookArtifact) -> PlaybookArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(id) = &self.id {
            next.id = id.clone();
        }
        if let Some(version) = &self.version {
            next.version = version.clone();
        }
        if let Some(title) = &self.title {
            next.title = title.clone();
        }
        if let Some(document) = &self.document {
            next.document = document.clone();
        }
        if let Some(flow) = &self.flow {
            next.flow = flow.clone();
        }
        if let Some(list) = &self.selected_ids {
            next.selected_ids = list.values.clone();
        }
        if let Some(value) = &self.locale {
            next.locale = value.clone();
        }
        if let Some(value) = &self.contributions_json {
            next.contributions_json = value.clone();
        }
        next
    }
}

impl MutationDiff<PlaybookSnapshot> for PlaybookDiff {
    fn apply(&self, snapshot: &PlaybookSnapshot) -> PlaybookSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(id) = &self.id {
            next.id = id.clone();
        }
        if let Some(version) = &self.version {
            next.version = version.clone();
        }
        if let Some(title) = &self.title {
            next.title = title.clone();
        }
        if let Some(document) = &self.document {
            next.document = document.clone();
        }
        if let Some(flow) = &self.flow {
            next.flow = flow.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(id);
        take!(version);
        take!(title);
        take!(document);
        take!(flow);
        take!(selected_ids);
        take!(locale);
        take!(contributions_json);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Builders
/// 📸️ Whole-snapshot replacement diff.
pub fn diff_set_snapshot(snapshot: &PlaybookSnapshot) -> PlaybookDiff {
    PlaybookDiff {
        artifact: Some(Box::new(PlaybookArtifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}

/// 🔺️ Mints new content-addressed `document`+`flow` handles for the whole-scene replacement
/// `steps` and seeds the working-scene cache with them (`playbook_content_handles_and_cache`) —
/// real handcrafted construction, never apply-then-capture. Every one of the nine step/block
/// mutation triads' `🔺️diff` leaf reads the CURRENT scene off `base` (via `playbook_working_scene`),
/// applies its own specific semantics to that scene, then calls this shared builder — mirrors
/// writer's `diff_set_text`/flow's `diff_replace_content`.
pub fn diff_replace_content(title: Option<&str>, steps: Vec<PlaybookStep>) -> PlaybookDiff {
    let (document, flow) = crate::artifacts::playbook::playbook_content_handles_and_cache(title, steps);
    PlaybookDiff { document: Some(document), flow: Some(flow), ..Default::default() }
}
//#endregion 🔖️Builders
