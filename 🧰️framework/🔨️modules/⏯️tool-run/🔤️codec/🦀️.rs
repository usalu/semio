//! 🔤️ Tool run codec: record specs and the pure encode and decode of identity, step, progress and page records.

use super::*;

//#region 🔖️Codec
/// 🧨️ Encode or decode failure of a tool run wire value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToolRunCodecError {
    Pack(String),
    Malformed(&'static str),
    Limit(&'static str),
}

impl std::fmt::Display for ToolRunCodecError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pack(detail) => write!(formatter, "toolRun.codec.pack: {detail}"),
            Self::Malformed(what) => write!(formatter, "toolRun.codec.malformed: {what}"),
            Self::Limit(what) => write!(formatter, "toolRun.codec.limit: {what}"),
        }
    }
}

impl std::error::Error for ToolRunCodecError {}

pub(crate) fn field(id: u16, key: &str, shape: Shape) -> FieldSpec {
    FieldSpec::new(id, key, shape)
}

fn spec(fields: Vec<FieldSpec>) -> RecordSpec {
    RecordSpec::new(None, semio_framework_dsl_record::RecordLayout::Inline, fields)
}

fn identity_spec_owned() -> RecordSpec {
    spec(vec![field(1, "appInstanceId", Shape::UInt), field(2, "run", Shape::UInt), field(3, "generation", Shape::UInt), field(4, "baseRevision", Shape::Bytes64)])
}

fn step_spec_owned() -> RecordSpec {
    spec(vec![field(1, "sequence", Shape::UInt), field(2, "kind", Shape::UInt), field(3, "stage", Shape::UInt), field(4, "reason", Shape::UInt), field(5, "subject", Shape::UInt).optional(), field(6, "repeat", Shape::UInt), field(7, "args", Shape::Bytes64).optional()])
}

fn progress_spec_owned() -> RecordSpec {
    spec(vec![
        field(1, "identity", Shape::Record(identity_spec_producer())),
        field(2, "sequence", Shape::UInt),
        field(3, "state", Shape::UInt),
        field(4, "stage", Shape::UInt),
        field(5, "completed", Shape::UInt),
        field(6, "total", Shape::UInt).optional(),
        field(7, "counters", Shape::Bytes64).optional(),
        field(8, "unitsPerSecond", Shape::Float),
        field(9, "conflicts", Shape::UInt),
        field(10, "steps", Shape::List(Box::new(Shape::Record(step_spec_producer())))).optional(),
    ])
}

