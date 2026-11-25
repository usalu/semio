//! 🔁️ Mounted XLSX natural-file import over the original admitted source and step budgets: the same workbook the whole-buffer decoder reads, produced in turns of at most `max_work_units_per_step` units that honour cancellation and report their progress.

use semio_framework_deflate::{DeflateError, InflateOutcome, Inflater};
use semio_framework_value::{ByteReadSource, ErasedSnapshotRetirement, RetainedAllocationEffect, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind};
use semio_plugin::{MediaError, NaturalFileDecodeCursor, NaturalFileDecodeOwner, NaturalFileDecodeProgress, NaturalFileDecodeStep, NaturalFilePagedSource, PluginLifecycleStep, StepContext, ToolExecutionContract, NATURAL_FILE_PORT};
use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::refusal::XlsxError;
use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::{xlsx_part_is_xml, XlsxSnapshot, XlsxXmlPart};
use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::{XmlCursorStep, XmlDocumentCursor};
use semio_s_artifact_stdio_zip::opc::{check_opc_member_names, opc_content_types_from_document, opc_part_content_type, opc_relationships_from_document, OpcError, OpcPackage, OpcPart, CONTENT_TYPES_PART};
use semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::{close_zip_directory, open_zip_directory, plan_zip_entry, validate_zip_comment, validate_zip_entry_headers, Crc32, ZipDirectory, ZipEntryPlan, ZipError};
use std::mem::size_of;
use store::os_store::snapshot_read::{artifact_retirement_box_close_step, artifact_retirement_box_demands, settle_close_poll};

/// 🚦️ The subset-specific acceptance a finished workbook still has to pass before it is published.
pub type XlsxConformanceGate = fn(&XlsxSnapshot) -> Result<(), XlsxError>;

/// 📏️ Bounds the independently admitted ordinary XLSX source extent.
pub const XLSX_PAGED_NATURAL_SOURCE_MAXIMUM_BYTES: u32 = 16 * 1024 * 1024;

/// 🔓️ The acceptance of the base subset: every structurally valid workbook.
pub fn accept_every_workbook(_snapshot: &XlsxSnapshot) -> Result<(), XlsxError> {
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase { Gather, OpenDirectory, Scan, Begin, Inflate, Seal, Parse, Assemble, Done, Closing }

enum Target {
    ContentTypes,
    Relationships(String),
    Part { path: String, content_type: String },
}

/// 📥️ Selects the mounted decoder before moving its original source.
pub fn admit_xlsx_natural_decode(
    bytes: &mut Option<Vec<u8>>, contract: ToolExecutionContract, gate: XlsxConformanceGate,
    receiving: &mut Option<NaturalFileDecodeOwner<XlsxSnapshot>>, context: &mut StepContext<'_>,
) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
    if receiving.is_some() { return Ok(true); }
    let source = bytes.as_ref().ok_or_else(|| ValueError::invariant("missing original XLSX natural file source"))?;
    let controlled = source.len() > contract.max_work_units_per_step as usize;
    let capacity = if controlled { size_of::<NaturalXlsxDecoder>() } else { 0 };
    let copy = capacity + size_of::<NaturalFileDecodeOwner<XlsxSnapshot>>();
    let grant = context.retained_grant();
    if context.should_yield() || context.is_cancelled() || grant.maximum_items == 0 || grant.maximum_depth == 0
        || copy > grant.maximum_copy_bytes || capacity > grant.maximum_capacity_bytes
    {
        return Ok(false);
    }
    let original = bytes.take().expect("original XLSX source is present");
    *receiving = Some(if controlled {
        NaturalFileDecodeOwner::Controlled(Box::new(NaturalXlsxDecoder::new(contract, gate, Some(original), None)))
    } else {
        NaturalFileDecodeOwner::Synchronous(original)
    });
    context.consume_fuel(1);
    context.consume_retained(RetainedCloneProgress {
        copied_items: 1, copied_bytes: copy, retained_capacity_bytes: capacity, ..Default::default()
    })?;
    Ok(true)
}

