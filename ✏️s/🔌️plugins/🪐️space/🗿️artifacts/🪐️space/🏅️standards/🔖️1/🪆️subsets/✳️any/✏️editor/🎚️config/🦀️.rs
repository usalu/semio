//! 🎚️ SpaceIndexEditor view-state — the folded-directory read model slice (members/visibility) plus
//! per-artifact live presence, both host-pushed via the `fold-directory-events`/`presence-heartbeat`
//! commands (never duplicated into the shared `SSpaceSnapshot` document — contract §C4: "space
//! name/kind/visibility/members are directory-owned ... never duplicated into this document"). Local
//! view state only, mirrors `DrawConfig`'s handcrafted DSL/pack codec shape.

use protocol::Mutation;

use crate::standards::v1::subsets::any::schema::snapshot::{SpaceArtifactDialect, SpaceArtifactRow};
use semio_framework_os_kernel::os_directory::DirectoryIndexedDocumentViewV1;

//#region 🔖️Member
/// 🧑️ One space member, projected from `semio_framework_os::os_directory::MemberView` into the
/// space app's own local view-state vocabulary (`role` kept as the wire string `"author"`/
/// `"spectator"` rather than re-importing the directory crate's enum into render code).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceIndexMember {
    pub user_id: String,
    pub email: String,
    pub display_name: String,
    pub role: String,
}
//#endregion 🔖️Member

//#region 🔖️Presence
/// 👥️ Live peers on one artifact's documents (all surfaces/documents of that artifact, folded to a
/// flat actor-id list) — `actors_csv` avoids nesting `Vec<String>` inside a `#[dsl(table)]` row
/// (unproven by any existing facet in this tree); split on `,` for display.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceIndexArtifactPresence {
    pub artifact_id: String,
    pub actors_csv: String,
}

impl SpaceIndexArtifactPresence {
    /// 🪪️ The live actor ids for this artifact, empty-string-safe.
    pub fn actor_ids(&self) -> Vec<&str> {
        if self.actors_csv.is_empty() {
            Vec::new()
        } else {
            self.actors_csv.split(',').collect()
        }
    }
}
//#endregion 🔖️Presence

//#region 🔖️Config
/// 🎚️ `SpaceIndexEditor`'s real `ArtifactApp::Config` — whole-record, DSL/pack codec handcrafted
/// (mirrors `SSpaceSnapshot`'s own handcrafted pair, `🧬️schema/📸️snapshot/🦀️.rs`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "sspacecfg")]
#[dsl(layout = "lines")]
pub struct SpaceIndexConfig {
    pub visibility: String,
    #[dsl(table)]
    pub members: Vec<SpaceIndexMember>,
    #[dsl(table)]
    pub indexed_artifacts: Vec<SpaceArtifactRow>,
    #[dsl(table)]
    pub presence: Vec<SpaceIndexArtifactPresence>,
}

impl Default for SpaceIndexConfig {
    fn default() -> Self {
        Self { visibility: "private".into(), members: Vec::new(), indexed_artifacts: Vec::new(), presence: Vec::new() }
    }
}

impl SpaceIndexConfig {
    /// 📇️ Projects one Directory-owned indexed document into the Space app's bounded read-only row.
    pub fn indexed_artifact_from_directory(row: &DirectoryIndexedDocumentViewV1) -> Option<SpaceArtifactRow> {
        let created_at_ms = u64::try_from(row.created_at_ms).ok()?;
        Some(SpaceArtifactRow {
            id: row.descriptor.document_id.clone(),
            name: row.entry.name.clone(),
            kind_id: row.descriptor.artifact_kind.clone(),
            schema: row.descriptor.artifact_schema.clone(),
            dialect: SpaceArtifactDialect { artifact_kind: row.entry.dialect.artifact_kind.clone(), standard: row.entry.dialect.standard.clone(), subset: row.entry.dialect.subset.clone() },
            created_at_ms,
            created_by: row.created_by.clone(),
            updated_at_ms: created_at_ms,
            updated_by: row.created_by.clone(),
        })
    }