fn identity_spec_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<RecordSpec,semio_framework_value::ValueError>{control.scoped_stage(|control|{control.begin_stage(4)?;let mut fields=control.allocate_vec(4)?;for(id,key,shape)in[(1,"appInstanceId",Shape::UInt),(2,"run",Shape::UInt),(3,"generation",Shape::UInt),(4,"baseRevision",Shape::Bytes64)]{fields.push(semio_framework_dsl_record::producer::field(id,key,shape,control)?);control.step()?;}semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Inline,fields,control)})}
fn step_spec_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<RecordSpec,semio_framework_value::ValueError>{control.scoped_stage(|control|{control.begin_stage(7)?;let mut fields=control.allocate_vec(7)?;for(id,key,shape,optional)in[(1,"sequence",Shape::UInt,false),(2,"kind",Shape::UInt,false),(3,"stage",Shape::UInt,false),(4,"reason",Shape::UInt,false),(5,"subject",Shape::UInt,true),(6,"repeat",Shape::UInt,false),(7,"args",Shape::Bytes64,true)]{let mut field=semio_framework_dsl_record::producer::field(id,key,shape,control)?;field.optional=optional;fields.push(field);control.step()?;}semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Inline,fields,control)})}
fn progress_spec_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<RecordSpec,semio_framework_value::ValueError>{control.scoped_stage(|control|{control.begin_stage(10)?;let mut fields=control.allocate_vec(10)?;let steps=Shape::List(semio_framework_dsl_record::producer::boxed(Shape::Record(step_spec_producer()),control)?);for(id,key,shape,optional)in[(1,"identity",Shape::Record(identity_spec_producer()),false),(2,"sequence",Shape::UInt,false),(3,"state",Shape::UInt,false),(4,"stage",Shape::UInt,false),(5,"completed",Shape::UInt,false),(6,"total",Shape::UInt,true),(7,"counters",Shape::Bytes64,true),(8,"unitsPerSecond",Shape::Float,false),(9,"conflicts",Shape::UInt,false),(10,"steps",steps,true)]{let mut field=semio_framework_dsl_record::producer::field(id,key,shape,control)?;field.optional=optional;fields.push(field);control.step()?;}semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Inline,fields,control)})}
fn identity_spec_producer()->semio_framework_dsl_record::RecordSpecProducer{semio_framework_dsl_record::RecordSpecProducer{ordinary:identity_spec_owned,decoding:|control|identity_spec_controlled(control),encoding:|control|identity_spec_controlled(control)}}
fn step_spec_producer()->semio_framework_dsl_record::RecordSpecProducer{semio_framework_dsl_record::RecordSpecProducer{ordinary:step_spec_owned,decoding:|control|step_spec_controlled(control),encoding:|control|step_spec_controlled(control)}}
fn progress_spec_producer()->semio_framework_dsl_record::RecordSpecProducer{semio_framework_dsl_record::RecordSpecProducer{ordinary:progress_spec_owned,decoding:|control|progress_spec_controlled(control),encoding:|control|progress_spec_controlled(control)}}
fn payload_update_spec_owned()->RecordSpec{spec(vec![field(1,"kind",Shape::UInt),field(2,"publication",Shape::UInt).optional(),field(3,"totalBytes",Shape::UInt).optional(),field(4,"offset",Shape::UInt).optional(),field(5,"chunkBytes",Shape::UInt).optional(),field(6,"final",Shape::UInt).optional()])}
fn payload_update_spec_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<RecordSpec,semio_framework_value::ValueError>{control.scoped_stage(|control|{control.begin_stage(6)?;let mut fields=control.allocate_vec(6)?;for(id,key)in[(1,"kind"),(2,"publication"),(3,"totalBytes"),(4,"offset"),(5,"chunkBytes"),(6,"final")]{let mut field=semio_framework_dsl_record::producer::field(id,key,Shape::UInt,control)?;field.optional=id!=1;fields.push(field);control.step()?;}semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Inline,fields,control)})}
fn payload_update_spec_producer()->semio_framework_dsl_record::RecordSpecProducer{semio_framework_dsl_record::RecordSpecProducer{ordinary:payload_update_spec_owned,decoding:|control|payload_update_spec_controlled(control),encoding:|control|payload_update_spec_controlled(control)}}
pub(crate) fn payload_update_record(update:ToolRunPayloadUpdate)->RecordValue{let mut record=RecordValue::default();for id in 1..=update.wire_field_count(){record.fields.insert(id as u16,FieldValue::UInt(update.wire_field(id).expect("closed payload update field")));}record}
pub(crate) fn payload_update_from_record(record:&RecordValue)->Result<ToolRunPayloadUpdate,ToolRunCodecError>{let count=record.fields.values().filter(|value|!matches!(value,FieldValue::Absent)).count();match uint(record,1)?{0 if count==1=>Ok(ToolRunPayloadUpdate::Replace),1 if count==6=>{let final_chunk=match uint(record,6)?{0=>false,1=>true,_=>return Err(ToolRunCodecError::Malformed("payload finality"))};Ok(ToolRunPayloadUpdate::Range{publication:uint(record,2)?,total_bytes:uint(record,3)?,offset:uint(record,4)?,chunk_bytes:narrow(uint(record,5)?,"payload stride")?,final_chunk})},_=>Err(ToolRunCodecError::Malformed("closed payload update"))}}
pub fn validate_tick_payload(payload:Option<&[u8]>,update:Option<ToolRunPayloadUpdate>)->Result<(),ToolRunCodecError>{if payload.is_some()!=update.is_some(){return Err(ToolRunCodecError::Malformed("payload and update must be paired"));}if let(Some(payload),Some(update))=(payload,update){update.validate(payload.len()).map_err(|_|ToolRunCodecError::Malformed("payload update range"))?;}Ok(())}

