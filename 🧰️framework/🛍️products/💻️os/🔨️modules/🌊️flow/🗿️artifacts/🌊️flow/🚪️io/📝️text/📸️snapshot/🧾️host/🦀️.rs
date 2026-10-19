//! 🚪️ Native text representation of the Flow host snapshot over the model's component codecs.

use std::collections::BTreeMap;

use crate::io;
use crate::component_dsl::{synapse_from_dsl, synapse_to_dsl, widget_dsl_to_widget, widget_to_widget_dsl, SynapseDsl, WidgetDsl};
use crate::{CameraJson, FlowHostSnapshot, WidgetLayout};

/// 📜️ Handcrafted ArtifactDsl (P6): derive no longer emits ArtifactDsl/ArtifactPack.
impl pack::snapshot::ArtifactDsl for FlowHostSnapshotDsl {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, crate::os_store::TextError> {
        let body = match crate::os_store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        crate::os_store::semio_format::wrap_text(&envelope, &body)
    }
}

impl pack::snapshot::ArtifactDsl for FlowHostSnapshot {
    const EXTENSION: &'static str = "flow";

    fn envelope_id() -> &'static str {
        <FlowHostSnapshotDsl as pack::snapshot::ArtifactDsl>::envelope_id()
    }

    fn parse_dsl(text: &str) -> Result<Self, crate::os_store::TextError> {
        let dsl_fixture = <FlowHostSnapshotDsl as pack::snapshot::ArtifactDsl>::parse_dsl(text)?;
        flow_host_snapshot_dsl_to_host_snapshot(dsl_fixture).map_err(|message| crate::os_store::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message, crate::os_store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        <FlowHostSnapshotDsl as pack::snapshot::ArtifactDsl>::print_dsl(&flow_host_snapshot_to_dsl(self))
    }
}

/// 📄️ Local mirror of `FlowHostSnapshot` — see this region's opening doc comment for why `widgets:
/// Vec<Widget>` (which embeds foreign `Dictionary`/`Tree` types) can't stay as-is under a direct
/// `#[derive(crate::os_dsl::DslArtifact)]`. `FlowArtifact` (the derived read-view built by
/// `FlowHostSnapshot::to_artifact()`) deliberately does NOT get this treatment — it's a computed
/// snapshot for rendering, never itself round-tripped through DSL text.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, crate::os_dsl::DslArtifact)]
#[artifact(id = "flow.flow")]
#[dsl(layout = "lines")]
pub(crate) struct FlowHostSnapshotDsl {
    schema: String,
    #[dsl(block)]
    camera: CameraJson,
    #[dsl(statements, block)]
    widgets: Vec<WidgetDsl>,
    #[dsl(table)]
    synapses: Vec<SynapseDsl>,
    layout: BTreeMap<String, WidgetLayout>,
}

pub(crate) fn flow_host_snapshot_to_dsl(host_snapshot: &FlowHostSnapshot) -> FlowHostSnapshotDsl {
    FlowHostSnapshotDsl {
        schema: host_snapshot.schema.clone(),
        camera: host_snapshot.camera.clone(),
        widgets: host_snapshot.widgets.iter().map(widget_to_widget_dsl).collect(),
        synapses: host_snapshot.synapses.iter().map(synapse_to_dsl).collect(),
        layout: host_snapshot.layout.iter().map(|(key, value)| (key.clone(), value.clone())).collect(),
    }
}

pub(crate) fn flow_host_snapshot_dsl_to_host_snapshot(dsl: FlowHostSnapshotDsl) -> Result<FlowHostSnapshot, String> {
    Ok(FlowHostSnapshot {
        schema: dsl.schema,
        camera: dsl.camera,
        widgets: dsl.widgets.into_iter().map(widget_dsl_to_widget).collect::<Result<Vec<_>, _>>()?,
        synapses: dsl.synapses.into_iter().map(synapse_from_dsl).collect::<Result<Vec<_>, _>>()?,
        layout: dsl.layout.into_iter().collect(),
    })
}

impl semio_framework_dsl_record::BorrowedDslField for FlowHostSnapshot {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<FlowHostSnapshotDsl>);
}

/// 📄️ Explicit import payloads share the artifact's intrinsic DSL schema.
impl semio_framework_dsl_record::DslField for FlowHostSnapshot {
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::FieldValue, semio_framework_value::ValueError> { io::sqlite::snapshot::flow_native_carrier::encode_field(self, control) }
    fn from_value_controlled(value: &semio_framework_dsl_record::FieldValue, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> { io::sqlite::snapshot::flow_native_carrier::decode_field(value, control) }
    fn shape() -> semio_framework_dsl_record::Shape { semio_framework_dsl_record::Shape::Record(FlowHostSnapshotDsl::__dsl_spec_producer()) }
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{control.checkpoint()?;Ok(semio_framework_dsl_record::Shape::Record(FlowHostSnapshotDsl::__dsl_spec_producer()))}
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue { semio_framework_dsl_record::FieldValue::Record(flow_host_snapshot_to_dsl(self).__dsl_to_record()) }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Record(record) => flow_host_snapshot_dsl_to_host_snapshot(FlowHostSnapshotDsl::__dsl_from_record(record).map_err(|error| error.message)?),
            other => Err(format!("expected Flow fixture record, found {other:?}")),
        }
    }
}
