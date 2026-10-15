//! 🧾️ Plugin manifest: topic contributions, versions and pins, `tree_pin!`, plugin dependencies, artifact contributions,
//! `PluginManifest`, host-resolved argument choices and the dependency graph.
//!
//! Split from the manifest root by section C of `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/SEAM-MAP-PLUGIN-MANIFEST-HUB.md`.
use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️TopicContribution
/// 🗂️ Open contribution shape: a plugin declares a `topic` string instead of a hardcoded enum variant,
/// so the generic framework never has to know plugin-specific names. `topic` reuses the same
/// dot-namespaced vocabulary as a crate's existing `contributes`/`consumes` metadata (e.g.
/// `"flow.extension"`, `"playbook.blockKind"`, `"cad.computer"`) — each producer/consumer picks its
/// own topic string; this type does not enumerate them. See `component.ts`'s `TopicContribution` for
/// the mirror.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct TopicContribution {
    pub topic: String,
    pub payload: DslValue,
}

impl TopicContribution {
    pub fn new(topic: impl Into<String>, payload: DslValue) -> Self {
        Self { topic: topic.into(), payload }
    }

    /// 📕️ Decodes `payload` into a caller-chosen typed shape.
    pub fn decode<T: FromValue>(&self) -> Result<T, ValueError> {
        T::from_value(self.payload.clone())
    }
}

/// 🌉️ Hand-written, not derived (avoids a new `semio-framework-value-derive` dependency edge on
/// this file's two host crates — this shape is a two-field plain record, trivial either way):
/// `topic`/`payload` mirror the `#[serde(rename_all = "camelCase")]` wire shape exactly.
impl ToValue for TopicContribution {
    fn to_value(&self) -> DslValue {
        DslValue::object([("topic".to_string(), ToValue::to_value(&self.topic)), ("payload".to_string(), self.payload.clone())])
    }
}
impl FromValue for TopicContribution {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(fields) = value else {
            return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for TopicContribution, found {value:?}")));
        };
        let mut topic = None;
        let mut payload = None;
        for (key, entry) in fields {
            match key.as_str() {
                "topic" => topic = Some(String::from_value(entry).map_err(|e| e.under("topic"))?),
                "payload" => payload = Some(entry),
                _ => {}
            }
        }
        Ok(TopicContribution {
            topic: topic.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "TopicContribution missing topic"))?,
            payload: payload.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "TopicContribution missing payload"))?,
        })
    }
}

// 🚧️ Needed in serde form too: referenced by a `🚧️ BLOCKED` serde-only manifest type — hand-written
// (not derived, same reasoning as the `ToValue`/`FromValue` pair above), same `topic`/`payload`
// camelCase wire shape.
impl Serialize for TopicContribution {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("TopicContribution", 2)?;
        state.serialize_field("topic", &self.topic)?;
        state.serialize_field("payload", &self.payload)?;
        state.end()
    }
}
impl<'de> Deserialize<'de> for TopicContribution {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Raw {
            topic: String,
            payload: DslValue,
        }
        let raw = Raw::deserialize(deserializer)?;
        Ok(TopicContribution { topic: raw.topic, payload: raw.payload })
    }
}
//#endregion 🔖️TopicContribution

//#region 🔖️PluginDependency
/// 🔢️ A frozen `major.minor.patch` version triple — no external semver crate (contract freeze
/// `26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS` §3). `Ord` is
/// derived field-in-order (major, then minor, then patch), which is exactly semver precedence.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

/// 🌉️ Hand-written, not derived: `#[value(…)]` has no `into`/`try_from` equivalent — this mirrors
/// the retired `#[serde(into = "String", try_from = "String")]` bridge via the existing
/// `Display`/`FromStr` impls below.
impl ToValue for Version {
    fn to_value(&self) -> DslValue {
        DslValue::String(self.to_string())
    }
}
impl FromValue for Version {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => Version::parse(&s).map_err(|e| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string())),
            other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

