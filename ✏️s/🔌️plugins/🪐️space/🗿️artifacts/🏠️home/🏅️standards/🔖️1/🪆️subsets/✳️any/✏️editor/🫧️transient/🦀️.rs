//! 🫧️ S Home launcher — app-local transient state: the folded hub directory projection both Home surfaces list.
//!
//! The directory projection is DERIVED hub state. The host re-bootstraps it from the hub's origin (`after = 0`) every time a
//! Home surface opens (`🏛️ShellHost/📇️directory-bootstrap`), so it never has to survive a reload and never belongs in
//! undoable history. It lives in the app's TRANSIENT lane (`store::TransientStore`: one `Arc` root, never persisted,
//! shared or undone), read by every window of one Home instance, by the editor and by the read-only viewer.
//!
//! Each authenticated page is ONE bounded, non-invertible transient item ([`mutations::ApplyDirectoryPage`], at most one
//! canonical page of `store::os_directory::DIRECTORY_EVENT_PAGE_MAX_BYTES`). Its publication cost is the page, never the
//! projection, so a directory of any size folds page by page under the one-item bound and no edit or ledger row is ever
//! written for it. The projection is typed rows behind `Arc`s — one immutable row per space — folded by the canonical
//! `store::os_directory::fold` one row at a time with copy-on-write of exactly the rows a page touches. A retained job
//! reads the one row it needs from the root captured at its dispatch; a later page publishes a NEW root and never mixes
//! into it. No contiguous encoding of the whole directory is built on the hot path: the text and pack codecs below exist
//! for tooling and fixtures only.
//! Ticket 26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP (SH2 14c: the config lane sealed the whole projection with its
//! inverse copy and refused `Space Home config publication exceeds its complete retained envelope` past ~1 MiB).
//! @see ../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🦀️.rs `fold`

use semio_framework_plugin::{Fault, FaultOrigin};
use std::collections::BTreeMap;
use std::sync::Arc;

//#region 🔖️Receipt
/// 🧾️ Exact terminal proof that one authenticated directory frontier is the Home transient projection.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectoryProjectionReceiptV1 {
    pub schema: String,
    pub session_binding_sha256: String,
    pub authorization_generation: u64,
    pub through_seq_inclusive: u64,
    pub receipt_sha256: String,
}

impl DirectoryProjectionReceiptV1 {
    pub const SCHEMA: &'static str = "semio.space.home.directory-projection-receipt.v1";

    /// 🛡️ Validates the complete browser-visible receipt without admitting resume-only fields.
    pub fn validate(&self) -> bool {
        self.schema == Self::SCHEMA
            && directory_sha256_is_valid(&self.session_binding_sha256)
            && self.authorization_generation > 0
            && self.authorization_generation <= store::os_directory::DOCUMENT_OPEN_MAX_SAFE_INTEGER
            && self.through_seq_inclusive <= store::os_directory::DOCUMENT_OPEN_MAX_SAFE_INTEGER
            && directory_sha256_is_valid(&self.receipt_sha256)
    }

    /// 🧾️ The receipt of one sealed page: the frontier the projection holds once that page is folded.
    pub fn of_page(page: &store::os_directory::DirectoryEventPageV1) -> Self {
        Self {
            schema: Self::SCHEMA.into(),
            session_binding_sha256: page.session_binding_sha256.clone(),
            authorization_generation: page.authorization_generation,
            through_seq_inclusive: page.through_seq_inclusive,
            receipt_sha256: page.receipt_sha256.clone(),
        }
    }
}

fn directory_sha256_is_valid(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}
//#endregion 🔖️Receipt

//#region 🔖️Projection
/// 🚦️ How the projection answers one sealed page before anything is folded.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectoryPageAdmission {
    /// 🟰️ The projection already holds exactly this frontier: answer its receipt, fold nothing.
    Held,
    /// 📄️ The page continues the held frontier, or restarts the projection from the origin.
    Fold,
}