/// 🏗️ Creates the same native owner for explicit cold tool construction.
pub fn build_xlsx_natural_decode(bytes: Vec<u8>, contract: ToolExecutionContract, gate: XlsxConformanceGate) -> Result<NaturalFileDecodeOwner<XlsxSnapshot>, MediaError> {
    if bytes.len() <= contract.max_work_units_per_step as usize { return Ok(NaturalFileDecodeOwner::Synchronous(bytes)); }
    Ok(NaturalFileDecodeOwner::Controlled(Box::new(NaturalXlsxDecoder::new(contract, gate, Some(bytes), None))))
}

/// 📄️ Donates the exact sealed page owner to a paid XLSX decoder frame.
pub fn admit_xlsx_paged_natural_decode(
    source: &mut Option<NaturalFilePagedSource>, contract: ToolExecutionContract, gate: XlsxConformanceGate,
    receiving: &mut Option<NaturalFileDecodeOwner<XlsxSnapshot>>, context: &mut StepContext<'_>,
) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
    if receiving.is_some() { return Ok(true); }
    let original = source.as_ref().filter(|owner| owner.original_is_untouched()).and_then(|owner| owner.original())
        .ok_or_else(|| ValueError::invariant("missing sealed original XLSX pages"))?;
    if original.byte_len() == 0 || original.byte_len() > XLSX_PAGED_NATURAL_SOURCE_MAXIMUM_BYTES as usize {
        return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "original XLSX extent exceeds its declared source ceiling").into());
    }
    let capacity = size_of::<NaturalXlsxDecoder>();
    let copy = capacity + size_of::<NaturalFileDecodeOwner<XlsxSnapshot>>();
    let grant = context.retained_grant();
    if context.should_yield() || context.is_cancelled() || grant.maximum_items == 0 || grant.maximum_depth == 0
        || copy > grant.maximum_copy_bytes || capacity > grant.maximum_capacity_bytes
    {
        return Ok(false);
    }
    let paged = source.take().expect("sealed original XLSX pages");
    *receiving = Some(NaturalFileDecodeOwner::Controlled(Box::new(NaturalXlsxDecoder::new(contract, gate, None, Some(paged)))));
    context.consume_fuel(1);
    context.consume_retained(RetainedCloneProgress {
        copied_items: 1, copied_bytes: copy, retained_capacity_bytes: capacity, ..Default::default()
    })?;
    Ok(true)
}

struct NaturalXlsxDecoder {
    phase: Phase,
    gate: XlsxConformanceGate,
    contract: ToolExecutionContract,
    paged: Option<NaturalFilePagedSource>,
    archive: Option<Vec<u8>>,
    total: usize,
    directory: Option<ZipDirectory>,
    position: usize,
    members: Vec<Option<ZipEntryPlan>>,
    content_types: usize,
    ordinal: usize,
    current: Option<ZipEntryPlan>,
    inflater: Option<Box<Inflater>>,
    pending: Option<u8>,
    input: usize,
    crc: Crc32,
    output: Option<Vec<u8>>,
    target: Option<Target>,
    parsing: Option<Box<XmlDocumentCursor>>,
    parsed: usize,
    snapshot: Option<XlsxSnapshot>,
    partial: Option<XmlNode>,
    text: Option<Vec<u8>>,
    retained_bytes: usize,
    progress: NaturalFileDecodeProgress,
    retirement: Option<Box<dyn ErasedSnapshotRetirement>>,
}

impl NaturalXlsxDecoder {
    fn new(contract: ToolExecutionContract, gate: XlsxConformanceGate, contiguous: Option<Vec<u8>>, paged: Option<NaturalFilePagedSource>) -> Self {
        let total = contiguous.as_ref().map_or_else(|| paged.as_ref().and_then(|owner| owner.original()).map_or(0, |bytes| bytes.byte_len()), Vec::len);
        let phase = if contiguous.is_some() { Phase::OpenDirectory } else { Phase::Gather };
        Self {
            phase, gate, contract, paged, archive: contiguous, total, directory: None, position: 0, members: Vec::new(), content_types: 0, ordinal: 0,
            current: None, inflater: None, pending: None, input: 0, crc: Crc32::new(), output: None, target: None, parsing: None, parsed: 0,
            snapshot: Some(XlsxSnapshot::from_parts(OpcPackage::default(), Vec::new())), partial: None, text: None, retained_bytes: 0,
            progress: Default::default(), retirement: None,
        }
    }

