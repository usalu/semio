//! 🔎️ Mutation leaf source scope: the provenance, domain operation and owner layout records, the source validator, the validated scope and the leaf trait. See [`super`].

use super::*;

//#region 🪪️MutationLeaf
/// 🧭️ Compile-time source facts for one direct mutation leaf; excluded from descriptor wire identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MutationSourceProvenance {
    pub workspace_token: [u8; 32],
    pub mutation_root: &'static str,
    pub owner: &'static str,
    pub source_path: &'static str,
    pub descriptor_path: &'static str,
    pub taxonomy_path: &'static str,
}

/// 🏘️ One explicitly registered physical domain-operation owner and its unchanged semantic identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MutationDomainOperation {
    pub owner: &'static str,
    pub semantic_kind: &'static str,
}

/// 🗺️ Exact aggregate layout authority; domain owners never infer identities or accept arbitrary descendants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MutationOwnerLayout {
    Flat,
    DomainOperations(&'static [MutationDomainOperation]),
}

/// 🌿️ Aggregate-owned source facts used to validate one explicitly owned mutation leaf.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MutationLeafSourceScope {
    pub workspace_token: [u8; 32],
    pub mutation_root: &'static str,
    pub owner_layout: MutationOwnerLayout,
    pub taxonomy_path: &'static str,
    pub mutation_payload_facet: &'static str,
    pub source_filename: &'static str,
    pub descriptor_filename: &'static str,
}

/// 🚫️ Identifies one aggregate-source fact a mutation leaf does not prove.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MutationLeafSourceValidationError {
    pub field: &'static str,
    pub requirement: &'static str,
}

/// ⚖️ Validates a direct leaf's descriptor and provenance against aggregate-owned source facts.
pub const fn validate_mutation_leaf_source(
    descriptor: &MutationLeafDescriptor,
    provenance: &MutationSourceProvenance,
    scope: &MutationLeafSourceScope,
) -> Result<(), MutationLeafSourceValidationError> {
    if let Err(error) = validate_mutation_leaf_descriptor(descriptor) {
        return Err(MutationLeafSourceValidationError { field: error.field, requirement: error.requirement });
    }
    match scope.validate() {
        Ok(checked) => checked.validate_leaf(descriptor, provenance),
        Err(error) => Err(error),
    }
}

/// 🪪️ Aggregate authority whose root, filenames and physical layout were validated once.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValidatedMutationLeafSourceScope {
    pub(super) scope: MutationLeafSourceScope,
}

impl MutationLeafSourceScope {
    /// ☑️ Checks shared aggregate facts before validating each leaf against the same immutable authority.
    pub const fn validate(&self) -> Result<ValidatedMutationLeafSourceScope, MutationLeafSourceValidationError> {
        let scope = self;
        if !mutation_leaf_descriptor_root(scope.mutation_root) || !mutation_leaf_source_path(scope.mutation_root) {
            return Err(MutationLeafSourceValidationError { field: "mutationRoot", requirement: "must be a safe normalized mutation root" });
        }
        if !mutation_leaf_source_path(scope.taxonomy_path) {
            return Err(MutationLeafSourceValidationError { field: "taxonomyPath", requirement: "must be a safe normalized portable path" });
        }
        if !mutation_leaf_source_filename(scope.mutation_payload_facet) {
            return Err(MutationLeafSourceValidationError { field: "mutationPayloadFacet", requirement: "must be one safe normalized portable segment" });
        }
        if !mutation_leaf_source_filename(scope.source_filename) {
            return Err(MutationLeafSourceValidationError { field: "sourceFilename", requirement: "must be a safe normalized portable filename" });
        }
        if !mutation_leaf_source_filename(scope.descriptor_filename) {
            return Err(MutationLeafSourceValidationError { field: "descriptorFilename", requirement: "must be a safe normalized portable filename" });
        }
        if !mutation_owner_layout_valid(scope.mutation_root, scope.owner_layout) {
            return Err(MutationLeafSourceValidationError { field: "owner", requirement: "must match the aggregate's exact physical owner and semantic identity layout" });
        }
        Ok(ValidatedMutationLeafSourceScope { scope: *scope })
    }
}

