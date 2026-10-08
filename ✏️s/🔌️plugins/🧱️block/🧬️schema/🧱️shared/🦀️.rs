//! 🧱️ Document records shared by the block artifact family.

//#region 🔖️Identity
/// 🪪️ The single kind definition a block document edits — name/label/variant/description/icon/unit
/// apply uniformly whether the document is a `NodeKind` (2d), `ObjectKind` (3d) or `PartKind` (5d).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "test-serde"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "test-serde"), serde(rename_all = "camelCase"))]
pub struct BlockKindIdentity {
    pub id: String,
    pub name: String,
    pub label: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default, skip_serializing_if = "Option::is_none"))]
    pub variant: Option<String>,
    #[value(default)]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default))]
    pub description: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default, skip_serializing_if = "Option::is_none"))]
    pub icon: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default, skip_serializing_if = "Option::is_none"))]
    pub unit: Option<String>,
}
//#endregion 🔖️Identity

//#region 🔖️Metadata
/// 🏷️ One free-form key/value attribute on a kind (optionally naming the attribute definition it
/// instantiates).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "test-serde"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "test-serde"), serde(rename_all = "camelCase"))]
pub struct BlockAttribute {
    pub key: String,
    pub value: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default, skip_serializing_if = "Option::is_none"))]
    pub definition: Option<String>,
}

/// 👤️ One author credited on a kind.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "test-serde"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "test-serde"), serde(rename_all = "camelCase"))]
pub struct BlockAuthor {
    pub id: String,
    pub name: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default, skip_serializing_if = "Option::is_none"))]
    pub email: Option<String>,
}

/// 🔗️ One allowed (or, unidirectional, one-way-allowed) compatibility pair between two handle/vortex/
/// grip kind ids — the `id` lets ops remove a specific row without re-keying on `(source, target)`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "test-serde"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "test-serde"), serde(rename_all = "camelCase"))]
pub struct BlockCompatibilityRule {
    pub id: String,
    pub source: String,
    pub target: String,
    #[value(default)]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default))]
    pub bidirectional: bool,
}

/// 🧱️ One representation (mesh at a LOD/tag combination) a kind ships with — semio_compose_rs's "Representation".
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "test-serde"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "test-serde"), serde(rename_all = "camelCase"))]
pub struct BlockRepresentation {
    pub id: String,
    pub name: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default, skip_serializing_if = "Option::is_none"))]
    pub mesh_url: Option<String>,
    #[value(default)]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default))]
    pub tags: Vec<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default, skip_serializing_if = "Option::is_none"))]
    pub lod: Option<String>,
    #[value(default)]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default))]
    pub description: String,
    #[value(default)]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default))]
    #[dsl(table)]
    pub attributes: Vec<BlockAttribute>,
}
//#endregion 🔖️Metadata

//#region 🔖️Cameras
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "test-serde"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "test-serde"), serde(rename_all = "camelCase"))]
pub struct BlockCamera2d {
    #[value(default)]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default))]
    pub x: f64,
    #[value(default)]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default))]
    pub y: f64,
    #[value(default = "block_one_f64")]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default = "block_one_f64"))]
    pub zoom: f64,
}

impl Default for BlockCamera2d {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "test-serde"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "test-serde"), serde(rename_all = "camelCase"))]
pub struct BlockCamera3d {
    #[value(default)]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default))]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[value(default)]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default))]
    #[dsl(coord)]
    pub target: [f64; 3],
    #[value(default = "block_one_f64")]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default = "block_one_f64"))]
    pub zoom: f64,
}

impl Default for BlockCamera3d {
    fn default() -> Self {
        Self { position: [0.0, 0.0, 0.0], target: [0.0, 0.0, 0.0], zoom: 1.0 }
    }
}

fn block_one_f64() -> f64 {
    1.0
}
//#endregion 🔖️Cameras

//#region 🔖️Meta
/// 📝️ Free-text description carried alongside a block document (distinct from the kind's own
/// `BlockKindIdentity::description`, which describes the kind; this describes the editing session).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "test-serde"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "test-serde"), serde(rename_all = "camelCase"))]
pub struct BlockMeta {
    #[value(default)]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default))]
    pub description: String,
}
//#endregion 🔖️Meta