    /// 👥️ The live actor ids on `artifact_id`'s documents, empty when nothing is folded in yet.
    pub fn presence_for(&self, artifact_id: &str) -> Vec<&str> {
        self.presence.iter().find(|row| row.artifact_id == artifact_id).map(SpaceIndexArtifactPresence::actor_ids).unwrap_or_default()
    }
}

//#region 🔖️ArtifactCodec
impl store::ArtifactDsl for SpaceIndexConfig {
    const EXTENSION: &'static str = "sspacecfg";
    fn envelope_id() -> &'static str {
        "s.space.config"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SpaceIndexConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️ArtifactCodec

impl store::ConfigRecord for SpaceIndexConfig {}

/// 📇️ The directory-owned slice of the config (visibility, members, indexed documents): what a directory fold replaces as one entity.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceIndexDirectoryProjection {
    pub visibility: String,
    #[dsl(table)]
    pub members: Vec<SpaceIndexMember>,
    #[dsl(table)]
    pub indexed_artifacts: Vec<SpaceArtifactRow>,
}

/// 🩹 Patch of one presence row.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceIndexPresencePatch {
    pub artifact_id: String,
    pub actors_csv: Option<String>,
}

/// 🧩 Artifact-keyed delta of the live presence rows.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceIndexPresenceDelta {
    pub added: Vec<SpaceIndexArtifactPresence>,
    pub removed: Vec<String>,
    pub patched: Vec<SpaceIndexPresencePatch>,
    pub reordered: Option<Vec<String>>,
}

/// 🔺️ Sparse field delta over [`SpaceIndexConfig`]; the directory slots are whole-entity replacements (their kind replaces exactly that
/// slice), the presence slot is a keyed row delta.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceIndexConfigDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<SpaceIndexMember>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub indexed_artifacts: Option<Vec<SpaceArtifactRow>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub presence: Option<SpaceIndexPresenceDelta>,
}

//#region 🧺️KeyedDelta
/// 🧺️ Id-keyed ordered-collection delta (`added`/`removed`/`patched`/`reordered`) and its algebra: apply, composition (create∘delete
/// cancels, delete∘create replaces, patch∘patch composes, patch∘create folds), negative delta and state delta.
pub trait KeyedDelta: Sized {
    type Row: Clone;
    type Patch: Clone;
    fn added(&self) -> &[Self::Row];
    fn removed(&self) -> &[String];
    fn patched(&self) -> &[Self::Patch];
    fn reordered(&self) -> Option<&[String]>;
    fn assemble(added: Vec<Self::Row>, removed: Vec<String>, patched: Vec<Self::Patch>, reordered: Option<Vec<String>>) -> Self;
    fn row_key(row: &Self::Row) -> &str;
    fn patch_key(patch: &Self::Patch) -> &str;
    fn patch_fold(patch: &Self::Patch, row: &mut Self::Row) -> Result<(), protocol::MutationApplyError>;
    fn patch_compose(first: &Self::Patch, later: &Self::Patch) -> Self::Patch;
    fn patch_inverse(patch: &Self::Patch, base: &Self::Row) -> Self::Patch;
    fn patch_between(base: &Self::Row, other: &Self::Row) -> Option<Self::Patch>;
    fn patch_is_empty(patch: &Self::Patch) -> bool;
}

fn keyed_error(code: &str, message: &str, at: [&str; 2]) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(code, message).at(at)
}

