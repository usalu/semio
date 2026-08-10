//! 🧬️ DAG snapshot schema — persistent fields only.

use crate::artifacts::dag::{DagFixtureEdge, DagNodeSpec, DAG_DOCUMENT_SCHEMA};
use infinite_board_port_directed_dag::directed_dag::{DagMedia, DagNodeKind, DagPreviewContent, IoPortSpec};
use math::graph::manifest::PropertyBag;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted DAG document snapshot (nodes + edges).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.dag.dag")]
pub struct DagSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub nodes: Vec<DagNodeSpec>,
    #[state(persistent)]
    #[serde(default)]
    pub edges: Vec<DagFixtureEdge>,
}

impl Default for DagSnapshot {
    fn default() -> Self {
        default_snapshot()
    }
}

/// 🌱 Canonical default document used by the play app and examples.
pub fn default_snapshot() -> DagSnapshot {
    crate::artifacts::dag::dsl::parse_dsl(crate::artifacts::dag::dsl::DAG_EXAMPLE_TEXT)
        .expect("bundled dag example DSL must parse")
}
//#endregion 🔖️Snapshot

//#region 🔖️DslMirror
// 🧬️ `DagNodeKind` is `#[serde(flatten)]`-merged onto `DagNodeSpec` at the JSON level, and its own
// `Preview` variant carries a nested tagged enum (`DagPreviewContent`). The crate::os_dsl:: derive engine
// represents "exactly one nested tagged value" via `#[dsl(statements)] Box<T>` (`RequiredStatements`),
// which needs a `Box` wrapper the REAL `DagNodeKind`/`DagNodeSpec` fields deliberately don't carry
// (dozens of call sites here and in `dag-plugin`/`framework/surface/node-graph`/`flow/core` destructure
// `node.kind`/`DagNodeKind::Preview { content, .. }` directly — boxing those fields would ripple far
// outside this crate's ownership). So, exactly like `imperative/core/rs`'s `ImperativeMutationDsl`
// mirror, `DagNodeKindDsl`/`DagNodeSpecDsl`/`DagNodePatchDsl`/`DagSnapshotDsl`/`DagMutationDsl` are
// LOCAL structural twins that box only where the derive requires it; the real domain types keep their
// original unboxed shape and never leave this crate — conversion happens right at this boundary.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
pub(crate) enum DagNodeKindDsl {
    Computation {
        #[dsl(table)]
        inputs: Vec<IoPortSpec>,
        #[dsl(table)]
        outputs: Vec<IoPortSpec>,
        variadic_inputs: bool,
        variadic_outputs: bool,
    },
    Slider {
        min: f64,
        max: f64,
        step: f64,
        value: f64,
        output: IoPortSpec,
    },
    Select {
        options: Vec<String>,
        selected: usize,
        output: IoPortSpec,
    },
    Screen {
        media: Option<DagMedia>,
        input: IoPortSpec,
    },
    Note {
        text: String,
        output: IoPortSpec,
    },
    Image {
        src: String,
        output: IoPortSpec,
    },
    Preview {
        #[dsl(statements)]
        content: Box<DagPreviewContent>,
        expanded: Vec<String>,
        input: IoPortSpec,
    },
    Action {
        label: String,
        input: IoPortSpec,
    },
    Export {
        label: String,
        format: String,
        input: IoPortSpec,
    },
    Cluster {
        #[dsl(table)]
        inputs: Vec<IoPortSpec>,
        #[dsl(table)]
        outputs: Vec<IoPortSpec>,
    },
    AppInstance {
        instance_id: String,
        plugin_id: String,
        app_id: String,
        icon: String,
        #[dsl(table)]
        inputs: Vec<IoPortSpec>,
        #[dsl(table)]
        outputs: Vec<IoPortSpec>,
    },
}

