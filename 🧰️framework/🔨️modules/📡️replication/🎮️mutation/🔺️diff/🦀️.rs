//! 🔺️ Original sealed diff application, keyed-row algebra and sparse schema authoring.
use crate::{DslValue,ToValue,FromValue,ValueError,ValueRefusalKind};
use std::collections::BTreeMap;
/// 🚫️ Structured rejection of a diff that cannot be applied to its supplied base.
/// The shape is protocol-owned and wire-safe: callers never need a technology crate's error type
/// to preserve the stable machine code, human diagnostic, and outermost-first target address.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MutationApplyError {
    pub code: String,
    pub message: String,
    pub target: Vec<String>,
}

/// 🌱️ Hand-written, not derived — same DAG reason `MutationMessage`'s hand-written twin below
/// documents (this crate sits below `os-kernel`). Mirrors the pre-existing `#[serde(default,
/// skip_serializing_if = "Vec::is_empty")]` sparse-emission on `target` byte-for-byte.
impl semio_framework_value_intrinsic::ToValue for MutationApplyError {
    fn to_value(&self) -> semio_framework_value_intrinsic::DslValue {
        let mut entries = vec![("code".to_string(), semio_framework_value_intrinsic::ToValue::to_value(&self.code)), ("message".to_string(), semio_framework_value_intrinsic::ToValue::to_value(&self.message))];
        if !self.target.is_empty() {
            entries.push(("target".to_string(), semio_framework_value_intrinsic::ToValue::to_value(&self.target)));
        }
        semio_framework_value_intrinsic::DslValue::object(entries)
    }
}
impl semio_framework_value_intrinsic::FromValue for MutationApplyError {
    fn from_value(value: semio_framework_value_intrinsic::DslValue) -> Result<Self, semio_framework_value_intrinsic::ValueError> {
        let semio_framework_value_intrinsic::DslValue::Object(fields) = value else {
            return Err(semio_framework_value_intrinsic::ValueError::new(semio_framework_value_intrinsic::ValueRefusalKind::InvalidValue, format!("expected an object for MutationApplyError, found {value:?}")));
        };
        let mut code = None;
        let mut message = None;
        let mut target = Vec::new();
        for (key, entry) in fields {
            match key.as_str() {
                "code" => code = Some(<String as semio_framework_value_intrinsic::FromValue>::from_value(entry).map_err(|e| e.under("code"))?),
                "message" => message = Some(<String as semio_framework_value_intrinsic::FromValue>::from_value(entry).map_err(|e| e.under("message"))?),
                "target" => target = <Vec<String> as semio_framework_value_intrinsic::FromValue>::from_value(entry).map_err(|e| e.under("target"))?,
                _ => {}
            }
        }
        Ok(MutationApplyError {
            code: code.ok_or_else(|| semio_framework_value_intrinsic::ValueError::new(semio_framework_value_intrinsic::ValueRefusalKind::InvalidValue, "MutationApplyError missing code"))?,
            message: message.ok_or_else(|| semio_framework_value_intrinsic::ValueError::new(semio_framework_value_intrinsic::ValueRefusalKind::InvalidValue, "MutationApplyError missing message"))?,
            target,
        })
    }
}

impl std::fmt::Display for MutationApplyError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for MutationApplyError {}

impl MutationApplyError {
    /// 🏗️ Builds an untargeted typed rejection.
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self { code: code.into(), message: message.into(), target: Vec::new() }
    }

    /// 🎯️ Attaches the rejected diff address, outermost segment first.
    pub fn at(mut self, target: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.target = target.into_iter().map(Into::into).collect();
        self
    }

    /// 🪆️ Prefixes outer address segments while retaining the inner error's exact target.
    pub fn under(mut self, prefix: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let mut target: Vec<String> = prefix.into_iter().map(Into::into).collect();
        target.append(&mut self.target);
        self.target = target;
        self
    }
}

/// 🛡️ Crate-owned result of applying a diff to a snapshot.
pub type MutationApplyResult<P> = Result<P, MutationApplyError>;

/// 🔑️ Proof that the caller is the central applier. The private field makes this module the only place that can construct it,
/// and [`commit_retained_diff`] is the only function here that does.
#[derive(Clone, Copy, Debug)]
pub struct ApplyCapability {
    _sealed: (),
}