pub fn keyed_apply<D: KeyedDelta>(rows: &[D::Row], delta: &D) -> Result<Vec<D::Row>, protocol::MutationApplyError> {
    let key = D::row_key;
    for (index, id) in delta.removed().iter().enumerate() {
        if delta.removed()[..index].contains(id) {
            return Err(keyed_error("mutation.apply.duplicate-target", "row is removed more than once", ["removed", &index.to_string()]));
        }
        if !rows.iter().any(|row| key(row) == id) {
            return Err(keyed_error("mutation.apply.missing-target", "removed row does not exist", ["removed", &index.to_string()]));
        }
    }
    let mut next: Vec<D::Row> = rows.iter().filter(|row| !delta.removed().iter().any(|id| id == key(row))).cloned().collect();
    for (index, row) in delta.added().iter().enumerate() {
        if next.iter().any(|existing| key(existing) == key(row)) {
            return Err(keyed_error("mutation.apply.duplicate-target", "added row identity already exists", ["added", &index.to_string()]));
        }
        next.push(row.clone());
    }
    for (index, patch) in delta.patched().iter().enumerate() {
        if delta.patched()[..index].iter().any(|earlier| D::patch_key(earlier) == D::patch_key(patch)) {
            return Err(keyed_error("mutation.apply.duplicate-target", "row is patched more than once", ["patched", &index.to_string()]));
        }
        let row = next.iter_mut().find(|row| key(row) == D::patch_key(patch)).ok_or_else(|| keyed_error("mutation.apply.missing-target", "patched row does not exist", ["patched", &index.to_string()]))?;
        D::patch_fold(patch, row).map_err(|error| error.under(["patched".to_string(), index.to_string()]))?;
    }
    let Some(order) = delta.reordered() else { return Ok(next) };
    if order.len() != next.len() || order.iter().enumerate().any(|(index, id)| order[..index].contains(id) || !next.iter().any(|row| key(row) == id)) {
        return Err(protocol::MutationApplyError::new("mutation.apply.invalid-order", "reorder must be a complete unique permutation").at(["reordered".to_string()]));
    }
    Ok(order.iter().filter_map(|id| next.iter().find(|row| key(row) == id).cloned()).collect())
}

fn canonical<D: KeyedDelta>(mut added: Vec<D::Row>, mut removed: Vec<String>, mut patched: Vec<D::Patch>, reordered: Option<Vec<String>>) -> D {
    if let Some(order) = &reordered {
        added.sort_by_key(|row| order.iter().position(|id| id == D::row_key(row)).unwrap_or(usize::MAX));
    }
    let reordered = reordered.filter(|order| {
        let tail = added.len();
        !(order.len() <= tail + 1 && order.len() >= tail && order[order.len() - tail..].iter().map(String::as_str).eq(added.iter().map(D::row_key)))
    });
    removed.sort();
    removed.dedup();
    patched.retain(|patch| !D::patch_is_empty(patch));
    patched.sort_by(|left, right| D::patch_key(left).cmp(D::patch_key(right)));
    D::assemble(added, removed, patched, reordered)
}

/// ➕️ Normal form of `first` then `later`: create∘delete cancels, delete∘create replaces, patch∘patch composes, patch∘create folds.
pub fn keyed_absorb<D: KeyedDelta>(first: &D, later: &D) -> D {
    let mut added: Vec<D::Row> = first.added().to_vec();
    let mut removed: Vec<String> = first.removed().to_vec();
    let mut patched: Vec<D::Patch> = Vec::new();
    for patch in first.patched() {
        match added.iter_mut().find(|row| D::row_key(row) == D::patch_key(patch)) {
            Some(row) => {
                let _ = D::patch_fold(patch, row);
            }
            None => patched.push(patch.clone()),
        }
    }
    for id in later.removed() {
        if let Some(position) = added.iter().position(|row| D::row_key(row) == id) {
            added.remove(position);
        } else {
            patched.retain(|patch| D::patch_key(patch) != id);
            if !removed.contains(id) {
                removed.push(id.clone());
            }
        }
    }
    added.extend(later.added().iter().cloned());
    for patch in later.patched() {
        let key = D::patch_key(patch);
        if let Some(row) = added.iter_mut().find(|row| D::row_key(row) == key) {
            let _ = D::patch_fold(patch, row);
        } else if let Some(existing) = patched.iter_mut().find(|existing| D::patch_key(existing) == key) {
            *existing = D::patch_compose(existing, patch);
        } else {
            patched.push(patch.clone());
        }
    }
    let reordered = match (later.reordered(), first.reordered()) {
        (Some(order), _) => Some(order.to_vec()),
        (None, Some(order)) => Some(order.iter().filter(|id| !later.removed().contains(id)).cloned().chain(later.added().iter().map(|row| D::row_key(row).to_string())).collect()),
        (None, None) => None,
    };
    canonical::<D>(added, removed, patched, reordered)
}

fn ids_after<D: KeyedDelta>(base: &[String], delta: &D) -> Vec<String> {
    match delta.reordered() {
        Some(order) => order.to_vec(),
        None => base.iter().filter(|id| !delta.removed().contains(id)).cloned().chain(delta.added().iter().map(|row| D::row_key(row).to_string())).collect(),
    }
}