/// 🚧️ Failure parsing a `Version` (`major.minor.patch`, all-numeric segments) or a `VersionPin`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VersionParseError {
    Malformed(String),
    NonNumeric(String, String),
}

impl std::fmt::Display for VersionParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Malformed(input) => write!(formatter, "expected `major.minor.patch`, got {input:?}"),
            Self::NonNumeric(input, segment) => write!(formatter, "non-numeric version segment {segment:?} in {input:?}"),
        }
    }
}

impl std::error::Error for VersionParseError {}

impl Version {
    // 🚫️async: E1 transitive — `TryFrom<String>`/`FromStr` (external) construct this synchronously;
    // pure field assembly, no I/O (R9).
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self { major, minor, patch }
    }

    /// 🔢️ Parses a strict `major.minor.patch` triple — no pre-release/build metadata, no leniency.
    // 🚫️async: E1 transitive — `FromStr::from_str`/`TryFrom<String>` (external) call this
    // synchronously; pure string parsing, no I/O (R9).
    pub fn parse(input: &str) -> Result<Self, VersionParseError> {
        let mut segments = input.split('.');
        let (Some(major), Some(minor), Some(patch), None) = (segments.next(), segments.next(), segments.next(), segments.next()) else {
            return Err(VersionParseError::Malformed(input.to_string()));
        };
        let segment = |raw: &str| raw.parse::<u64>().map_err(|_| VersionParseError::NonNumeric(input.to_string(), raw.to_string()));
        Ok(Self { major: segment(major)?, minor: segment(minor)?, patch: segment(patch)? })
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl std::str::FromStr for Version {
    type Err = VersionParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Version::parse(input)
    }
}

impl From<Version> for String {
    fn from(version: Version) -> Self {
        version.to_string()
    }
}

impl TryFrom<String> for Version {
    type Error = VersionParseError;
    fn try_from(raw: String) -> Result<Self, Self::Error> {
        Version::parse(&raw)
    }
}

/// 📌️ A dependency pin: the exact version (`=X.Y.Z`) of the plugin a manifest depends on. One tree is one catalog, and a
/// trusted catalog admits only exact pins inside its own closure (`trustedBootstrapDescriptorClaims`,
/// `🌎️hub/📦️packages/🦀️rust/📜️script.ts`), so a manifest carries no range grammar: a declaration pins the version its
/// own tree builds, through [`tree_pin!`](crate::tree_pin).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VersionPin(pub Version);

impl VersionPin {
    /// 🔢️ Parses the one wire form, `=X.Y.Z`; every range (`*`, `^`, `~`, `>=`, a bare triple) is refused.
    // 🚫️async: E1 transitive — `FromValue::from_value` (this file, below) calls this synchronously; pure string
    // parsing, no I/O (R9).
    pub fn parse(input: &str) -> Result<Self, VersionPinParseError> {
        let trimmed = input.trim();
        let Some(rest) = trimmed.strip_prefix('=') else { return Err(VersionPinParseError::NotExact(trimmed.to_string())) };
        Ok(Self(Version::parse(rest)?))
    }

    /// 📌️ Pins the version a crate of this tree is compiled at. [`tree_pin!`](crate::tree_pin) evaluates it in a
    /// `const` over the declaring crate's own `CARGO_PKG_VERSION`, so a version that is not a strict
    /// `major.minor.patch` triple is a compile error in that crate, never a guest trap.
    pub const fn of_tree(crate_version: &str) -> Self {
        let bytes = crate_version.as_bytes();
        let mut segments = [0u64; 3];
        let mut segment = 0;
        let mut digits = 0;
        let mut index = 0;
        while index < bytes.len() {
            let byte = bytes[index];
            if byte == b'.' {
                if digits == 0 || segment == 2 {
                    panic!("the compiled crate version is not a strict major.minor.patch triple");
                }
                segment += 1;
                digits = 0;
            } else if byte.is_ascii_digit() {
                segments[segment] = segments[segment] * 10 + (byte - b'0') as u64;
                digits += 1;
            } else {
                panic!("the compiled crate version is not a strict major.minor.patch triple");
            }
            index += 1;
        }
        if segment != 2 || digits == 0 {
            panic!("the compiled crate version is not a strict major.minor.patch triple");
        }
        Self(Version { major: segments[0], minor: segments[1], patch: segments[2] })
    }