/// 📇️ The folded hub directory: one immutable `Arc` row per space in id order, the user side table the member rows join,
/// the last folded sequence and the authenticated resume authority (session binding, authorization generation, receipt)
/// of the last folded page. All empty before the first page.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HomeDirectoryProjection {
    session_binding_sha256: String,
    authorization_generation: u64,
    receipt_sha256: String,
    cursor: u64,
    spaces: BTreeMap<Arc<str>, Arc<store::os_directory::DirectorySpace>>,
    users: Arc<BTreeMap<String, store::os_directory::UserView>>,
}

/// 🏷️ The one space a directory event addresses, or `None` for an event that touches no space row.
fn directory_event_space(body: &store::os_directory::DirectoryEventBody) -> Option<&str> {
    use store::os_directory::DirectoryEventBody as Body;
    match body {
        Body::SpaceCreated { space_id, .. }
        | Body::SpaceRenamed { space_id, .. }
        | Body::SpaceVisibilityChanged { space_id, .. }
        | Body::SpaceArchived { space_id }
        | Body::SpaceDeleted { space_id }
        | Body::MemberUpserted { space_id, .. }
        | Body::MemberRemoved { space_id, .. }
        | Body::InviteRedeemed { space_id, .. } => Some(space_id),
        Body::DocumentAnnounced { descriptor } => Some(&descriptor.space_id),
        Body::DocumentIndexed { scope, .. } => Some(&scope.space_id),
        Body::UserCreated { .. } | Body::ArtifactCheckpointPublished { .. } | Body::ArtifactRetentionAdvanced { .. } | Body::UserPreferenceRecorded { .. } => None,
    }
}

/// 🙋️ The one user a directory event reads (a member join) or writes (`user.created`) in the side table.
fn directory_event_user(body: &store::os_directory::DirectoryEventBody) -> Option<&str> {
    use store::os_directory::DirectoryEventBody as Body;
    match body {
        Body::UserCreated { user_id, .. } | Body::MemberUpserted { user_id, .. } | Body::InviteRedeemed { user_id, .. } => Some(user_id),
        _ => None,
    }
}

impl HomeDirectoryProjection {
    /// 🔢️ The last folded directory sequence.
    pub fn cursor(&self) -> u64 {
        self.cursor
    }

    /// 🔢️ How many spaces the projection lists.
    pub fn space_count(&self) -> usize {
        self.spaces.len()
    }

    /// 🏠️ Every folded space, in id order.
    pub fn spaces(&self) -> impl Iterator<Item = &store::os_directory::DirectorySpace> {
        self.spaces.values().map(Arc::as_ref)
    }

    /// 🔎️ One folded space by id.
    pub fn space(&self, space_id: &str) -> Option<&store::os_directory::DirectorySpace> {
        self.spaces.get(space_id).map(Arc::as_ref)
    }

    /// 🧵️ One folded space row as a shared handle — what a retained job holds when it needs one row, never the directory.
    pub fn space_row(&self, space_id: &str) -> Option<Arc<store::os_directory::DirectorySpace>> {
        self.spaces.get(space_id).cloned()
    }

    /// 🧾️ The receipt of the frontier the projection holds, or `None` before the first authenticated page.
    pub fn receipt(&self) -> Option<DirectoryProjectionReceiptV1> {
        let receipt = DirectoryProjectionReceiptV1 {
            schema: DirectoryProjectionReceiptV1::SCHEMA.into(),
            session_binding_sha256: self.session_binding_sha256.clone(),
            authorization_generation: self.authorization_generation,
            through_seq_inclusive: self.cursor,
            receipt_sha256: self.receipt_sha256.clone(),
        };
        receipt.validate().then_some(receipt)
    }

