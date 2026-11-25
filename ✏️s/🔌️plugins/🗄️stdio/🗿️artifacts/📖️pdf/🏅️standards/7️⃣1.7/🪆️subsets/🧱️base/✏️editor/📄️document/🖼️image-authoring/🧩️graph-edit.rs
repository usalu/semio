//! 🧩 Assembles paid native PDF graph rows into the Store's paged mutation input.

use super::{ImagePlacement, ObjRef, PdfMutation, PdfOp, PdfSnapshot, PngGraphWriter, place_graph_png_named};
use semio_plugin::__job::{OriginalSourceBinding, OriginalTurnFailure, StepContext, close_original_effect_step};
use semio_plugin::__value::{
    RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind, paged::ByteReadSource,
    retained_clone::retained_clone_grant_admits, retirement::controlled::ControlledRetirement,
};
use semio_os_kernel::os_store::one_item_publication::ArtifactMutationPages;
use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts::{artifact_reference_bytes, png_source::PNG_GRAPH_CHUNK_BYTES};
use std::io::{Cursor, Write};

type Rows = ArtifactMutationPages<PdfMutation>;

pub struct PngGraphEdit {
    source: OriginalSourceBinding,
    revision: [u8; 32],
    writer: PngGraphWriter,
    placement: ImagePlacement,
    size: (u32, u32),
    needs_alpha: bool,
    rows: ControlledRetirement<Option<Rows>>,
    object_index: usize,
    maximum_object: u32,
    existing_scan_done: bool,
    name_index: usize,
    candidate: usize,
    occupied: u64,
    color: Option<usize>,
    mask: Option<usize>,
    done: bool,
}

impl PngGraphEdit {
    /// 🧱 Funds the edit directory before opening the original graph writer.
    pub fn admit<S: ByteReadSource>(
        source: &S, revision: [u8; 32], placement: ImagePlacement, size: (u32, u32), needs_alpha: bool,
        slot: &mut Option<Self>, cx: &mut StepContext<'_>,
    ) -> Result<bool, OriginalTurnFailure> {
        if slot.is_some() { return Ok(true); }
        let capacity = source.byte_len().div_ceil(PNG_GRAPH_CHUNK_BYTES) + 2 + usize::from(needs_alpha);
        let mut demand = Rows::directory_demand(capacity);
        demand.copy_bytes = size_of::<Option<Self>>();
        if !admitted(cx, demand) { return Ok(false); }
        let writer = PngGraphWriter::begin(source, revision)?;
        let rows = Rows::new(capacity)?;
        let bytes = rows.directory_bytes();
        let rows = ControlledRetirement::new(Some(rows)).map_err(|(error, _)| error)?;
        cx.consume_retained(progress(demand, bytes))?;
        cx.consume_fuel(1);
        *slot = Some(Self {
            source: OriginalSourceBinding::borrow(cx), revision, writer, placement, size, needs_alpha, rows,
            object_index: 0, maximum_object: 0, existing_scan_done: false,
            name_index: 0, candidate: 1, occupied: 0, color: None, mask: None, done: false,
        });
        Ok(true)
    }

    /// 🏗️ Advances one borrowed scan, a funded native graph row, or the final placement.
    pub fn advance<S: ByteReadSource>(
        &mut self, source: &S, snapshot: &PdfSnapshot, revision: &[u8; 32], cx: &mut StepContext<'_>,
    ) -> Result<bool, OriginalTurnFailure> {
        self.check_source(cx)?;
        if self.revision != *revision { return Err(refusal("PDF image publication lost its live revision").into()); }
        if self.done || cx.should_yield() || cx.is_cancelled() { return Ok(self.done); }
        if self.writer.resource().is_none() {
            let mut row = None;
            let grant = cx.retained_grant();
            if let Some((result, performed)) = cx.with_original_source_effect(grant, |_, effect| {
                self.writer.advance(source, revision, ObjRef { num: 0, gen: 0 }, &mut row, effect)
            })? {
                cx.consume_fuel(u64::try_from(performed).map_err(|_| refusal("PDF image graph hashing exceeded its grant"))?);
                cx.consume_retained(result?)?;
                debug_assert!(row.is_none());
            }
            return Ok(false);
        }
        if !self.existing_scan_done {
            self.check_existing(source, snapshot, revision, cx)?;
            return Ok(false);
        }
        let rows = self.rows.original_mut().and_then(Option::as_mut).ok_or_else(|| refusal("PDF image edit is closing"))?;
        if let Some(demand) = rows.next_page_demand() {
            if admitted(cx, demand) {
                cx.consume_retained(rows.admit_page(cx.retained_grant())?)?;
                cx.consume_fuel(1);
            }
            return Ok(false);
        }
        if !self.writer.rows_done() {
            let used = u32::try_from(rows.len()).map_err(|_| refusal("PDF image edit has too many native rows"))?;
            let num = self.maximum_object.checked_add(used).and_then(|value| value.checked_add(1))
                .ok_or_else(|| refusal("PDF image edit exhausted native object identities"))?;
            let mut row = None;
            let grant = cx.retained_grant();
            if let Some((progress, performed)) = cx.with_original_source_effect(grant, |_, effect| {
                self.writer.advance(source, revision, ObjRef { num, gen: 0 }, &mut row, effect)
            })? {
                cx.consume_fuel(u64::try_from(performed).map_err(|_| refusal("PDF image graph work exceeded its grant"))?);
                cx.consume_retained(progress?)?;
                if let Some(row) = row { assert!(rows.push(row).is_ok(), "funded PNG mutation page and slot"); }
            }
            return Ok(false);
        }
        if self.color.is_none() || self.needs_alpha && self.mask.is_none() {
            self.find_name(snapshot, cx)?;
            return Ok(false);
        }
        self.append_placement(cx)
    }

