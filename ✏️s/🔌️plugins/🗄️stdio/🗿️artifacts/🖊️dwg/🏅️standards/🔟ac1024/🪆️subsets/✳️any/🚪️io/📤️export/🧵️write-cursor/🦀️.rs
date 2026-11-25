//! 🧵️ The stepped DWG writer: the same bytes as [`dwg_to_bytes`], produced one bounded unit per call so a caller can
//! budget, report progress and cancel between units. Every unit is one object index or frame, one handle row, one
//! section encoding, one compressed page or one page copy. Two units are not subdivided: building the logical snapshot
//! from the drawing and sorting the handle locations.
//!
//! @see <https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf> — §4 R2004 file structure

use super::*;
use semio_s_artifact_stdio_dwg_write_objects::standards::v_ac1024::subsets::any::io::export::objects::{HandleMapBlocks, ObjectRecordStep, ObjectRecords};

/// 🧩️ The result of one unit of writer work.
pub enum DwgWriteStep {
    Working,
    Complete(Vec<u8>),
}

enum Phase {
    Snapshot,
    Objects,
    Handles,
    Slots,
    Pages,
    Layout,
    Assemble,
    Seal,
    Taken,
    Failed(String),
}

/// ✍️ Resumable writer over one drawing or snapshot. Cancelling or failing drops every buffer at once.
pub struct DwgWriteCursor {
    phase: Phase,
    drawing: Option<DwgDrawing>,
    snapshot: Option<DwgSnapshot>,
    records: Option<ObjectRecords>,
    objects: Vec<u8>,
    pairs: Vec<(u64, usize)>,
    sorted: bool,
    handle_blocks: Option<HandleMapBlocks>,
    handles: Vec<u8>,
    handle_next: usize,
    slots: Vec<Option<R2004Section>>,
    slot_next: usize,
    sections: Vec<R2004Section>,
    page_section: usize,
    page_offset: usize,
    pages: Vec<EncodedR2004Page>,
    layout: Option<R2004Layout>,
    output: Vec<u8>,
    written: usize,
}

impl DwgWriteCursor {
    fn empty(phase: Phase) -> Self {
        Self {
            phase, drawing: None, snapshot: None, records: None, objects: Vec::new(), pairs: Vec::new(), sorted: false,
            handle_blocks: None, handles: Vec::new(), handle_next: 0, slots: Vec::new(), slot_next: 0, sections: Vec::new(),
            page_section: 0, page_offset: 0, pages: Vec::new(), layout: None, output: Vec::new(), written: 0,
        }
    }

    /// 🎟️ Starts a new AC1024 document holding `drawing`, as [`dwg_to_bytes`] does.
    pub fn new(drawing: DwgDrawing) -> Self {
        Self { drawing: Some(drawing), ..Self::empty(Phase::Snapshot) }
    }

    /// 🎟️ Starts from an existing logical document, as [`encode_r2004_snapshot`] does.
    pub fn from_snapshot(snapshot: DwgSnapshot) -> Self {
        Self::begin_objects(snapshot)
    }

    fn begin_objects(snapshot: DwgSnapshot) -> Self {
        let records = ObjectRecords::new(&snapshot.drawing.objects);
        Self { snapshot: Some(snapshot), records: Some(records), objects: vec![0xca, 0x0d, 0x00, 0x00], ..Self::empty(Phase::Objects) }
    }

    /// 📈️ Completed fraction in `0.0..=1.0`, never decreasing.
    pub fn progress(&self) -> f32 {
        let fraction = |done: usize, total: usize| (done as f32 / total.max(1) as f32).clamp(0.0, 1.0);
        match &self.phase {
            Phase::Snapshot => 0.0,
            Phase::Objects => {
                let total = self.snapshot.as_ref().map_or(0, |snapshot| snapshot.drawing.objects.len());
                0.05 + 0.4 * fraction(self.pairs.len(), total)
            }
            Phase::Handles => 0.45 + 0.1 * fraction(self.handle_next, self.pairs.len()),
            Phase::Slots => 0.55 + 0.05 * fraction(self.slot_next, SECTION_SLOTS),
            Phase::Pages => 0.6 + 0.25 * fraction(self.page_section, self.sections.len()),
            Phase::Layout => 0.85,
            Phase::Assemble => 0.86 + 0.12 * fraction(self.written, self.pages.len()),
            Phase::Seal => 0.99,
            Phase::Taken => 1.0,
            Phase::Failed(_) => 0.0,
        }
    }

    /// 🛑️ Drops all work; every later step reports the cancellation.
    pub fn cancel(&mut self) {
        *self = Self::empty(Phase::Failed("DWG write cancelled".into()));
    }

    /// ⏱️ Runs one unit. An error is terminal and releases every buffer.
    pub fn step(&mut self) -> Result<DwgWriteStep, String> {
        match self.advance() {
            Ok(step) => Ok(step),
            Err(error) => {
                *self = Self::empty(Phase::Failed(error.clone()));
                Err(error)
            }
        }
    }