/// 🆚️ THE central applier: the only mint point of [`ApplyCapability`] and the one callable entry that turns a diff into a
/// snapshot. Store lanes, replay, merge, backbone, plugin transaction folds, tool folds, the db and composite planners all
/// route through it; mutation leaves never call [`ColdMutationDiff::apply`].
pub fn apply_diff_cold_into<P, D: ColdMutationDiff<P>>(diff: &D, unpublished: &mut P) -> MutationApplyResult<()> {
    apply_with(diff, unpublished, mint_capability())
}

/// ❄️ Evaluates a diff on an owned cold scratch projection; live retained owners use [`commit_retained_diff`].
pub fn apply_diff_cold<P: Clone, D: ColdMutationDiff<P>>(diff: &D, base: &P) -> MutationApplyResult<P> {
    let mut unpublished = base.clone();
    if let Err(error) = apply_diff_cold_into(diff, &mut unpublished) {
        D::retire_projection(unpublished);
        return Err(error);
    }
    Ok(unpublished)
}

/// 🔏️ The one place the sealed capability is constructed; private, so only [`commit_retained_diff`] reaches it.
fn mint_capability() -> ApplyCapability {
    ApplyCapability { _sealed: () }
}

/// 🧪️ A capability for the conformance laws of retained mutation owners, which install a field or exchange a row without a diff to commit.
#[cfg(feature = "testkit")]
pub fn testing_apply_capability() -> ApplyCapability {
    mint_capability()
}

/// ▶️ Hands the capability to the mandatory typed unpublished-owner step.
fn apply_with<P, D: ColdMutationDiff<P>>(diff: &D, unpublished: &mut P, capability: ApplyCapability) -> MutationApplyResult<()> {
    diff.apply(unpublished, capability)
}

/// 📦️ Owns one prepared field until atomic installation or typed retirement on refusal.
pub struct PreparedField<T> {
    value: Option<T>,
    retire: fn(T),
    capability: ApplyCapability,
}

impl<T> PreparedField<T> {
    /// 🎟️ Retains a prepared field under the central applier's capability and its disposal owner.
    pub fn new(value: T, retire: fn(T), capability: ApplyCapability) -> Self {
        Self { value: Some(value), retire, capability }
    }

    /// 🧰️ Stages one changed field, retaining its candidate through refusal until atomic installation.
    pub fn prepare(base: &T, retire: fn(T), capability: ApplyCapability, edit: impl FnOnce(&mut T) -> MutationApplyResult<()>) -> MutationApplyResult<Self>
    where
        T: Clone,
    {
        let mut prepared = Self::new(base.clone(), retire, capability);
        edit(prepared.value_mut())?;
        Ok(prepared)
    }

    /// 🛠️ Advances sequential work inside the guarded candidate before its final installation.
    pub fn value_mut(&mut self) -> &mut T {
        self.value.as_mut().expect("prepared field owns its candidate until installation")
    }

    /// 📥️ Installs a validated field and retires the displaced field through the same typed owner.
    pub fn install(mut self, field: &mut T) {
        let _capability = self.capability;
        let next = self.value.take().expect("prepared field owns its value until installation");
        (self.retire)(std::mem::replace(field, next));
    }
}

impl<T> Drop for PreparedField<T> {
    fn drop(&mut self) {
        if let Some(value) = self.value.take() { (self.retire)(value); }
    }
}

/// 🟢️ Typed persisted delta vocabulary shared by retained native application and explicit wire reference algebra.
///
/// 🛬️ Typed delta admission belongs to its funded native receiver; stored diff ownership does not require a synchronous decoder.
pub trait MutationDiff<P>: Default + PartialEq + semio_framework_value_intrinsic::ToValue + DiffAlgebra<P> {


    /// 🧊️ Explicit cold disposal of a diff nobody will apply again. The default IS a plain drop,
    /// which is correct for every diff built out of plain values; a technology whose delta owns a
    /// fail-closed root (an `OrderedMap`, a neural `Dictionary`) MUST override it, because such a
    /// root aborts the process on a bare drop (`🌱️value/🗂️ordered/🦀️.rs`'s `Drop`). Every generic
    /// replay/fold seam that builds a delta and throws it away — `os_vcs::apply_mutation` and the
    /// store's history folds — routes through this instead of dropping
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn retire_cold(self)
    where
        Self: Sized,
    {
    }