/// 🔁️ The negative delta: removes what `delta` added, restores what it removed, undoes its patches, restores the base order.
pub fn keyed_inverse<D: KeyedDelta>(delta: &D, base: &[D::Row]) -> D {
    let base_ids: Vec<String> = base.iter().map(|row| D::row_key(row).to_string()).collect();
    let removed: Vec<String> = delta.added().iter().map(|row| D::row_key(row).to_string()).collect();
    let added: Vec<D::Row> = delta.removed().iter().filter_map(|id| base.iter().find(|row| D::row_key(row) == id).cloned()).collect();
    let patched: Vec<D::Patch> = delta
        .patched()
        .iter()
        .filter(|patch| !removed.iter().any(|id| id == D::patch_key(patch)))
        .filter_map(|patch| base.iter().find(|row| D::row_key(row) == D::patch_key(patch)).map(|row| D::patch_inverse(patch, row)))
        .collect();
    let after = ids_after(&base_ids, delta);
    let natural: Vec<String> = after.iter().filter(|id| !removed.contains(id)).cloned().chain(added.iter().map(|row| D::row_key(row).to_string())).collect();
    let reordered = (natural != base_ids).then_some(base_ids);
    canonical::<D>(added, removed, patched, reordered)
}

/// 🧭️ The delta turning `base` into `other` (sync/import only).
pub fn keyed_between<D: KeyedDelta>(base: &[D::Row], other: &[D::Row]) -> D {
    let base_ids: Vec<String> = base.iter().map(|row| D::row_key(row).to_string()).collect();
    let other_ids: Vec<String> = other.iter().map(|row| D::row_key(row).to_string()).collect();
    let removed: Vec<String> = base_ids.iter().filter(|id| !other_ids.contains(id)).cloned().collect();
    let added: Vec<D::Row> = other.iter().filter(|row| !base_ids.iter().any(|id| id == D::row_key(row))).cloned().collect();
    let patched: Vec<D::Patch> = other.iter().filter_map(|row| base.iter().find(|candidate| D::row_key(candidate) == D::row_key(row)).and_then(|candidate| D::patch_between(candidate, row))).collect();
    let natural: Vec<String> = base_ids.iter().filter(|id| !removed.contains(id)).cloned().chain(added.iter().map(|row| D::row_key(row).to_string())).collect();
    let reordered = (natural != other_ids).then_some(other_ids);
    canonical::<D>(added, removed, patched, reordered)
}

pub fn keyed_is_empty<D: KeyedDelta>(delta: &D) -> bool {
    delta.added().is_empty() && delta.removed().is_empty() && delta.patched().iter().all(D::patch_is_empty) && delta.reordered().is_none()
}
//#endregion 🧺️KeyedDelta

impl KeyedDelta for SpaceIndexPresenceDelta {
    type Row = SpaceIndexArtifactPresence;
    type Patch = SpaceIndexPresencePatch;
    fn added(&self) -> &[SpaceIndexArtifactPresence] {
        &self.added
    }
    fn removed(&self) -> &[String] {
        &self.removed
    }
    fn patched(&self) -> &[SpaceIndexPresencePatch] {
        &self.patched
    }
    fn reordered(&self) -> Option<&[String]> {
        self.reordered.as_deref()
    }
    fn assemble(added: Vec<SpaceIndexArtifactPresence>, removed: Vec<String>, patched: Vec<SpaceIndexPresencePatch>, reordered: Option<Vec<String>>) -> Self {
        Self { added, removed, patched, reordered }
    }
    fn row_key(row: &SpaceIndexArtifactPresence) -> &str {
        &row.artifact_id
    }
    fn patch_key(patch: &SpaceIndexPresencePatch) -> &str {
        &patch.artifact_id
    }
    fn patch_fold(patch: &SpaceIndexPresencePatch, row: &mut SpaceIndexArtifactPresence) -> Result<(), protocol::MutationApplyError> {
        if let Some(actors_csv) = &patch.actors_csv {
            row.actors_csv = actors_csv.clone();
        }
        Ok(())
    }
    fn patch_compose(first: &SpaceIndexPresencePatch, later: &SpaceIndexPresencePatch) -> SpaceIndexPresencePatch {
        SpaceIndexPresencePatch { artifact_id: first.artifact_id.clone(), actors_csv: later.actors_csv.clone().or_else(|| first.actors_csv.clone()) }
    }
    fn patch_inverse(patch: &SpaceIndexPresencePatch, base: &SpaceIndexArtifactPresence) -> SpaceIndexPresencePatch {
        SpaceIndexPresencePatch { artifact_id: patch.artifact_id.clone(), actors_csv: patch.actors_csv.as_ref().map(|_| base.actors_csv.clone()) }
    }
    fn patch_between(base: &SpaceIndexArtifactPresence, other: &SpaceIndexArtifactPresence) -> Option<SpaceIndexPresencePatch> {
        (base.actors_csv != other.actors_csv).then(|| SpaceIndexPresencePatch { artifact_id: other.artifact_id.clone(), actors_csv: Some(other.actors_csv.clone()) })
    }
    fn patch_is_empty(patch: &SpaceIndexPresencePatch) -> bool {
        patch.actors_csv.is_none()
    }
}