    /// ✅️ Whether `version` is exactly the pinned version.
    // 🚫️async: E1 transitive — pure comparison consumed by `matches_raw`, itself required sync (R9).
    pub fn matches(&self, version: &Version) -> bool {
        &self.0 == version
    }

    /// ✅️ Convenience for the dependency graph: parses `raw` and compares; an unparsable target version never matches.
    // 🚫️async: E1 transitive — dependency-graph validation calls this synchronously via `!`; pure parse-and-compare,
    // no I/O (R9).
    pub fn matches_raw(&self, raw: &str) -> bool {
        Version::parse(raw).is_ok_and(|version| self.matches(&version))
    }
}

impl std::fmt::Display for VersionPin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "={}", self.0)
    }
}

/// 🚧️ Failure parsing a `VersionPin`: anything but `=X.Y.Z` (a range such as `*`, `^`, `~` or `>=` included).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VersionPinParseError {
    NotExact(String),
    Version(VersionParseError),
}

impl std::fmt::Display for VersionPinParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotExact(input) => write!(formatter, "a dependency pins an exact version `=X.Y.Z`, got {input:?}"),
            Self::Version(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for VersionPinParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Version(error) => Some(error),
            Self::NotExact(_) => None,
        }
    }
}

impl From<VersionParseError> for VersionPinParseError {
    fn from(error: VersionParseError) -> Self {
        Self::Version(error)
    }
}

impl ToValue for VersionPin {
    fn to_value(&self) -> DslValue {
        DslValue::String(self.to_string())
    }
}
impl FromValue for VersionPin {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => VersionPin::parse(&s).map_err(|e| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string())),
            other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only
// manifest type above/below (`PluginDependency.version`) — hand-written, same reasoning as the
// `ToValue`/`FromValue` pair above.
impl Serialize for VersionPin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> Deserialize<'de> for VersionPin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        VersionPin::parse(&raw).map_err(serde::de::Error::custom)
    }
}

/// 📌️ The exact pin of the tree the invoking crate is compiled from: `semio_framework::tree_pin!()` expands in the
/// declaring crate, so `CARGO_PKG_VERSION` is that crate's own workspace version, checked at compile time.
#[macro_export]
macro_rules! tree_pin {
    () => {{
        const TREE_PIN: $crate::VersionPin = $crate::VersionPin::of_tree(env!("CARGO_PKG_VERSION"));
        TREE_PIN
    }};
}

/// 🔗️ One direct plugin dependency: the depended-on plugin id plus the exact version it pins — see
/// `resolve_load_order`/`validate_dependency_graph`.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct PluginDependency {
    pub plugin_id: String,
    pub version: VersionPin,
}

impl PluginDependency {
    pub fn new(plugin_id: impl Into<String>, version: VersionPin) -> Self {
        Self { plugin_id: plugin_id.into(), version }
    }
}
//#endregion 🔖️PluginDependency

//#region 🔖️ArtifactContribution
/// 🗂️ The `verb`/`entity`/`kind`/`record` semantic identity of one contributed mutation, carried as
/// owned strings on the wire (the native `SemanticDescriptor` this mirrors lives in the os-kernel
/// protocol crate, which `semio-framework` must not require plugin manifests to link against).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ContributedMutationSemantics {
    pub verb: String,
    pub entity: String,
    pub kind: String,
    pub record: String,
}

