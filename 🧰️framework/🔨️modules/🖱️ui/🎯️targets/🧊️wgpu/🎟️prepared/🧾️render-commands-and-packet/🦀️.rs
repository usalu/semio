//! 🧾️ Render commands and packet: the command kinds, the command pages and directories, the raster keep cursor and the prepared render packet with its drop. See [`super`].

use super::*;

pub type PreparedRenderUploads = PreparedFixedList<PreparedRenderUpload>;

pub type PreparedRenderEvictions = PreparedFixedList<PreparedRenderEviction>;

pub type PreparedRenderScissors = PreparedFixedList<ScissorRect>;

pub type PreparedRenderDirectives = PreparedFixedList<RenderDirective>;

pub type PreparedRasterProducers = PreparedFixedList<PreparedRasterProducer>;

/// 🧩 One immutable scalar command emitted by a retained preparation child.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreparedRenderCommandKind {
    Validate,
    Snap,
    Order,
    Tessellate,
    Batch,
    Hash,
    Upload,
    Packet,
}

impl PreparedRenderCommandKind {
    pub fn code(self) -> u32 {
        match self {
            Self::Validate => 0,
            Self::Snap => 1,
            Self::Order => 2,
            Self::Tessellate => 3,
            Self::Batch => 4,
            Self::Hash => 5,
            Self::Upload => 6,
            Self::Packet => 7,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PreparedRenderCommand {
    pub kind: PreparedRenderCommandKind,
    pub source: usize,
    pub digest: u64,
    pub draw_cursor: Option<DrawMeasureCursor>,
    pub packet_overlay: bool,
}

impl PreparedRenderCommand {
    pub fn kind(&self) -> PreparedRenderCommandKind {
        self.kind
    }

    pub fn source(&self) -> usize {
        self.source
    }

    pub fn digest(&self) -> u64 {
        self.digest
    }

    pub fn draw_cursor(&self) -> Option<DrawMeasureCursor> {
        self.draw_cursor
    }

    pub fn packet_overlay(&self) -> bool {
        self.packet_overlay
    }
}

#[derive(Debug)]
pub(super) struct PreparedRenderCommandPage {
    pub(super) slots: [Option<PreparedRenderCommand>; PREPARED_RENDER_COMMAND_PAGE_ITEMS],
    pub(super) len: usize,
}

const PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS: usize = 256;

pub(super) const PREPARED_RENDER_COMMAND_DIRECTORIES: usize = PREPARED_RENDER_COMMAND_PAGES / PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS;

#[derive(Debug)]
pub(super) struct PreparedRenderCommandDirectory {
    pub(super) pages: [Option<Box<PreparedRenderCommandPage>>; PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS],
    pub(super) len: usize,
}

impl Default for PreparedRenderCommandDirectory {
    fn default() -> Self {
        Self { pages: std::array::from_fn(|_| None), len: 0 }
    }
}

impl Default for PreparedRenderCommandPage {
    fn default() -> Self {
        Self { slots: [None; PREPARED_RENDER_COMMAND_PAGE_ITEMS], len: 0 }
    }
}

#[derive(Debug)]
pub struct PreparedRenderCommandPages {
    directories: [Option<Box<PreparedRenderCommandDirectory>>; PREPARED_RENDER_COMMAND_DIRECTORIES],
    page_count: usize,
    pub(super) len: usize,
}

impl Default for PreparedRenderCommandPages {
    fn default() -> Self {
        Self { directories: std::array::from_fn(|_| None), page_count: 0, len: 0 }
    }
}

impl PreparedRenderCommandPages {
    pub(super) fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(if self.len != 0 { 0 } else if self.page_count != 0 { size_of::<PreparedRenderCommandPage>() } else if self.directories.iter().any(Option::is_some) { size_of::<PreparedRenderCommandDirectory>() } else { 0 })
    }

    pub(super) fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        let demand = self.next_close_release_byte_demand();
        if let Some(step) = prepared_close_gate(grant, self.terminal_is_empty(), demand.clone()) { return step; }
        self.close_step();
        prepared_close_progress(demand.unwrap_or(0))
    }

    pub fn try_push(&mut self, command: PreparedRenderCommand) -> Result<(), PreparedRenderCommand> {
        let page = self.len / PREPARED_RENDER_COMMAND_PAGE_ITEMS;
        let scalar = self.len % PREPARED_RENDER_COMMAND_PAGE_ITEMS;
        if page >= PREPARED_RENDER_COMMAND_PAGES {
            return Err(command);
        }
        let directory_index = page / PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS;
        let page_index = page % PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS;
        if self.directories[directory_index].is_none() {
            self.directories[directory_index] = Some(Box::new(PreparedRenderCommandDirectory::default()));
        }
        let Some(directory) = self.directories[directory_index].as_mut() else { return Err(command) };
        if directory.pages[page_index].is_none() {
            directory.pages[page_index] = Some(Box::new(PreparedRenderCommandPage::default()));
            let Some(directory_len) = directory.len.checked_add(1) else { return Err(command) };
            let Some(page_count) = self.page_count.checked_add(1) else { return Err(command) };
            directory.len = directory_len;
            self.page_count = page_count;
        }
        let Some(owner) = directory.pages[page_index].as_mut() else { return Err(command) };
        owner.slots[scalar] = Some(command);
        let Some(owner_len) = owner.len.checked_add(1) else { return Err(command) };
        let Some(len) = self.len.checked_add(1) else { return Err(command) };
        owner.len = owner_len;
        self.len = len;
        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<&PreparedRenderCommand> {
        let page = index / PREPARED_RENDER_COMMAND_PAGE_ITEMS;
        let scalar = index % PREPARED_RENDER_COMMAND_PAGE_ITEMS;
        let directory = self.directories.get(page / PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS)?.as_ref()?;
        directory.pages.get(page % PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS)?.as_ref()?.slots.get(scalar)?.as_ref()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn close_step(&mut self) -> bool {
        if let Some(index) = self.len.checked_sub(1) {
            let page = index / PREPARED_RENDER_COMMAND_PAGE_ITEMS;
            let scalar = index % PREPARED_RENDER_COMMAND_PAGE_ITEMS;
            let Some(directory) = self.directories.get_mut(page / PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS).and_then(Option::as_mut) else { return false };
            let Some(owner) = directory.pages.get_mut(page % PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS).and_then(Option::as_mut) else { return false };
            owner.slots[scalar] = None;
            let Some(owner_len) = owner.len.checked_sub(1) else { return false };
            owner.len = owner_len;
            self.len = index;
            return false;
        }
        if let Some(page) = self.page_count.checked_sub(1) {
            let directory_index = page / PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS;
            let page_index = page % PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS;
            let Some(directory) = self.directories[directory_index].as_mut() else { return false };
            if !directory.pages[page_index].as_ref().is_some_and(|page| page.len == 0) {
                return false;
            }
            directory.pages[page_index] = None;
            let Some(directory_len) = directory.len.checked_sub(1) else { return false };
            directory.len = directory_len;
            self.page_count = page;
            return false;
        }
        if let Some(index) = self.directories.iter().position(|directory| directory.as_ref().is_some_and(|directory| directory.len == 0)) {
            self.directories[index] = None;
            return false;
        }
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.page_count == 0 && self.directories.iter().all(Option::is_none)
    }
}

/// 🧱️ Send-capable frame data prepared without window, surface, device, or queue access.
pub struct PreparedRenderPacket {
    pub(super) has_animated_primitives: bool,
    pub scene_revision: u64,
    pub preview_generation: u64,
    pub damage: PreparedRenderScissors,
    pub clips: PreparedRenderScissors,
    pub directives: PreparedRenderDirectives,
    pub uploads: PreparedRenderUploads,
    pub evictions: PreparedRenderEvictions,
    pub draw: DrawList,
    pub overlay: Option<DrawList>,
    pub commands: PreparedRenderCommandPages,
    pub time_seconds: f32,
    pub usage: PreparedRenderUsage,
    pub limits: PreparedRenderLimits,
    pub(super) permit: Option<PreparedRenderProcessPermit>,
    pub(super) retirement_phase: u8,
    pub(super) abandonment_slot: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PreparedRasterKeepCursorV1 {
    pub(super) owner: u8,
    pub(super) draw: semio_framework_ui_draw_contract::RasterKeepCursorV1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreparedRasterKeepStepV1<'a> {
    Pending,
    Key(&'a str),
    Complete,
}

impl PreparedRenderPacket {
    pub(super) fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { self.uploads.get(self.uploads.len().saturating_sub(1)).map_or(Ok(0), PreparedRenderUpload::next_close_copy_byte_demand) }

    pub(super) fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if let Some(upload) = self.uploads.get(self.uploads.len().saturating_sub(1)) { return upload.next_close_release_byte_demand(); }
        
        if let Some(PreparedRenderEviction::Mesh { key }) = self.evictions.get(self.evictions.len().saturating_sub(1)) { return Ok(if key.is_empty() { key.capacity() } else { 0 }); }
        if !self.draw.retirement_is_empty() { return self.draw.next_retirement_release_byte_demand(); }
        if let Some(overlay) = self.overlay.as_ref() { return overlay.next_retirement_release_byte_demand(); }
        if !self.commands.terminal_is_empty() { return self.commands.next_close_release_byte_demand(); }
        if !self.damage.is_empty() || !self.clips.is_empty() || !self.directives.is_empty() { return Ok(0); }
        if !self.uploads.terminal_is_empty() { return self.uploads.next_close_release_byte_demand(); }
        if !self.evictions.terminal_is_empty() { return self.evictions.next_close_release_byte_demand(); }
        if !self.damage.terminal_is_empty() { return self.damage.next_close_release_byte_demand(); }
        if !self.clips.terminal_is_empty() { return self.clips.next_close_release_byte_demand(); }
        if !self.directives.terminal_is_empty() { return self.directives.next_close_release_byte_demand(); }
        Ok(0)
    }

    pub(super) fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        let demand = self.next_close_release_byte_demand();
        if let Some(step) = prepared_close_gate(grant, self.retirement_is_empty(), demand) { return step; }
        if let Some(upload) = self.uploads.last_mut() {
            if !upload.terminal_is_empty() { return prepared_close_child(upload.close_granted(grant)); }
            self.uploads.pop(); return prepared_close_progress(0);
        }
        
        if let Some(PreparedRenderEviction::Mesh { key }) = self.evictions.last_mut() {
            if !key.is_empty() || key.capacity() != 0 { return prepared_close_progress(prepared_close_text(key)); }
            self.evictions.pop(); return prepared_close_progress(0);
        }
        if !self.draw.retirement_is_empty() { return prepared_close_child(self.draw.retire_granted(grant)); }
        if let Some(overlay) = self.overlay.as_mut() {
            if !overlay.retirement_is_empty() { return prepared_close_child(overlay.retire_granted(grant)); }
            self.overlay = None; return prepared_close_progress(0);
        }
        if !self.commands.terminal_is_empty() { return prepared_close_child(self.commands.close_granted(grant)); }
        if self.damage.pop().is_some() || self.clips.pop().is_some() || self.directives.pop().is_some() { return prepared_close_progress(0); }
        if !self.uploads.terminal_is_empty() { return prepared_close_child(self.uploads.close_backing_granted(grant)); }
        if !self.evictions.terminal_is_empty() { return prepared_close_child(self.evictions.close_backing_granted(grant)); }
        if !self.damage.terminal_is_empty() { return prepared_close_child(self.damage.close_backing_granted(grant)); }
        if !self.clips.terminal_is_empty() { return prepared_close_child(self.clips.close_backing_granted(grant)); }
        if !self.directives.terminal_is_empty() { return prepared_close_child(self.directives.close_backing_granted(grant)); }
        if let Some(permit) = self.permit.as_mut() {
            let phase = permit.release_phase;
            if permit.release_step() { self.permit = None; }
            else if permit.release_phase == phase { return Ok(CloseStep::Blocked); }
            return prepared_close_progress(0);
        }
        match self.retirement_phase {
            0 => self.scene_revision = 0,
            1 => self.preview_generation = 0,
            2 => self.time_seconds = 0.0,
            3 => self.usage = PreparedRenderUsage::default(),
            4 => self.limits = PreparedRenderLimits::default(),
            5 => {
                if self.abandonment_slot != u8::MAX {
                    let Some(state) = PREPARED_RENDER_PACKET_ABANDONMENT_STATE.get(usize::from(self.abandonment_slot)) else { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original prepared packet abandonment slot is absent")); };
                    let current = state.load(Ordering::Acquire);
                    if !matches!(current, 1 | 3) || state.compare_exchange(current, if current == 3 { 3 } else { 0 }, Ordering::AcqRel, Ordering::Acquire).is_err() { return Ok(CloseStep::Blocked); }
                    self.abandonment_slot = u8::MAX;
                }
            }
            _ => return Ok(CloseStep::Complete { progress: RetainedCloneProgress::default() }),
        }
        self.retirement_phase += 1;
        prepared_close_progress(0)
    }

    pub fn draw(&self) -> &DrawList { &self.draw }

    pub fn overlay(&self) -> Option<&DrawList> { self.overlay.as_ref() }

    pub fn time_seconds(&self) -> f32 { self.time_seconds }

    pub fn has_animated_primitives(&self) -> bool {
        self.has_animated_primitives
    }

    #[cfg(test)]
    const RETIRE_PAGE_BYTES: usize = 16 * 1024;

    pub fn scene_revision(&self) -> u64 {
        self.scene_revision
    }

    pub fn preview_generation(&self) -> u64 {
        self.preview_generation
    }

    pub fn damage(&self) -> &PreparedRenderScissors {
        &self.damage
    }

    pub fn clips(&self) -> &PreparedRenderScissors {
        &self.clips
    }

    pub fn directives(&self) -> &PreparedRenderDirectives {
        &self.directives
    }

    pub fn uploads(&self) -> &PreparedRenderUploads {
        &self.uploads
    }

    pub fn evictions(&self) -> &PreparedRenderEvictions {
        &self.evictions
    }

    pub fn usage(&self) -> PreparedRenderUsage {
        self.usage
    }

    pub fn limits(&self) -> PreparedRenderLimits {
        self.limits
    }

    pub fn is_within_credits(&self) -> bool {
        self.usage.fits(self.limits) && self.permit.as_ref().is_some_and(PreparedRenderProcessPermit::matches)
    }

    pub fn command_pages(&self) -> &PreparedRenderCommandPages {
        &self.commands
    }

    pub fn raster_keep_step<'a>(&'a self, cursor: &mut PreparedRasterKeepCursorV1) -> PreparedRasterKeepStepV1<'a> {
        let draw = match cursor.owner {
            0 => Some(&self.draw),
            1 => self.overlay.as_ref(),
            _ => return PreparedRasterKeepStepV1::Complete,
        };
        let Some(draw) = draw else {
            cursor.owner += 1;
            cursor.draw = semio_framework_ui_draw_contract::RasterKeepCursorV1::default();
            return PreparedRasterKeepStepV1::Pending;
        };
        match draw.raster_keep_step(&mut cursor.draw) {
            semio_framework_ui_draw_contract::RasterKeepStepV1::Pending => PreparedRasterKeepStepV1::Pending,
            semio_framework_ui_draw_contract::RasterKeepStepV1::Key(key) => PreparedRasterKeepStepV1::Key(key),
            semio_framework_ui_draw_contract::RasterKeepStepV1::Complete => {
                cursor.owner += 1;
                cursor.draw = semio_framework_ui_draw_contract::RasterKeepCursorV1::default();
                PreparedRasterKeepStepV1::Pending
            }
        }
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact admitted packet without allocating another owner.")]
    pub fn try_arm_abandonment(mut self) -> Result<Self, Self> {
        let Some(slot) = self.permit.as_ref().map(|permit| usize::from(permit.slot)) else { return Err(self) };
        let Some(state) = PREPARED_RENDER_PACKET_ABANDONMENT_STATE.get(slot) else { return Err(self) };
        if state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(self);
        }
        self.abandonment_slot = slot as u8;
        Ok(self)
    }

    /// 🧹 Advances one owner from an interrupted packet Drop handback.

    pub fn next_close_demands(&self, copy: usize) -> Result<RetainedCloneGrant, ValueError> {
        if let Some(upload) = self.uploads.get(self.uploads.len().saturating_sub(1)) { if !upload.terminal_is_empty() { return upload.next_close_demands(copy); } }
        let mut demand = prepared_owner_demands(self.retirement_is_empty(), self.next_close_release_byte_demand())?;
        demand.maximum_copy_bytes = self.next_close_copy_byte_demand()?;
        Ok(demand)
    }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> { self.close_granted(grant) }
    pub fn next_abandoned_close_demands(copy: usize) -> Result<RetainedCloneGrant, ValueError> {
        prepared_abandoned_demands(PREPARED_RENDER_PACKET_ABANDONMENT_STATE.iter().zip(PREPARED_RENDER_PACKET_ABANDONMENT_OWNER.iter()), 2, copy, Self::next_close_demands, Self::retirement_is_empty)
    }
    pub fn close_abandoned_step(grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        prepared_abandoned_close(PREPARED_RENDER_PACKET_ABANDONMENT_STATE.iter().zip(PREPARED_RENDER_PACKET_ABANDONMENT_OWNER.iter()), 2, grant, Self::retirement_is_empty, Self::close_granted)
    }

    pub fn retirement_is_empty(&self) -> bool {
        self.retirement_phase >= 6
            && self.uploads.is_empty()
            && self.evictions.is_empty()
            && self.draw.retirement_is_empty()
            && self.overlay.as_ref().is_none_or(DrawList::retirement_is_empty)
            && self.commands.terminal_is_empty()
            && self.uploads.terminal_is_empty()
            && self.evictions.terminal_is_empty()
            && self.damage.terminal_is_empty()
            && self.clips.terminal_is_empty()
            && self.directives.terminal_is_empty()
            && self.permit.is_none()
            && self.abandonment_slot == u8::MAX
    }
}

impl Drop for PreparedRenderPacket {
    fn drop(&mut self) {
        if self.retirement_is_empty() || self.abandonment_slot == u8::MAX {
            return;
        }
        let slot = usize::from(self.abandonment_slot);
        let Some(state) = PREPARED_RENDER_PACKET_ABANDONMENT_STATE.get(slot) else { return };
        if state.load(Ordering::Acquire) != 1 {
            return;
        }
        let packet = Box::new(Self {
            has_animated_primitives: self.has_animated_primitives,
            scene_revision: std::mem::take(&mut self.scene_revision),
            preview_generation: std::mem::take(&mut self.preview_generation),
            damage: std::mem::take(&mut self.damage),
            clips: std::mem::take(&mut self.clips),
            directives: std::mem::take(&mut self.directives),
            uploads: std::mem::take(&mut self.uploads),
            evictions: std::mem::take(&mut self.evictions),
            draw: std::mem::replace(&mut self.draw, DrawList::empty()),
            overlay: self.overlay.take(),
            commands: std::mem::take(&mut self.commands),
            time_seconds: std::mem::take(&mut self.time_seconds),
            usage: std::mem::take(&mut self.usage),
            limits: std::mem::take(&mut self.limits),
            permit: self.permit.take(),
            retirement_phase: self.retirement_phase,
            abandonment_slot: self.abandonment_slot,
        });
        self.retirement_phase = 6;
        self.abandonment_slot = u8::MAX;
        PREPARED_RENDER_PACKET_ABANDONMENT_OWNER[slot].store(Box::into_raw(packet), Ordering::Release);
        state.store(2, Ordering::Release);
    }
}