impl protocol::MutationDiff<SpaceIndexConfig> for SpaceIndexConfigDiff {
    fn apply(&self, base: &SpaceIndexConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SpaceIndexConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.visibility {
            next.visibility = value.clone();
        }
        if let Some(value) = &self.members {
            next.members = value.clone();
        }
        if let Some(value) = &self.indexed_artifacts {
            next.indexed_artifacts = value.clone();
        }
        if let Some(delta) = &self.presence {
            next.presence = keyed_apply(&next.presence, delta).map_err(|error| error.under(["presence"]))?;
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.visibility.is_some() {
            self.visibility = other.visibility;
        }
        if other.members.is_some() {
            self.members = other.members;
        }
        if other.indexed_artifacts.is_some() {
            self.indexed_artifacts = other.indexed_artifacts;
        }
        self.presence = match (self.presence.take(), other.presence) {
            (Some(first), Some(later)) => Some(keyed_absorb(&first, &later)),
            (first, later) => later.or(first),
        };
    }
}

impl protocol::DiffAlgebra<SpaceIndexConfig> for SpaceIndexConfigDiff {
    fn inverse(&self, base: &SpaceIndexConfig) -> Self {
        Self {
            visibility: self.visibility.as_ref().map(|_| base.visibility.clone()),
            members: self.members.as_ref().map(|_| base.members.clone()),
            indexed_artifacts: self.indexed_artifacts.as_ref().map(|_| base.indexed_artifacts.clone()),
            presence: self.presence.as_ref().map(|delta| keyed_inverse(delta, &base.presence)),
        }
    }
    fn between(base: &SpaceIndexConfig, other: &SpaceIndexConfig) -> Self {
        let presence = keyed_between::<SpaceIndexPresenceDelta>(&base.presence, &other.presence);
        Self {
            visibility: (base.visibility != other.visibility).then(|| other.visibility.clone()),
            members: (base.members != other.members).then(|| other.members.clone()),
            indexed_artifacts: (base.indexed_artifacts != other.indexed_artifacts).then(|| other.indexed_artifacts.clone()),
            presence: (!keyed_is_empty(&presence)).then_some(presence),
        }
    }
    fn is_empty(&self) -> bool {
        self.visibility.is_none() && self.members.is_none() && self.indexed_artifacts.is_none() && self.presence.as_ref().is_none_or(keyed_is_empty)
    }
}
//#endregion 🔖️Config

//#region 🔖️ConfigMutation
/// 🧮️ The config's mutation vocabulary: the host-folded directory slice replaces as one entity, and each presence heartbeat sets
/// (or clears) the live actors of one artifact.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum)]
pub enum SpaceIndexConfigMutation {
    #[dsl(key = "directory-projection")]
    ReplaceDirectoryProjection {
        #[dsl(block)]
        projection: SpaceIndexDirectoryProjection,
    },
    #[dsl(key = "artifact-presence")]
    SetArtifactPresence { artifact_id: String, actors_csv: String },
    #[dsl(key = "clear-artifact-presence")]
    ClearArtifactPresence { artifact_id: String },
}

impl Mutation<SpaceIndexConfig> for SpaceIndexConfigMutation {
    type Diff = SpaceIndexConfigDiff;

