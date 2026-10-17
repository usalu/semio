//! 📋️ Mutation metadata enums: the invertibility, diff participation, outcome class, composition and language surface enums with their value conversions. See [`super`].

//#region 🪪️MutationLeafDescriptor
/// 🧷️ Schema vocabulary for one direct mutation's inversion behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MutationInvertibility {
    SelfInvertible,
    ExplicitMutation,
    Plan,
    NonInvertible,
}

/// 🌱️ Hand-written, not derived — same DAG reason `MutationMessage`'s hand-written twin below
/// documents. `Serialize`-only in the original (no `Deserialize`: these are static roster metadata,
/// never hydrated from the wire), so only `ToValue` is mirrored — capability parity, not more.
impl semio_framework_value::ToValue for MutationInvertibility {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::String(
            match self {
                MutationInvertibility::SelfInvertible => "self",
                MutationInvertibility::ExplicitMutation => "explicit-mutation",
                MutationInvertibility::Plan => "plan",
                MutationInvertibility::NonInvertible => "non-invertible",
            }
            .to_string(),
        )
    }
}

/// 🧷️ Schema vocabulary for one direct mutation's diff participation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MutationDiffParticipation {
    Detect,
    ApplyOnly,
    Plan,
    None,
}

/// 🌱️ Hand-written, not derived — same reason as `MutationInvertibility` above.
impl semio_framework_value::ToValue for MutationDiffParticipation {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::String(
            match self {
                MutationDiffParticipation::Detect => "detect",
                MutationDiffParticipation::ApplyOnly => "apply-only",
                MutationDiffParticipation::Plan => "plan",
                MutationDiffParticipation::None => "none",
            }
            .to_string(),
        )
    }
}

/// 🧷️ The protocol outcome class one direct mutation can reach — the vocabulary fixtures, manifests and bridges
/// share verbatim, so no layer projects one record onto another.
///
/// @see 🧰️framework/🔨️modules/🧪️test/🧬️schema/🔣️.json — `$defs/MutationOutcomeClass`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MutationOutcomeClass {
    Applied,
    NoOp,
    Empty,
    Disjoint,
    Rejected,
}

impl MutationOutcomeClass {
    /// 🏷️ The wire spelling of this class.
    pub const fn as_str(self) -> &'static str {
        match self {
            MutationOutcomeClass::Applied => "applied",
            MutationOutcomeClass::NoOp => "no-op",
            MutationOutcomeClass::Empty => "empty",
            MutationOutcomeClass::Disjoint => "disjoint",
            MutationOutcomeClass::Rejected => "rejected",
        }
    }
}

/// 🌱️ Hand-written, not derived — same reason as `MutationInvertibility` above.
impl semio_framework_value::ToValue for MutationOutcomeClass {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::String(self.as_str().to_string())
    }
}

/// 🧷️ Schema vocabulary for one direct mutation's composition form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MutationComposition {
    Atomic,
    Composite,
}

/// 🌱️ Hand-written, not derived — same reason as `MutationInvertibility` above.
impl semio_framework_value::ToValue for MutationComposition {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::String(match self { MutationComposition::Atomic => "atomic", MutationComposition::Composite => "composite" }.to_string())
    }
}

/// 🧷️ Schema vocabulary for a direct mutation's required language surfaces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MutationLanguageSurface {
    Rust,
    Typescript,
    Graphql,
    Protobuf,
    JsonSchema,
    Text,
    Binary,
}

/// 🌱️ Hand-written, not derived — same reason as `MutationInvertibility` above.
impl semio_framework_value::ToValue for MutationLanguageSurface {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::String(
            match self {
                MutationLanguageSurface::Rust => "rust",
                MutationLanguageSurface::Typescript => "typescript",
                MutationLanguageSurface::Graphql => "graphql",
                MutationLanguageSurface::Protobuf => "protobuf",
                MutationLanguageSurface::JsonSchema => "json-schema",
                MutationLanguageSurface::Text => "text",
                MutationLanguageSurface::Binary => "binary",
            }
            .to_string(),
        )
    }
}
//#endregion 🪪️MutationLeafDescriptor