    /// ➡️ Explicit cold disposal of a SCRATCH projection the replay arithmetic built and displaced.
    /// A history fold walks `base → mid₁ → mid₂ → … → head`, and every intermediate is an owned `P`
    /// the next step's assignment throws away; when `P` owns a fail-closed root that bare drop aborts
    /// the process, so `undo`/`redo` and every `.pack`/`.spr` reload aborted the app for any artifact
    /// whose projection carries an `OrderedMap`. It hangs off the DIFF, not off the projection, because
    /// `P` itself carries no bound at those seams while `Self::Diff` always does
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn retire_projection(projection: P) {
        drop(projection);
    }
}


/// 🧮️ Base-independent emptiness of a technology's typed delta.
/// There is deliberately no snapshot-differencing constructor: import, example switch and load are load paths, and sync
/// travels its own frontier delta.
pub trait DiffAlgebra<P>: Sized {
    /// 🕳️ Whether this diff changes nothing relative to whatever base it was built against.
    fn is_empty(&self) -> bool;
}

/// 🔗️ Wire values provide reference inversion without sharing or allocating retained native row authorities.
pub trait ColdDiffAlgebra<P>:DiffAlgebra<P>{fn inverse(&self,base:&P)->Self;}
/// 📘️ Reference projections apply synchronously only when callers select this distinct wire-value contract.
pub trait ColdMutationDiff<P>:MutationDiff<P>+semio_framework_value_intrinsic::FromValue{
    /// ➕️ Composes `self` (base→mid) with `other` (mid→after) into base→after, in place.
    /// Normative absorb contract (`.claude/plans/the-current-schemas-are-scalable-journal.md`
    /// `## Absorb`): **structural** (operates on the diff's own key/index/field shape, never on
    /// applied snapshot values), **total** (defined for every pair of diffs over the same
    /// artifact, including out-of-range/no-op cases — never panics), **base-free** (no snapshot
    /// parameter; the two diffs alone determine the result), and **sequential-coalesce only**
    /// (this composes two diffs known to have been applied in sequence by the same actor;
    /// concurrent-edit merging is an authority's `MergePolicy`/`📡️spr/⚔️conflict` job, never this
    /// method's — the CRDT-era concurrent-diff merge helper this docstring used to point at is
    /// deleted, see `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`).
    /// LAW: whenever sequential application succeeds,
    /// `apply_diff_cold(&absorb(d1, d2), base) == apply_diff_cold(&d1, base).and_then(|mid| apply_diff_cold(&d2, &mid))`, associative
    /// over further absorbs of the same artifact's diff vocabulary. A rejection remains a
    /// rejection; absorb must not manufacture an implicit success path.
    fn absorb(&mut self, other: Self);
    fn apply(&self,unpublished:&mut P,capability:ApplyCapability)->MutationApplyResult<()>;
}

/// 💼️ Authorizes one typed original retained commit after its caller admitted the finite work grant.
pub fn with_retained_apply_capability<D,T>(diff:semio_framework_value_clone_source::RetainedCloneRef<'_,D>,grant:semio_framework_value_intrinsic::RetainedCloneGrant,commit:impl FnOnce(ApplyCapability)->Result<T,ValueError>)->Result<T,ValueError>{
    if grant.maximum_items==0||grant.maximum_copy_bytes<std::mem::size_of::<bool>()||grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"retained diff capability requires original source and admitted commit grant"));}
    let _original=diff;
    commit(mint_capability())
}

/// 🗝️ One keyed row of a map-valued field's sparse diff: insert a new key, replace an existing key's value, or remove an existing
/// key. Rows are the unit of change for keyed collections (a per-window camera, a per-id record), never the whole map.
#[derive(Clone, Debug, PartialEq)]
pub enum KeyedRow<V> {
    Insert(V),
    Replace(V),
    Remove,
}

impl<V> KeyedRow<V> {
    /// ⏮️ Composes this row (applied first) with a `later` row of the same key into the single row of the pair, `None` when they
    /// cancel (insert then remove): replace∘replace is one replace, insert∘replace is one insert, remove∘insert is a replace.
    pub fn then(self, later: Self) -> Option<Self> {
        match (self, later) {
            (Self::Insert(_), Self::Remove) => None,
            (Self::Insert(_), Self::Replace(value)) => Some(Self::Insert(value)),
            (Self::Replace(_), Self::Remove) => Some(Self::Remove),
            (Self::Remove, Self::Insert(value)) => Some(Self::Replace(value)),
            (_, later) => Some(later),
        }
    }
}

