//! 📄️ Payload pages and ledger: the payload size constants, the payload stream and admission fault, the page source, the rejected page, the operation ledger and the retained job payload with its reader. See [`super`].

use super::*;

//#region 📄️RetainedPayload
pub const JOB_PAYLOAD_PAGE_BYTES: usize = 16 * 1024;

pub const JOB_PAYLOAD_OPERATION_PAGES: usize = 256;

pub const JOB_PAYLOAD_OPERATION_BYTES: usize = JOB_PAYLOAD_PAGE_BYTES * JOB_PAYLOAD_OPERATION_PAGES;

pub const JOB_PAYLOAD_PROCESS_BYTES: usize = 64 * 1024 * 1024;

/// 🏛️ The live sum of every operation ledger's retained payload pages, capped by
/// [`JOB_PAYLOAD_PROCESS_BYTES`]. It is a CEILING over the whole process — every concurrent
/// operation moves it — so it stays private and no owner samples its absolute value; an owner
/// reads its own share through `JobPayloadOperationLedger::process_share_bytes`.
pub static JOB_PAYLOAD_PROCESS_OWNED_BYTES: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum JobPayloadStream {
    CheckpointState = 0,
    Preview = 1,
    CommitState = 2,
    CommitOutput = 3,
    Fault = 4,
}

impl JobPayloadStream {
    const COUNT: usize = 5;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobPayloadAdmissionFault {
    OpportunityExhausted,
    OperationItems,
    OperationBytes,
    ProcessBytes,
    StreamItems,
    StreamBytes,
    WriterFull,
    WriterSealed,
    RejectedSourcePending,
}

/// 🚧️ Admission faults surface to value-level callers as work-limit refusals; sealed or pending sources are invariant violations.
impl From<JobPayloadAdmissionFault> for crate::ValueError {
    fn from(fault: JobPayloadAdmissionFault) -> Self {
        let (kind, message) = match fault {
            JobPayloadAdmissionFault::OpportunityExhausted => (crate::ValueRefusalKind::WorkLimit, "job payload opportunity exhausted"),
            JobPayloadAdmissionFault::OperationItems => (crate::ValueRefusalKind::WorkLimit, "job payload operation item limit reached"),
            JobPayloadAdmissionFault::OperationBytes => (crate::ValueRefusalKind::WorkLimit, "job payload operation byte limit reached"),
            JobPayloadAdmissionFault::ProcessBytes => (crate::ValueRefusalKind::WorkLimit, "job payload process byte limit reached"),
            JobPayloadAdmissionFault::StreamItems => (crate::ValueRefusalKind::WorkLimit, "job payload stream item limit reached"),
            JobPayloadAdmissionFault::StreamBytes => (crate::ValueRefusalKind::WorkLimit, "job payload stream byte limit reached"),
            JobPayloadAdmissionFault::WriterFull => (crate::ValueRefusalKind::WorkLimit, "job payload writer is full"),
            JobPayloadAdmissionFault::WriterSealed => (crate::ValueRefusalKind::InvariantViolated, "job payload writer is sealed"),
            JobPayloadAdmissionFault::RejectedSourcePending => (crate::ValueRefusalKind::InvariantViolated, "job payload rejected source is pending"),
        };
        crate::ValueError::literal(kind, message)
    }
}

pub struct JobPayloadPageSource {
    pub storage: Box<[MaybeUninit<u8>]>,
}

impl std::fmt::Debug for JobPayloadPageSource {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("JobPayloadPageSource").field("backing_identity", &self.backing_identity()).finish()
    }
}

impl JobPayloadPageSource {
    pub fn admit(extent:usize,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>>{
        if extent==0||extent>JOB_PAYLOAD_PAGE_BYTES{return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"job payload page extent is outside its canonical bound")),work:0}}
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<extent{return crate::RetainedWorkOutcome{result:Ok(None),work:0}}
        let layout=std::alloc::Layout::array::<MaybeUninit<u8>>(extent).expect("canonical payload extent has a valid allocation layout");
        let pointer=unsafe{std::alloc::alloc(layout)}.cast::<MaybeUninit<u8>>();
        if pointer.is_null(){return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"job payload page allocation refused")),work:1}}
        let storage=unsafe{Box::from_raw(std::ptr::slice_from_raw_parts_mut(pointer,extent))};
        crate::RetainedWorkOutcome{result:Ok(Some((Self{storage},RetainedCloneProgress{copied_items:1,retained_capacity_bytes:extent,..Default::default()}))),work:1}
    }

    pub fn allocated_capacity_bytes(&self) -> usize { self.storage.len() }

    pub fn backing_identity(&self) -> *const MaybeUninit<u8> {
        self.storage.as_ptr()
    }
}

pub struct JobPayloadRejectedPage {
    pub fault: JobPayloadAdmissionFault,
    pub source: ManuallyDrop<Option<JobPayloadPageSource>>,
}