    fn archive(&self) -> Result<&[u8], MediaError> {
        self.archive.as_deref().ok_or_else(|| failure("XLSX archive is no longer owned"))
    }

    fn member_index(&self) -> usize {
        match self.ordinal {
            0 => self.content_types,
            ordinal if ordinal - 1 < self.content_types => ordinal - 1,
            ordinal => ordinal,
        }
    }

    fn owned(&self) -> usize {
        self.archive.as_ref().map_or(0, Vec::len).saturating_add(self.retained_bytes)
    }

    fn next(&mut self, allowance: u64) -> Result<(u64, Option<XlsxSnapshot>), MediaError> {
        match self.phase {
            Phase::Gather => self.gather(allowance).map(|units| (units, None)),
            Phase::OpenDirectory => {
                let directory = open_zip_directory(self.archive()?).map_err(zip_failure)?;
                validate_zip_comment(&directory.comment, directory.comment_utf8).map_err(zip_failure)?;
                if directory.count > self.contract.max_decoded_items {
                    return Err(failure(&format!("workbook declares {} members; maximum is {}", directory.count, self.contract.max_decoded_items)));
                }
                self.members.try_reserve_exact(directory.count).map_err(|_| failure("unable to allocate the XLSX member table"))?;
                self.position = directory.offset;
                self.directory = Some(directory);
                self.progress.consumed_input_bytes = self.total;
                self.phase = Phase::Scan;
                Ok((1, None))
            }
            Phase::Scan => self.scan(allowance).map(|units| (units, None)),
            Phase::Begin => self.begin().map(|units| (units, None)),
            Phase::Inflate => self.inflate(allowance).map(|units| (units, None)),
            Phase::Seal => self.seal(allowance).map(|units| (units, None)),
            Phase::Parse => self.parse(allowance).map(|units| (units, None)),
            Phase::Assemble => self.assemble().map(|snapshot| (1, Some(snapshot))),
            Phase::Done | Phase::Closing => Err(failure("XLSX natural file decoder no longer advanceable")),
        }
    }

    fn gather(&mut self, allowance: u64) -> Result<u64, MediaError> {
        let paged = self.paged.as_ref().ok_or_else(|| failure("XLSX source pages were retired before gathering"))?;
        let original = paged.original().ok_or_else(|| failure("XLSX source pages were retired before gathering"))?;
        let archive = self.archive.get_or_insert_with(Vec::new);
        if archive.capacity() == 0 {
            archive.try_reserve_exact(self.total).map_err(|_| failure("unable to allocate the gathered XLSX archive"))?;
        }
        let start = archive.len();
        let count = usize::try_from(allowance).unwrap_or(usize::MAX).min(self.total - start);
        archive.resize(start + count, 0);
        let read = original.read_bytes(start, &mut archive[start..]);
        archive.truncate(start + read);
        if read == 0 { return Err(failure("XLSX source pages ended before their declared extent")); }
        self.progress.consumed_input_bytes = archive.len();
        if archive.len() == self.total { self.phase = Phase::OpenDirectory; }
        Ok(read as u64)
    }

    fn scan(&mut self, allowance: u64) -> Result<u64, MediaError> {
        let directory = self.directory.clone().ok_or_else(|| failure("XLSX directory is missing"))?;
        let mut units = 0;
        while units < allowance && self.members.len() < directory.count {
            let (mut plan, next) = plan_zip_entry(self.archive()?, self.position).map_err(zip_failure)?;
            validate_zip_entry_headers(&plan.name, &plan.metadata).map_err(zip_failure)?;
            plan.metadata = Default::default();
            self.position = next;
            self.members.push(Some(plan));
            units += 1;
        }
        if self.members.len() < directory.count { return Ok(units); }
        close_zip_directory(&directory, self.position).map_err(zip_failure)?;
        let names = self.members.iter().filter_map(|plan| plan.as_ref().map(|plan| plan.name.as_str()));
        check_opc_member_names(names).map_err(opc_failure)?;
        self.content_types = self.members.iter().position(|plan| plan.as_ref().is_some_and(|plan| plan.name == CONTENT_TYPES_PART)).ok_or_else(|| opc_failure(OpcError::MissingContentTypes))?;
        if let Some(snapshot) = self.snapshot.as_mut() { snapshot.opc.comment = directory.comment; }
        self.phase = Phase::Begin;
        Ok(units.max(1))
    }