    /// 🚦️ Judges one validated page against the held frontier. A page from the origin (`after_seq_exclusive == 0`) rebuilds
    /// the projection under any authority: the reader's visible set can change retroactively (a human added to an existing
    /// space sees that space's earlier events), so the worker re-reads from the origin on the hub's `access-changed` signal.
    /// A later page must continue the held frontier of the same authority.
    pub fn admit_page(&self, page: &store::os_directory::DirectoryEventPageV1) -> Result<DirectoryPageAdmission, Fault> {
        let same_authority = self.session_binding_sha256 == page.session_binding_sha256 && self.authorization_generation == page.authorization_generation;
        if same_authority && self.cursor == page.through_seq_inclusive && self.receipt_sha256 == page.receipt_sha256 {
            return Ok(DirectoryPageAdmission::Held);
        }
        if page.after_seq_exclusive == 0 {
            return Ok(DirectoryPageAdmission::Fold);
        }
        if !same_authority {
            return Err(Fault::new(FaultOrigin::App, "s.home.directory-event-page-rebootstrap-required", "a page of another authority must restart from the origin"));
        }
        if page.after_seq_exclusive != self.cursor {
            return Err(Fault::new(FaultOrigin::App, "s.home.directory-event-page-frontier-race", "the page does not continue the held frontier"));
        }
        Ok(DirectoryPageAdmission::Fold)
    }

    /// 📄️ The projection after one admitted page: rebuilt from nothing for an origin page, otherwise this projection's
    /// rows with exactly the rows the page's events address replaced, the frontier advanced to the page's own.
    pub fn fold_page(&self, page: &store::os_directory::DirectoryEventPageV1) -> Self {
        let mut next = if page.after_seq_exclusive == 0 { Self::default() } else { self.clone() };
        for event in &page.events {
            next.fold_event(event);
        }
        next.cursor = page.through_seq_inclusive;
        next.session_binding_sha256 = page.session_binding_sha256.clone();
        next.authorization_generation = page.authorization_generation;
        next.receipt_sha256 = page.receipt_sha256.clone();
        next
    }

    /// 🧮️ Folds one event through the canonical `store::os_directory::fold` over a one-row read model: the addressed row
    /// (moved out, cloned only when an older root still shares it) and the one user the event joins or creates.
    fn fold_event(&mut self, event: &store::os_directory::DirectoryEvent) {
        let space_id = directory_event_space(&event.body);
        let user_id = directory_event_user(&event.body);
        let mut model = store::os_directory::DirectoryReadModel { spaces: BTreeMap::new(), cursor: self.cursor, users: BTreeMap::new() };
        if let Some((id, row)) = space_id.and_then(|id| self.spaces.remove(id).map(|row| (id, row))) {
            model.spaces.insert(id.to_owned(), Arc::unwrap_or_clone(row));
        }
        if let Some(user) = user_id.and_then(|id| self.users.get(id)) {
            model.users.insert(user.id.clone(), user.clone());
        }
        let folded = store::os_directory::fold(model, event);
        self.cursor = folded.cursor;
        for (id, space) in folded.spaces {
            self.spaces.insert(Arc::from(id.as_str()), Arc::new(space));
        }
        if let store::os_directory::DirectoryEventBody::UserCreated { user_id, .. } = &event.body {
            if let Some(user) = folded.users.get(user_id) {
                Arc::make_mut(&mut self.users).insert(user_id.clone(), user.clone());
            }
        }
    }

    /// 📏️ The heap this projection retains, estimated per row without encoding it — what its retirement pages through.
    pub fn retained_bytes(&self) -> usize {
        const ROW_BYTES: usize = 256;
        const MEMBER_BYTES: usize = 128;
        const DOCUMENT_BYTES: usize = 512;
        const USER_BYTES: usize = 128;
        let rows = self.spaces.values().map(|space| ROW_BYTES + space.view.id.len() + space.view.name.len() + space.members.len() * MEMBER_BYTES + (space.documents.len() + space.indexed_documents.len()) * DOCUMENT_BYTES).sum::<usize>();
        rows + self.users.len() * USER_BYTES + self.session_binding_sha256.len() + self.receipt_sha256.len() + 16
    }