pub(crate) fn dag_node_kind_to_dsl(kind: &DagNodeKind) -> DagNodeKindDsl {
    match kind {
        DagNodeKind::Computation { inputs, outputs, variadic_inputs, variadic_outputs } => DagNodeKindDsl::Computation { inputs: inputs.clone(), outputs: outputs.clone(), variadic_inputs: *variadic_inputs, variadic_outputs: *variadic_outputs },
        DagNodeKind::Slider { min, max, step, value, output } => DagNodeKindDsl::Slider { min: *min, max: *max, step: *step, value: *value, output: output.clone() },
        DagNodeKind::Select { options, selected, output } => DagNodeKindDsl::Select { options: options.clone(), selected: *selected, output: output.clone() },
        DagNodeKind::Screen { media, input } => DagNodeKindDsl::Screen { media: media.clone(), input: input.clone() },
        DagNodeKind::Note { text, output } => DagNodeKindDsl::Note { text: text.clone(), output: output.clone() },
        DagNodeKind::Image { src, output } => DagNodeKindDsl::Image { src: src.clone(), output: output.clone() },
        DagNodeKind::Preview { content, expanded, input } => DagNodeKindDsl::Preview { content: Box::new(content.clone()), expanded: expanded.iter().cloned().collect(), input: input.clone() },
        DagNodeKind::Action { label, input } => DagNodeKindDsl::Action { label: label.clone(), input: input.clone() },
        DagNodeKind::Export { label, format, input } => DagNodeKindDsl::Export { label: label.clone(), format: format.clone(), input: input.clone() },
        DagNodeKind::Cluster { inputs, outputs } => DagNodeKindDsl::Cluster { inputs: inputs.clone(), outputs: outputs.clone() },
        DagNodeKind::AppInstance { instance_id, plugin_id, app_id, icon, inputs, outputs } => {
            DagNodeKindDsl::AppInstance { instance_id: instance_id.clone(), plugin_id: plugin_id.clone(), app_id: app_id.clone(), icon: icon.clone(), inputs: inputs.clone(), outputs: outputs.clone() }
        }
    }
}

pub(crate) fn dag_node_kind_from_dsl(kind: DagNodeKindDsl) -> DagNodeKind {
    match kind {
        DagNodeKindDsl::Computation { inputs, outputs, variadic_inputs, variadic_outputs } => DagNodeKind::Computation { inputs, outputs, variadic_inputs, variadic_outputs },
        DagNodeKindDsl::Slider { min, max, step, value, output } => DagNodeKind::Slider { min, max, step, value, output },
        DagNodeKindDsl::Select { options, selected, output } => DagNodeKind::Select { options, selected, output },
        DagNodeKindDsl::Screen { media, input } => DagNodeKind::Screen { media, input },
        DagNodeKindDsl::Note { text, output } => DagNodeKind::Note { text, output },
        DagNodeKindDsl::Image { src, output } => DagNodeKind::Image { src, output },
        DagNodeKindDsl::Preview { content, expanded, input } => DagNodeKind::Preview { content: *content, expanded: expanded.into_iter().collect(), input },
        DagNodeKindDsl::Action { label, input } => DagNodeKind::Action { label, input },
        DagNodeKindDsl::Export { label, format, input } => DagNodeKind::Export { label, format, input },
        DagNodeKindDsl::Cluster { inputs, outputs } => DagNodeKind::Cluster { inputs, outputs },
        DagNodeKindDsl::AppInstance { instance_id, plugin_id, app_id, icon, inputs, outputs } => DagNodeKind::AppInstance { instance_id, plugin_id, app_id, icon, inputs, outputs },
    }
}

/// 🧬️ Mirror of {@link DagNodeSpec} — every field identical except `kind`, boxed only here (see the
/// region's opening doc comment).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub(crate) struct DagNodeSpecDsl {
    id: String,
    name: String,
    abbreviation: String,
    icon: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    operator_kind: Option<String>,
    properties: PropertyBag,
    #[dsl(statements)]
    kind: Box<DagNodeKindDsl>,
}

pub(crate) fn dag_node_spec_to_dsl(node: &DagNodeSpec) -> DagNodeSpecDsl {
    DagNodeSpecDsl {
        id: node.id.clone(),
        name: node.name.clone(),
        abbreviation: node.abbreviation.clone(),
        icon: node.icon.clone(),
        x: node.x,
        y: node.y,
        width: node.width,
        height: node.height,
        operator_kind: node.operator_kind.clone(),
        properties: node.properties.clone(),
        kind: Box::new(dag_node_kind_to_dsl(&node.kind)),
    }
}