impl<V: ToValue> ToValue for KeyedRow<V> {
    fn to_value(&self) -> DslValue {
        let (op, value) = match self {
            Self::Insert(value) => ("insert", Some(value)),
            Self::Replace(value) => ("replace", Some(value)),
            Self::Remove => ("remove", None),
        };
        let mut entries = vec![("op".to_string(), DslValue::String(op.to_string()))];
        if let Some(value) = value {
            entries.push(("value".to_string(), value.to_value()));
        }
        DslValue::object(entries)
    }
}

impl<V: FromValue> FromValue for KeyedRow<V> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        let field = |name: &str| entries.iter().find(|(key, _)| key == name).map(|(_, value)| value.clone());
        let operation = match field("op") {
            Some(DslValue::String(operation)) => operation,
            _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue, "a keyed row requires a string op")),
        };
        let payload = || field("value").ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, format!("a keyed {operation} row requires a value"))).and_then(|value| V::from_value(value).map_err(|error| error.under("value")));
        match operation.as_str() {
            "insert" => Ok(Self::Insert(payload()?)),
            "replace" => Ok(Self::Replace(payload()?)),
            "remove" => Ok(Self::Remove),
            other => Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("unknown keyed row op {other}"))),
        }
    }
}

/// 🗂️ The keyed rows of one map-valued field.
pub type KeyedRows<V> = BTreeMap<String, KeyedRow<V>>;

/// ⌨️ The map after `rows`: an insert of an existing key, or a replace/remove of a missing one, is refused (`field` names it).
pub fn keyed_rows_apply<V: Clone>(base: &BTreeMap<String, V>, rows: &KeyedRows<V>, field: &str) -> MutationApplyResult<BTreeMap<String, V>> {
    let mut next = base.clone();
    for (key, row) in rows {
        let refusal = |code: &str, message: &str| MutationApplyError::new(code, format!("{message} {key}")).at([field, key.as_str()]);
        match row {
            KeyedRow::Insert(value) => {
                if next.insert(key.clone(), value.clone()).is_some() {
                    return Err(refusal("mutation.apply.duplicate-target", "row already exists:"));
                }
            }
            KeyedRow::Replace(value) => {
                if next.insert(key.clone(), value.clone()).is_none() {
                    return Err(refusal("mutation.apply.missing-target", "row does not exist:"));
                }
            }
            KeyedRow::Remove => {
                if next.remove(key).is_none() {
                    return Err(refusal("mutation.apply.missing-target", "row does not exist:"));
                }
            }
        }
    }
    Ok(next)
}

/// ↩️ The rows that undo `rows` from `base`: an insert is removed, a replace restores the base value, a remove re-inserts it.
pub fn keyed_rows_inverse<V: Clone>(base: &BTreeMap<String, V>, rows: &KeyedRows<V>) -> KeyedRows<V> {
    rows.iter()
        .filter_map(|(key, row)| {
            let prior = base.get(key);
            let inverse = match (row, prior) {
                (KeyedRow::Insert(_), _) => KeyedRow::Remove,
                (KeyedRow::Replace(_), Some(prior)) => KeyedRow::Replace(prior.clone()),
                (KeyedRow::Remove, Some(prior)) => KeyedRow::Insert(prior.clone()),
                _ => return None,
            };
            Some((key.clone(), inverse))
        })
        .collect()
}

/// 🧱️ The rows turning `base` into `next`: inserts, replaces where the value differs, removes of keys `next` lacks.
pub fn keyed_rows_changing<V: Clone + PartialEq>(base: &BTreeMap<String, V>, next: &BTreeMap<String, V>) -> KeyedRows<V> {
    let mut rows = KeyedRows::new();
    for (key, value) in next {
        match base.get(key) {
            None => {
                rows.insert(key.clone(), KeyedRow::Insert(value.clone()));
            }
            Some(prior) if prior != value => {
                rows.insert(key.clone(), KeyedRow::Replace(value.clone()));
            }
            Some(_) => {}
        }
    }
    for key in base.keys().filter(|key| !next.contains_key(*key)) {
        rows.insert(key.clone(), KeyedRow::Remove);
    }
    rows
}

/// 🔐️ Composes `later` rows into `rows` key by key through [`KeyedRow::then`], dropping rows that cancel.
pub fn keyed_rows_absorb<V>(rows: &mut KeyedRows<V>, later: KeyedRows<V>) {
    for (key, row) in later {
        match rows.remove(&key) {
            Some(first) => {
                if let Some(combined) = first.then(row) {
                    rows.insert(key, combined);
                }
            }
            None => {
                rows.insert(key, row);
            }
        }
    }
}