//#region 🔖️Patches
/// 🩹 Rejection raised when a patch or row delta meets a base it does not fit; every artifact lifts it into its own typed apply error.
#[derive(Clone, Debug, PartialEq)]
pub struct BlockPatchError {
    pub code: &'static str,
    pub message: String,
    pub target: Vec<String>,
}

impl BlockPatchError {
    /// 🏗️ Builds a rejection addressed at `target`, outermost segment first.
    pub fn new(code: &'static str, message: impl Into<String>, target: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self { code, message: message.into(), target: target.into_iter().map(Into::into).collect() }
    }

    /// 🪆 Prefixes one outer address segment.
    #[must_use]
    pub fn under(mut self, prefix: impl Into<String>) -> Self {
        self.target.insert(0, prefix.into());
        self
    }
}

/// 🩹 Field-sparse patch over one row `R`: names exactly the fields it sets, with the values they take.
pub trait BlockPatch: Clone + Default + PartialEq + Sized {
    /// 🧱️ The row this patch applies to.
    type Row: Clone;
    /// 🔑️ The row with this patch's fields set; a nested list that does not fit the row is rejected.
    fn patched(&self, row: &Self::Row) -> Result<Self::Row, BlockPatchError>;
    /// 🔁️ The patch that, applied after this one to the patched row, restores `base`.
    fn inverse(&self, base: &Self::Row) -> Self;
    /// ➕️ Composes a later patch into this one: the later field wins, nested list rows coalesce.
    fn absorb(&mut self, later: Self);
    /// 🕳️ Whether the patch sets nothing.
    fn is_empty(&self) -> bool;
}

/// 📍️ The after-list slot a new row takes: `index` when it lies inside the base list, the end otherwise.
pub fn block_insert_index(len: usize, index: Option<u32>) -> usize {
    index.map_or(len, |index| (index as usize).min(len))
}

/// 🩹 Lifts a patch rejection into the framework's typed apply error.
pub fn block_apply_error(error: BlockPatchError) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(error.code, error.message).at(error.target)
}

/// 🔑️ Applies an optional sub-document patch to `row`.
pub fn block_patch_apply<P: BlockPatch>(patch: &Option<P>, row: &P::Row, field: &str) -> Result<P::Row, BlockPatchError> {
    match patch {
        Some(patch) => patch.patched(row).map_err(|error| error.under(field)),
        None => Ok(row.clone()),
    }
}

/// 🔁️ The optional patch restoring `base` after `patch`.
pub fn block_patch_inverse<P: BlockPatch>(patch: &Option<P>, base: &P::Row) -> Option<P> {
    patch.as_ref().map(|patch| patch.inverse(base)).filter(|inverse| !inverse.is_empty())
}

/// ➕️ Composes an optional later patch into `target`.
pub fn block_patch_absorb<P: BlockPatch>(target: &mut Option<P>, later: Option<P>) {
    if let Some(later) = later {
        match target {
            Some(patch) => patch.absorb(later),
            None => *target = Some(later),
        }
    }
}

/// 🕳️ Whether an optional patch sets nothing.
pub fn block_patch_is_empty<P: BlockPatch>(patch: &Option<P>) -> bool {
    patch.as_ref().is_none_or(P::is_empty)
}