/// 🗂️ One mutation a plugin contributes onto an artifact kind it depends on — the manifest-declared
/// counterpart of a `contributor.list-artifact-mutations` roster entry (contract freeze §3/§6).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ContributedMutationMetadata {
    /// 🪪️ `"<target-document-schema>#<contributor-plugin-id>:<kebab-kind>"` (contract freeze §3).
    pub mutation_id: String,
    pub semantics: ContributedMutationSemantics,
    pub schema_version: u32,
    pub algorithm_version: u32,
}

/// 💡️ One inference a plugin contributes onto an artifact kind it depends on — mirrors the native
/// `ArtifactInferenceServiceMetadata` fields (owned strings instead of `&'static str`, since this
/// travels over the wire in a manifest), plus `contributor`/`depends_on` for the contribution's own
/// identity and ordering (contract freeze §4: `owner == contributor`, `artifact_kind == target`).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ContributedInferenceMetadata {
    pub owner: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub artifact_schema_version: u32,
    pub inference_schema: String,
    pub inference_schema_version: u32,
    pub algorithm_version: u32,
    pub policy_version: u32,
    pub contributor: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
    /// 📜️ The PUBLISHED request/response contract of this inference — what a generic client has to
    /// send and what it gets back. Absent for an inference whose owner has not declared one: a
    /// gateway that cannot read a contract refuses the call by name instead of dispatching a body
    /// the guest will reject after minutes of guest time (`📓️pz2-puzzle-describe-under-budget.md`
    /// §5.2 measured exactly that: `inference_run` on `s.wfc.bitmap.solve` never answered inside
    /// 240 s because nothing in the tree said the payload needed a `snapshot`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<InferencePayloadContract>,
}

/// 📜️ One inference's published payload contract — the schema-first answer to "what do I send?".
/// `input_schema`/`output_schema` are JSON Schema 2020-12 documents as TEXT, authored beside the
/// inference and carried verbatim, so the gateway validates against the plugin's own declaration
/// instead of a host-side guess. `progress_unit` names what the bounded job counts, so a client can
/// label a progress bar without knowing the algorithm.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct InferencePayloadContract {
    pub payload_schema_id: String,
    pub input_schema: String,
    pub output_schema: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    #[value(default, skip_serializing_if = "String::is_empty")]
    pub progress_unit: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub artifact_binding: Option<InferenceArtifactBinding>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<InferenceCommitBinding>,
}

/// 📌️ How one inference's result becomes a document edit: the artifact action that commits it. The
/// action receives the result's fields it declares as arguments, and runs through the ordinary edit
/// path — its own policy, undo and ledger — so a committed result is an edit like any other.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct InferenceCommitBinding {
    pub action: String,
}

/// 🔗️ How one inference's canonical request is BOUND to an artifact document. An inference over an
/// artifact cannot be expressed by a client at all — nobody can type 4 096 bitmap cells into a tool
/// call — so the plugin declares the payload field its own document goes into and the host fills it
/// from the artifact the caller named. Declared, never invented: the host writes exactly the field
/// this row names, in exactly the encoding it names, and refuses anything else by name.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct InferenceArtifactBinding {
    pub field: String,
    pub encoding: String,
    pub required: bool,
}

/// 📦️ The ONE encoding an artifact binding may name today: the artifact's canonical `pack`/`spr`
/// pair, each base64, as `{ "pack": "…", "spr": "…" }` under the declared field. A binding naming
/// anything else is refused at the gateway rather than silently filled with a shape the guest
/// cannot read.
pub const INFERENCE_ARTIFACT_PACK_BASE64: &str = "artifact-pack-base64";