pub(crate) fn page_spec() -> &'static RecordSpec {
    static SPEC: OnceLock<RecordSpec> = OnceLock::new();
    SPEC.get_or_init(|| {
        let mut fields = vec![field(1, "identity", Shape::Record(identity_spec_producer())), field(2, "page", Shape::UInt)];
        for (id, key) in [(3, "opKinds"), (4, "keys"), (5, "verdicts"), (6, "reasons"), (7, "subjectKinds"), (8, "meshes"), (9, "shapes"), (10, "entities"), (11, "floats")] {
            fields.push(field(id, key, Shape::Bytes64).optional());
        }
        spec(fields)
    })
}

pub(crate) fn delta_spec() -> &'static RecordSpec {
    static SPEC: OnceLock<RecordSpec> = OnceLock::new();
    SPEC.get_or_init(|| spec(vec![field(1, "identity", Shape::Record(identity_spec_producer())), field(2, "clear", Shape::Bool), field(3, "next", Shape::UInt), field(4, "pages", Shape::List(Box::new(Shape::Bytes64))).optional()]))
}

pub(crate) fn tick_spec() -> &'static RecordSpec {
    static SPEC: OnceLock<RecordSpec> = OnceLock::new();
    SPEC.get_or_init(|| {
        spec(vec![
            field(1, "identity", Shape::Record(identity_spec_producer())),
            field(2, "sequence", Shape::UInt),
            field(3, "progress", Shape::Record(progress_spec_producer())).optional(),
            field(4, "steps", Shape::List(Box::new(Shape::Record(step_spec_producer())))).optional(),
            field(5, "trace", Shape::List(Box::new(Shape::Bytes64))).optional(),
            field(6, "appendOps", Shape::List(Box::new(Shape::Bytes64))).optional(),
            field(7, "appendEntities", Shape::Bytes64).optional(),
            field(8, "retractTo", Shape::UInt).optional(),
            field(9, "payload", Shape::Bytes64).optional(),
            field(10,"payloadUpdate",Shape::Record(payload_update_spec_producer())).optional(),
        ])
    })
}

pub(crate) fn encode_body(spec: &RecordSpec, record: &RecordValue) -> Result<Vec<u8>, ToolRunCodecError> {
    encode_record_body(spec, record, &EncodeOptions::default()).map_err(|error| ToolRunCodecError::Pack(error.to_string()))
}

pub(crate) fn decode_body(spec: &RecordSpec, bytes: &[u8]) -> Result<RecordValue, ToolRunCodecError> {
    decode_record_body_exact(bytes, spec, &DecodeOptions::default()).map_err(|error| ToolRunCodecError::Pack(error.to_string()))
}

pub(crate) fn identity_record(identity: &ToolRunIdentity) -> RecordValue {
    let mut record = RecordValue::default();
    record.fields.insert(1, FieldValue::UInt(u64::from(identity.id.app_instance_id)));
    record.fields.insert(2, FieldValue::UInt(identity.id.run));
    record.fields.insert(3, FieldValue::UInt(u64::from(identity.generation)));
    record.fields.insert(4, FieldValue::Bytes64(identity.base_revision.to_vec()));
    record
}

fn identity_from_record(record: &RecordValue) -> Result<ToolRunIdentity, ToolRunCodecError> {
    let base_revision = bytes_field(record, 4)?.try_into().map_err(|_| ToolRunCodecError::Malformed("baseRevision"))?;
    Ok(ToolRunIdentity { id: ToolRunId { app_instance_id: narrow(uint(record, 1)?, "appInstanceId")?, run: uint(record, 2)? }, generation: narrow(uint(record, 3)?, "generation")?, base_revision })
}