pub(crate) fn dag_node_spec_from_dsl(mirror: DagNodeSpecDsl) -> DagNodeSpec {
    DagNodeSpec {
        id: mirror.id,
        name: mirror.name,
        abbreviation: mirror.abbreviation,
        icon: mirror.icon,
        x: mirror.x,
        y: mirror.y,
        width: mirror.width,
        height: mirror.height,
        operator_kind: mirror.operator_kind,
        properties: mirror.properties,
        kind: dag_node_kind_from_dsl(*mirror.kind),
    }
}

/// 🧬️ Mirror of {@link DagSnapshot} — `nodes: Vec<DagNodeSpecDsl>` instead of `Vec<DagNodeSpec>` since
/// `DagNodeSpec` itself can't implement `dsl::DslField` (its `kind` field isn't boxed).
#[derive(Clone, Debug, PartialEq, dsl::DslArtifact)]
#[dsl(extension = "dag")]
#[dsl(layout = "lines")]
pub(crate) struct DagSnapshotDsl {
    schema: String,
    nodes: Vec<DagNodeSpecDsl>,
    #[dsl(table)]
    edges: Vec<DagFixtureEdge>,
}

pub(crate) fn dag_snapshot_to_dsl(snapshot: &DagSnapshot) -> DagSnapshotDsl {
    DagSnapshotDsl { schema: snapshot.schema.clone(), nodes: snapshot.nodes.iter().map(dag_node_spec_to_dsl).collect(), edges: snapshot.edges.clone() }
}

pub(crate) fn dag_snapshot_from_dsl(mirror: DagSnapshotDsl) -> DagSnapshot {
    DagSnapshot { schema: mirror.schema, nodes: mirror.nodes.into_iter().map(dag_node_spec_from_dsl).collect(), edges: mirror.edges }
}


impl store::ArtifactDsl for DagSnapshotDsl {
    const EXTENSION: &'static str = "dag";
    fn envelope_id() -> &'static str {
        "dag.dag"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for DagSnapshotDsl {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl store::ArtifactDsl for DagSnapshot {
    const EXTENSION: &'static str = "dag";
    fn envelope_id() -> &'static str {
        "dag.dag"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let parsed = <DagSnapshotDsl as store::ArtifactDsl>::parse_dsl(text)?;
        let mut snapshot = dag_snapshot_from_dsl(parsed);
        snapshot.schema = DAG_DOCUMENT_SCHEMA.into();
        Ok(snapshot)
    }
    fn print_dsl(&self) -> String {
        <DagSnapshotDsl as store::ArtifactDsl>::print_dsl(&dag_snapshot_to_dsl(self))
    }
}

impl store::ArtifactPack for DagSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <DagSnapshotDsl as store::ArtifactPack>::encode_pack_with(&dag_snapshot_to_dsl(self), options)
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let parsed = <DagSnapshotDsl as store::ArtifactPack>::decode_pack_with(bytes, options)?;
        let mut snapshot = dag_snapshot_from_dsl(parsed);
        snapshot.schema = DAG_DOCUMENT_SCHEMA.into();
        Ok(snapshot)
    }
}

//#region 🔖️ArtifactCodecs
impl From<DagSnapshot> for infinite_board_port_directed_dag::DagSnapshot {
    fn from(value: DagSnapshot) -> Self {
        Self { schema: value.schema, nodes: value.nodes, edges: value.edges }
    }
}

impl From<infinite_board_port_directed_dag::DagSnapshot> for DagSnapshot {
    fn from(value: infinite_board_port_directed_dag::DagSnapshot) -> Self {
        Self { schema: value.schema, nodes: value.nodes, edges: value.edges }
    }
}

impl From<&DagSnapshot> for infinite_board_port_directed_dag::DagSnapshot {
    fn from(value: &DagSnapshot) -> Self {
        value.clone().into()
    }
}

//#endregion 🔖️ArtifactCodecs