/// 🎚️ One optional scalar field set to a value or cleared — a patch field that must tell "unchanged" from "now absent".
#[macro_export]
macro_rules! block_optional {
    ($serde:meta; $(#[$doc:meta])* $name:ident($ty:ty)) => {
        $(#[$doc])*
        #[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
        #[cfg_attr($serde, derive(serde::Serialize, serde::Deserialize))]
        #[value(rename_all = "camelCase", default)]
        #[cfg_attr($serde, serde(rename_all = "camelCase", default))]
        pub struct $name {
            pub value: Option<$ty>,
        }
    };
}

/// 🩹 Declares a field-sparse patch struct for `$row` and its [`BlockPatch`] impl: `plain` fields set a value, `optional` fields set or clear an `Option` through their wrapper.
#[macro_export]
macro_rules! block_patch {
    ($serde:meta; $(#[$doc:meta])* $name:ident for $row:ty { plain { $($field:ident : $ty:ty),* } optional { $($ofield:ident : $wrap:ident),* } }) => {
        $(#[$doc])*
        #[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
        #[cfg_attr($serde, derive(serde::Serialize, serde::Deserialize))]
        #[value(rename_all = "camelCase", default)]
        #[cfg_attr($serde, serde(rename_all = "camelCase", default))]
        pub struct $name {
            $(pub $field: Option<$ty>,)*
            $(pub $ofield: Option<$wrap>,)*
        }

        impl $crate::BlockPatch for $name {
            type Row = $row;
            fn patched(&self, row: &$row) -> Result<$row, $crate::BlockPatchError> {
                #[allow(unused_mut)]
                let mut next = row.clone();
                $(if let Some(value) = &self.$field { next.$field = value.clone(); })*
                $(if let Some(value) = &self.$ofield { next.$ofield = value.value.clone(); })*
                Ok(next)
            }
            fn inverse(&self, base: &$row) -> Self {
                Self { $($field: self.$field.as_ref().map(|_| base.$field.clone()),)* $($ofield: self.$ofield.as_ref().map(|_| $wrap { value: base.$ofield.clone() }),)* }
            }
            fn absorb(&mut self, later: Self) {
                $(if later.$field.is_some() { self.$field = later.$field; })*
                $(if later.$ofield.is_some() { self.$ofield = later.$ofield; })*
            }
            fn is_empty(&self) -> bool {
                true $(&& self.$field.is_none())* $(&& self.$ofield.is_none())*
            }
        }

        impl protocol::list_delta::RowPatch<$row> for $name {
            fn commit_into(&self, row: &mut $row, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
                *row = $crate::BlockPatch::patched(self, row).map_err($crate::block_apply_error)?;
                Ok(())
            }
            fn absorb(&mut self, later: Self) {
                $crate::BlockPatch::absorb(self, later);
            }
            fn inverse(&self, row: &$row) -> Self {
                $crate::BlockPatch::inverse(self, row)
            }
            fn is_empty(&self) -> bool {
                $crate::BlockPatch::is_empty(self)
            }
        }
    };
}

block_optional!(any(test, feature = "test-serde"); /// 🔤 An optional text field set to a value or cleared.
    BlockOptionalText(String));
block_optional!(any(test, feature = "test-serde"); /// 🔢 An optional number field set to a value or cleared.
    BlockOptionalNumber(f64));
block_optional!(any(test, feature = "test-serde"); /// 🧭 An optional orientation quaternion set to a value or cleared.
    BlockOptionalOrientation([f64; 4]));
block_optional!(any(test, feature = "test-serde"); /// 📏 An optional scale triple set to a value or cleared.
    BlockOptionalScale([f64; 3]));

block_patch!(any(test, feature = "test-serde"); /// 🪪️ Field patch over a [`BlockKindIdentity`].
    BlockKindIdentityPatch for BlockKindIdentity { plain { id: String, name: String, label: String, description: String } optional { variant: BlockOptionalText, icon: BlockOptionalText, unit: BlockOptionalText } });
block_patch!(any(test, feature = "test-serde"); /// 🏷️ Field patch over a [`BlockAttribute`] (its key is the row identity).
    BlockAttributePatch for BlockAttribute { plain { value: String } optional { definition: BlockOptionalText } });
block_patch!(any(test, feature = "test-serde"); /// 👤️ Field patch over a [`BlockAuthor`] (its id is the row identity).
    BlockAuthorPatch for BlockAuthor { plain { name: String } optional { email: BlockOptionalText } });
block_patch!(any(test, feature = "test-serde"); /// 🔗️ Field patch over a [`BlockCompatibilityRule`] (its id is the row identity).
    BlockCompatibilityRulePatch for BlockCompatibilityRule { plain { source: String, target: String, bidirectional: bool } optional { } });
block_patch!(any(test, feature = "test-serde"); /// 🎥 Field patch over a [`BlockCamera2d`].
    BlockCamera2dPatch for BlockCamera2d { plain { x: f64, y: f64, zoom: f64 } optional { } });
block_patch!(any(test, feature = "test-serde"); /// 🎬 Field patch over a [`BlockCamera3d`].
    BlockCamera3dPatch for BlockCamera3d { plain { position: [f64; 3], target: [f64; 3], zoom: f64 } optional { } });
block_patch!(any(test, feature = "test-serde"); /// 📝️ Field patch over a [`BlockMeta`].
    BlockMetaPatch for BlockMeta { plain { description: String } optional { } });

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-serde"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-serde"), serde(rename_all = "camelCase"))]
    /// 📂 Positional row delta over the attributes of a kind.
    pub BlockAttributesDelta { removal: BlockAttributesRemoval, insertion: BlockAttributesInsertion, relocation: BlockAttributesRelocation, modification: BlockAttributesPatchEntry, row: BlockAttribute, patch: BlockAttributePatch, key: key, values_only }
}
protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-serde"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-serde"), serde(rename_all = "camelCase"))]
    /// 📂 Positional row delta over the credited authors of a kind.
    pub BlockAuthorsDelta { removal: BlockAuthorsRemoval, insertion: BlockAuthorsInsertion, relocation: BlockAuthorsRelocation, modification: BlockAuthorsPatchEntry, row: BlockAuthor, patch: BlockAuthorPatch, key: id, values_only }
}
protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-serde"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-serde"), serde(rename_all = "camelCase"))]
    /// 📂 Positional row delta over the compatibility rules of a kind.
    pub BlockCompatibilityDelta { removal: BlockCompatibilityRemoval, insertion: BlockCompatibilityInsertion, relocation: BlockCompatibilityRelocation, modification: BlockCompatibilityPatchEntry, row: BlockCompatibilityRule, patch: BlockCompatibilityRulePatch, key: id, values_only }
}

