//! 🧱️ Prepares paid native PDF graph rows from the retained original compressed PNG.

use super::{InsertObject, ObjRef, PdfMutation, PdfObject};
use semio_framework_value::{
    OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneProgress, RetirementDemand, ValueError,
    ValueRefusalKind, paged::ByteReadSource, retained_clone::retained_clone_grant_admits,
    retirement::controlled::ControlledRetirement,
};
use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts::png_source::{
    PNG_GRAPH_CHUNK_BYTES, PNG_GRAPH_MAXIMUM_BYTES, PngGraphExtent, append_hex, hex_octet, matching_png_part,
};
use semio_framework_hash::Sha256;

const READ_QUANTUM: usize = 4096;

fn refused(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, message) }

/// 🎟️ Retains only one private physical PDF graph row until its receiver is funded.
pub struct PngGraphWriter {
    source: usize,
    extent: usize,
    revision: [u8; 32],
    digest: Sha256,
    hashed: usize,
    ordinal: usize,
    existing_parts: u32,
    existing_cursor: Option<(usize, usize)>,
    pending: ControlledRetirement<Option<String>>,
}

impl PngGraphWriter {
    pub fn birth_demand() -> RetirementDemand { RetirementDemand { copy_bytes: size_of::<Self>(), depth: 1, ..Default::default() } }

    pub fn begin<S: ByteReadSource>(source: &S, revision: [u8; 32]) -> Result<Self, ValueError> {
        if !(8..=PNG_GRAPH_MAXIMUM_BYTES).contains(&source.byte_len()) {
            return Err(refused("original PNG does not fit the PDF graph resource extent"));
        }
        Ok(Self { source: source as *const S as usize, extent: source.byte_len(), revision,
            digest: Sha256::new(), hashed: 0, ordinal: 0, existing_parts: 0, existing_cursor: None,
            pending: ControlledRetirement::new(None).map_err(|(error, _)| error)? })
    }

    pub fn resource(&self) -> Option<PngGraphExtent> {
        (self.hashed == self.extent).then(|| PngGraphExtent { digest: self.digest.clone().finalize(), length: self.extent })
    }

    pub fn rows_done(&self) -> bool { self.ordinal == self.extent.div_ceil(PNG_GRAPH_CHUNK_BYTES) }

    /// 🔍️ Selects an existing graph part with the exact original digest before comparing its body.
    pub fn matches_existing(&self, object: &PdfObject) -> Result<bool, ValueError> {
        let extent = self.resource().ok_or_else(|| refused("original PNG graph digest is still open"))?;
        Ok(matching_png_part(object, &extent.digest).map_err(|_| refused("stored PNG part is not canonical"))?.is_some())
    }