    fn begin(&mut self) -> Result<u64, MediaError> {
        let index = self.member_index();
        let plan = self.members[index].take().ok_or_else(|| failure("XLSX member was planned twice"))?;
        if plan.uncompressed_size > self.contract.max_output_bytes {
            return Err(failure(&format!("{} expands to {} bytes; maximum is {}", plan.name, plan.uncompressed_size, self.contract.max_output_bytes)));
        }
        let mut output = Vec::new();
        output.try_reserve_exact(plan.uncompressed_size).map_err(|_| failure("unable to allocate the XLSX member output"))?;
        self.retained_bytes = self.retained_bytes.saturating_add(output.capacity());
        self.output = Some(output);
        self.inflater = plan.deflated.then(|| Box::new(Inflater::new()));
        self.crc = Crc32::new();
        self.pending = None;
        self.input = plan.payload.start;
        self.current = Some(plan);
        self.phase = Phase::Inflate;
        Ok(1)
    }

    fn inflate(&mut self, allowance: u64) -> Result<u64, MediaError> {
        let plan = self.current.as_ref().ok_or_else(|| failure("XLSX member is missing"))?;
        let (end, size) = (plan.payload.end, plan.uncompressed_size);
        let archive = self.archive.as_deref().ok_or_else(|| failure("XLSX archive is no longer owned"))?;
        let output = self.output.as_mut().ok_or_else(|| failure("XLSX member output is missing"))?;
        let Some(inflater) = self.inflater.as_mut() else {
            let count = usize::try_from(allowance).unwrap_or(usize::MAX).min(end - self.input);
            let chunk = &archive[self.input..self.input + count];
            output.extend_from_slice(chunk);
            self.crc.update(chunk);
            self.input += count;
            if self.input == end { self.phase = Phase::Seal; }
            return Ok(count.max(1) as u64);
        };
        let mut units = 0;
        while units < allowance {
            if self.pending.is_none() && self.input < end {
                self.pending = Some(archive[self.input]);
                self.input += 1;
            }
            let outcome = inflater.advance(&mut self.pending, self.input == end).map_err(inflate_failure)?;
            units += 1;
            match outcome {
                InflateOutcome::NeedInput => {}
                InflateOutcome::Wrote(byte) => {
                    if output.len() == size { return Err(failure(&format!("{} expands past its declared {size} bytes", plan.name))); }
                    output.push(byte);
                    self.crc.update(&[byte]);
                }
                InflateOutcome::Done => {
                    self.phase = Phase::Seal;
                    break;
                }
            }
        }
        Ok(units)
    }

    fn seal(&mut self, allowance: u64) -> Result<u64, MediaError> {
        let plan = self.current.take().ok_or_else(|| failure("XLSX member is missing"))?;
        let raw = self.output.take().ok_or_else(|| failure("XLSX member output is missing"))?;
        self.inflater = None;
        self.retained_bytes = self.retained_bytes.saturating_sub(raw.capacity());
        let entry = plan.finish(raw, self.crc.value()).map_err(zip_failure)?;
        let (name, data) = (entry.name, entry.data);
        let snapshot = self.snapshot.as_mut().ok_or_else(|| failure("XLSX snapshot is missing"))?;
        let target = if name == CONTENT_TYPES_PART {
            Target::ContentTypes
        } else if name.ends_with(".rels") {
            Target::Relationships(name)
        } else {
            let content_type = opc_part_content_type(&snapshot.opc.content_types, &name).map_err(opc_failure)?;
            if !xlsx_part_is_xml(&name, &content_type) {
                self.retained_bytes = self.retained_bytes.saturating_add(data.capacity());
                snapshot.opc.parts.push(OpcPart { path: name, content_type, bytes: data });
                self.finish_member();
                return Ok(1);
            }
            Target::Part { path: name, content_type }
        };
        let length = data.len();
        let text = String::from_utf8(data).map_err(|_| match &target {
            Target::ContentTypes => opc_failure(OpcError::MalformedContentTypes("not valid utf-8".into())),
            Target::Relationships(part) => opc_failure(OpcError::MalformedRelationships { part: part.clone(), detail: "not valid utf-8".into() }),
            Target::Part { path, .. } => xlsx_failure(XlsxError::Xml { part: path.clone(), detail: "not valid utf-8".into() }),
        })?;
        self.retained_bytes = self.retained_bytes.saturating_add(length);
        self.parsing = Some(Box::new(XmlDocumentCursor::new(text)));
        self.parsed = 0;
        self.target = Some(target);
        self.phase = Phase::Parse;
        Ok((length as u64).div_ceil(64).clamp(1, allowance))
    }