pub(crate) fn identity_from_field(record: &RecordValue, id: u16) -> Result<ToolRunIdentity, ToolRunCodecError> {
    match record.get(id) {
        Some(FieldValue::Record(identity)) => identity_from_record(identity),
        _ => Err(ToolRunCodecError::Malformed("identity")),
    }
}

pub(crate) fn step_record(step: &ToolRunStep) -> Result<RecordValue, ToolRunCodecError> {
    if step.args.len() > TOOL_RUN_STEP_ARGS_MAX {
        return Err(ToolRunCodecError::Limit("step args"));
    }
    if step.repeat == 0 {
        return Err(ToolRunCodecError::Malformed("repeat"));
    }
    let mut record = RecordValue::default();
    record.fields.insert(1, FieldValue::UInt(step.sequence));
    record.fields.insert(2, FieldValue::UInt(u64::from(step.kind.ordinal())));
    record.fields.insert(3, FieldValue::UInt(u64::from(step.stage)));
    record.fields.insert(4, FieldValue::UInt(u64::from(step.reason)));
    if let Some(subject) = step.subject {
        record.fields.insert(5, FieldValue::UInt(subject));
    }
    record.fields.insert(6, FieldValue::UInt(u64::from(step.repeat)));
    if !step.args.is_empty() {
        let mut args = Vec::with_capacity(9 * step.args.len());
        for arg in &step.args {
            match *arg {
                ToolRunStepArg::Unsigned(value) => {
                    args.push(0);
                    args.extend_from_slice(&value.to_le_bytes());
                }
                ToolRunStepArg::Float(value) => {
                    args.push(1);
                    args.extend_from_slice(&value.to_le_bytes());
                }
            }
        }
        record.fields.insert(7, FieldValue::Bytes64(args));
    }
    Ok(record)
}

pub(crate) fn step_from_record(record: &RecordValue) -> Result<ToolRunStep, ToolRunCodecError> {
    let raw_args = bytes_field(record, 7)?;
    let (args, rest) = raw_args.as_chunks::<9>();
    if !rest.is_empty() || args.len() > TOOL_RUN_STEP_ARGS_MAX {
        return Err(ToolRunCodecError::Malformed("step args"));
    }
    let args = args
        .iter()
        .map(|[kind, value @ ..]| match kind {
            0 => Ok(ToolRunStepArg::Unsigned(u64::from_le_bytes(*value))),
            1 => Ok(ToolRunStepArg::Float(f64::from_le_bytes(*value))),
            _ => Err(ToolRunCodecError::Malformed("step arg kind")),
        })
        .collect::<Result<_, _>>()?;
    let repeat = narrow(uint(record, 6)?, "repeat")?;
    if repeat == 0 {
        return Err(ToolRunCodecError::Malformed("repeat"));
    }
    Ok(ToolRunStep {
        sequence: uint(record, 1)?,
        kind: ToolRunStepKind::from_ordinal(uint(record, 2)?).ok_or(ToolRunCodecError::Malformed("step kind"))?,
        stage: narrow(uint(record, 3)?, "stage")?,
        reason: narrow(uint(record, 4)?, "reason")?,
        subject: optional_uint(record, 5)?,
        repeat,
        args,
    })
}