/// 🏷️ Applies an ordered-set delta to `items`: removals by key first, then appended additions; both must fit.
fn set_apply<T: Clone>(items: &[T], removed: &[String], added: &[T], key: impl Fn(&T) -> &str, field: &str) -> Result<Vec<T>, BlockPatchError> {
    let mut next = items.to_vec();
    for (index, id) in removed.iter().enumerate() {
        if removed[..index].contains(id) {
            return Err(BlockPatchError::new("mutation.apply.duplicate-target", "entry is removed more than once", [field, "removed", id.as_str()]));
        }
        let position = next.iter().position(|item| key(item) == id).ok_or_else(|| BlockPatchError::new("mutation.apply.missing-target", "removed entry does not exist", [field, "removed", id.as_str()]))?;
        next.remove(position);
    }
    for item in added {
        if next.iter().any(|entry| key(entry) == key(item)) {
            return Err(BlockPatchError::new("mutation.apply.duplicate-target", "added entry already exists", [field, "added", key(item)]));
        }
        next.push(item.clone());
    }
    Ok(next)
}

/// 🔁️ The ordered-set delta restoring `base` after (`removed`, `added`) was applied to it — position-exact.
fn set_inverse<T: Clone>(base: &[T], removed: &[String], added: &[T], key: impl Fn(&T) -> &str) -> (Vec<String>, Vec<T>) {
    let first = base.iter().position(|item| removed.iter().any(|id| id == key(item)));
    let tail = first.map_or(&base[base.len()..], |first| &base[first..]);
    let dropped = tail.iter().filter(|item| !removed.iter().any(|id| id == key(item))).map(|item| key(item).to_string()).chain(added.iter().map(|item| key(item).to_string())).collect();
    (dropped, tail.to_vec())
}

/// ➕️ Composes a later ordered-set delta: an entry added then removed cancels, anything else concatenates.
fn set_absorb<T>(removed: &mut Vec<String>, added: &mut Vec<T>, later_removed: Vec<String>, later_added: Vec<T>, key: impl Fn(&T) -> &str) {
    for id in later_removed {
        match added.iter().position(|item| key(item) == id) {
            Some(position) => {
                added.remove(position);
            }
            None => removed.push(id),
        }
    }
    added.extend(later_added);
}