impl ValidatedMutationLeafSourceScope {
    /// ✅️ Checks each descriptor and its exact source provenance against the validated aggregate.
    pub const fn validate_leaf(&self, descriptor: &MutationLeafDescriptor, provenance: &MutationSourceProvenance) -> Result<(), MutationLeafSourceValidationError> {
        let scope = &self.scope;
        if let Err(error) = validate_mutation_leaf_descriptor(descriptor) {
            return Err(MutationLeafSourceValidationError { field: error.field, requirement: error.requirement });
        }
        if !mutation_leaf_source_path(descriptor.owner) {
            return Err(MutationLeafSourceValidationError { field: "owner", requirement: "must be a safe normalized portable path" });
        }
        if !mutation_owner_layout_identity_matches(scope.mutation_root, scope.owner_layout, descriptor.owner, descriptor.semantic_kind) {
            return Err(MutationLeafSourceValidationError { field: "owner", requirement: "must match the aggregate's exact physical owner and semantic identity layout" });
        }
        if !mutation_leaf_source_tokens_match(&scope.workspace_token, &provenance.workspace_token) {
            return Err(MutationLeafSourceValidationError { field: "workspaceToken", requirement: "must equal the aggregate workspace token" });
        }
        if !mutation_leaf_descriptor_str_eq(provenance.mutation_root, scope.mutation_root) {
            return Err(MutationLeafSourceValidationError { field: "mutationRoot", requirement: "must equal the aggregate mutation root" });
        }
        if !mutation_leaf_descriptor_str_eq(provenance.owner, descriptor.owner) {
            return Err(MutationLeafSourceValidationError { field: "owner", requirement: "must equal the descriptor owner" });
        }
        if !mutation_leaf_descriptor_str_eq(provenance.taxonomy_path, scope.taxonomy_path) {
            return Err(MutationLeafSourceValidationError { field: "taxonomyPath", requirement: "must equal the aggregate taxonomy path" });
        }
        if !mutation_leaf_source_path_matches_direct_or_payload(descriptor.owner, scope.mutation_payload_facet, scope.source_filename, provenance.source_path) {
            return Err(MutationLeafSourceValidationError { field: "sourcePath", requirement: "must equal owner plus the canonical source filename, directly or under the canonical payload facet" });
        }
        if !mutation_leaf_source_path_matches(descriptor.owner, scope.descriptor_filename, provenance.descriptor_path) {
            return Err(MutationLeafSourceValidationError { field: "descriptorPath", requirement: "must equal owner plus the canonical descriptor filename" });
        }
        Ok(())
    }
}

pub(super) const fn mutation_leaf_source_path(value: &str) -> bool {
    let bytes = value.as_bytes();
    mutation_leaf_descriptor_relative_path(bytes) && mutation_leaf_descriptor_path_safe(bytes) && mutation_leaf_source_no_compose_ascii_case(bytes)
}

const fn mutation_leaf_source_filename(value: &str) -> bool {
    let bytes = value.as_bytes();
    if !mutation_leaf_source_path(value) { return false; }
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'/' { return false; }
        index += 1;
    }
    true
}

const fn mutation_leaf_source_no_compose_ascii_case(bytes: &[u8]) -> bool {
    let mut index = 0;
    while index < bytes.len() {
        if (index == 0 || bytes[index - 1] == b'/') && mutation_leaf_source_bytes_at_ascii_case(bytes, index, b"compose") && (index + 7 == bytes.len() || bytes[index + 7] == b'/') { return false; }
        index += 1;
    }
    true
}

const fn mutation_leaf_source_bytes_at_ascii_case(bytes: &[u8], start: usize, expected: &[u8]) -> bool {
    if start + expected.len() > bytes.len() { return false; }
    let mut index = 0;
    while index < expected.len() {
        let byte = bytes[start + index];
        let folded = if byte >= b'A' && byte <= b'Z' { byte + (b'a' - b'A') } else { byte };
        if folded != expected[index] { return false; }
        index += 1;
    }
    true
}

const fn mutation_leaf_source_tokens_match(left: &[u8; 32], right: &[u8; 32]) -> bool {
    let mut index = 0;
    while index < 32 {
        if left[index] != right[index] { return false; }
        index += 1;
    }
    true
}