/// 🗂️ Everything one plugin contributes onto one artifact kind it depends on — see the registration
/// gates in contract freeze §4 (accepted only when `artifact_kind`'s owner is a direct
/// `PluginManifest.dependencies` entry).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ArtifactContributionDescriptor {
    pub artifact_kind: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub mutations: Vec<ContributedMutationMetadata>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub inferences: Vec<ContributedInferenceMetadata>,
}
//#endregion 🔖️ArtifactContribution

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PluginManifest {
    pub plugin_id: String,
    pub label: String,
    pub version: String,
    pub apps: Vec<AppDefinition>,
    pub examples: Vec<ExampleDefinition>,
    #[serde(default)]
    #[value(default)]
    pub capabilities: Vec<kernel::CapabilityRequirement>,
    /// 🗂️ Open plugin contributions — see `TopicContribution`.
    #[serde(default)]
    #[value(default)]
    pub topic_contributions: Vec<TopicContribution>,
    /// 🎛️ Plugin-scope commands this program exposes — apply whenever any of its apps is focused.
    #[serde(default)]
    #[value(default)]
    pub commands: Vec<CommandDefinition>,
    /// 🗂️ Plugin-level artifact kinds (library plugins with zero apps declare kinds here).
    #[serde(default)]
    #[value(default)]
    pub artifact_kinds: Vec<ArtifactKindSpec>,
    /// 🏠️ Artifact kinds another package owns whose documents this plugin's apps open — one row per kind and document
    /// schema its owner's codecs decode (`PluginBuilder::host_artifact`, or a surface of another plugin's artifact the plugin
    /// registers). Distinct from `artifact_kinds`: the codec rows stay
    /// the owner's, and a trusted catalog binds these kinds to the owner's codecs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub hosted_artifact_kinds: Vec<HostedArtifactKind>,
    /// 🔗️ Direct plugin dependencies this plugin requires to load — see `PluginDependency`/
    /// `resolve_load_order`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<PluginDependency>,
    /// 🗂️ Artifact-kind contributions (mutations/inferences) this plugin contributes onto artifact
    /// kinds it depends on — see `ArtifactContributionDescriptor`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub contributions: Vec<ArtifactContributionDescriptor>,
}

//#region 🔖️HostResolvedArgs
/// 🗂️ One artifact-kind choice offered by an `ActionArgControl::ArtifactKind` dialog field —
/// resolved by the host from its live plugin catalogue (`artifact_kind_choices`) into a plain
/// `Select { options }` right before the dialog renders. Round-trips through `ActionArgOption.value`
/// as JSON via `encode_artifact_kind_choice`/`decode_artifact_kind_choice` — the frozen wire shape
/// (contract §C8.1): `{"kindId":"s.draw.draw","schema":"draw.document","dialect":{"artifactKind":
/// "s.draw.draw","standard":"1","subset":"*"},"label":{"en":"Draw","de":"Zeichnung"}}`. TS twin:
/// `ArtifactKindChoice` (`🟦️.ts`) — both codecs must agree byte-for-byte over the pinned
/// fixtures.
#[derive(Clone, Debug, PartialEq)]
pub struct ArtifactKindChoice {
    pub kind_id: String,
    pub schema: String,
    pub dialect: ArtifactDialect,
    pub label: LocalizedLabel,
}

/// 🎭️ One `(pluginId, appId, role)` choice offered by an `ActionArgControl::SurfaceApp` dialog
/// field — resolved by the host against the dialect coordinate found in the dialog's seed argument
/// named `dialect_arg`. Round-trips through `ActionArgOption.value` as JSON via
/// `encode_surface_app_choice`/`decode_surface_app_choice`: `{"pluginId":"draw","appId":"s.draw.draw
/// @1/*#editor","role":"editor"}`. TS twin: `SurfaceAppChoice` (`🟦️.ts`).
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceAppChoice {
    pub app: AppRef,
    pub role: AppRole,
}