/// 🧱️ Field patch over a [`BlockRepresentation`] (its id is the row identity): scalar fields set a value, `tags` and `attributes` are ordered sets changed by removed/added entries.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "test-serde"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(any(test, feature = "test-serde"), serde(rename_all = "camelCase", default))]
pub struct BlockRepresentationPatch {
    pub name: Option<String>,
    pub mesh_url: Option<BlockOptionalText>,
    pub lod: Option<BlockOptionalText>,
    pub description: Option<String>,
    pub tags_removed: Vec<String>,
    pub tags_added: Vec<String>,
    pub attributes_removed: Vec<String>,
    pub attributes_added: Vec<BlockAttribute>,
}

impl BlockPatch for BlockRepresentationPatch {
    type Row = BlockRepresentation;
    fn patched(&self, row: &BlockRepresentation) -> Result<BlockRepresentation, BlockPatchError> {
        let mut next = row.clone();
        if let Some(name) = &self.name {
            next.name.clone_from(name);
        }
        if let Some(mesh_url) = &self.mesh_url {
            next.mesh_url.clone_from(&mesh_url.value);
        }
        if let Some(lod) = &self.lod {
            next.lod.clone_from(&lod.value);
        }
        if let Some(description) = &self.description {
            next.description.clone_from(description);
        }
        next.tags = set_apply(&row.tags, &self.tags_removed, &self.tags_added, String::as_str, "tags")?;
        next.attributes = set_apply(&row.attributes, &self.attributes_removed, &self.attributes_added, |attribute| attribute.key.as_str(), "attributes")?;
        Ok(next)
    }
    fn inverse(&self, base: &BlockRepresentation) -> Self {
        let (tags_removed, tags_added) = set_inverse(&base.tags, &self.tags_removed, &self.tags_added, String::as_str);
        let (attributes_removed, attributes_added) = set_inverse(&base.attributes, &self.attributes_removed, &self.attributes_added, |attribute| attribute.key.as_str());
        Self {
            name: self.name.as_ref().map(|_| base.name.clone()),
            mesh_url: self.mesh_url.as_ref().map(|_| BlockOptionalText { value: base.mesh_url.clone() }),
            lod: self.lod.as_ref().map(|_| BlockOptionalText { value: base.lod.clone() }),
            description: self.description.as_ref().map(|_| base.description.clone()),
            tags_removed,
            tags_added,
            attributes_removed,
            attributes_added,
        }
    }
    fn absorb(&mut self, later: Self) {
        if later.name.is_some() {
            self.name = later.name;
        }
        if later.mesh_url.is_some() {
            self.mesh_url = later.mesh_url;
        }
        if later.lod.is_some() {
            self.lod = later.lod;
        }
        if later.description.is_some() {
            self.description = later.description;
        }
        set_absorb(&mut self.tags_removed, &mut self.tags_added, later.tags_removed, later.tags_added, String::as_str);
        set_absorb(&mut self.attributes_removed, &mut self.attributes_added, later.attributes_removed, later.attributes_added, |attribute| attribute.key.as_str());
    }
    fn is_empty(&self) -> bool {
        self.name.is_none() && self.mesh_url.is_none() && self.lod.is_none() && self.description.is_none() && self.tags_removed.is_empty() && self.tags_added.is_empty() && self.attributes_removed.is_empty() && self.attributes_added.is_empty()
    }
}

impl protocol::list_delta::RowPatch<BlockRepresentation> for BlockRepresentationPatch {
    fn commit_into(&self, row: &mut BlockRepresentation, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        *row = BlockPatch::patched(self, row).map_err(block_apply_error)?;
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        BlockPatch::absorb(self, later);
    }
    fn inverse(&self, row: &BlockRepresentation) -> Self {
        BlockPatch::inverse(self, row)
    }
    fn is_empty(&self) -> bool {
        BlockPatch::is_empty(self)
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-serde"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-serde"), serde(rename_all = "camelCase"))]
    /// 📂 Positional row delta over the representations of a kind.
    pub BlockRepresentationsDelta { removal: BlockRepresentationsRemoval, insertion: BlockRepresentationsInsertion, relocation: BlockRepresentationsRelocation, modification: BlockRepresentationsPatchEntry, row: BlockRepresentation, patch: BlockRepresentationPatch, key: id, values_only }
}
//#endregion 🔖️Patches