pub(crate) fn progress_record(progress: &ToolRunProgress) -> Result<RecordValue, ToolRunCodecError> {
    if progress.counters.len() > TOOL_RUN_COUNTERS_MAX {
        return Err(ToolRunCodecError::Limit("counters"));
    }
    let mut record = RecordValue::default();
    record.fields.insert(1, FieldValue::Record(identity_record(&progress.identity)));
    record.fields.insert(2, FieldValue::UInt(progress.sequence));
    record.fields.insert(3, FieldValue::UInt(u64::from(progress.state.ordinal())));
    record.fields.insert(4, FieldValue::UInt(u64::from(progress.stage)));
    record.fields.insert(5, FieldValue::UInt(progress.completed));
    if let Some(total) = progress.total {
        record.fields.insert(6, FieldValue::UInt(total));
    }
    if !progress.counters.is_empty() {
        record.fields.insert(7, FieldValue::Bytes64(progress.counters.iter().flat_map(|counter| counter.counter.to_le_bytes().into_iter().chain(counter.value.to_le_bytes())).collect()));
    }
    record.fields.insert(8, FieldValue::Float(f64::from(progress.units_per_second)));
    record.fields.insert(9, FieldValue::UInt(u64::from(progress.conflicts)));
    if !progress.steps.is_empty() {
        record.fields.insert(10, FieldValue::List(progress.steps.iter().map(|step| step_record(step).map(FieldValue::Record)).collect::<Result<_, _>>()?));
    }
    Ok(record)
}

pub(crate) fn progress_from_record(record: &RecordValue) -> Result<ToolRunProgress, ToolRunCodecError> {
    let raw_counters = bytes_field(record, 7)?;
    let (counters, rest) = raw_counters.as_chunks::<10>();
    if !rest.is_empty() || counters.len() > TOOL_RUN_COUNTERS_MAX {
        return Err(ToolRunCodecError::Malformed("counters"));
    }
    let counters = counters.iter().map(|[c0, c1, value @ ..]| ToolRunCounter { counter: u16::from_le_bytes([*c0, *c1]), value: u64::from_le_bytes(*value) }).collect();
    let steps = list(record, 10)?.iter().map(|item| step_from_record(item_record(item)?)).collect::<Result<Vec<_>, _>>()?;
    let units_per_second = match record.get(8) {
        Some(FieldValue::Float(value)) => *value as f32,
        _ => return Err(ToolRunCodecError::Malformed("unitsPerSecond")),
    };
    Ok(ToolRunProgress {
        identity: identity_from_field(record, 1)?,
        sequence: uint(record, 2)?,
        state: ToolRunState::from_ordinal(uint(record, 3)?).ok_or(ToolRunCodecError::Malformed("state"))?,
        stage: narrow(uint(record, 4)?, "stage")?,
        completed: uint(record, 5)?,
        total: optional_uint(record, 6)?,
        counters,
        units_per_second,
        conflicts: narrow(uint(record, 9)?, "conflicts")?,
        steps: ToolRunStepRing::from_steps(steps)?,
    })
}

pub(crate) fn page_record(page: &ToolRunTracePage) -> Result<RecordValue, ToolRunCodecError> {
    if page.ops.len() > TOOL_RUN_TRACE_PAGE_OPS_MAX {
        return Err(ToolRunCodecError::Limit("trace page ops"));
    }
    let mut columns: [Vec<u8>; 9] = Default::default();
    let [kinds, keys, verdicts, reasons, subject_kinds, meshes, shapes, entities, floats] = &mut columns;
    for op in &page.ops {
        match *op {
            ToolRunTraceOp::Upsert { key, verdict, reason, subject } => {
                kinds.push(0);
                keys.extend_from_slice(&key.to_le_bytes());
                verdicts.push(verdict.ordinal());
                reasons.extend_from_slice(&reason.to_le_bytes());
                subject_kinds.push(subject.ordinal());
                match subject {
                    ToolRunTraceSubject::Instance3d { mesh, position, rotation, scale } => {
                        meshes.extend_from_slice(&mesh.to_le_bytes());
                        for value in position.into_iter().chain(rotation).chain([scale]) {
                            floats.extend_from_slice(&value.to_le_bytes());
                        }
                    }
                    ToolRunTraceSubject::Placement2d { shape, position, rotation } => {
                        shapes.extend_from_slice(&shape.to_le_bytes());
                        for value in position.into_iter().chain([rotation]) {
                            floats.extend_from_slice(&value.to_le_bytes());
                        }
                    }
                    ToolRunTraceSubject::Entity { entity } => entities.extend_from_slice(&entity.to_le_bytes()),
                }
            }
            ToolRunTraceOp::Retire { key } => {
                kinds.push(1);
                keys.extend_from_slice(&key.to_le_bytes());
            }
            ToolRunTraceOp::Clear => kinds.push(2),
        }
    }
    let mut record = RecordValue::default();
    record.fields.insert(1, FieldValue::Record(identity_record(&page.identity)));
    record.fields.insert(2, FieldValue::UInt(u64::from(page.page)));
    for (id, column) in (3u16..).zip(columns) {
        if !column.is_empty() {
            record.fields.insert(id, FieldValue::Bytes64(column));
        }
    }
    Ok(record)
}