/// 🧬️ Generates the sparse per-field diff of `$record` from its field list: one `Option<FieldType>` slot per field, `absorb` is
/// later-slot-wins, `inverse` is the base values of the set slots, `is_empty` is every slot `None`, `apply` builds the record from
/// the set slots over `base` (the struct literal names every field, so a field added to the record without joining a list fails
/// to compile), and `$diff::changing(&base, &record)` sets exactly the slots where `record` differs from `base`. An optional
/// `keyed: { name: Value, … }` list names the record's `BTreeMap<String, Value>` fields, whose slots are [`KeyedRows`] (insert /
/// replace / remove one key) instead of a whole map; `absorb` composes rows per key and cancels insert-then-remove. Schema-derived
/// concrete code, not a runtime generic: the invoking crate owns the emitted type. `$record` must be `Clone + PartialEq`, every
/// field type `Clone + PartialEq + ToValue + FromValue`.
///
/// 🎯️ The `whole` arm (a single `replacement` slot) is for an indivisible root — an ephemeral transient root moved whole by the
/// ephemeral transfer, or an `Arc`-shared engine state — never for persisted configs.
#[macro_export]
macro_rules! sparse_record_diff {
    (record: $record:ident, diff: $diff:ident, whole $(,)?) => {
        $crate::sparse_whole_diff! { record: $record, diff: $diff }
    };
    (record: $record:ident, diff: $diff:ident, fields: { $($field:ident : $ty:ty),+ $(,)? } $(,)?) => {
        $crate::sparse_record_diff! { record: $record, diff: $diff, fields: { $($field: $ty),+ }, keyed: {} }
    };
    (
        record: $record:ident,
        diff: $diff:ident,
        fields: { $($field:ident : $ty:ty),+ $(,)? },
        keyed: { $($kf:ident : $kv:ty),* $(,)? } $(,)?
    ) => {
        #[doc = concat!("🔺️ Sparse diff of [`", stringify!($record), "`]: one optional slot per field, keyed rows per map field.")]
        #[derive(Clone, Debug, Default, PartialEq)]
        pub struct $diff {
            $(pub $field: Option<$ty>,)+
            $(pub $kf: $crate::KeyedRows<$kv>,)*
        }

        impl $diff {
            #[doc = concat!("🧱️ The diff that sets exactly the slots and rows where `record` differs from `base` — the sparse diff of a `replace-", stringify!($record), "` kind, read from its payload and its base.")]
            pub fn changing(base: &$record, record: &$record) -> Self {
                Self {
                    $($field: (base.$field != record.$field).then(|| record.$field.clone()),)+
                    $($kf: $crate::keyed_rows_changing(&base.$kf, &record.$kf),)*
                }
            }
        }

        impl $crate::__value::ToValue for $diff {
            fn to_value(&self) -> $crate::__value::DslValue {
                let mut entries: Vec<(String, $crate::__value::DslValue)> = Vec::new();
                $(if let Some(value) = &self.$field {
                    entries.push((stringify!($field).to_string(), $crate::__value::ToValue::to_value(value)));
                })+
                $(if !self.$kf.is_empty() {
                    entries.push((stringify!($kf).to_string(), $crate::__value::DslValue::object(self.$kf.iter().map(|(key, row)| (key.clone(), $crate::__value::ToValue::to_value(row))).collect::<Vec<_>>())));
                })*
                $crate::__value::DslValue::object(entries)
            }
        }

        impl $crate::__value::FromValue for $diff {
            fn from_value(value: $crate::__value::DslValue) -> Result<Self, $crate::__value::ValueError> {
                let mut diff = Self::default();
                for (key, entry) in value.into_object()? {
                    $(if key == stringify!($field) {
                        diff.$field = Some(<$ty as $crate::__value::FromValue>::from_value(entry).map_err(|error| error.under(stringify!($field)))?);
                        continue;
                    })+
                    $(if key == stringify!($kf) {
                        for (row_key, row) in entry.into_object().map_err(|error| error.under(stringify!($kf)))? {
                            diff.$kf.insert(row_key.clone(), <$crate::KeyedRow<$kv> as $crate::__value::FromValue>::from_value(row).map_err(|error| error.under(stringify!($kf)).under(row_key))?);
                        }
                        continue;
                    })*
                    return Err($crate::__value::ValueError::new($crate::__value::ValueRefusalKind::InvalidValue, format!("unknown {} field {key}", stringify!($diff))));
                }
                Ok(diff)
            }
        }

        impl $crate::DiffAlgebra<$record> for $diff {


            fn is_empty(&self) -> bool {
                true $(&& self.$field.is_none())+ $(&& self.$kf.is_empty())*
            }
        }
impl $crate::ColdDiffAlgebra<$record> for $diff {
fn inverse(&self, base: &$record) -> Self {
                Self {
                    $($field: self.$field.as_ref().map(|_| base.$field.clone()),)+
                    $($kf: $crate::keyed_rows_inverse(&base.$kf, &self.$kf),)*
                }
            }
}

        impl $crate::MutationDiff<$record> for $diff {}
        impl $crate::ColdMutationDiff<$record> for $diff {
            fn apply(&self, base: &mut $record, capability: $crate::ApplyCapability) -> $crate::MutationApplyResult<()> {
                let prepared=$crate::PreparedField::new($record {
                    $($field: self.$field.clone().unwrap_or_else(|| base.$field.clone()),)+
                    $($kf: $crate::keyed_rows_apply(&base.$kf, &self.$kf, stringify!($kf))?,)*
                },drop,capability);prepared.install(base);Ok(())
            }

            fn absorb(&mut self, other: Self) {
                $(if other.$field.is_some() {
                    self.$field = other.$field;
                })+
                $($crate::keyed_rows_absorb(&mut self.$kf, other.$kf);)*
            }
        }
    };
}

