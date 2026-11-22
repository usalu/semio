//! 🧾 Accounts native raw picker ingress within the existing retained kernel request queue.
use semio_framework::kernel::{CommandPageCursor, Event, FixedCommandPage, FolderArchiveLoadEvent, OriginalMediaControlEvent,
    NaturalSourceControlEvent, NaturalSourcePrepare, NaturalSourceWireDeclaration,
    OriginalMediaWireControl, COMMAND_MAXIMUM_BYTES, COMMAND_PAGE_MAXIMUM_BYTES,
    NATURAL_SOURCE_COMMAND_KIND, ORIGINAL_MEDIA_COMMAND_KIND};

pub(super) enum QueuedOriginalMedia {
    Control(OriginalMediaControlEvent),
    NaturalPrepare(NaturalSourcePrepare),
    NaturalControl(NaturalSourceControlEvent),
    FolderArchive(FolderArchiveLoadEvent),
    Page(CommandPageCursor, FixedCommandPage),
    Poll,
}

impl QueuedOriginalMedia {
    pub(super) fn admit(event: Event) -> Result<Self, Event> {
        match event {
            Event::OriginalMediaControl(control) if valid_control(&control) => Ok(Self::Control(control)),
            Event::NaturalSourcePrepare(prepare) if prepare.instance_id != 0 && prepare.seq != 0
                && valid_natural_declaration(&prepare.declaration, prepare.extent) => Ok(Self::NaturalPrepare(prepare)),
            Event::NaturalSourceControl(control) if valid_natural_control(&control) => Ok(Self::NaturalControl(control)),
            Event::FolderArchiveLoad(load) if load.is_authentic() => Ok(Self::FolderArchive(load)),
            Event::CommandIngressPage { cursor, bytes } if cursor.kind == NATURAL_SOURCE_COMMAND_KIND
                && cursor.instance != 0 && cursor.owner == u64::from(cursor.instance) && cursor.generation != 0
                && cursor.seq != 0 && cursor.command_index == 0 && cursor.command_count == 1 && cursor.item_count == 1
                && cursor.page_count == cursor.metadata.div_ceil(COMMAND_PAGE_MAXIMUM_BYTES as u32)
                && cursor.page_index < cursor.page_count
                && bytes.len() == (cursor.metadata as usize - cursor.page_index as usize * COMMAND_PAGE_MAXIMUM_BYTES).min(COMMAND_PAGE_MAXIMUM_BYTES) =>
                Ok(Self::Page(cursor, bytes)),
            Event::CommandIngressPage { cursor, bytes } if cursor.kind == ORIGINAL_MEDIA_COMMAND_KIND
                && cursor.command_index == 0 && cursor.command_count == 1 && cursor.item_count == 1
                && cursor.instance != 0 && cursor.owner != 0 && cursor.seq != 0
                && cursor.metadata as usize <= COMMAND_MAXIMUM_BYTES
                && cursor.page_count == cursor.metadata.div_ceil(COMMAND_PAGE_MAXIMUM_BYTES as u32)
                && cursor.page_index < cursor.page_count
                && bytes.len() == (cursor.metadata as usize - cursor.page_index as usize * COMMAND_PAGE_MAXIMUM_BYTES).min(COMMAND_PAGE_MAXIMUM_BYTES) =>
                Ok(Self::Page(cursor, bytes)),
            Event::Wake => Ok(Self::Poll),
            rejected => Err(rejected),
        }
    }

    pub(super) fn into_event(self) -> Event {
        match self {
            Self::Control(control) => Event::OriginalMediaControl(control),
            Self::NaturalPrepare(prepare) => Event::NaturalSourcePrepare(prepare),
            Self::NaturalControl(control) => Event::NaturalSourceControl(control),
            Self::FolderArchive(load) => Event::FolderArchiveLoad(load),
            Self::Page(cursor, bytes) => Event::CommandIngressPage { cursor, bytes },
            Self::Poll => Event::Wake,
        }
    }

    pub(super) fn credits(&self) -> (usize, usize) {
        match self {
            Self::Control(control) => (0, original_control_bytes(control)),
            Self::NaturalPrepare(prepare) => (0, natural_declaration_bytes(&prepare.declaration)),
            Self::NaturalControl(control) => (0, natural_control_bytes(control)),
            Self::FolderArchive(load) => (0, load.request.len()),
            Self::Page(..) => (1, COMMAND_PAGE_MAXIMUM_BYTES),
            Self::Poll => (0, 0),
        }
    }
}

pub(super) fn natural_declaration_bytes(declaration: &NaturalSourceWireDeclaration) -> usize {
    declaration.format_kind.capacity() + declaration.extension.capacity() + declaration.media_type.capacity()
}

pub(super) fn valid_natural_declaration(declaration: &NaturalSourceWireDeclaration, extent: u32) -> bool {
    extent != 0 && declaration.binary && !declaration.format_kind.is_empty()
        && declaration.extension.strip_prefix('.').is_some_and(|extension| !extension.is_empty())
        && !declaration.media_type.is_empty()
        && natural_declaration_bytes(declaration) <= COMMAND_PAGE_MAXIMUM_BYTES - 32
}

fn valid_natural_control(control: &NaturalSourceControlEvent) -> bool {
    let loan = &control.loan;
    let valid_hex = |value: &str| value.len() == 16 && value != "0000000000000000"
        && value.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'));
    control.kind == NATURAL_SOURCE_COMMAND_KIND && control.seq != 0 && loan.instance_id != 0
        && control.owner == u64::from(loan.instance_id) && control.extent == loan.extent
        && valid_natural_declaration(&loan.declaration, loan.extent)
        && valid_hex(&loan.operation_id) && valid_hex(&loan.generation)
        && natural_control_bytes(control) <= COMMAND_PAGE_MAXIMUM_BYTES
}

fn natural_control_bytes(control: &NaturalSourceControlEvent) -> usize {
    let loan = &control.loan;
    natural_declaration_bytes(&loan.declaration) + loan.operation_id.capacity() + loan.generation.capacity()
}

fn valid_control(control: &OriginalMediaControlEvent) -> bool {
    let file_valid = match &control.control {
        OriginalMediaWireControl::Begin { name, media_type } => !name.is_empty() && name.len() <= 1_020
            && !media_type.is_empty() && media_type.len() <= 255 && control.extent != 0,
        OriginalMediaWireControl::Complete => control.extent != 0,
        OriginalMediaWireControl::Cancel => true,
    };
    control.kind == ORIGINAL_MEDIA_COMMAND_KIND && control.owner != 0 && control.seq != 0
        && control.loan.instance_id != 0 && control.extent as usize <= COMMAND_MAXIMUM_BYTES
        && control.loan.operation_id.len() == 16 && control.loan.generation.len() == 16
        && control.loan.base_revision.len() == 64 && !control.loan.document_id.is_empty()
        && control.loan.document_id.len() <= 1_024 && file_valid
        && original_control_bytes(control) <= COMMAND_PAGE_MAXIMUM_BYTES
}

fn original_control_bytes(control: &OriginalMediaControlEvent) -> usize {
    let loan = &control.loan;
    loan.operation_id.capacity() + loan.generation.capacity() + loan.base_revision.capacity()
        + loan.document_id.capacity() + match &control.control {
            OriginalMediaWireControl::Begin { name, media_type } => name.capacity() + media_type.capacity(),
            _ => 0,
        }
}