pub(crate) fn page_from_record(record: &RecordValue) -> Result<ToolRunTracePage, ToolRunCodecError> {
    let kinds = bytes_field(record, 3)?;
    if kinds.len() > TOOL_RUN_TRACE_PAGE_OPS_MAX {
        return Err(ToolRunCodecError::Limit("trace page ops"));
    }
    let keys = u64_column(bytes_field(record, 4)?, "keys")?;
    let verdicts = bytes_field(record, 5)?;
    let reasons = bytes_field(record, 6)?;
    let subject_kinds = bytes_field(record, 7)?;
    let meshes = u32_column(bytes_field(record, 8)?, "meshes")?;
    let shapes = u32_column(bytes_field(record, 9)?, "shapes")?;
    let entities = u64_column(bytes_field(record, 10)?, "entities")?;
    let floats = f32_column(bytes_field(record, 11)?)?;
    let upserts = kinds.iter().filter(|kind| **kind == 0).count();
    let keyed = kinds.iter().filter(|kind| **kind < 2).count();
    let instances = subject_kinds.iter().filter(|kind| **kind == 0).count();
    let placements = subject_kinds.iter().filter(|kind| **kind == 1).count();
    let entity_subjects = subject_kinds.iter().filter(|kind| **kind == 2).count();
    if kinds.iter().any(|kind| *kind > 2) || keys.len() != keyed || verdicts.len() != upserts || reasons.len() != 2 * upserts || subject_kinds.len() != upserts || instances + placements + entity_subjects != upserts || meshes.len() != instances || shapes.len() != placements || entities.len() != entity_subjects || floats.len() != 8 * instances + 3 * placements {
        return Err(ToolRunCodecError::Malformed("trace page columns"));
    }
    let (mut key_at, mut upsert_at, mut mesh_at, mut shape_at, mut entity_at, mut float_at) = (0, 0, 0, 0, 0, 0);
    let mut ops = Vec::with_capacity(kinds.len());
    for kind in kinds {
        match kind {
            0 => {
                let subject = match subject_kinds[upsert_at] {
                    0 => {
                        let f = &floats[float_at..float_at + 8];
                        float_at += 8;
                        mesh_at += 1;
                        ToolRunTraceSubject::Instance3d { mesh: meshes[mesh_at - 1], position: [f[0], f[1], f[2]], rotation: [f[3], f[4], f[5], f[6]], scale: f[7] }
                    }
                    1 => {
                        let f = &floats[float_at..float_at + 3];
                        float_at += 3;
                        shape_at += 1;
                        ToolRunTraceSubject::Placement2d { shape: shapes[shape_at - 1], position: [f[0], f[1]], rotation: f[2] }
                    }
                    _ => {
                        entity_at += 1;
                        ToolRunTraceSubject::Entity { entity: entities[entity_at - 1] }
                    }
                };
                let verdict = ToolRunVerdict::from_ordinal(verdicts[upsert_at]).ok_or(ToolRunCodecError::Malformed("verdict"))?;
                let reason = u16::from_le_bytes([reasons[2 * upsert_at], reasons[2 * upsert_at + 1]]);
                ops.push(ToolRunTraceOp::Upsert { key: keys[key_at], verdict, reason, subject });
                upsert_at += 1;
                key_at += 1;
            }
            1 => {
                ops.push(ToolRunTraceOp::Retire { key: keys[key_at] });
                key_at += 1;
            }
            _ => ops.push(ToolRunTraceOp::Clear),
        }
    }
    Ok(ToolRunTracePage { identity: identity_from_field(record, 1)?, page: narrow(uint(record, 2)?, "page")?, ops })
}