impl std::fmt::Debug for JobPayloadRejectedPage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("JobPayloadRejectedPage").field("fault", &self.fault).field("source", &self.source).finish()
    }
}

impl JobPayloadRejectedPage {
    pub fn source(&self) -> &JobPayloadPageSource {
        self.source.as_ref().expect("rejected job payload page already returned")
    }

    pub fn into_source(mut self) -> JobPayloadPageSource {
        self.source.take().expect("rejected job payload page already returned")
    }
}

impl Drop for JobPayloadRejectedPage {
    fn drop(&mut self) {
        if self.source.is_none() {
            unsafe { ManuallyDrop::drop(&mut self.source) };
        } else {
            debug_assert!(false, "rejected job payload page requires exact source handback");
        }
    }
}

#[doc(hidden)]
pub struct JobPayloadOperationLedger {
    pub operation: OperationId,
    pub generation: Generation,
    pub pages: AtomicUsize,
    pub bytes: AtomicUsize,
    pub stream_pages: [AtomicUsize; JobPayloadStream::COUNT],
    pub stream_bytes: [AtomicUsize; JobPayloadStream::COUNT],
}

impl JobPayloadOperationLedger {
    pub fn reservation_copy_bytes()->usize{5*2*size_of::<usize>()+size_of::<JobPayloadStream>()}


    pub fn new(operation: OperationId, generation: Generation) -> Self {
        Self { operation, generation, pages: AtomicUsize::new(0), bytes: AtomicUsize::new(0), stream_pages: std::array::from_fn(|_| AtomicUsize::new(0)), stream_bytes: std::array::from_fn(|_| AtomicUsize::new(0)) }
    }

    pub fn reserve(&self,stream:JobPayloadStream,extent:usize)->crate::RetainedWorkOutcome<(),JobPayloadAdmissionFault>{
        let turn=self.reserve_original(stream,extent);crate::RetainedWorkOutcome{result:turn.result.map(|_|()).map_err(|(fault,_)|fault),work:turn.work}
    }
    pub fn reserve_original(&self,stream:JobPayloadStream,extent:usize)->crate::RetainedWorkOutcome<usize,(JobPayloadAdmissionFault,usize)>{
        let index=stream as usize;let owners=[(&self.pages,1,JOB_PAYLOAD_OPERATION_PAGES,JobPayloadAdmissionFault::OperationItems),(&self.bytes,extent,JOB_PAYLOAD_OPERATION_BYTES,JobPayloadAdmissionFault::OperationBytes),(&self.stream_pages[index],1,JOB_PAYLOAD_OPERATION_PAGES,JobPayloadAdmissionFault::StreamItems),(&self.stream_bytes[index],extent,JOB_PAYLOAD_OPERATION_BYTES,JobPayloadAdmissionFault::StreamBytes),(&JOB_PAYLOAD_PROCESS_OWNED_BYTES,extent,JOB_PAYLOAD_PROCESS_BYTES,JobPayloadAdmissionFault::ProcessBytes)];
        for(done,(owner,amount,maximum,fault))in owners.iter().enumerate(){
            if owner.try_update(Ordering::AcqRel,Ordering::Acquire,|value|value.checked_add(*amount).filter(|value|*value<=*maximum)).is_err(){
                for(original,amount,_,_)in owners[..done].iter().rev(){original.fetch_sub(*amount,Ordering::AcqRel);}
                return crate::RetainedWorkOutcome{result:Err((*fault,0)),work:1}
            }
        }
        crate::RetainedWorkOutcome{result:Ok(0),work:1}
    }

    pub fn release(&self, stream: JobPayloadStream, extent: usize) {
        let stream_index = stream as usize;
        JOB_PAYLOAD_PROCESS_OWNED_BYTES.fetch_sub(extent, Ordering::AcqRel);
        self.stream_bytes[stream_index].fetch_sub(extent, Ordering::AcqRel);
        self.stream_pages[stream_index].fetch_sub(1, Ordering::AcqRel);
        self.bytes.fetch_sub(extent, Ordering::AcqRel);
        self.pages.fetch_sub(1, Ordering::AcqRel);
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.pages.load(Ordering::Acquire) == 0 && self.bytes.load(Ordering::Acquire) == 0 && self.stream_pages.iter().all(|count| count.load(Ordering::Acquire) == 0) && self.stream_bytes.iter().all(|count| count.load(Ordering::Acquire) == 0)
    }

    /// 🧾️ This ledger's own share of [`JOB_PAYLOAD_PROCESS_BYTES`]. [`Self::reserve`] and
    /// [`Self::release`] are the ONLY mutators of `JOB_PAYLOAD_PROCESS_OWNED_BYTES`, and each moves
    /// the process counter and this ledger's `bytes` by the same page in the same call, so the
    /// process counter is exactly the sum of every live ledger's share and a ledger reading zero
    /// here has returned every page it ever took from the process budget. This — not the process
    /// counter's absolute value — is the observable an owner may assert: the absolute value is
    /// moved by every other operation alive in the process, so sampling it before and after a
    /// close ladder answers about the whole process rather than about this owner.
    pub fn process_share_bytes(&self) -> usize {
        self.bytes.load(Ordering::Acquire)
    }
}