    fn resume_state_is_valid(&self) -> bool {
        (self.session_binding_sha256.is_empty() && self.authorization_generation == 0 && self.receipt_sha256.is_empty())
            || (self.authorization_generation > 0
                && self.authorization_generation <= store::os_directory::DOCUMENT_OPEN_MAX_SAFE_INTEGER
                && directory_sha256_is_valid(&self.session_binding_sha256)
                && directory_sha256_is_valid(&self.receipt_sha256))
    }
}
//#endregion 🔖️Projection

//#region 🔖️Transient
/// 🫧️ `HomeApp::Transient` / `HomeViewer::Transient` — the folded directory behind one `Arc`, so the transient store's own
/// root copies and a retained job's captured snapshot share it instead of copying it.
#[derive(Clone, Debug, Default)]
pub struct HomeTransient {
    directory: Arc<HomeDirectoryProjection>,
}

impl PartialEq for HomeTransient {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.directory, &other.directory) || self.directory == other.directory
    }
}

impl HomeTransient {
    /// 🫧️ A transient holding `directory`.
    pub fn with_directory(directory: HomeDirectoryProjection) -> Self {
        Self { directory: Arc::new(directory) }
    }

    /// 📇️ The folded hub directory.
    pub fn directory(&self) -> &HomeDirectoryProjection {
        &self.directory
    }
}
//#endregion 🔖️Transient

//#region 🔖️Wire
/// 📇️ One space row in the framework's camel-case `DirectoryReadModel` wire (`📇️directory/🧬️schema/🔣️.json`).
#[derive(value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct DirectorySpaceWire {
    view: store::os_directory::SpaceView,
    members: Vec<store::os_directory::MemberView>,
    documents: Vec<store::os_directory::DocumentDescriptor>,
    indexed_documents: Vec<store::os_directory::DirectoryIndexedDocumentViewV1>,
}

/// 📇️ The framework's `DirectoryReadModel` wire: spaces keyed by id, the folded cursor and the user side table.
#[derive(value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct DirectoryReadModelWire {
    spaces: BTreeMap<String, DirectorySpaceWire>,
    cursor: u64,
    users: BTreeMap<String, store::os_directory::UserView>,
}

/// 🫧️ The transient's JSON projection (`🧬️schema/🔣️.json` `HomeTransient`): the resume authority plus the directory wire.
#[derive(value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct HomeTransientWire {
    session_binding_sha256: String,
    authorization_generation: u64,
    receipt_sha256: String,
    directory: DirectoryReadModelWire,
}

impl HomeTransient {
    fn wire(&self) -> HomeTransientWire {
        let directory = &self.directory;
        HomeTransientWire {
            session_binding_sha256: directory.session_binding_sha256.clone(),
            authorization_generation: directory.authorization_generation,
            receipt_sha256: directory.receipt_sha256.clone(),
            directory: DirectoryReadModelWire {
                spaces: directory
                    .spaces
                    .iter()
                    .map(|(id, space)| (id.to_string(), DirectorySpaceWire { view: space.view.clone(), members: space.members.clone(), documents: space.documents.clone(), indexed_documents: space.indexed_documents.clone() }))
                    .collect(),
                cursor: directory.cursor,
                users: directory.users.as_ref().clone(),
            },
        }
    }

    fn from_wire(wire: HomeTransientWire) -> Result<Self, protocol::ValueError> {
        let mut spaces = BTreeMap::new();
        for (id, space) in wire.directory.spaces {
            if space.view.id != id {
                return Err(protocol::ValueError::new(format!("s.home.directory-projection-malformed: space row {id} carries the id {}", space.view.id)));
            }
            spaces.insert(Arc::from(id.as_str()), Arc::new(store::os_directory::DirectorySpace { view: space.view, members: space.members, documents: space.documents, indexed_documents: space.indexed_documents }));
        }
        let directory = HomeDirectoryProjection {
            session_binding_sha256: wire.session_binding_sha256,
            authorization_generation: wire.authorization_generation,
            receipt_sha256: wire.receipt_sha256,
            cursor: wire.directory.cursor,
            spaces,
            users: Arc::new(wire.directory.users),
        };
        if !directory.resume_state_is_valid() {
            return Err(protocol::ValueError::new("s.home.directory-projection-malformed: the resume authority is neither unbound nor a complete authenticated frontier"));
        }
        Ok(Self::with_directory(directory))
    }
}

