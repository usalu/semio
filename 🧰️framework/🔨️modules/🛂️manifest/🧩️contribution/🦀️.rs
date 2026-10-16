//! 🧩️ Plugin contributions: topic rows, versions and pins, inference and mutation metadata, space index and canonical trees.

use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_pack_json::ArtifactCanonicalJsonNode as Node;
use semio_framework_pack_json::ArtifactCanonicalJsonText as Text;
use semio_framework_pack_json::ArtifactCanonicalJsonTree as Tree;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

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
/// 🧩️ One host-aggregated plugin contribution entry — the element shape of the paged
/// `setContributions` command payload, never a view-state field.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ProgramContributionEntry {
    pub plugin_id: String,
    #[value(default)]
    pub topic_contribution: Option<TopicContribution>,
}

/// 📕️ Parses one assembled `setContributions` payload into typed entries — the ONE route
/// contributions reach a guest by.
pub fn parse_contributions(json: &str) -> Vec<ProgramContributionEntry> {
    semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default()
}

//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
//#region 🔖️SpaceIndexContribution
/// 📇️ Topic under which a plugin names the artifact-index document the os shell opens for a hub space; the framework reads the contribution and never names the plugin that supplies it.
pub const SPACE_INDEX_TOPIC: &str = "os.space-index";

/// 📇️ Payload of [`SPACE_INDEX_TOPIC`]: the dialect of the app that edits the space index and the document schema the shell opens it with.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpaceIndexContribution {
    pub dialect: ArtifactDialect,
    pub document_schema: String,
}

impl SpaceIndexContribution {
    /// 📤️ Wraps the contribution as the open topic row a plugin publishes.
    pub fn into_topic_contribution(self) -> TopicContribution {
        TopicContribution::new(SPACE_INDEX_TOPIC, ToValue::to_value(&self))
    }

    /// 📥️ Reads the first well-formed [`SPACE_INDEX_TOPIC`] row among a manifest's topic contributions.
    pub fn from_topic_contributions(contributions: &[TopicContribution]) -> Option<Self> {
        contributions.iter().filter(|row| row.topic == SPACE_INDEX_TOPIC).find_map(|row| row.decode::<Self>().ok())
    }
}

//#endregion 🔖️SpaceIndexContribution
fn absent(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}

impl Tree for TopicContribution {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(2))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn Tree,ValueError>{match ordinal{0=>Ok(&self.topic),1=>Ok(&self.payload),_=>Err(absent("canonical topic contribution ordinal is absent"))}}
 fn canonical_tree_key(&self,ordinal:usize)->Result<Text<'_>,ValueError>{match ordinal{0=>Ok("topic".into()),1=>Ok("payload".into()),_=>Err(absent("canonical topic contribution key is absent"))}}
}

impl Tree for ProgramContributionEntry {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(2))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn Tree,ValueError>{match ordinal{0=>Ok(&self.plugin_id),1=>Ok(&self.topic_contribution),_=>Err(absent("canonical program contribution ordinal is absent"))}}
 fn canonical_tree_key(&self,ordinal:usize)->Result<Text<'_>,ValueError>{match ordinal{0=>Ok("pluginId".into()),1=>Ok("topicContribution".into()),_=>Err(absent("canonical program contribution key is absent"))}}
}

#[cfg(test)]
use semio_framework_value::Number;
#[cfg(test)]
use semio_framework_value::retained_clone::RetainedCloneGrant;

#[cfg(test)]
#[path = "../🌲️canonical/🧪️tests/🦀️.rs"]
mod tests;
