//! 🧪️ Private runner-case examples using the actual production process and plan contracts.
use semio_framework_repo_test_runner::*;
use serde::{Deserialize, Serialize};

/// 🧭️ One runner-detection vector.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct DetectionVector {
    /// 🏷️ Scenario-local id.
    pub id: String,
    /// 📦️ The bundle root to probe.
    #[serde(rename = "bundleRoot", default)]
    pub bundle_root: String,
    /// 🎯️ Filter handed to the JavaScript runner detection.
    #[serde(rename = "testFilter", default)]
    pub test_filter: String,
    /// 🗣️ The language detection must answer.
    #[serde(rename = "expectedLanguage", default)]
    pub expected_language: String,
    /// 📜️ The JavaScript runner argv detection must answer, where the vector declares one.
    #[serde(rename = "expectedArgv", default, skip_serializing_if = "Option::is_none")]
    pub expected_argv: Option<Vec<String>>,
}

/// 🧭️ The runner-detection fixture.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct DetectionVectors {
    /// 🌍️ The world the vectors probe.
    #[serde(default)]
    pub snapshot: FilesystemSnapshot,
    /// 🧭️ The vectors.
    #[serde(default)]
    pub vectors: Vec<DetectionVector>,
}

/// 🗺️ One invocation-planning vector.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct PlanningVector {
    /// 🏷️ Scenario-local id.
    pub id: String,
    /// 🔭️ The scope to plan.
    pub scope: TestScope,
    /// 🗺️ The plan the scope must produce.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected: Option<InvocationPlan>,
}

/// 🗺️ The invocation-planning fixture.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct PlanningVectors {
    /// 🌍️ The world the scopes are planned against.
    #[serde(default)]
    pub snapshot: FilesystemSnapshot,
    /// 🗺️ The vectors.
    #[serde(default)]
    pub vectors: Vec<PlanningVector>,
}

/// 🧬️ The scope-identifier fixture.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct SelectorVectors {
    /// 🧬️ Raw selectors as a caller would type them.
    #[serde(default)]
    pub selectors: Vec<String>,
}

/// 📊️ One recorded runner report.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TranscriptVector {
    /// 🏷️ Scenario-local id.
    pub id: String,
    /// 🏭️ The runner that produced it.
    pub runner: Runner,
    /// 📤️ The recorded report.
    pub output: ProcessOutput,
}

/// 📊️ The result-parsing fixture.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct TranscriptVectors {
    /// 📊️ The vectors.
    #[serde(default)]
    pub vectors: Vec<TranscriptVector>,
}

/// 🛑️ The cancellation fixture.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct CancellationVectors {
    /// 🗺️ The plan to run.
    #[serde(default)]
    pub plan: InvocationPlan,
    /// 🎞️ The transcripts that answer it.
    #[serde(default)]
    pub transcripts: Vec<RecordedTranscript>,
    /// 🛑️ Cancel once this many invocations have completed.
    #[serde(rename = "cancelAfter", default)]
    pub cancel_after: usize,
}

/// 🧭️ Reads a runner-detection fixture.
pub fn parse_detection_vectors(bytes: &[u8]) -> Result<DetectionVectors, String> {
    serde_json::from_slice(bytes).map_err(|error| error.to_string())
}

/// 🗺️ Reads an invocation-planning fixture.
pub fn parse_planning_vectors(bytes: &[u8]) -> Result<PlanningVectors, String> {
    serde_json::from_slice(bytes).map_err(|error| error.to_string())
}

/// 🧬️ Reads a scope-identifier fixture.
pub fn parse_selector_vectors(bytes: &[u8]) -> Result<SelectorVectors, String> {
    serde_json::from_slice(bytes).map_err(|error| error.to_string())
}

/// 📊️ Reads a result-parsing fixture.
pub fn parse_transcript_vectors(bytes: &[u8]) -> Result<TranscriptVectors, String> {
    serde_json::from_slice(bytes).map_err(|error| error.to_string())
}

/// 🛑️ Reads a cancellation fixture.
pub fn parse_cancellation_vectors(bytes: &[u8]) -> Result<CancellationVectors, String> {
    serde_json::from_slice(bytes).map_err(|error| error.to_string())
}