    /// 🧷️ Provisional per-variant leaf metadata for this hand-written (non-derived) aggregate — one
    /// entry per variant, in declaration order.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️replace-directory-projection",
            semantic_kind: "replace-directory-projection",
            display_name: "Replace Directory Projection",
            emoji: "⚙️",
            aggregate_variant: "ReplaceDirectoryProjection",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-artifact-presence",
            semantic_kind: "set-artifact-presence",
            display_name: "Set Artifact Presence",
            emoji: "⚙️",
            aggregate_variant: "SetArtifactPresence",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️clear-artifact-presence",
            semantic_kind: "clear-artifact-presence",
            display_name: "Clear Artifact Presence",
            emoji: "⚙️",
            aggregate_variant: "ClearArtifactPresence",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Self::ReplaceDirectoryProjection { .. } => &Self::DESCRIPTORS[0],
            Self::SetArtifactPresence { .. } => &Self::DESCRIPTORS[1],
            Self::ClearArtifactPresence { .. } => &Self::DESCRIPTORS[2],
        }
    }

    fn diff(&self, base: &SpaceIndexConfig) -> protocol::MutationOutcome<SpaceIndexConfigDiff> {
        protocol::MutationOutcome::new(match self {
            Self::ReplaceDirectoryProjection { projection } => SpaceIndexConfigDiff {
                visibility: (base.visibility != projection.visibility).then(|| projection.visibility.clone()),
                members: (base.members != projection.members).then(|| projection.members.clone()),
                indexed_artifacts: (base.indexed_artifacts != projection.indexed_artifacts).then(|| projection.indexed_artifacts.clone()),
                presence: None,
            },
            Self::SetArtifactPresence { artifact_id, actors_csv } => {
                let delta = match base.presence.iter().find(|row| row.artifact_id == *artifact_id) {
                    Some(row) if row.actors_csv == *actors_csv => None,
                    Some(_) => Some(SpaceIndexPresenceDelta { patched: vec![SpaceIndexPresencePatch { artifact_id: artifact_id.clone(), actors_csv: Some(actors_csv.clone()) }], ..Default::default() }),
                    None => Some(SpaceIndexPresenceDelta { added: vec![SpaceIndexArtifactPresence { artifact_id: artifact_id.clone(), actors_csv: actors_csv.clone() }], ..Default::default() }),
                };
                SpaceIndexConfigDiff { presence: delta, ..Default::default() }
            }
            Self::ClearArtifactPresence { artifact_id } => SpaceIndexConfigDiff { presence: base.presence.iter().any(|row| row.artifact_id == *artifact_id).then(|| SpaceIndexPresenceDelta { removed: vec![artifact_id.clone()], ..Default::default() }), ..Default::default() },
        })
    }

    fn inverse(&self, base: &SpaceIndexConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(match self {
            Self::ReplaceDirectoryProjection { .. } => vec![Self::ReplaceDirectoryProjection { projection: SpaceIndexDirectoryProjection { visibility: base.visibility.clone(), members: base.members.clone(), indexed_artifacts: base.indexed_artifacts.clone() } }],
            Self::SetArtifactPresence { artifact_id, .. } | Self::ClearArtifactPresence { artifact_id } => match base.presence.iter().find(|row| row.artifact_id == *artifact_id) {
                Some(row) => vec![Self::SetArtifactPresence { artifact_id: artifact_id.clone(), actors_csv: row.actors_csv.clone() }],
                None if matches!(self, Self::SetArtifactPresence { .. }) => vec![Self::ClearArtifactPresence { artifact_id: artifact_id.clone() }],
                None => Vec::new(),
            },
        })
    }
}

//#region 🔖️OpCodec
impl protocol::OpText for SpaceIndexConfigMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown mutation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

impl protocol::OpBinary for SpaceIndexConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1.ordinary)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = (spec_fn.ordinary)();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}
//#endregion 🔖️OpCodec
//#endregion 🔖️ConfigMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "🧬️schema/🦀️.rs"]
pub mod schema;