impl protocol::ToValue for HomeTransient {
    fn to_value(&self) -> protocol::DslValue {
        protocol::ToValue::to_value(&self.wire())
    }
}

impl protocol::FromValue for HomeTransient {
    fn from_value(value: protocol::DslValue) -> Result<Self, protocol::ValueError> {
        Self::from_wire(<HomeTransientWire as protocol::FromValue>::from_value(value)?)
    }
}
//#endregion 🔖️Wire

//#region 🔖️ArtifactCodec
/// 📜️ The transient's text/pack record: its JSON projection as one field. Tooling and fixtures only — the transient
/// lane itself never encodes its root.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(id = "home.transient")]
#[dsl(extension = "hometransient")]
#[dsl(layout = "lines")]
struct HomeTransientRecord {
    projection_json: String,
}

impl HomeTransientRecord {
    fn of(transient: &HomeTransient) -> Self {
        Self { projection_json: pack::to_json_string(&transient.wire()) }
    }

    fn transient(&self) -> Result<HomeTransient, store::TextError> {
        let wire: HomeTransientWire = pack::from_json_str(&self.projection_json).map_err(|error| dsl::__rt::field_error(format!("s.home.directory-projection-malformed: {error}")))?;
        HomeTransient::from_wire(wire).map_err(|error| dsl::__rt::field_error(error.to_string()))
    }
}

impl store::ArtifactDsl for HomeTransient {
    const EXTENSION: &'static str = HomeTransientRecord::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        "home.transient"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &HomeTransientRecord::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        HomeTransientRecord::__dsl_from_record(&record)?.transient()
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&HomeTransientRecord::of(self).__dsl_to_record(), &HomeTransientRecord::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Home transient envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for HomeTransient {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&HomeTransientRecord::__dsl_spec(), &HomeTransientRecord::of(self).__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &HomeTransientRecord::__dsl_spec(), options)?;
        HomeTransientRecord::__dsl_from_record(&record).and_then(|record| record.transient()).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(HomeTransientRecord::__dsl_spec())
    }
}
//#endregion 🔖️ArtifactCodec

//#region 🔖️Retirement
/// 🧹️ Retires a displaced or disposed Home transient root in byte grants measured by
/// [`HomeDirectoryProjection::retained_bytes`] — never by encoding the projection, which would build one contiguous copy
/// of the whole directory for every page the lane publishes.
pub struct HomeTransientRetirementFactory;

struct HomeTransientRootRetirement {
    root: Option<Arc<HomeTransient>>,
    retained_bytes: usize,
}

impl store::ErasedSnapshotRetirement for HomeTransientRootRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.retained_bytes > maximum_bytes {
            self.retained_bytes -= maximum_bytes;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: maximum_bytes });
        }
        if self.root.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.root.is_none()
    }
}

impl store::SnapshotRetirementFactory<HomeTransient> for HomeTransientRetirementFactory {
    fn retire(&self, snapshot: Arc<HomeTransient>) -> Box<dyn store::ErasedSnapshotRetirement> {
        let retained_bytes = snapshot.directory().retained_bytes();
        Box::new(HomeTransientRootRetirement { root: Some(snapshot), retained_bytes })
    }
}

impl store::ArtifactOwnedValueRetirementFactory<HomeTransient> for HomeTransientRetirementFactory {
    fn retire_owned(&self, value: HomeTransient) -> Box<dyn store::ErasedSnapshotRetirement> {
        store::SnapshotRetirementFactory::retire(self, Arc::new(value))
    }
}
//#endregion 🔖️Retirement

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub use mutations::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
