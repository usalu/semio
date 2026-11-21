//! 🖼️ Drives a native picker loan through the actor's fixed ingress pages and exact receipts.
use semio_framework::{DslValue, kernel::{
    CommandIngressStatus, CommandPageCursor, Event, FixedCommandPage, OriginalMediaControlEvent,
    OriginalMediaWireControl, OriginalMediaWireLoan, OriginalMediaWirePhase as Phase,
    OriginalMediaWireResult as Receipt, COMMAND_MAXIMUM_BYTES, COMMAND_PAGE_MAXIMUM_BYTES,
    ORIGINAL_MEDIA_COMMAND_KIND,
}};
use std::{future::Future, io::Read, time::{Duration, Instant}};

pub struct NativeOriginalMediaRequest {
    loan: OriginalMediaWireLoan,
    maximum: u32,
    generation: u64,
}

impl NativeOriginalMediaRequest {
    /// 🪪 Resolves the guest-issued descriptor for exactly the mounted instance.
    pub fn parse(args: &DslValue, instance: u32) -> Result<Option<Self>, &'static str> {
        let Some(request) = args.get("originalMedia") else { return Ok(None); };
        let invalid = "original-media.invalid-issued-request";
        let loan = request.get("loan").ok_or(invalid)?;
        if !matches!(request, DslValue::Object(rows) if rows.len() == 3)
            || !matches!(loan, DslValue::Object(rows) if rows.len() == 5) { return Err(invalid); }
        let instance_id = loan.get("instanceId").and_then(DslValue::as_u64).and_then(|value| u32::try_from(value).ok()).ok_or(invalid)?;
        let kind = request.get("kind").and_then(DslValue::as_u64).and_then(|value| u32::try_from(value).ok()).ok_or(invalid)?;
        let maximum = request.get("maximumFileBytes").and_then(DslValue::as_u64).and_then(|value| u32::try_from(value).ok()).ok_or(invalid)?;
        if instance == 0 || instance != instance_id || kind != u32::from(ORIGINAL_MEDIA_COMMAND_KIND)
            || maximum == 0 || maximum as usize > COMMAND_MAXIMUM_BYTES { return Err(invalid); }
        let operation_id = hex_value(loan.get("operationId"), 16).ok_or(invalid)?;
        let generation_hex = hex_value(loan.get("generation"), 16).ok_or(invalid)?;
        if operation_id == "0000000000000000" { return Err(invalid); }
        let generation = u64::from_str_radix(generation_hex, 16).map_err(|_| invalid)?;
        let base_revision = hex_value(loan.get("baseRevision"), 64).ok_or(invalid)?;
        let document_id = loan.get("documentId").and_then(DslValue::as_str)
            .filter(|document| !document.is_empty() && document.chars().take(257).count() <= 256).ok_or(invalid)?;
        Ok(Some(Self { loan: OriginalMediaWireLoan {
            operation_id: operation_id.to_owned(), generation: generation_hex.to_owned(), instance_id,
            document_id: document_id.to_owned(), base_revision: base_revision.to_owned(),
        }, maximum, generation }))
    }

    /// 🏠 Checks the mounted document before opening the picker. The guest validates its full content digest when it accepts the loan.
    pub fn matches_document(&self, instance: u32, document: Option<&str>) -> bool {
        instance == self.loan.instance_id && document == Some(self.loan.document_id.as_str())
    }
    pub fn instance_id(&self) -> u32 { self.loan.instance_id }

    /// 🎟️ Compares the full guest-issued loan independent of its file limit.
    pub fn same_loan(&self, other: &Self) -> bool { self.loan == other.loan }
}

fn hex_value(value: Option<&DslValue>, width: usize) -> Option<&str> {
    value.and_then(DslValue::as_str).filter(|text| {
        text.len() == width && text.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
    })
}

pub(super) struct NativeOriginalMediaTransfer<'a> {
    request: &'a NativeOriginalMediaRequest,
    sequence: u64,
    extent: u32,
}

enum Crossing<'a> {
    Begin(&'a str, &'a str),
    Page(u32, &'a [u8]),
    Complete,
    Cancel,
}

