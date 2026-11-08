//! 🗃️ Command page set and reader behavior: the page set, the paged command and the paged command reader implementations. See [`super`].

use super::*;

//#region 🔖️PagedCommandIngress
impl CommandPageSet {
    /// 🎟️ Quotes the actual retained page-set home and its exact initial page spine.
    pub fn source_admission_demand(declared:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
        if declared==0||declared>COMMAND_MAXIMUM_PAGES{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"command page declaration exceeds its fixed authority"))}
        Ok(RetirementDemand{copy_bytes:size_of::<Option<Self>>(),capacity_bytes:declared.checked_mul(size_of::<FixedCommandPage>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"command page spine extent overflow"))?,depth:1,..Default::default()})
    }
    /// 🌱️ Retains actual allocated backing before reporting a successful or refused admission receipt.
    pub fn admit_source_step(output:&mut Option<Self>,declared:usize,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<semio_framework_value::RetainedCloneStep>{
        use semio_framework_value::{RetainedCloneStep as Step,RetainedCloneProgress as Progress,RetainedWorkOutcome as Turn,ValueError,ValueRefusalKind};
        use semio_framework_value::retained_clone::retained_clone_grant_admits;
        if output.is_some(){return Turn{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"command page Source admission requires an empty destination")),work:0}}
        let demand=match Self::source_admission_demand(declared){Ok(demand)=>demand,Err(error)=>return Turn{result:Err(error),work:0}};
        if !retained_clone_grant_admits(grant,demand){return Turn{result:Ok(Step::Progress(Default::default())),work:0}}
        let mut pages=std::collections::VecDeque::new();
        if pages.try_reserve_exact(declared).is_err(){return Turn{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"command page Source could not reserve its declared spine")),work:1}}
        let capacity=pages.capacity()*size_of::<FixedCommandPage>();
        *output=Some(Self{pages,declared,byte_len:0,generic_shape_valid:true,all_nonempty:true});
        let progress=Progress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:capacity,..Default::default()};
        if !progress.fits(grant){return Turn{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"command page allocation exceeds its admitted Source extent").with_retained_progress(progress)),work:1}}
        Turn{result:Ok(Step::Complete(progress)),work:1}
    }
    /// 📏️ Prices caller clearing, the retained page slot and collection metadata without copying page bytes.
    pub fn source_capture_demand()->semio_framework_value::RetirementDemand{
        semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<(CommandPageCursor,FixedCommandPage)>>()+size_of::<FixedCommandPage>()+size_of::<Self>(),depth:1,..Default::default()}
    }
    /// 📥️ Captures the original page backing only after a real Source grant funds every actual metadata placement.
    pub fn capture_source_step(&mut self,input:&mut Option<(CommandPageCursor,FixedCommandPage)>,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<semio_framework_value::RetainedCloneStep>{
        use semio_framework_value::{RetainedCloneStep as Step,RetainedCloneProgress as Progress,RetainedWorkOutcome as Turn,ValueError,ValueRefusalKind};
        use semio_framework_value::retained_clone::retained_clone_grant_admits;
        let Some((_,page))=input.as_ref()else{return Turn{result:Ok(Step::Complete(Default::default())),work:0}};
        let Some(bytes)=self.source_push_byte_len(page)else{return Turn{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"command page Source has no slot or byte authority")),work:0}};
        let demand=Self::source_capture_demand();
        if !retained_clone_grant_admits(grant,demand){return Turn{result:Ok(Step::Progress(Default::default())),work:0}}
        let (_,page)=input.take().expect("checked command input remains present");
        self.push_validated(page,bytes);
        Turn{result:Ok(Step::Complete(Progress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}
    }

    pub fn try_new(declared: usize) -> Result<Self, semio_framework_diagnostic::Fault> {
        if declared == 0 || declared > COMMAND_MAXIMUM_PAGES {
            return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-count"), "a command page authority is declared for 1..=COMMAND_MAXIMUM_PAGES pages"));
        }
        let mut pages = std::collections::VecDeque::new();
        pages.try_reserve_exact(declared).map_err(|_| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-allocation"), "declared command page authority could not reserve its exact page slots"))?;
        Ok(Self { pages, declared, byte_len: 0, generic_shape_valid: true, all_nonempty: true })
    }

    /// 📏️ Contiguous bytes a `declared`-page authority reserves — what the reactor's footprint law
    /// compares against `semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`.
    pub const fn reservation_bytes(declared: usize) -> usize {
        declared * size_of::<FixedCommandPage>()
    }

    pub fn declared(&self) -> usize {
        self.declared
    }


    fn source_push_byte_len(&self,page:&FixedCommandPage)->Option<usize>{
        if self.pages.len()>=self.declared{return None}
        self.byte_len.checked_add(page.len()).filter(|total|*total<=COMMAND_MAXIMUM_BYTES)
    }
    fn checked_push_byte_len(&self,page:&FixedCommandPage)->Result<usize,semio_framework_diagnostic::Fault>{
        self.source_push_byte_len(page).ok_or_else(||semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework,semio_framework_diagnostic::FaultCode::new(if self.pages.len()>=self.declared{"plugin.command-page-count"}else{"plugin.command-byte-cap"}),if self.pages.len()>=self.declared{"command page authority is saturated"}else{"command exceeds the assembled host-answer authority a guest may be handed"}))
    }
    fn push_validated(&mut self,page:FixedCommandPage,byte_len:usize){
        if page.is_empty(){self.generic_shape_valid=false;self.all_nonempty=false}
        if self.pages.back().is_some_and(|previous|previous.len()!=COMMAND_PAGE_MAXIMUM_BYTES){self.generic_shape_valid=false}
        self.pages.push_back(page);self.byte_len=byte_len;
    }
    pub fn try_push(&mut self,page:FixedCommandPage)->Result<(),(semio_framework_diagnostic::Fault,FixedCommandPage)>{
        let byte_len=match self.checked_push_byte_len(&page){Ok(bytes)=>bytes,Err(fault)=>return Err((fault,page))};
        self.push_validated(page,byte_len);Ok(())
    }
    /// 📥️ Retains the actual caller page until every admission check has succeeded.
    pub fn try_push_input(&mut self,input:&mut Option<(CommandPageCursor,FixedCommandPage)>)->Result<Option<CommandPageCursor>,semio_framework_diagnostic::Fault>{
        let Some((_,page))=input.as_ref()else{return Ok(None)};
        let byte_len=self.checked_push_byte_len(page)?;
        let(cursor,page)=input.take().expect("validated command input remains present");
        self.push_validated(page,byte_len);Ok(Some(cursor))
    }

    pub fn len(&self) -> usize {
        self.pages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
        let Some(length) = self.pages.front().map(FixedCommandPage::len) else {
            return (true, 0);
        };
        if length > maximum_bytes {
            return (false, 0);
        }
        let page = self.pages.pop_front().expect("fixed command page was present");
        let released = page.len();
        self.byte_len -= released;
        drop(page);
        (self.pages.is_empty(), released)
    }
}