    fn advance(&mut self) -> Result<DwgWriteStep, String> {
        match &self.phase {
            Phase::Snapshot => {
                let drawing = self.drawing.take().ok_or("DWG write source missing")?;
                *self = Self::begin_objects(DwgSnapshot::from_drawing(&drawing)?);
            }
            Phase::Objects => self.step_objects()?,
            Phase::Handles => self.step_handles()?,
            Phase::Slots => self.step_slots()?,
            Phase::Pages => self.step_pages()?,
            Phase::Layout => self.step_layout()?,
            Phase::Assemble => self.step_assemble()?,
            Phase::Seal => {
                let layout = self.layout.take().ok_or("DWG write layout missing")?;
                let bytes = seal_r2004(&layout, std::mem::take(&mut self.output))?;
                *self = Self::empty(Phase::Taken);
                return Ok(DwgWriteStep::Complete(bytes));
            }
            Phase::Taken => return Err("DWG write already complete".into()),
            Phase::Failed(error) => return Err(error.clone()),
        }
        Ok(DwgWriteStep::Working)
    }

    fn step_objects(&mut self) -> Result<(), String> {
        let snapshot = self.snapshot.as_ref().ok_or("DWG write snapshot missing")?;
        let records = self.records.as_mut().ok_or("DWG write records missing")?;
        match records.step(&snapshot.drawing.objects)? {
            ObjectRecordStep::Indexed => {}
            ObjectRecordStep::Frame { handle, bytes } => {
                self.pairs.push((handle, self.objects.len()));
                self.objects.extend(bytes);
            }
            ObjectRecordStep::Complete => {
                self.records = None;
                self.handle_blocks = Some(HandleMapBlocks::new());
                self.phase = Phase::Handles;
            }
        }
        Ok(())
    }

    fn step_handles(&mut self) -> Result<(), String> {
        if !self.sorted {
            self.pairs.sort_unstable_by_key(|&(handle, _)| handle);
            self.sorted = true;
            return Ok(());
        }
        if let Some(&(handle, address)) = self.pairs.get(self.handle_next) {
            let blocks = self.handle_blocks.as_mut().ok_or("DWG write handle map missing")?;
            if let Some(block) = blocks.push(handle, address)? {
                self.handles.extend(block);
            }
            self.handle_next += 1;
            return Ok(());
        }
        let blocks = self.handle_blocks.take().ok_or("DWG write handle map missing")?;
        self.handles.extend(blocks.finish()?);
        self.pairs = Vec::new();
        self.slots = (0..SECTION_SLOTS).map(|_| None).collect();
        self.slots[OBJECTS_SLOT] = Some(streamed_slot(OBJECTS_SLOT, std::mem::take(&mut self.objects)));
        self.slots[HANDLES_SLOT] = Some(streamed_slot(HANDLES_SLOT, std::mem::take(&mut self.handles)));
        self.phase = Phase::Slots;
        Ok(())
    }

    fn step_slots(&mut self) -> Result<(), String> {
        while self.slots.get(self.slot_next).is_some_and(Option::is_some) {
            self.slot_next += 1;
        }
        if self.slot_next >= SECTION_SLOTS {
            self.sections = std::mem::take(&mut self.slots).into_iter().flatten().collect();
            self.phase = Phase::Pages;
            return Ok(());
        }
        let snapshot = self.snapshot.as_ref().ok_or("DWG write snapshot missing")?;
        self.slots[self.slot_next] = Some(encode_slot(snapshot, self.slot_next)?);
        self.slot_next += 1;
        Ok(())
    }

    fn step_pages(&mut self) -> Result<(), String> {
        loop {
            let Some(section) = self.sections.get(self.page_section) else {
                self.phase = Phase::Layout;
                return Ok(());
            };
            let span = if section.stored.is_some() { 1 } else { section.content.len() };
            if self.page_offset >= span {
                self.page_section += 1;
                self.page_offset = 0;
                continue;
            }
            let address = self.pages.last().map_or(0x100, |page| page.address + u64::from(page.allocation_size));
            let number = i32::try_from(self.pages.len() + 1).map_err(|_| "R2004 page count exceeds i32")?;
            let page = section.encode_page(self.page_offset, address, number)?;
            self.pages.push(page);
            self.page_offset += R2004_PAGE_SIZE;
            return Ok(());
        }
    }

    fn step_layout(&mut self) -> Result<(), String> {
        let snapshot = self.snapshot.take().ok_or("DWG write snapshot missing")?;
        let sections = std::mem::take(&mut self.sections);
        let descriptors = sections.iter().map(R2004Section::descriptor).collect();
        let history = sections.into_iter().find(|section| section.id == 12).map(|section| section.content).unwrap_or_default();
        let layout = layout_r2004(&snapshot, descriptors, &self.pages, &history)?;
        self.output = layout.prefix.clone();
        self.layout = Some(layout);
        self.phase = Phase::Assemble;
        Ok(())
    }

    fn step_assemble(&mut self) -> Result<(), String> {
        match self.pages.get(self.written) {
            Some(page) => {
                write_data_page(&mut self.output, page)?;
                self.written += 1;
            }
            None => {
                self.pages = Vec::new();
                self.phase = Phase::Seal;
            }
        }
        Ok(())
    }
}