/// 🧵️ Encodes an `ArtifactKindChoice` into the frozen `ActionArgOption.value` JSON shape — `label`
/// resolves under `Terminology::Native`, the only terminology this wire shape carries (a dialog
/// re-resolves display strings client-side under the active terminology from `kind_id`/`schema`
/// alone if it ever needs to, but the frozen shape itself is native-only, matching `IconSelect`'s own
/// `classifier_kind`-not-label precedent for host-resolved controls).
pub async fn encode_artifact_kind_choice(choice: &ArtifactKindChoice) -> String {
    semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
        ("kindId".to_string(), choice.kind_id.as_str().into()),
        ("schema".to_string(), choice.schema.as_str().into()),
        ("dialect".to_string(), semio_framework_pack_json::from_dsl_value(&choice.dialect.to_value())),
        (
            "label".to_string(),
            semio_framework_pack_json::object([
                ("en".to_string(), choice.label.resolve(Terminology::Native, Locale::En).to_string().into()),
                ("de".to_string(), choice.label.resolve(Terminology::Native, Locale::De).to_string().into()),
            ]),
        ),
    ]))
}

/// 🧵️ Inverse of `encode_artifact_kind_choice`.
pub async fn decode_artifact_kind_choice(value: &str) -> Result<ArtifactKindChoice, String> {
    let json = semio_framework_pack_json::parse(value, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("malformed artifact kind choice JSON: {error}"))?;
    let kind_id = json.get("kindId").and_then(semio_framework_pack_json::Value::as_str).ok_or_else(|| "artifact kind choice missing string field kindId".to_string())?.to_string();
    let schema = json.get("schema").and_then(semio_framework_pack_json::Value::as_str).ok_or_else(|| "artifact kind choice missing string field schema".to_string())?.to_string();
    let dialect: ArtifactDialect = json
        .get("dialect")
        .cloned()
        .ok_or_else(|| "artifact kind choice missing field dialect".to_string())
        .and_then(|value| ArtifactDialect::from_value(semio_framework_pack_json::to_dsl_value(&value)).map_err(|error| format!("artifact kind choice has a malformed dialect: {error}")))?;
    let en = json.pointer("/label/en").and_then(semio_framework_pack_json::Value::as_str).ok_or_else(|| "artifact kind choice missing string field label.en".to_string())?;
    let de = json.pointer("/label/de").and_then(semio_framework_pack_json::Value::as_str).ok_or_else(|| "artifact kind choice missing string field label.de".to_string())?;
    Ok(ArtifactKindChoice { kind_id, schema, dialect, label: LocalizedLabel::native(en, de) })
}

/// 🧵️ Encodes a `SurfaceAppChoice` into its frozen `ActionArgOption.value` JSON shape.
pub async fn encode_surface_app_choice(choice: &SurfaceAppChoice) -> String {
    semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
        ("pluginId".to_string(), choice.app.plugin_id.as_str().into()),
        ("appId".to_string(), choice.app.app_id.as_str().into()),
        ("role".to_string(), choice.role.as_str().into()),
    ]))
}

/// 🧵️ Inverse of `encode_surface_app_choice`.
pub async fn decode_surface_app_choice(value: &str) -> Result<SurfaceAppChoice, String> {
    let json = semio_framework_pack_json::parse(value, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("malformed surface app choice JSON: {error}"))?;
    let plugin_id = json.get("pluginId").and_then(semio_framework_pack_json::Value::as_str).ok_or_else(|| "surface app choice missing string field pluginId".to_string())?.to_string();
    let app_id = json.get("appId").and_then(semio_framework_pack_json::Value::as_str).ok_or_else(|| "surface app choice missing string field appId".to_string())?.to_string();
    let role_str = json.get("role").and_then(semio_framework_pack_json::Value::as_str).ok_or_else(|| "surface app choice missing string field role".to_string())?;
    let role: AppRole = role_str.parse()?;
    Ok(SurfaceAppChoice { app: AppRef { plugin_id, app_id }, role })
}