    /// 🧾️ Checks one funded window of a matching original graph part byte for byte.
    pub fn compare_existing<S: ByteReadSource>(
        &mut self, source: &S, current: &[u8; 32], object: &PdfObject, effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<(bool, RetainedCloneProgress), ValueError> {
        self.check_source(source, current)?;
        let extent = self.resource().ok_or_else(|| refused("original PNG graph digest is still open"))?;
        let (length, ordinal, hex) = matching_png_part(object, &extent.digest).map_err(|_| refused("stored PNG part is not canonical"))?
            .ok_or_else(|| refused("the stored PNG part changed while comparing"))?;
        if length != extent.length || self.existing_parts & (1 << ordinal) != 0 || self.ordinal != 0 {
            return Err(refused("stored PNG repeats an ordinal or changes the original extent"));
        }
        let identity = object as *const PdfObject as usize;
        let position = match self.existing_cursor {
            Some((pinned, position)) if pinned == identity => position,
            Some(_) => return Err(refused("the stored PNG part owner changed while comparing")),
            None => 0,
        };
        let metadata = size_of::<Option<(usize, usize)>>() + size_of::<u32>();
        let work = usize::try_from(effect.remaining_work_units()?).unwrap_or(usize::MAX);
        let count = (hex.len() / 2 - position).min(READ_QUANTUM / 3)
            .min(effect.grant().maximum_copy_bytes.min(work).saturating_sub(metadata) / 3);
        let demand = quote(count * 3 + metadata, 0);
        if count == 0 || !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(demand.copy_bytes as u64)? {
            return Ok((false, Default::default()));
        }
        let mut bytes = [0; READ_QUANTUM / 3];
        if source.read_bytes(ordinal * PNG_GRAPH_CHUNK_BYTES + position, &mut bytes[..count]) != count ||
            !hex[position * 2..(position + count) * 2].chunks_exact(2).zip(&bytes[..count])
                .all(|(pair, actual)| hex_octet(pair) == Some(*actual)) {
            return Err(refused("stored PDF PNG bytes differ from the exact original source"));
        }
        let done = position + count == hex.len() / 2;
        self.existing_cursor = (!done).then_some((identity, position + count));
        if done { self.existing_parts |= 1 << ordinal; }
        Ok((done, receipt(demand, 0)))
    }

    /// ♻️ Reuses the complete source backing, or leaves an absent source ready for new graph rows.
    pub fn finish_existing(&mut self) -> Result<(), ValueError> {
        if self.existing_cursor.is_some() || self.ordinal != 0 { return Err(refused("original PNG existing source scan did not finish")); }
        let count = self.resource().ok_or_else(|| refused("original PNG graph digest is still open"))?
            .chunks().map_err(|_| refused("original PNG graph extent changed"))?;
        match self.existing_parts {
            0 => Ok(()),
            parts if parts == (1_u32 << count) - 1 => { self.ordinal = count; Ok(()) }
            _ => Err(refused("a stored PDF PNG source is missing exact graph parts")),
        }
    }

    /// 📐️ Quotes one hash or copy window, independent payload allocation, or final row move.
    pub fn demand(&self) -> Result<RetirementDemand, ValueError> {
        if self.hashed != self.extent {
            return Ok(quote((self.extent - self.hashed).min(READ_QUANTUM), 0));
        }
        if self.rows_done() { return Ok(Default::default()); }
        let target = (self.extent - self.ordinal * PNG_GRAPH_CHUNK_BYTES).min(PNG_GRAPH_CHUNK_BYTES);
        let capacity = self.pending.original().ok_or_else(|| refused("original PDF PNG graph row is closing"))?;
        Ok(match capacity {
            None => quote(size_of::<String>(), target * 2),
            Some(bytes) if bytes.len() / 2 < target =>
                quote((target - bytes.len() / 2).min(READ_QUANTUM / 3) * 3, 0),
            Some(_) => quote(size_of::<Option<PdfMutation>>(), metadata_bound()),
        })
    }

    /// 📥️ Produces the next registered InsertObject once its exact stream owner and recipient are funded.
    pub fn advance<S: ByteReadSource>(
        &mut self, source: &S, current: &[u8; 32], id: ObjRef, recipient: &mut Option<PdfMutation>,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, ValueError> {
        self.check_source(source, current)?;
        if self.pending.original().is_none() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "original PDF PNG graph writer is closed"));
        }
        if self.rows_done() || recipient.is_some() { return Ok(Default::default()); }
        let demand = self.demand()?;
        if !retained_clone_grant_admits(effect.grant(), demand) ||
            !effect.performed(demand.copy_bytes.max(1) as u64)?
        {
            return Ok(Default::default());
        }
        let mut scratch = [0; READ_QUANTUM];
        if self.hashed != self.extent {
            let count = demand.copy_bytes;
            if source.read_bytes(self.hashed, &mut scratch[..count]) != count { return Err(refused("original PDF PNG hash range shortened")); }
            self.digest.update(&scratch[..count]);
            self.hashed += count;
            return Ok(receipt(demand, 0));
        }
        let pinned = self.resource().expect("encoded source has been sealed");
        let offset = self.ordinal * PNG_GRAPH_CHUNK_BYTES;
        let length = (self.extent - offset).min(PNG_GRAPH_CHUNK_BYTES);
        let pending = self.pending.original_mut().ok_or_else(|| refused("original PDF PNG graph row is closing"))?;
        let Some(data) = pending.as_mut() else {
            let mut data = String::new();
            data.try_reserve_exact(length * 2).map_err(|_| refused("original PDF PNG graph allocation was refused"))?;
            let retained = data.capacity();
            *pending = Some(data);
            return Ok(receipt(demand, retained));
        };
        if data.len() / 2 != length {
            let count = demand.copy_bytes / 3;
            if source.read_bytes(offset + data.len() / 2, &mut scratch[..count]) != count {
                return Err(refused("original PDF PNG copy range shortened"));
            }
            append_hex(data, &scratch[..count]);
            return Ok(receipt(demand, 0));
        }
        if id.num == 0 { return Err(refused("original PDF PNG resource requires a reserved native object identity")); }
        let value = pinned.part(self.ordinal, pending.take().expect("complete PNG chunk")).expect("verified PNG chunk extent");
        let super::PdfObject::Dict(dict) = &value else { unreachable!() };
        let retained = dict.capacity() * size_of::<super::PdfDictEntry>() + dict.iter().map(|entry|
            entry.key.capacity() + match &entry.value { super::PdfObject::Name(name) => name.capacity(),
                super::PdfObject::Str(bytes) => bytes.capacity(), _ => 0 }
        ).sum::<usize>();
        *recipient = Some(PdfMutation::InsertObject(InsertObject { id, value, index: None, admitted_stream_roles: None }));
        self.ordinal += 1;
        Ok(receipt(demand, retained))
    }

    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> { self.pending.next_demand(body) }
    pub fn close(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        self.pending.step(body, effect)
    }
    pub fn terminal_is_empty(&self) -> bool { self.pending.terminal_is_empty() }

    fn check_source<S: ByteReadSource>(&self, source: &S, current: &[u8; 32]) -> Result<(), ValueError> {
        if current != &self.revision || source as *const S as usize != self.source || source.byte_len() != self.extent {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "original PDF PNG graph source or revision changed"));
        }
        Ok(())
    }
}

fn quote(copy_bytes: usize, capacity_bytes: usize) -> RetirementDemand {
    RetirementDemand { copy_bytes, capacity_bytes, depth: 1, ..Default::default() }
}

fn receipt(demand: RetirementDemand, retained_capacity_bytes: usize) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes, ..Default::default() }
}

fn metadata_bound() -> usize {
    5 * size_of::<super::PdfDictEntry>() + "TypeDigestExtentOrdinalOctetsSemioPngSourceV1".len() + 32
}