    fn parse(&mut self, allowance: u64) -> Result<u64, MediaError> {
        let cursor = self.parsing.as_mut().ok_or_else(|| failure("XLSX XML cursor is missing"))?;
        let step = cursor.advance(usize::try_from(allowance).unwrap_or(usize::MAX));
        let units = (cursor.consumed().saturating_sub(self.parsed) as u64).clamp(1, allowance);
        self.parsed = cursor.consumed();
        let target = self.target.as_ref().ok_or_else(|| failure("XLSX XML target is missing"))?;
        let document = match step {
            Ok(XmlCursorStep::More) => return Ok(units),
            Ok(XmlCursorStep::Complete(document)) => document,
            Err(detail) => return Err(match target {
                Target::ContentTypes => opc_failure(OpcError::Xml { part: CONTENT_TYPES_PART.into(), detail }),
                Target::Relationships(part) => opc_failure(OpcError::Xml { part: part.clone(), detail }),
                Target::Part { path, .. } => xlsx_failure(XlsxError::Xml { part: path.clone(), detail }),
            }),
        };
        let snapshot = self.snapshot.as_mut().ok_or_else(|| failure("XLSX snapshot is missing"))?;
        match self.target.take().expect("XLSX XML target checked above") {
            Target::ContentTypes => snapshot.opc.content_types = opc_content_types_from_document(&document).map_err(opc_failure)?,
            Target::Relationships(part) => {
                let (owner, relationships) = opc_relationships_from_document(&document, &part).map_err(opc_failure)?;
                snapshot.opc.relationships.replace_owner(owner, relationships);
            }
            Target::Part { path, content_type } => snapshot.xml_parts.push(XlsxXmlPart { path, content_type, document }),
        }
        self.parsing = None;
        self.finish_member();
        Ok(units)
    }

    fn finish_member(&mut self) {
        self.ordinal += 1;
        self.progress.decoded_items = self.ordinal;
        self.phase = if self.ordinal == self.members.len() { Phase::Assemble } else { Phase::Begin };
    }

    fn assemble(&mut self) -> Result<XlsxSnapshot, MediaError> {
        let snapshot = self.snapshot.as_ref().ok_or_else(|| failure("XLSX snapshot is missing"))?;
        snapshot.validate_authority().map_err(xlsx_failure)?;
        (self.gate)(snapshot).map_err(xlsx_failure)?;
        self.phase = Phase::Done;
        self.snapshot.take().ok_or_else(|| failure("XLSX snapshot is missing"))
    }

    fn quote(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(retirement) = self.retirement.as_ref() { return artifact_retirement_box_demands(retirement, body); }
        if self.output.is_some() || self.text.is_some() || self.archive.is_some() { return Ok(retirement_birth::<Vec<u8>>()); }
        if self.partial.is_some() { return Ok(retirement_birth::<XmlNode>()); }
        if self.snapshot.is_some() { return Ok(retirement_birth::<XlsxSnapshot>()); }
        if let Some(source) = self.paged.as_ref() { return source.next_demand(body); }
        Ok(RetirementDemand { depth: 1, ..Default::default() })
    }
}

