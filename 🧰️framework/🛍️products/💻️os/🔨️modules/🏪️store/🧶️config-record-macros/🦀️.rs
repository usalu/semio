//! 🧶️ Config record macros: the field set mutations generator, the config record declaration and the config diff declaration. See [`super`].

//#region 🔖️Config
/// 🎚️ Generates `$set`, the per-field set mutations of `$record`: one `Set…(value)` variant per field whose diff sets that one
/// slot of `$diff` (empty when the base already holds the value) and whose inverse is the same variant carrying the base value.
/// There is deliberately no whole-record variant: a caller holding a whole next record dispatches `$set::setting(&base, &next)`, one set
/// mutation per changed field. (The one exception is an ephemeral transient root, whose transfer moves a whole root — see
/// `transient_root!` in the plugin crate.) The enum owns its leaf descriptors (`owner/kind`), its `{"kind": …, "value": …}` wire shape
/// and semantic value lowering. Representation owners explicitly declare their text and binary codecs.
#[macro_export]
macro_rules! field_set_mutations {
    (
        record: $record:ident,
        diff: $diff:ident,
        set: $set:ident,
        owner: $owner:literal,
        payload_schema: $schema:literal,
        emoji: $emoji:literal,
        fields: { $($field:ident : $ty:ty => $variant:ident $kind:literal),+ $(,)? } $(,)?
    ) => {
        #[doc = concat!("🎚️ The set mutations of [`", stringify!($record), "`].")]
        #[derive(Clone, Debug, PartialEq, $crate::__value::RetainedClone)]
        pub enum $set {
            $($variant($ty)),+
        }

        impl $crate::snapshot_clone_preparation::ConfigApplyMutation<$record> for $set {
            fn exchange(self, post: &mut $record) -> Result<Self, ($crate::__value::ValueError, Self)> {
                Ok(match self {
                    $(Self::$variant(value) => Self::$variant(std::mem::replace(&mut post.$field, value))),+
                })
            }
        }

        impl $crate::__value::retirement::RetireOwned for $set {
            fn retirement(self)->Box<dyn $crate::__value::retirement::RetirementCursor>{match self{$(Self::$variant(original)=>$crate::__value::retirement::RetireOwned::retirement(original)),+}}
            fn retirement_birth_bytes(&self)->Option<usize>{match self{$(Self::$variant(original)=>$crate::__value::retirement::RetireOwned::retirement_birth_bytes(original)),+}}
            fn controlled_retirement_supported()->bool{true}
        }

        impl $crate::__value::ToValue for $set {
            fn to_value(&self) -> $crate::__value::DslValue {
                match self {
                    $(Self::$variant(value) => $crate::__value::DslValue::object(vec![
                        ("kind".to_string(), $crate::__value::DslValue::String($kind.to_string())),
                        ("value".to_string(), $crate::__value::ToValue::to_value(value)),
                    ]),)+
                }
            }
        }

        impl $crate::__value::FromValue for $set {
            fn from_value(value: $crate::__value::DslValue) -> Result<Self, $crate::__value::ValueError> {
                let entries = value.into_object()?;
                if let Some((member, _)) = entries.iter().find(|(key, _)| key != "kind" && key != "value") {
                    return Err($crate::__value::ValueError::new($crate::__value::ValueRefusalKind::InvalidValue, format!("unknown {} member {member}", stringify!($set))));
                }
                let kind = match entries.iter().find(|(key, _)| key == "kind") {
                    Some((_, $crate::__value::DslValue::String(kind))) => kind.clone(),
                    _ => return Err($crate::__value::ValueError::new($crate::__value::ValueRefusalKind::InvalidValue, concat!(stringify!($set), " requires a string kind"))),
                };
                let entry = |name: &str| {
                    entries
                        .iter()
                        .find(|(key, _)| key == name)
                        .map(|(_, value)| value.clone())
                        .ok_or_else(|| $crate::__value::ValueError::new($crate::__value::ValueRefusalKind::InvalidValue, format!("{} {kind} requires {name}", stringify!($set))))
                };
                $(if kind == $kind {
                    return Ok(Self::$variant(<$ty as $crate::__value::FromValue>::from_value(entry("value")?).map_err(|error| error.under("value"))?));
                })+
                Err($crate::__value::ValueError::new($crate::__value::ValueRefusalKind::InvalidValue, format!("unknown {} kind {kind}", stringify!($set))))
            }
        }

        impl $crate::Mutation<$record> for $set {
            type Diff = $diff;

            const DESCRIPTORS: &'static [$crate::MutationLeafDescriptor] = &[
                $($crate::MutationLeafDescriptor {
                    schema_version: 1,
                    owner: concat!($owner, "/", $kind),
                    semantic_kind: $kind,
                    display_name: $kind,
                    emoji: $emoji,
                    aggregate_variant: stringify!($variant),
                    payload_schema: $schema,
                    text_opcode: None,
                    binary_tag: None,
                    invertibility: $crate::MutationInvertibility::ExplicitMutation,
                    diff_participation: $crate::MutationDiffParticipation::Detect,
                    outcome_classes: &[$crate::MutationOutcomeClass::Applied],
                    composition: $crate::MutationComposition::Atomic,
                    required_language_surfaces: &[$crate::MutationLanguageSurface::Rust, $crate::MutationLanguageSurface::JsonSchema],
                }),+
            ];

            fn descriptor(&self) -> &'static $crate::MutationLeafDescriptor {
                let variant = match self {
                    $(Self::$variant(_) => stringify!($variant)),+
                };
                Self::DESCRIPTORS.iter().find(|descriptor| descriptor.aggregate_variant == variant).expect("every set mutation variant declares its descriptor")
            }

            fn diff(&self, base: &$record) -> $crate::MutationOutcome<$diff> {
                match self {
                    $(Self::$variant(value) => {
                        if base.$field == *value {
                            return $crate::MutationOutcome::empty();
                        }
                        $crate::MutationOutcome::new($diff { $field: Some(value.clone()), ..<$diff as Default>::default() })
                    })+
                }
            }

            fn inverse(&self, base: &$record) -> Result<Vec<Self>, $crate::__value::ValueError> {
                Ok(vec![match self {
                    $(Self::$variant(_) => Self::$variant(base.$field.clone())),+
                }])
            }
        }

        impl $set {
            #[doc = concat!("🎚️ The set mutations that turn `base` into `next`: one per field where they differ, in field order.")]
            pub fn setting(base: &$record, next: &$record) -> Vec<Self> {
                let mut mutations = Vec::new();
                $(if base.$field != next.$field {
                    mutations.push(Self::$variant(next.$field.clone()));
                })+
                mutations
            }
        }

    };
}