    fn check_existing<S: ByteReadSource>(
        &mut self, source: &S, snapshot: &PdfSnapshot, revision: &[u8; 32], cx: &mut StepContext<'_>,
    ) -> Result<(), OriginalTurnFailure> {
        let Some(object) = snapshot.objects.get(self.object_index) else {
            let demand = RetirementDemand { copy_bytes: size_of::<usize>() + size_of::<bool>(), depth: 1, ..Default::default() };
            if admitted(cx, demand) {
                self.writer.finish_existing()?;
                self.existing_scan_done = true;
                cx.consume_retained(progress(demand, 0))?;
                cx.consume_fuel(1);
            }
            return Ok(());
        };
        if self.writer.matches_existing(&object.value)? {
            let grant = cx.retained_grant();
            if let Some((result, performed)) = cx.with_original_source_effect(grant, |_, effect| {
                self.writer.compare_existing(source, revision, &object.value, effect)
            })? {
                cx.consume_fuel(u64::try_from(performed).map_err(|_| refusal("PDF stored image comparison exceeded its grant"))?);
                let (done, receipt) = result?;
                cx.consume_retained(receipt)?;
                if done {
                    self.maximum_object = self.maximum_object.max(object.id.num);
                    self.object_index += 1;
                }
            }
            return Ok(());
        }
        let count = (snapshot.objects.len() - self.object_index).min(128).min((cx.fuel_remaining() / 32) as usize);
        if count == 0 { return Ok(()); }
        let start = self.object_index;
        for row in &snapshot.objects[start..start + count] {
            if self.writer.matches_existing(&row.value)? { break; }
            self.maximum_object = self.maximum_object.max(row.id.num);
            self.object_index += 1;
        }
        let selected = self.object_index - start;
        cx.consume_fuel((selected + usize::from(selected < count)) as u64 * 32);
        Ok(())
    }

    fn find_name(&mut self, snapshot: &PdfSnapshot, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        let total = snapshot.images.len().checked_add(snapshot.forms.len()).ok_or_else(|| refusal("PDF image namespace is too large"))?;
        let count = (total - self.name_index).min(128).min((cx.fuel_remaining() / 32).min(128) as usize);
        let end = self.name_index + count;
        if count == 0 && end != total { return Ok(()); }
        for index in self.name_index..end {
            let name = snapshot.images.get(index).map_or_else(
                || snapshot.forms[index - snapshot.images.len()].id.as_str(),
                |image| image.id.as_str(),
            );
            let Some(suffix) = name.strip_prefix("SemioImage") else { continue; };
            if suffix.len() > 20 || suffix.starts_with('0') || !suffix.bytes().all(|byte| byte.is_ascii_digit()) { continue; }
            if let Ok(value) = suffix.parse::<usize>() {
                if let Some(offset) = value.checked_sub(self.candidate).filter(|offset| *offset < 64) {
                    self.occupied |= 1_u64 << offset;
                }
            }
        }
        cx.consume_fuel((count as u64 * 32).max(1));
        self.name_index = end;
        if end != total { return Ok(()); }
        let mut free = !self.occupied;
        for slot in [&mut self.color, &mut self.mask].into_iter().take(1 + usize::from(self.needs_alpha)) {
            if slot.is_some() || free == 0 { continue; }
            let value = self.candidate.checked_add(free.trailing_zeros() as usize)
                .ok_or_else(|| refusal("PDF image namespace is exhausted"))?;
            *slot = Some(value);
            free &= free - 1;
        }
        if self.color.is_none() || self.needs_alpha && self.mask.is_none() {
            self.candidate = self.candidate.checked_add(64).ok_or_else(|| refusal("PDF image namespace is exhausted"))?;
            self.name_index = 0;
            self.occupied = 0;
        }
        Ok(())
    }