impl PagedCommand {
    pub fn try_from_pages(pages: CommandPageSet) -> Result<Self, (semio_framework_diagnostic::Fault, CommandPageSet)> {
        if pages.is_empty() || pages.len() > COMMAND_MAXIMUM_PAGES {
            return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-count"), "command requires 1..=COMMAND_MAXIMUM_PAGES admitted pages"), pages));
        }
        if !pages.generic_shape_valid {
            return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-shape"), "command pages must be nonempty, at most 4096 bytes, and every nonterminal page must be full"), pages));
        }
        let Some(kind) = pages.pages.front().and_then(|page| page.as_slice().first()).copied() else {
            return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-empty"), "command has no kind byte"), pages));
        };
        Ok(Self { pages: pages.pages, byte_len: pages.byte_len, kind, metadata: 0, item_count: 0 })
    }

    pub fn try_from_presence_pages(own_color: Option<u8>, pages: CommandPageSet, item_count: usize) -> Result<Self, (semio_framework_diagnostic::Fault, CommandPageSet)> {
        if item_count > COMMAND_BATCH_MAXIMUM_ITEMS || pages.len() != item_count.max(1) {
            return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-presence-item-cap"), "Presence command requires one exact page per peer and at most 64 peers"), pages));
        }
        if (item_count == 0 && pages.byte_len != 0) || (item_count != 0 && !pages.all_nonempty) {
            return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-presence-page-shape"), "each Presence peer page must be nonempty and at most 4096 bytes"), pages));
        }
        let metadata = own_color.map_or(0, |color| (1u32 << 8) | u32::from(color));
        Ok(Self { pages: pages.pages, byte_len: pages.byte_len, kind: 28, metadata, item_count: item_count as u32 })
    }