const fn mutation_leaf_source_path_matches(owner: &str, filename: &str, value: &str) -> bool {
    let owner = owner.as_bytes();
    let filename = filename.as_bytes();
    let value = value.as_bytes();
    value.len() == owner.len() + 1 + filename.len()
        && mutation_leaf_descriptor_bytes_at(value, 0, owner)
        && value[owner.len()] == b'/'
        && mutation_leaf_descriptor_bytes_at(value, owner.len() + 1, filename)
}

const fn mutation_leaf_source_path_matches_direct_or_payload(owner: &str, payload_facet: &str, filename: &str, value: &str) -> bool {
    if mutation_leaf_source_path_matches(owner, filename, value) { return true; }
    let owner = owner.as_bytes();
    let payload_facet = payload_facet.as_bytes();
    let filename = filename.as_bytes();
    let value = value.as_bytes();
    value.len() == owner.len() + 1 + payload_facet.len() + 1 + filename.len()
        && mutation_leaf_descriptor_bytes_at(value, 0, owner)
        && value[owner.len()] == b'/'
        && mutation_leaf_descriptor_bytes_at(value, owner.len() + 1, payload_facet)
        && value[owner.len() + 1 + payload_facet.len()] == b'/'
        && mutation_leaf_descriptor_bytes_at(value, owner.len() + 1 + payload_facet.len() + 1, filename)
}

/// 🎚️ Ownership contract for a direct mutation leaf, and the editable payload of one of its operations.
pub trait MutationLeaf {
    const DESCRIPTOR: MutationLeafDescriptor;
    const PROVENANCE: MutationSourceProvenance;
    /// 🧬️ The leaf's normative payload JSON Schema text (`DESCRIPTOR.payload_schema`, embedded at compile time by
    /// `#[derive(dsl::MutationLeaf)]`) — the source of its input descriptors (`manifest::mutation_input_defs`).
    const PAYLOAD_SCHEMA: &'static str;
    /// 🔗️ Every schema document [`Self::PAYLOAD_SCHEMA`] references by `$id`, transitively, that the leaf's own plugin (or
    /// framework module) tree holds — embedded by `#[derive(dsl::MutationLeaf)]` and published at runtime, so every input resolves
    /// whatever facet or artifact of the plugin it lives in (design §16.3).
    const PAYLOAD_SCHEMA_DOCUMENTS: &'static [&'static str] = &[];
    /// 📥️ [`Self::PAYLOAD_SCHEMA`] when this operation is user-editable, else `None`. A leaf whose type wraps its payload
    /// in one variant of an enum (`#[mutation_leaf(payload = Apply)]`) is editable only in that variant; its other
    /// variants (an internal inverse such as `Restore`) are not.
    fn input_schema(&self) -> Option<&'static str> {
        Some(Self::PAYLOAD_SCHEMA)
    }
    /// ⌨️ The value [`Self::input_schema`] describes: the whole leaf value, or the content of the `payload` variant.
    fn input_value(&self) -> semio_framework_value::DslValue
    where
        Self: semio_framework_value::ToValue,
    {
        semio_framework_value::ToValue::to_value(self)
    }
    /// ✏️ Rebuilds this operation from an edited [`Self::input_value`]; an operation that is not editable refuses.
    fn with_input_value(&self, value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError>
    where
        Self: Sized + semio_framework_value::FromValue,
    {
        <Self as semio_framework_value::FromValue>::from_value(value)
    }
    /// 🌱️ A new editable operation of this leaf from its [`Self::input_value`] alone: the whole leaf value, or the content
    /// of the `payload` variant.
    fn from_input_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError>
    where
        Self: Sized + semio_framework_value::FromValue,
    {
        <Self as semio_framework_value::FromValue>::from_value(value)
    }
    /// 🧾️ The upper bound of the rows `Mutation::inverse` yields for this operation on any base, declared schema-first by the
    /// leaf payload schema's `x-semio-inverse-rows` (`fixed`, `perTarget`, `bounded`; absent = one row) and generated by
    /// `#[derive(dsl::MutationLeaf)]` (design §20.5).
    fn inverse_rows(&self) -> usize;
}
//#endregion 🪪️MutationLeaf