    fn append_placement(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let mut color_text = [0; 32];
        let mut mask_text = [0; 32];
        let color = resource_name(self.color.expect("funded PDF color name"), &mut color_text)?;
        let mask = self.mask.map(|index| resource_name(index, &mut mask_text)).transpose()?;
        let temporary = 3 * size_of::<PdfMutation>();
        let path = usize::from(matches!(self.placement, ImagePlacement::Insert { .. })) * 4 * size_of::<PdfOp>();
        let body = path + color.len() * 2 + artifact_reference_bytes("s.stdio.png.rgb")
            + mask.map_or(0, |text| text.len() * 2 + artifact_reference_bytes("s.stdio.png.alpha"));
        let demand = RetirementDemand {
            copy_bytes: temporary + body, capacity_bytes: temporary + body, release_bytes: temporary,
            depth: 1,
        };
        if !admitted(cx, demand) { return Ok(false); }
        let placements = place_graph_png_named(
            self.placement, &self.writer, self.size.0, self.size.1, color.to_owned(), mask.map(str::to_owned),
        )?;
        let rows = self.rows.original_mut().and_then(Option::as_mut).expect("funded PDF PNG mutation pages");
        let work = placements.len() as u64;
        for row in placements { assert!(rows.push(row).is_ok(), "funded PDF PNG placement row"); }
        cx.consume_retained(RetainedCloneProgress { released_bytes: temporary, ..progress(demand, temporary + body) })?;
        cx.consume_fuel(work.min(cx.fuel_remaining()));
        self.done = true;
        Ok(true)
    }

    /// 📬 Moves the complete edit to the Store publication input.
    pub fn take_pages(&mut self, revision: &[u8; 32], output: &mut Option<Rows>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.check_source(cx)?;
        let demand = RetirementDemand { copy_bytes: size_of::<Option<Rows>>(), depth: 1, ..Default::default() };
        if !self.done || self.revision != *revision || output.is_some() || !admitted(cx, demand) { return Ok(false); }
        let Some(rows) = self.rows.original_mut().and_then(Option::take) else { return Ok(false); };
        cx.consume_retained(progress(demand, 0))?;
        cx.consume_fuel(1);
        *output = Some(rows);
        Ok(true)
    }

    /// ♻️ Returns private graph state and any unpublished rows under the Original caller's authority.
    pub fn close(&mut self, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        self.check_source(cx)?;
        let body = cx.retained_grant().maximum_copy_bytes;
        if !self.writer.terminal_is_empty() {
            close_original_effect_step(cx, self.writer.close_demand(body)?, |effect| self.writer.close(body, effect))?;
        } else if !self.rows.terminal_is_empty() {
            close_original_effect_step(cx, self.rows.next_demand(body)?, |effect| self.rows.step(body, effect))?;
        }
        Ok(())
    }

    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.writer.terminal_is_empty() { self.writer.close_demand(body) } else { self.rows.next_demand(body) }
    }

    pub fn terminal_is_empty(&self) -> bool { self.writer.terminal_is_empty() && self.rows.terminal_is_empty() }

    fn check_source(&self, cx: &StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        if self.source.matches(cx) { Ok(()) } else { Err(refusal("PDF image graph changed Original caller").into()) }
    }
}

fn resource_name(index: usize, buffer: &mut [u8]) -> Result<&str, ValueError> {
    let mut cursor = Cursor::new(buffer);
    write!(cursor, "SemioImage{index}").map_err(|_| refusal("PDF image name exceeds its fixed scratch"))?;
    let length = cursor.position() as usize;
    std::str::from_utf8(&cursor.into_inner()[..length]).map_err(|_| refusal("PDF image name is not UTF-8"))
}

fn refusal(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}

fn admitted(cx: &StepContext<'_>, demand: RetirementDemand) -> bool {
    !cx.should_yield() && !cx.is_cancelled() && cx.fuel_remaining() > 0 && retained_clone_grant_admits(cx.retained_grant(), demand)
}

fn progress(demand: RetirementDemand, bytes: usize) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: bytes, ..Default::default() }
}