impl NativeOriginalMediaTransfer<'_> {
    pub(super) async fn run<F: Future<Output = Result<CommandIngressStatus, String>>>(
        request: &NativeOriginalMediaRequest, sequence: u64,
        source: Result<Option<(impl Read, impl AsRef<str>, &str, u64)>, String>,
        mut cross: impl FnMut(Event) -> F,
        current: impl Fn() -> bool, mut progress: impl FnMut(u64, u64),
    ) -> Result<bool, String> {
        let mut transfer = NativeOriginalMediaTransfer { request, sequence, extent: 0 };
        if sequence == 0 { return Err("original-media.transport-sequence".into()); }
        let result = async {
            let Some((mut source, name, mime, bytes)) = source? else { return Ok(false); };
            let name = name.as_ref();
            if !current() { return Ok(false); }
            if bytes == 0 || bytes > u64::from(request.maximum) {
                return Err("original-media.extent".into());
            }
            if name.is_empty() || name.chars().take(256).count() > 255 {
                return Err("original-media.metadata".into());
            }
            transfer.extent = bytes as u32;
            transfer.cross(Crossing::Begin(name, mime), &mut cross, &current).await?;
            let mut buffer = [0; COMMAND_PAGE_MAXIMUM_BYTES];
            let mut offset = 0;
            for index in 0..bytes.div_ceil(COMMAND_PAGE_MAXIMUM_BYTES as u64) as u32 {
                if !current() { return Ok(false); }
                let count = (bytes - offset).min(COMMAND_PAGE_MAXIMUM_BYTES as u64) as usize;
                source.read_exact(&mut buffer[..count]).map_err(|error| error.to_string())?;
                transfer.cross(Crossing::Page(index, &buffer[..count]), &mut cross, &current).await?;
                offset += count as u64;
                progress(offset, bytes);
            }
            if !current() { return Ok(false); }
            if source.read(&mut buffer[..1]).map_err(|error| error.to_string())? != 0 {
                return Err("original-media.changed-extent".into());
            }
            transfer.cross(Crossing::Complete, &mut cross, &current).await?;
            Ok(true)
        }.await;
        if matches!(result, Ok(true)) { return result; }
        match transfer.cross(Crossing::Cancel, &mut cross, &|| true).await {
            Ok(()) if !current() => Ok(false),
            Ok(()) => result,
            Err(close) => Err(match result {
                Err(failure) => format!("{failure}; original-media close: {close}"),
                _ => close,
            }),
        }
    }

    async fn cross<F: Future<Output = Result<CommandIngressStatus, String>>>(
        &self, crossing: Crossing<'_>,
        cross: &mut impl FnMut(Event) -> F,
        current: &impl Fn() -> bool,
    ) -> Result<(), String> {
        let deadline = Instant::now() + Duration::from_secs(300);
        let (phase, ordinal) = match crossing {
            Crossing::Begin(..) => (Phase::Begin, None),
            Crossing::Page(index, _) => (Phase::Page, Some(index)),
            Crossing::Complete => (Phase::Complete, None),
            Crossing::Cancel => (Phase::Cancel, None),
        };
        let mut offer = true;
        loop {
            if !current() { return Err("original-media.stale-owner".into()); }
            if Instant::now() >= deadline { return Err("original-media.ack-stalled".into()); }
            let status = cross(if offer { self.event(&crossing)? } else { Event::Wake }).await?;
            offer = match status {
                CommandIngressStatus::OriginalMediaAck(ack) => {
                    if ack.loan != self.request.loan || ack.owner != self.sequence || ack.seq != self.sequence
                        || ack.kind != ORIGINAL_MEDIA_COMMAND_KIND || ack.phase != phase || ack.page_index != ordinal
                        || ack.extent != self.extent { return Err("original-media.foreign-ack".into()); }
                    match ack.result {
                        Receipt::Accepted => return Ok(()),
                        Receipt::Stale if phase == Phase::Cancel => return Ok(()),
                        Receipt::Stale => return Err("original-media.stale".into()),
                        Receipt::Refused => return Err("original-media.refused".into()),
                        Receipt::Retry => true,
                        Receipt::Pending => false,
                    }
                }
                CommandIngressStatus::Idle => false,
                CommandIngressStatus::Backpressure(cursor) if ordinal.is_some_and(|index| cursor == self.cursor(index)) => true,
                _ => return Err("original-media.foreign-ingress".into()),
            };
        }
    }

    fn event(&self, crossing: &Crossing<'_>) -> Result<Event, String> {
        let control = match crossing {
            Crossing::Begin(name, mime) => OriginalMediaWireControl::Begin {
                name: (*name).to_owned(), media_type: (*mime).to_owned(),
            },
            Crossing::Complete => OriginalMediaWireControl::Complete,
            Crossing::Cancel => OriginalMediaWireControl::Cancel,
            Crossing::Page(index, bytes) => return Ok(Event::CommandIngressPage {
                cursor: self.cursor(*index),
                bytes: FixedCommandPage::try_copy_from(bytes).map_err(|error| error.describe())?,
            }),
        };
        Ok(Event::OriginalMediaControl(OriginalMediaControlEvent {
            loan: self.request.loan.clone(), owner: self.sequence, seq: self.sequence,
            kind: ORIGINAL_MEDIA_COMMAND_KIND, extent: self.extent, control,
        }))
    }

    fn cursor(&self, index: u32) -> CommandPageCursor {
        CommandPageCursor {
            owner: self.sequence, generation: self.request.generation, command_index: 0, command_count: 1,
            instance: self.request.loan.instance_id, seq: self.sequence, kind: ORIGINAL_MEDIA_COMMAND_KIND,
            page_index: index, page_count: self.extent.div_ceil(COMMAND_PAGE_MAXIMUM_BYTES as u32),
            item_count: 1, metadata: self.extent,
        }
    }
}