/// 📹️ The `whole` arm of [`sparse_record_diff!`] for a record that is one indivisible entity (an `Arc`-shared root, an opaque
/// engine state): a single `replacement` slot, set exactly when the requested record differs from the base.
#[doc(hidden)]
#[macro_export]
macro_rules! sparse_whole_diff {
    (record: $record:ident, diff: $diff:ident) => {
        #[doc = concat!("🔺️ Diff of the indivisible [`", stringify!($record), "`]: its replacement, or nothing.")]
        #[derive(Clone, Debug, Default, PartialEq)]
        pub struct $diff {
            pub replacement: Option<$record>,
        }

        impl $diff {
            #[doc = "🧱️ The diff that installs `record`."]
            pub fn replacing(record: &$record) -> Self {
                Self { replacement: Some(record.clone()) }
            }

            #[doc = "🧱️ The diff that installs `record` unless `base` already holds it."]
            pub fn changing(base: &$record, record: &$record) -> Self {
                Self { replacement: (base != record).then(|| record.clone()) }
            }
        }

        impl $crate::__value::ToValue for $diff {
            fn to_value(&self) -> $crate::__value::DslValue {
                let mut entries: Vec<(String, $crate::__value::DslValue)> = Vec::new();
                if let Some(value) = &self.replacement {
                    entries.push(("replacement".to_string(), $crate::__value::ToValue::to_value(value)));
                }
                $crate::__value::DslValue::object(entries)
            }
        }

        impl $crate::__value::FromValue for $diff {
            fn from_value(value: $crate::__value::DslValue) -> Result<Self, $crate::__value::ValueError> {
                let mut diff = Self::default();
                for (key, entry) in value.into_object()? {
                    if key == "replacement" {
                        diff.replacement = Some(<$record as $crate::__value::FromValue>::from_value(entry).map_err(|error| error.under("replacement"))?);
                        continue;
                    }
                    return Err($crate::__value::ValueError::new($crate::__value::ValueRefusalKind::InvalidValue, format!("unknown {} field {key}", stringify!($diff))));
                }
                Ok(diff)
            }
        }

        impl $crate::DiffAlgebra<$record> for $diff {


            fn is_empty(&self) -> bool {
                self.replacement.is_none()
            }
        }
impl $crate::ColdDiffAlgebra<$record> for $diff {
fn inverse(&self, base: &$record) -> Self {
                Self { replacement: self.replacement.as_ref().map(|_| base.clone()) }
            }
}

        impl $crate::MutationDiff<$record> for $diff {}
        impl $crate::ColdMutationDiff<$record> for $diff {
            fn apply(&self, base: &mut $record, capability: $crate::ApplyCapability) -> $crate::MutationApplyResult<()> {
                if let Some(replacement)=&self.replacement{$crate::PreparedField::new(replacement.clone(),drop,capability).install(base);}Ok(())
            }

            fn absorb(&mut self, other: Self) {
                if other.replacement.is_some() {
                    self.replacement = other.replacement;
                }
            }
        }
    };
}


#[path="../🚫️error/♻️retirement/🦀️.rs"]
mod apply_error_retirement;