    pub fn byte_len(&self) -> usize {
        self.byte_len
    }

    pub fn page_len(&self) -> usize {
        self.pages.len()
    }

    pub fn front_page(&self) -> Option<&FixedCommandPage> {
        self.pages.front()
    }

    pub fn release_front_page(&mut self, maximum_bytes: usize) -> Option<(bool, usize)> {
        let page_len = self.pages.front().map(FixedCommandPage::len)?;
        if page_len > maximum_bytes {
            return None;
        }
        let page = self.pages.pop_front().expect("front page was present");
        self.byte_len -= page.len();
        let released = page.len();
        drop(page);
        Some((self.pages.is_empty(), released))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.pages.is_empty() && self.byte_len == 0
    }

    pub fn kind(&self) -> u8 {
        self.kind
    }

    pub fn metadata(&self) -> u32 {
        self.metadata
    }

    pub fn item_count(&self) -> u32 {
        self.item_count
    }
}

impl PagedCommandReader {
    pub fn new(command: PagedCommand) -> Self {
        Self { command, offset: 0 }
    }

    pub fn kind(&self) -> u8 {
        self.command.kind()
    }

    pub fn read_byte(&mut self) -> Result<u8, semio_framework_diagnostic::Fault> {
        let byte = self
            .command
            .front_page()
            .and_then(|page| page.as_slice().get(self.offset))
            .copied()
            .ok_or_else(|| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-decode-truncated"), "paged command ended inside a field"))?;
        self.offset += 1;
        if self.command.front_page().is_some_and(|page| self.offset == page.len()) {
            let _ = self.command.release_front_page(COMMAND_PAGE_MAXIMUM_BYTES).expect("fully consumed fixed page is releasable");
            self.offset = 0;
        }
        Ok(byte)
    }

    pub fn read_varint(&mut self) -> Result<u64, semio_framework_diagnostic::Fault> {
        let mut value = 0u64;
        for shift in (0..70).step_by(7) {
            let byte = self.read_byte()?;
            if shift == 63 && byte > 1 {
                return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-decode-varint"), "paged command varint overflowed u64"));
            }
            value |= u64::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-decode-varint"), "paged command varint exceeds ten bytes"))
    }

    pub fn read_bounded_bytes(&mut self, maximum: usize) -> Result<Vec<u8>, semio_framework_diagnostic::Fault> {
        let length = usize::try_from(self.read_varint()?).map_err(|_| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-field-length"), "paged command field length is not representable"))?;
        if length > maximum {
            return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-field-cap"), "paged command field exceeds its exact bounded decode authority"));
        }
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(length).map_err(|_| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-field-allocation"), "paged command field could not reserve its exact bounded authority"))?;
        for _ in 0..length {
            bytes.push(self.read_byte()?);
        }
        Ok(bytes)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.offset == 0 && self.command.terminal_is_empty()
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
        let Some(page_len) = self.command.front_page().map(FixedCommandPage::len) else {
            return (self.offset == 0, 0);
        };
        if page_len > maximum_bytes {
            return (false, 0);
        }
        let released = self.command.release_front_page(maximum_bytes).expect("front fixed page was grant-admitted").1;
        self.offset = 0;
        (self.command.terminal_is_empty(), released)
    }
}
//#endregion 🔖️PagedCommandIngress
