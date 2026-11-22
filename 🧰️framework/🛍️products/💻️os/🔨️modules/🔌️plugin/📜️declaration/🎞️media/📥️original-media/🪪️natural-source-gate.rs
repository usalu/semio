//! 🪪 Fresh owner admission for one selected natural file and its fixed-page command.

pub use semio_framework_replication_command_ingress::NATURAL_SOURCE_COMMAND_KIND;
pub const NATURAL_SOURCE_TRANSPORT_MAXIMUM_BYTES: u32 = 512 * 1024 * 1024;
pub const NATURAL_SOURCE_PAGE_BYTES: u32 = 4096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NaturalSourceStatus { Retry, Pending, Accepted, Refused, Stale }

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Stage { Prepared, Receive, Sealed, Lent, Closing, Cancelled, Published, Failed }

impl Stage {
    pub(crate) fn complete(self, extent: u32, acknowledged: u32, pending_page: bool) -> NaturalSourceStatus {
        match self {
            Self::Published => NaturalSourceStatus::Accepted,
            Self::Failed => NaturalSourceStatus::Refused,
            Self::Sealed | Self::Lent => NaturalSourceStatus::Pending,
            Self::Receive if !pending_page && acknowledged == extent.div_ceil(NATURAL_SOURCE_PAGE_BYTES) => {
                NaturalSourceStatus::Pending
            }
            Self::Receive => NaturalSourceStatus::Refused,
            _ => NaturalSourceStatus::Stale,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NaturalSourceDeclaration<'a> {
    pub format_kind: &'a str,
    pub extension: &'a str,
    pub media_type: &'a str,
    pub binary: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NaturalSourceOperation {
    pub operation: u64,
    pub generation: u64,
    pub instance: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NaturalSourceLoan<'a> {
    pub operation: NaturalSourceOperation,
    pub declaration: NaturalSourceDeclaration<'a>,
    pub extent: u32,
}

#[derive(Clone, Copy)]
pub struct NaturalSourceOffer<'a> {
    pub declaration: NaturalSourceDeclaration<'a>,
    pub extent: u32,
}

#[derive(Clone, Copy)]
pub struct NaturalSourceBegin<'a> {
    pub loan: NaturalSourceLoan<'a>,
    pub owner: u64,
    pub seq: u64,
    pub kind: u8,
    pub extent: u32,
}

#[derive(Clone, Copy)]
pub struct NaturalSourcePage {
    pub owner: u64,
    pub generation: u64,
    pub command_index: u32,
    pub command_count: u32,
    pub instance: u32,
    pub seq: u64,
    pub kind: u8,
    pub page_index: u32,
    pub page_count: u32,
    pub item_count: u32,
    pub metadata: u32,
}

pub struct PreparedNaturalSource<'a>(NaturalSourceLoan<'a>);

pub struct NaturalSourceCommand<'a> {
    loan: NaturalSourceLoan<'a>,
    seq: u64,
}

impl<'a> PreparedNaturalSource<'a> {
    /// 🔐 Mints a loan using the guest's current operation, declaration and source limit.
    pub fn issue(
        operation: NaturalSourceOperation,
        fresh: bool,
        declaration: NaturalSourceDeclaration<'a>,
        maximum: u32,
        selected: NaturalSourceOffer<'_>,
    ) -> Option<Self> {
        if !fresh || operation.operation == 0 || operation.generation == 0 || operation.instance == 0
            || selected.declaration != declaration || maximum > NATURAL_SOURCE_TRANSPORT_MAXIMUM_BYTES
            || selected.extent == 0 || selected.extent > maximum
        {
            return None;
        }
        Some(Self(NaturalSourceLoan { operation, declaration, extent: selected.extent }))
    }

    pub fn loan(&self) -> NaturalSourceLoan<'a> { self.0 }

    /// 🧾 Checks the complete wire echo before the paid caller consumes preparation.
    pub fn bind(
        &self, begin: NaturalSourceBegin<'_>, live: NaturalSourceOperation, fresh: bool,
    ) -> Option<NaturalSourceCommand<'a>> {
        if !matches_control(self.0, begin, live, fresh) || begin.seq == 0 {
            return None;
        }
        Some(NaturalSourceCommand { loan: self.0, seq: begin.seq })
    }
}

impl NaturalSourceCommand<'_> {
    /// 🎫 Authenticates an end or cancellation control against the selected source.
    pub fn admits_control(&self, control: NaturalSourceBegin<'_>, live: NaturalSourceOperation, fresh: bool) -> bool {
        control.seq == self.seq && matches_control(self.loan, control, live, fresh)
    }

    /// 📄 Validates one canonical cursor against the current paid page frontier.
    pub fn admits_page(
        &self, cursor: NaturalSourcePage, len: u32, next_page: u32, live: NaturalSourceOperation, fresh: bool,
    ) -> bool {
        if !fresh || live != self.loan.operation
            || cursor.owner != u64::from(live.instance) || cursor.instance != live.instance || cursor.generation != live.generation
            || cursor.seq != self.seq || cursor.kind != NATURAL_SOURCE_COMMAND_KIND || cursor.metadata != self.loan.extent
            || cursor.command_index != 0 || cursor.command_count != 1 || cursor.item_count != 1
            || cursor.page_index != next_page || cursor.page_count != self.loan.extent.div_ceil(NATURAL_SOURCE_PAGE_BYTES)
        {
            return false;
        }

        cursor.page_index.checked_mul(NATURAL_SOURCE_PAGE_BYTES)
            .filter(|offset| *offset < self.loan.extent)
            .is_some_and(|offset| len == (self.loan.extent - offset).min(NATURAL_SOURCE_PAGE_BYTES))
    }
}

fn matches_control(
    issued: NaturalSourceLoan<'_>, control: NaturalSourceBegin<'_>, live: NaturalSourceOperation, fresh: bool,
) -> bool {
    fresh && issued.operation == live && control.loan == issued && control.owner == u64::from(live.instance)
        && control.kind == NATURAL_SOURCE_COMMAND_KIND && control.extent == issued.extent
}
