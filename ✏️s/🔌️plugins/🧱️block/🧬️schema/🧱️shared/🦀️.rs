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
    /// 🧭️ The patch that turns `from` into `to`.
    fn between(from: &Self::Row, to: &Self::Row) -> Self;
    /// 🔁️ The patch that, applied after this one to the patched row, restores `base`.
    fn inverse(&self, base: &Self::Row) -> Self;
    /// ➕️ Composes a later patch into this one: the later field wins, nested list rows coalesce.
    fn absorb(&mut self, later: Self);
    /// 🕳️ Whether the patch sets nothing.
    fn is_empty(&self) -> bool;
}

/// 📂 Id-keyed row delta over a list of `R`: removals, appended additions, per-row patches, then an optional full order.
pub trait BlockRows: Clone + Default + PartialEq + Sized {
    /// 🧱️ The row type of the list.
    type Row: Clone;
    /// 🔑️ The list after the delta; unknown targets and duplicated identities are rejected.
    fn apply(&self, items: &[Self::Row]) -> Result<Vec<Self::Row>, BlockPatchError>;
    /// 🔁️ The delta that, applied after this one, restores `base` rows and order exactly.
    fn inverse(&self, base: &[Self::Row]) -> Self;
    /// 🧭️ The delta that turns `from` into `to`.
    fn between(from: &[Self::Row], to: &[Self::Row]) -> Self;
    /// ➕️ Composes a later delta into this one: create∘delete cancels, patch∘delete drops the patch, patch∘patch coalesces.
    fn absorb(&mut self, later: Self);
    /// 🕳️ Whether the delta changes nothing.
    fn is_empty(&self) -> bool;
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

/// 🧭️ The optional patch from `from` to `to`; absent when nothing differs.
pub fn block_patch_between<P: BlockPatch>(from: &P::Row, to: &P::Row) -> Option<P> {
    Some(P::between(from, to)).filter(|patch| !patch.is_empty())
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

/// 🔑️ Applies an optional row delta to `items`.
pub fn block_rows_apply<D: BlockRows>(delta: &Option<D>, items: &[D::Row], field: &str) -> Result<Vec<D::Row>, BlockPatchError> {
    match delta {
        Some(delta) => delta.apply(items).map_err(|error| error.under(field)),
        None => Ok(items.to_vec()),
    }
}

/// 🔁️ The optional row delta restoring `base` after `delta`.
pub fn block_rows_inverse<D: BlockRows>(delta: &Option<D>, base: &[D::Row]) -> Option<D> {
    delta.as_ref().map(|delta| delta.inverse(base)).filter(|inverse| !inverse.is_empty())
}

/// 🧭️ The optional row delta from `from` to `to`; absent when nothing differs.
pub fn block_rows_between<D: BlockRows>(from: &[D::Row], to: &[D::Row]) -> Option<D> {
    Some(D::between(from, to)).filter(|delta| !delta.is_empty())
}

/// ➕️ Composes an optional later row delta into `target`.
pub fn block_rows_absorb<D: BlockRows>(target: &mut Option<D>, later: Option<D>) {
    if let Some(later) = later {
        match target {
            Some(delta) => delta.absorb(later),
            None => *target = Some(later),
        }
    }
}

/// 🕳️ Whether an optional row delta changes nothing.
pub fn block_rows_is_empty<D: BlockRows>(delta: &Option<D>) -> bool {
    delta.as_ref().is_none_or(D::is_empty)
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
            fn between(from: &$row, to: &$row) -> Self {
                Self { $($field: (from.$field != to.$field).then(|| to.$field.clone()),)* $($ofield: (from.$ofield != to.$ofield).then(|| $wrap { value: to.$ofield.clone() }),)* }
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
    };
}

/// 📂 Declares an id-keyed row delta `$delta` (with its patch entry `$entry`) for rows `$row` keyed by field `$id`, patched by `$patch`, and its [`BlockRows`] impl.
#[macro_export]
macro_rules! block_rows {
    ($serde:meta; $(#[$doc:meta])* $delta:ident, $entry:ident, $row:ty, $patch:ty, $id:ident) => {
        /// 🩹 One patched row, addressed by its base identity.
        #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
        #[cfg_attr($serde, derive(serde::Serialize, serde::Deserialize))]
        #[value(rename_all = "camelCase")]
        #[cfg_attr($serde, serde(rename_all = "camelCase"))]
        pub struct $entry {
            pub id: String,
            pub patch: $patch,
        }

        $(#[$doc])*
        #[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
        #[cfg_attr($serde, derive(serde::Serialize, serde::Deserialize))]
        #[value(rename_all = "camelCase", default)]
        #[cfg_attr($serde, serde(rename_all = "camelCase", default))]
        pub struct $delta {
            pub added: Vec<$row>,
            pub removed: Vec<String>,
            pub patched: Vec<$entry>,
            pub reordered: Option<Vec<String>>,
        }

        impl $crate::BlockRows for $delta {
            type Row = $row;
            fn apply(&self, items: &[$row]) -> Result<Vec<$row>, $crate::BlockPatchError> {
                use $crate::BlockPatch as _;
                use $crate::BlockPatchError as Error;
                let mut next = items.to_vec();
                let mut seen = std::collections::HashSet::new();
                for id in &self.removed {
                    if !seen.insert(id.as_str()) {
                        return Err(Error::new("mutation.apply.duplicate-target", "item is removed more than once", ["removed", id.as_str()]));
                    }
                    let position = next.iter().position(|item| item.$id == *id).ok_or_else(|| Error::new("mutation.apply.missing-target", "removed item does not exist", ["removed", id.as_str()]))?;
                    next.remove(position);
                }
                seen.clear();
                for item in &self.added {
                    let id = item.$id.as_str();
                    if !seen.insert(id) || next.iter().any(|entry| entry.$id == id) {
                        return Err(Error::new("mutation.apply.duplicate-target", "added item identity already exists", ["added", id]));
                    }
                    next.push(item.clone());
                }
                seen.clear();
                for entry in &self.patched {
                    if !seen.insert(entry.id.as_str()) {
                        return Err(Error::new("mutation.apply.duplicate-target", "item is patched more than once", ["patched", entry.id.as_str()]));
                    }
                    let position = next.iter().position(|row| row.$id == entry.id).ok_or_else(|| Error::new("mutation.apply.missing-target", "patched item does not exist", ["patched", entry.id.as_str()]))?;
                    let patched = entry.patch.patched(&next[position]).map_err(|error| error.under(entry.id.clone()).under("patched"))?;
                    next[position] = patched;
                }
                if let Some(order) = &self.reordered {
                    if order.len() != next.len() {
                        return Err(Error::new("mutation.apply.incomplete-diff", format!("order has length {}, expected {}", order.len(), next.len()), ["reordered"]));
                    }
                    seen.clear();
                    let mut ordered = Vec::with_capacity(next.len());
                    for id in order {
                        if !seen.insert(id.as_str()) {
                            return Err(Error::new("mutation.apply.duplicate-target", "item appears more than once in order", ["reordered", id.as_str()]));
                        }
                        let position = next.iter().position(|entry| entry.$id == *id).ok_or_else(|| Error::new("mutation.apply.missing-target", "ordered item does not exist", ["reordered", id.as_str()]))?;
                        ordered.push(next[position].clone());
                    }
                    next = ordered;
                }
                Ok(next)
            }

            fn inverse(&self, base: &[$row]) -> Self {
                use $crate::BlockPatch as _;
                let find = |id: &str| base.iter().find(|row| row.$id == id);
                let removed: Vec<String> = self.added.iter().map(|row| row.$id.clone()).collect();
                let added: Vec<$row> = self.removed.iter().filter_map(|id| find(id).cloned()).collect();
                let patched: Vec<$entry> = self
                    .patched
                    .iter()
                    .filter(|entry| !self.removed.contains(&entry.id) && !self.added.iter().any(|row| row.$id == entry.id))
                    .filter_map(|entry| find(&entry.id).map(|row| $entry { id: entry.id.clone(), patch: entry.patch.inverse(row) }))
                    .filter(|entry| !entry.patch.is_empty())
                    .collect();
                let base_ids: Vec<String> = base.iter().map(|row| row.$id.clone()).collect();
                let mut order: Vec<String> = match &self.reordered {
                    Some(order) => order.clone(),
                    None => base_ids.iter().filter(|id| !self.removed.contains(id)).cloned().chain(self.added.iter().map(|row| row.$id.clone())).collect(),
                };
                order.retain(|id| !removed.contains(id));
                order.extend(added.iter().map(|row| row.$id.clone()));
                let reordered = (order != base_ids).then_some(base_ids);
                Self { added, removed, patched, reordered }
            }

            fn between(from: &[$row], to: &[$row]) -> Self {
                use $crate::BlockPatch as _;
                let removed: Vec<String> = from.iter().filter(|row| !to.iter().any(|other| other.$id == row.$id)).map(|row| row.$id.clone()).collect();
                let added: Vec<$row> = to.iter().filter(|row| !from.iter().any(|other| other.$id == row.$id)).cloned().collect();
                let patched: Vec<$entry> = from
                    .iter()
                    .filter_map(|row| to.iter().find(|other| other.$id == row.$id).filter(|other| *other != row).map(|other| $entry { id: row.$id.clone(), patch: <$patch as $crate::BlockPatch>::between(row, other) }))
                    .collect();
                let natural: Vec<&str> = from.iter().filter(|row| !removed.contains(&row.$id)).map(|row| row.$id.as_str()).chain(added.iter().map(|row| row.$id.as_str())).collect();
                let target: Vec<&str> = to.iter().map(|row| row.$id.as_str()).collect();
                let reordered = (natural != target).then(|| target.iter().map(|id| (*id).to_string()).collect());
                Self { added, removed, patched, reordered }
            }

            fn absorb(&mut self, later: Self) {
                use $crate::BlockPatch as _;
                for id in later.removed {
                    if let Some(position) = self.added.iter().position(|row| row.$id == id) {
                        self.added.remove(position);
                    } else {
                        self.patched.retain(|entry| entry.id != id);
                        self.removed.push(id.clone());
                    }
                    if let Some(order) = &mut self.reordered {
                        order.retain(|entry| *entry != id);
                    }
                }
                for row in later.added {
                    if let Some(order) = &mut self.reordered {
                        order.push(row.$id.clone());
                    }
                    self.added.push(row);
                }
                for entry in later.patched {
                    if let Some(position) = self.added.iter().position(|row| row.$id == entry.id) {
                        if let Ok(row) = entry.patch.patched(&self.added[position]) {
                            self.added[position] = row;
                        }
                    } else if let Some(existing) = self.patched.iter_mut().find(|existing| existing.id == entry.id) {
                        existing.patch.absorb(entry.patch);
                    } else {
                        self.patched.push(entry);
                    }
                }
                if later.reordered.is_some() {
                    self.reordered = later.reordered;
                }
            }

            fn is_empty(&self) -> bool {
                self.added.is_empty() && self.removed.is_empty() && self.patched.is_empty() && self.reordered.is_none()
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

block_rows!(any(test, feature = "test-serde"); /// 📂 Row delta over the attributes of a kind.
    BlockAttributesDelta, BlockAttributesPatchEntry, BlockAttribute, BlockAttributePatch, key);
block_rows!(any(test, feature = "test-serde"); /// 📂 Row delta over the credited authors of a kind.
    BlockAuthorsDelta, BlockAuthorsPatchEntry, BlockAuthor, BlockAuthorPatch, id);
block_rows!(any(test, feature = "test-serde"); /// 📂 Row delta over the compatibility rules of a kind.
    BlockCompatibilityDelta, BlockCompatibilityPatchEntry, BlockCompatibilityRule, BlockCompatibilityRulePatch, id);

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

/// 🧭️ The ordered-set delta turning `from` into `to`; rewrites the whole set when the kept entries are not a prefix of `to`.
fn set_between<T: Clone + PartialEq>(from: &[T], to: &[T], key: impl Fn(&T) -> &str) -> (Vec<String>, Vec<T>) {
    let kept: Vec<&T> = from.iter().filter(|item| to.iter().any(|other| key(other) == key(item))).collect();
    if kept.len() <= to.len() && kept.iter().zip(to).all(|(left, right)| *left == right) {
        (from.iter().filter(|item| !to.iter().any(|other| key(other) == key(item))).map(|item| key(item).to_string()).collect(), to[kept.len()..].to_vec())
    } else {
        (from.iter().map(|item| key(item).to_string()).collect(), to.to_vec())
    }
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
    fn between(from: &BlockRepresentation, to: &BlockRepresentation) -> Self {
        let (tags_removed, tags_added) = set_between(&from.tags, &to.tags, String::as_str);
        let (attributes_removed, attributes_added) = set_between(&from.attributes, &to.attributes, |attribute| attribute.key.as_str());
        Self {
            name: (from.name != to.name).then(|| to.name.clone()),
            mesh_url: (from.mesh_url != to.mesh_url).then(|| BlockOptionalText { value: to.mesh_url.clone() }),
            lod: (from.lod != to.lod).then(|| BlockOptionalText { value: to.lod.clone() }),
            description: (from.description != to.description).then(|| to.description.clone()),
            tags_removed,
            tags_added,
            attributes_removed,
            attributes_added,
        }
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

block_rows!(any(test, feature = "test-serde"); /// 📂 Row delta over the representations of a kind.
    BlockRepresentationsDelta, BlockRepresentationsPatchEntry, BlockRepresentation, BlockRepresentationPatch, id);
//#endregion 🔖️Patches