pub(crate) fn uint(record: &RecordValue, id: u16) -> Result<u64, ToolRunCodecError> {
    optional_uint(record, id)?.ok_or(ToolRunCodecError::Malformed("required unsigned field"))
}

pub(crate) fn optional_uint(record: &RecordValue, id: u16) -> Result<Option<u64>, ToolRunCodecError> {
    match record.get(id) {
        None | Some(FieldValue::Absent) => Ok(None),
        Some(FieldValue::UInt(value)) => Ok(Some(*value)),
        Some(_) => Err(ToolRunCodecError::Malformed("unsigned field")),
    }
}

pub(crate) fn boolean(record: &RecordValue, id: u16) -> Result<bool, ToolRunCodecError> {
    match record.get(id) {
        Some(FieldValue::Bool(value)) => Ok(*value),
        _ => Err(ToolRunCodecError::Malformed("boolean field")),
    }
}

pub(crate) fn bytes_field(record: &RecordValue, id: u16) -> Result<&[u8], ToolRunCodecError> {
    match record.get(id) {
        None | Some(FieldValue::Absent) => Ok(&[]),
        Some(FieldValue::Bytes64(bytes)) => Ok(bytes),
        Some(_) => Err(ToolRunCodecError::Malformed("bytes field")),
    }
}

pub(crate) fn list(record: &RecordValue, id: u16) -> Result<&[FieldValue], ToolRunCodecError> {
    match record.get(id) {
        None | Some(FieldValue::Absent) => Ok(&[]),
        Some(FieldValue::List(items)) => Ok(items),
        Some(_) => Err(ToolRunCodecError::Malformed("list field")),
    }
}

pub(crate) fn item_bytes(item: &FieldValue) -> Result<&[u8], ToolRunCodecError> {
    match item {
        FieldValue::Bytes64(bytes) => Ok(bytes),
        _ => Err(ToolRunCodecError::Malformed("bytes item")),
    }
}

pub(crate) fn item_record(item: &FieldValue) -> Result<&RecordValue, ToolRunCodecError> {
    match item {
        FieldValue::Record(record) => Ok(record),
        _ => Err(ToolRunCodecError::Malformed("record item")),
    }
}

pub(crate) fn narrow<T: TryFrom<u64>>(value: u64, what: &'static str) -> Result<T, ToolRunCodecError> {
    T::try_from(value).map_err(|_| ToolRunCodecError::Malformed(what))
}

pub(crate) fn u64_column(bytes: &[u8], what: &'static str) -> Result<Vec<u64>, ToolRunCodecError> {
    let (chunks, rest) = bytes.as_chunks::<8>();
    if !rest.is_empty() {
        return Err(ToolRunCodecError::Malformed(what));
    }
    Ok(chunks.iter().map(|chunk| u64::from_le_bytes(*chunk)).collect())
}

fn u32_column(bytes: &[u8], what: &'static str) -> Result<Vec<u32>, ToolRunCodecError> {
    let (chunks, rest) = bytes.as_chunks::<4>();
    if !rest.is_empty() {
        return Err(ToolRunCodecError::Malformed(what));
    }
    Ok(chunks.iter().map(|chunk| u32::from_le_bytes(*chunk)).collect())
}

fn f32_column(bytes: &[u8]) -> Result<Vec<f32>, ToolRunCodecError> {
    let (chunks, rest) = bytes.as_chunks::<4>();
    if !rest.is_empty() {
        return Err(ToolRunCodecError::Malformed("floats"));
    }
    Ok(chunks.iter().map(|chunk| f32::from_le_bytes(*chunk)).collect())
}
//#endregion 🔖️Codec