impl NaturalFileDecodeCursor<XlsxSnapshot> for NaturalXlsxDecoder {
    fn advance(&mut self, context: &mut StepContext<'_>) -> Result<NaturalFileDecodeStep<XlsxSnapshot>, MediaError> {
        let limit = self.contract.max_work_units_per_step.min(context.fuel_remaining());
        let mut spent = 0u64;
        let mut finished = None;
        while spent < limit && !context.is_cancelled() && !(spent > 0 && context.should_yield()) {
            let (units, snapshot) = self.next(limit - spent)?;
            spent = spent.saturating_add(units.min(limit - spent));
            if snapshot.is_some() { finished = snapshot; break; }
        }
        context.consume_fuel(spent);
        self.progress.completed_work_units = self.progress.completed_work_units.saturating_add(spent);
        self.progress.owned_output_bytes = self.owned();
        match finished {
            Some(snapshot) => Ok(NaturalFileDecodeStep::Complete { snapshot, progress: self.progress }),
            None if spent == 0 => Ok(NaturalFileDecodeStep::Yield),
            None => Ok(NaturalFileDecodeStep::Progress(self.progress)),
        }
    }

    fn begin_close(&mut self) {
        if let Some(mut cursor) = self.parsing.take() {
            self.partial = cursor.take_partial();
            self.text = Some(cursor.take_text().into_bytes());
        }
        self.inflater = None;
        self.current = None;
        self.members = Vec::new();
        self.directory = None;
        self.target = None;
        self.phase = Phase::Closing;
    }

    fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<PluginLifecycleStep, ValueError> {
        if self.retirement.is_some() {
            let body = effect.grant().maximum_copy_bytes;
            let step = settle_close_poll(artifact_retirement_box_close_step(&mut self.retirement, body, effect))?;
            return Ok(PluginLifecycleStep::Progress(step.progress()));
        }
        if self.output.is_some() { return admit_retirement(&mut self.output, &mut self.retirement, effect); }
        if self.text.is_some() { return admit_retirement(&mut self.text, &mut self.retirement, effect); }
        if self.archive.is_some() { return admit_retirement(&mut self.archive, &mut self.retirement, effect); }
        if self.partial.is_some() { return admit_retirement(&mut self.partial, &mut self.retirement, effect); }
        if self.snapshot.is_some() { return admit_retirement(&mut self.snapshot, &mut self.retirement, effect); }
        let Some(source) = self.paged.as_mut() else { return Ok(PluginLifecycleStep::Complete(Default::default())); };
        let settled = settle_close_poll(source.step(effect.grant().maximum_copy_bytes, effect))?;
        if source.terminal_is_empty() { self.paged = None; return Ok(PluginLifecycleStep::Complete(settled.progress())); }
        Ok(PluginLifecycleStep::Progress(settled.progress()))
    }

    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.quote(body)
    }

    fn terminal_is_empty(&self) -> bool {
        self.phase == Phase::Closing && self.retirement.is_none() && self.output.is_none() && self.text.is_none() && self.partial.is_none()
            && self.snapshot.is_none() && self.archive.is_none() && self.paged.is_none()
    }
}

fn retirement_birth<T: semio_framework_value::RetireOwned>() -> RetirementDemand {
    RetirementDemand { capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<T>(), depth: 1, ..Default::default() }
}

fn admit_retirement<T: semio_framework_value::RetireOwned + Send + 'static>(
    value: &mut Option<T>, owner: &mut Option<Box<dyn ErasedSnapshotRetirement>>, effect: &mut RetainedAllocationEffect<'_>,
) -> Result<PluginLifecycleStep, ValueError> {
    let progress = match semio_framework_value::retirement::admit_owned_retirement(value, effect).result? {
        Some((retirement, progress)) => { *owner = Some(retirement); progress }
        None => RetainedCloneProgress::default(),
    };
    Ok(PluginLifecycleStep::Progress(progress))
}

fn failure(reason: &str) -> MediaError { MediaError::Payload(NATURAL_FILE_PORT.into(), reason.into()) }

fn xlsx_failure(error: XlsxError) -> MediaError { failure(&error.to_string()) }

fn opc_failure(error: OpcError) -> MediaError { xlsx_failure(XlsxError::Opc(error)) }

fn zip_failure(error: ZipError) -> MediaError { opc_failure(OpcError::Zip(error)) }

fn inflate_failure(error: DeflateError) -> MediaError { zip_failure(ZipError::Malformed(format!("inflate refusal: {error:?}"))) }