/// 🗂️ Every artifact-kind choice for the given `roles` — Rust twin of TS `artifactKindChoices` (law in
/// `🧪️tests/🧪️hostresolvedargs/🟦️.ts`): every app across `manifests` whose `role` is in `roles`, whose
/// `io.artifact_schema` is non-empty and whose package declares a kind of that schema (on any of its apps
/// — a viewer shares its editor's kind — or on the manifest) contributes one choice per dialect coordinate,
/// labelled with that kind's own label, never its app's; an undeclared schema is not a creatable kind.
/// Deduped by dialect coordinate (first manifest/app wins — callers pass owner manifests first so the
/// owner's label wins over a later contributor's), sorted by coordinate for determinism.
pub async fn artifact_kind_choices(manifests: &[PluginManifest], roles: &[AppRole]) -> Vec<ArtifactKindChoice> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    let mut by_coordinate: BTreeMap<String, ArtifactKindChoice> = BTreeMap::new();
    for manifest in manifests {
        for app in &manifest.apps {
            if !roles.contains(&app.role) || app.io.artifact_schema.is_empty() {
                continue;
            }
            let Some(kind) = manifest.apps.iter().flat_map(|other| other.artifact_kinds.iter()).chain(manifest.artifact_kinds.iter()).find(|kind| kind.schema == app.io.artifact_schema) else {
                continue;
            };
            by_coordinate.entry(app.dialect.to_coordinate()).or_insert_with(|| ArtifactKindChoice { kind_id: app.dialect.artifact_kind.clone(), schema: app.io.artifact_schema.clone(), dialect: app.dialect.clone(), label: kind.label.clone() });
        }
    }
    by_coordinate.into_values().collect()
}
//#endregion 🔖️HostResolvedArgs

//#region 🔖️DependencyGraph
/// 🚧️ Typed dependency-graph validation failures — contract freeze §4/§5: missing dependency,
/// version mismatch, or a cycle (naming every plugin id on the cycle, in traversal order).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DependencyGraphError {
    MissingDependency { plugin_id: String, depends_on: String },
    VersionMismatch { plugin_id: String, depends_on: String, required: String, actual: String },
    Cycle { members: Vec<String> },
}

impl std::fmt::Display for DependencyGraphError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingDependency { plugin_id, depends_on } => write!(formatter, "plugin `{plugin_id}` depends on unknown plugin `{depends_on}`"),
            Self::VersionMismatch { plugin_id, depends_on, required, actual } => {
                write!(formatter, "plugin `{plugin_id}` requires `{depends_on}` `{required}` but the loaded version is `{actual}`")
            }
            Self::Cycle { members } => write!(formatter, "dependency cycle among plugins: {}", members.join(" -> ")),
        }
    }
}

impl std::error::Error for DependencyGraphError {}

/// ✅️ Checks every declared dependency resolves to a loaded plugin at a satisfying version —
/// deterministic: manifests are checked in input order, each manifest's dependencies in declaration
/// order, so the first violation found is always the same for the same input.
fn validate_dependency_graph(manifests: &[PluginManifest]) -> Result<(), DependencyGraphError> {
    let by_id: BTreeMap<&str, &PluginManifest> = manifests.iter().map(|manifest| (manifest.plugin_id.as_str(), manifest)).collect();
    for manifest in manifests {
        for dependency in &manifest.dependencies {
            let Some(target) = by_id.get(dependency.plugin_id.as_str()) else {
                return Err(DependencyGraphError::MissingDependency { plugin_id: manifest.plugin_id.clone(), depends_on: dependency.plugin_id.clone() });
            };
            if !dependency.version.matches_raw(&target.version) {
                return Err(DependencyGraphError::VersionMismatch { plugin_id: manifest.plugin_id.clone(), depends_on: dependency.plugin_id.clone(), required: dependency.version.to_string(), actual: target.version.clone() });
            }
        }
    }
    Ok(())
}