pub struct JobPayloadPage {
    pub(crate) source: JobPayloadPageSource,
    pub(crate) length: usize,
}

impl JobPayloadPage {
    pub fn bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.source.storage.as_ptr().cast::<u8>(), self.length) }
    }
}

pub struct RetainedJobPayload {
    pub(crate) stream: JobPayloadStream,
    pub(crate) pages: [MaybeUninit<JobPayloadPage>; JOB_PAYLOAD_OPERATION_PAGES],
    pub(crate) page_count: usize,
    pub(crate) length: usize,
    pub(crate) ledger: ManuallyDrop<Option<Arc<JobPayloadOperationLedger>>>,
}

impl RetainedJobPayload {
    pub fn empty(stream: JobPayloadStream) -> Self {
        Self { stream, pages: [const {MaybeUninit::uninit()}; JOB_PAYLOAD_OPERATION_PAGES], page_count: 0, length: 0, ledger: ManuallyDrop::new(None) }
    }

    pub(crate) fn live_pages(&self)->&[JobPayloadPage]{unsafe{std::slice::from_raw_parts(self.pages.as_ptr().cast::<JobPayloadPage>(),self.page_count)}}
    pub(crate) fn last_page_mut(&mut self)->Option<&mut JobPayloadPage>{if self.page_count==0{None}else{Some(unsafe{self.pages[self.page_count-1].assume_init_mut()})}}
    pub(crate) fn push_page(&mut self,page:JobPayloadPage){assert!(self.page_count<JOB_PAYLOAD_OPERATION_PAGES);self.length+=page.length;self.pages[self.page_count].write(page);self.page_count+=1;}
    pub(crate) fn pop_page(&mut self)->Option<JobPayloadPage>{if self.page_count==0{return None}self.page_count-=1;let page=unsafe{self.pages[self.page_count].assume_init_read()};self.length-=page.length;Some(page)}

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn page_count(&self) -> usize {
        self.page_count
    }

    /// 📐️ Observes admitted page backing without borrowing reservation or release authority.
    pub fn allocated_capacity_bytes(&self)->usize{self.live_pages().iter().map(|page|page.source.allocated_capacity_bytes()).sum()}

    /// 🧱️ Reports one live native page extent separately from its initialized byte length.
    pub fn page_capacity_bytes(&self,index:usize)->Option<usize>{self.live_pages().get(index).map(|page|page.source.allocated_capacity_bytes())}

    /// 🔗️ Compares this live payload's original ledger without lending a reference or mutation.
    pub fn payload_authority_identity(&self)->Option<OriginalPayloadSourceIdentity>{self.ledger.as_ref().map(|ledger|OriginalPayloadSourceIdentity(Arc::as_ptr(ledger)as usize))}

    pub fn page(&self, index: usize) -> Option<&[u8]> {
        self.live_pages().get(index).map(JobPayloadPage::bytes)
    }

    pub fn single_page(&self) -> Option<&[u8]> {
        (self.page_count == 1).then(|| self.page(0)).flatten()
    }

    pub fn reader(&self) -> RetainedJobPayloadReader<'_> {
        RetainedJobPayloadReader { payload: self, page: 0 }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.page_count == 0 && self.length == 0 && self.ledger.is_none()
    }
}

pub struct RetainedJobPayloadReader<'a> {
    pub payload: &'a RetainedJobPayload,
    pub page: usize,
}

impl<'a> RetainedJobPayloadReader<'a> {
    pub fn read_page(&mut self, maximum_items: usize, maximum_bytes: usize) -> Option<&'a [u8]> {
        if maximum_items == 0 {
            return None;
        }
        let page = self.payload.page(self.page)?;
        if page.len() > maximum_bytes {
            return None;
        }
        self.page += 1;
        Some(page)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.page == self.payload.page_count()
    }
}

impl std::fmt::Debug for RetainedJobPayload {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("RetainedJobPayload").field("stream", &self.stream).field("page_count", &self.page_count).field("length", &self.length).finish()
    }
}

impl PartialEq for RetainedJobPayload {
    fn eq(&self, other: &Self) -> bool {
        self.stream == other.stream && self.length == other.length && self.page_count == other.page_count && (0..self.page_count).all(|index| self.page(index) == other.page(index))
    }
}

impl Eq for RetainedJobPayload {}

impl Drop for RetainedJobPayload {
    fn drop(&mut self) {
        assert!(std::thread::panicking()||self.terminal_is_empty(),"retained job payload abandoned original pages or operation ledger");
        
    }
}
//#endregion 📄️RetainedPayload