/// 🎯️ Declares a config record's sparse diff ([`sparse_record_diff!`]), its per-field set mutations ([`field_set_mutations!`]; no
/// whole-record variant, callers dispatch `$set::setting(&base, &next)`) and its `ConfigRecord` mark in one invocation: `config_record! { record: X, diff: XDiff, set: XSet, owner: "…", payload_schema:
/// "…", emoji: "…", fields: { field: Type => SetField "set-field", … } }`. Configs that keep a hand-written mutation enum use
/// `config_diff!` for the diff alone.
#[macro_export]
macro_rules! config_record {
    (
        record: $record:ident,
        diff: $diff:ident,
        set: $set:ident,
        owner: $owner:literal,
        payload_schema: $schema:literal,
        emoji: $emoji:literal,
        fields: { $($field:ident : $ty:ty => $variant:ident $kind:literal),+ $(,)? } $(,)?
    ) => {
        impl $crate::ConfigRecord for $record {}
        $crate::sparse_record_diff! { record: $record, diff: $diff, fields: { $($field: $ty),+ } }
        $crate::field_set_mutations! {
            record: $record, diff: $diff, set: $set, owner: $owner, payload_schema: $schema, emoji: $emoji,
            fields: { $($field: $ty => $variant $kind),+ }
        }
    };
}

/// 🎯️ Declares a config record's `ConfigRecord` mark and sparse diff ([`sparse_record_diff!`]) for a config whose mutation enum is
/// hand-written: `config_diff! { record: X, diff: XDiff, fields: { field: Type, … } [, keyed: { map_field: ValueType, … }] }`.
#[macro_export]
macro_rules! config_diff {
    (record: $record:ident, diff: $diff:ident, fields: { $($field:ident : $ty:ty),+ $(,)? }, keyed: { $($kf:ident : $kv:ty),* $(,)? } $(,)?) => {
        impl $crate::ConfigRecord for $record {}
        $crate::sparse_record_diff! { record: $record, diff: $diff, fields: { $($field: $ty),+ }, keyed: { $($kf: $kv),* } }
    };
    (record: $record:ident, diff: $diff:ident, fields: { $($field:ident : $ty:ty),+ $(,)? } $(,)?) => {
        impl $crate::ConfigRecord for $record {}
        $crate::sparse_record_diff! { record: $record, diff: $diff, fields: { $($field: $ty),+ } }
    };
}
//#endregion 🔖️Config