/// 🧭️ Kahn toposort of the plugin dependency graph: a dependency always precedes its dependents,
/// and among several simultaneously-ready plugins the lexicographically smallest id is always
/// picked next, so the returned order is a pure, deterministic function of the input set. Runs
/// `validate_dependency_graph` first, so a missing dependency or version mismatch is reported
/// before any cycle would be detected.
pub fn resolve_load_order(manifests: &[PluginManifest]) -> Result<Vec<String>, DependencyGraphError> {
    validate_dependency_graph(manifests)?;

    let mut in_degree: BTreeMap<&str, usize> = manifests.iter().map(|manifest| (manifest.plugin_id.as_str(), 0)).collect();
    let mut dependents_of: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for manifest in manifests {
        for dependency in &manifest.dependencies {
            *in_degree.get_mut(manifest.plugin_id.as_str()).expect("validated above") += 1;
            dependents_of.entry(dependency.plugin_id.as_str()).or_default().push(manifest.plugin_id.as_str());
        }
    }

    let mut ready: std::collections::BTreeSet<&str> = in_degree.iter().filter(|(_, degree)| **degree == 0).map(|(id, _)| *id).collect();
    let mut order: Vec<String> = Vec::with_capacity(manifests.len());
    while let Some(next) = ready.iter().next().copied() {
        ready.remove(next);
        order.push(next.to_string());
        if let Some(dependents) = dependents_of.get(next) {
            let mut sorted_dependents = dependents.clone();
            sorted_dependents.sort_unstable();
            for dependent in sorted_dependents {
                let degree = in_degree.get_mut(dependent).expect("validated above");
                *degree -= 1;
                if *degree == 0 {
                    ready.insert(dependent);
                }
            }
        }
    }

    if order.len() != manifests.len() {
        let resolved: std::collections::BTreeSet<&str> = order.iter().map(String::as_str).collect();
        let leftover: std::collections::BTreeSet<String> = manifests.iter().map(|manifest| manifest.plugin_id.as_str()).filter(|id| !resolved.contains(id)).map(str::to_string).collect();
        return Err(DependencyGraphError::Cycle { members: find_cycle_members(manifests, &leftover) });
    }
    Ok(order)
}

/// 🔁️ Walks the leftover (never-ready) subgraph depth-first from its lexicographically smallest
/// node, following each plugin's first declared dependency that is also leftover, until a node
/// repeats — the repeated slice of the walked path is the named cycle.
fn find_cycle_members(manifests: &[PluginManifest], leftover: &std::collections::BTreeSet<String>) -> Vec<String> {
    let by_id: BTreeMap<&str, &PluginManifest> = manifests.iter().map(|manifest| (manifest.plugin_id.as_str(), manifest)).collect();
    let mut visited: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for start in leftover {
        if visited.contains(start) {
            continue;
        }
        let mut path: Vec<String> = Vec::new();
        let mut on_path: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        let mut node = start.clone();
        loop {
            if on_path.contains(&node) {
                let cycle_start = path.iter().position(|id| id == &node).expect("on_path implies present in path");
                return path[cycle_start..].to_vec();
            }
            if visited.contains(&node) {
                break;
            }
            visited.insert(node.clone());
            on_path.insert(node.clone());
            path.push(node.clone());
            let next = by_id.get(node.as_str()).and_then(|manifest| manifest.dependencies.iter().map(|dependency| dependency.plugin_id.clone()).find(|id| leftover.contains(id)));
            match next {
                Some(next_node) => node = next_node,
                None => break,
            }
        }
    }
    leftover.iter().cloned().collect()
}

/// 🔎️ Every plugin (direct dependents only, not transitive) that declares `plugin_id` as a
/// dependency, sorted for determinism — used to refuse unload/hot-reload while dependents are
/// loaded (contract freeze §4).
pub fn dependents(manifests: &[PluginManifest], plugin_id: &str) -> Vec<String> {
    let mut result: Vec<String> = manifests.iter().filter(|manifest| manifest.dependencies.iter().any(|dependency| dependency.plugin_id == plugin_id)).map(|manifest| manifest.plugin_id.clone()).collect();
    result.sort_unstable();
    result
}
//#endregion 🔖️DependencyGraph

#[cfg(test)]
#[path = "../🧪️tests/🔬️plugin-dependency/🦀️.rs"]
mod plugin_dependency_tests;
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
