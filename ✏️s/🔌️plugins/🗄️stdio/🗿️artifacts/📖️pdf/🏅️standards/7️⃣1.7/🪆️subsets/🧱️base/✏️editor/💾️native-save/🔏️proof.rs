//! 🛡️ Verifies that a prepared retained graph still spells the pinned current typed PDF.

use crate::standards::v1_7::subsets::base::{
    modules::{
        lexer::CosGrant,
        lift::{ControlledDocumentLiftCursor, ControlledLiftEvent, ControlledLiftPartial, ControlledLiftReceipt, DocumentLiftCursor},
    },
    schema::{
        graph_source::ObjectSource,
        snapshot::{ObjRef, PdfColorSpace, PdfDictEntry, PdfFont, PdfFontKind, PdfImage, PdfImageBody, PdfObject, PdfOp, PdfPage, PdfSnapshot, PdfTextArrayItem, PdfTextString},
    },
};

const TEXT_CAP: usize = 128;
const TJ_ITEMS: usize = 16;
const FONTS: usize = 8;
pub const COMPARE_WORK: usize = 4096;

type GraphIdentity = (usize, usize, usize, usize, usize, usize, usize, Option<usize>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExactGraphTurn { Lift(ControlledLiftReceipt), Compared { work: usize, ready: bool } }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExactGraphRefusal { Shape, Source, Semantic, Cancelled }

pub struct ProjectionTrailer {
    entries: Vec<PdfDictEntry>,
    source: GraphIdentity,
}

impl ProjectionTrailer {
    pub fn into_entries(self) -> Vec<PdfDictEntry> { self.entries }
}

fn source_identity(loan: GraphLoan<'_>) -> GraphIdentity {
    (loan.objects.as_ptr() as usize, loan.objects.len(), std::ptr::from_ref(loan.trailer) as usize,
        loan.version.as_ptr() as usize, loan.version.len(),
        loan.overlay.page.map_or(0, |row| std::ptr::from_ref(row) as usize),
        loan.overlay.info.map_or(0, |row| std::ptr::from_ref(row) as usize), loan.overlay.remove)
}

/// 📜️ An owned small semantic trailer is funded separately from the original output trailer.
/// The controlled lifter consumes these control entries and ignores unrelated trailer keys.
pub fn projection_trailer_extent(loan: GraphLoan<'_>) -> Result<usize, ExactGraphRefusal> {
    let ids = loan.trailer.id.as_ref();
    if ids.is_some_and(|[a, b]| a.len() > TEXT_CAP || b.len() > TEXT_CAP) { return Err(ExactGraphRefusal::Shape); }
    let count = 1 + usize::from(loan.trailer.info.is_some()) + usize::from(ids.is_some());
    Ok(count * size_of::<PdfDictEntry>() + 4 + 4 * usize::from(loan.trailer.info.is_some())
        + ids.map_or(0, |[a, b]| 2 + 2 * size_of::<PdfObject>() + a.len() + b.len()))
}

pub fn projection_trailer(loan: GraphLoan<'_>, grant: CosGrant) -> Result<Option<(ProjectionTrailer, usize)>, ExactGraphRefusal> {
    let bytes = projection_trailer_extent(loan)?;
    let ids = loan.trailer.id.as_ref();
    let count = 1 + usize::from(loan.trailer.info.is_some()) + usize::from(ids.is_some());
    if grant.work == 0 || grant.allocation_bytes < bytes { return Ok(None); }
    let mut result = Vec::with_capacity(count);
    result.push(PdfDictEntry::new("Root", PdfObject::Ref(loan.trailer.root)));
    if let Some(info) = loan.trailer.info { result.push(PdfDictEntry::new("Info", PdfObject::Ref(info))); }
    if let Some([a, b]) = ids {
        result.push(PdfDictEntry::new("ID", PdfObject::Array(vec![PdfObject::Str(a.clone()), PdfObject::Str(b.clone())])));
    }
    Ok(Some((ProjectionTrailer { entries: result, source: source_identity(loan) }, bytes)))
}

pub struct ExactGraphProof {
    lift: ControlledDocumentLiftCursor,
    projection: Option<PdfSnapshot>,
    source: Option<(usize, usize, GraphIdentity)>,
    image: usize,
    sample: usize,
    page: usize,
    op: usize,
    document: bool,
    ready: bool,
}

impl ExactGraphProof {
    pub fn new() -> Self {
        Self { lift: DocumentLiftCursor::controlled(), projection: None, source: None, image: 0, sample: 0, page: 0, op: 0, document: false, ready: false }
    }

    pub fn comparing(&self) -> bool { self.projection.is_some() }
    pub fn next_inline_body_bytes(&self) -> usize { self.lift.next_inline_body_bytes() }
    pub fn reserving_empty_slots(&self) -> bool { self.lift.reserving_empty_slots() }

    /// 🔂 Reloans the exact request snapshot and candidate graph for each measured lift/comparison turn.
    pub fn advance(
        &mut self, loan: GraphLoan<'_>, control_trailer: &ProjectionTrailer, expected: &PdfSnapshot,
        grant: CosGrant, cancelled: bool,
    ) -> Result<ExactGraphTurn, ExactGraphRefusal> {
        let graph = source_identity(loan);
        if control_trailer.source != graph { return Err(ExactGraphRefusal::Source); }
        let signature = (std::ptr::from_ref(expected) as usize, std::ptr::from_ref(control_trailer) as usize, graph);
        if *self.source.get_or_insert(signature) != signature { return Err(ExactGraphRefusal::Source); }
        if cancelled { return Err(ExactGraphRefusal::Cancelled); }
        if self.ready { return Ok(ExactGraphTurn::Compared { work: 0, ready: true }); }
        if self.projection.is_none() {
            let mut source = OverlaySource(loan);
            let turn = self.lift.advance(&control_trailer.entries, loan.version, &mut source, grant, false);
            if turn.event == Ok(ControlledLiftEvent::Complete) {
                self.projection = self.lift.take_result();
            }
            return Ok(ExactGraphTurn::Lift(turn));
        }
        if grant.work < COMPARE_WORK { return Ok(ExactGraphTurn::Compared { work: 0, ready: false }); }
        let projected = self.projection.as_ref().ok_or(ExactGraphRefusal::Semantic)?;
        if !self.document {
            if !document_lanes(projected, expected) { return Err(ExactGraphRefusal::Semantic); }
            self.document = true;
        } else if let Some(image) = projected.images.get(self.image) {
            let current = expected.images.get(self.image).ok_or(ExactGraphRefusal::Semantic)?;
            if compare_image(image, current, &mut self.sample)? {
                self.image += 1;
                self.sample = 0;
            }
        } else if let Some(page) = projected.pages.get(self.page) {
            let current = expected.pages.get(self.page).ok_or(ExactGraphRefusal::Semantic)?;
            if self.op == 0 && !page_lanes(page, current) { return Err(ExactGraphRefusal::Semantic); }
            if let Some(op) = page.content.get(self.op) {
                if !narrow_op(op) || current.content.get(self.op) != Some(op) {
                    return Err(ExactGraphRefusal::Semantic);
                }
                self.op += 1;
            } else {
                self.page += 1;
                self.op = 0;
            }
        } else {
            self.ready = true;
        }
        Ok(ExactGraphTurn::Compared { work: COMPARE_WORK, ready: self.ready })
    }

    pub fn begin_close(&mut self) { self.ready = false; self.lift.begin_close(); }
    pub fn close_step(&mut self, grant: CosGrant) -> ControlledLiftReceipt { self.lift.close_step(grant) }

    /// ♻️ Returns exact original typed owners once the controlled lift has finished its own close.
    pub fn take_partial(&mut self) -> ControlledLiftPartial {
        let mut partial = self.lift.take_partial();
        if self.projection.is_some() {
            assert!(partial.snapshot.is_none());
            partial.snapshot = self.projection.take();
        }
        partial
    }
}

fn document_lanes(projected: &PdfSnapshot, current: &PdfSnapshot) -> bool {
    let supported = |pdf: &PdfSnapshot| {
        pdf.forms.is_empty() && pdf.ext_g_states.is_empty() && pdf.shadings.is_empty()
            && pdf.patterns.is_empty() && pdf.color_spaces.is_empty() && pdf.properties.is_empty()
            && pdf.outlines.is_empty() && pdf.named_destinations.is_empty() && pdf.page_labels.is_empty()
            && pdf.embedded_files.is_empty() && pdf.output_intents.is_empty() && pdf.acro_form.is_none()
            && pdf.optional_content.is_none() && pdf.viewer_preferences.is_none() && pdf.open_action.is_none()
            && pdf.metadata.is_none() && pdf.catalog_extra.is_empty() && pdf.encryption.is_none() && pdf.info.extra.is_empty()
    };
    supported(projected) && supported(current) && projected.fonts.len() <= FONTS && projected.fonts.iter().all(narrow_font)
        && projected.schema == current.schema && projected.declared_version == current.declared_version
        && projected.pages.len() == current.pages.len() && projected.fonts == current.fonts
        && projected.images.len() == current.images.len()
        && projected.info == current.info && projected.document_id == current.document_id
        && projected.language == current.language && projected.mark_info == current.mark_info
        && projected.page_layout == current.page_layout && projected.page_mode == current.page_mode
}

fn image_header(projected: &PdfImage, current: &PdfImage) -> bool {
    let supported = |image: &PdfImage| {
        image.id.len() <= TEXT_CAP && image.bits_per_component == 8
            && matches!(image.color_space, Some(PdfColorSpace::DeviceGray | PdfColorSpace::DeviceRgb))
            && !image.image_mask && image.decode.is_empty() && image.soft_mask.is_none()
            && image.soft_mask_in_data.is_none() && image.mask.is_none() && image.matte.is_none()
            && image.intent.is_none() && image.optional_content.is_none() && image.struct_parent.is_none()
            && image.extra.is_empty()
    };
    supported(projected) && supported(current) && projected.id == current.id
        && (projected.width, projected.height, projected.interpolate) == (current.width, current.height, current.interpolate)
        && projected.color_space == current.color_space
}

fn compare_image(projected: &PdfImage, current: &PdfImage, offset: &mut usize) -> Result<bool, ExactGraphRefusal> {
    if *offset == 0 && !image_header(projected, current) { return Err(ExactGraphRefusal::Semantic); }
    match (&projected.body, &current.body) {
        (PdfImageBody::Artifact { reference: left }, PdfImageBody::Artifact { reference: right }) if left == right => Ok(true),
        (PdfImageBody::Samples { values: left }, PdfImageBody::Samples { values: right }) if left.len() == right.len() => {
            let end = left.len().min(offset.saturating_add(COMPARE_WORK / size_of::<u32>()));
            if left[*offset..end] != right[*offset..end] { return Err(ExactGraphRefusal::Semantic); }
            *offset = end;
            Ok(end == left.len())
        }
        _ => Err(ExactGraphRefusal::Semantic),
    }
}

fn narrow_font(font: &PdfFont) -> bool {
    font.id.len() <= TEXT_CAP && font.to_unicode.is_none() && font.extra.is_empty()
        && matches!(&font.kind, PdfFontKind::Type1 { base_font, encoding, widths, descriptor: None, program: None, .. }
            if base_font.len() <= TEXT_CAP && encoding.differences.is_empty() && widths.len() <= 32)
}

fn page_lanes(projected: &PdfPage, current: &PdfPage) -> bool {
    let supported = |page: &PdfPage| {
        page.annotations.is_empty() && page.group.is_none() && page.thumbnail.is_none()
            && page.transition.is_none() && page.metadata.is_none() && page.additional_actions.is_empty() && page.extra.is_empty()
    };
    supported(projected) && supported(current) && projected.media_box == current.media_box
        && projected.crop_box == current.crop_box && projected.bleed_box == current.bleed_box
        && projected.trim_box == current.trim_box && projected.art_box == current.art_box
        && projected.rotate == current.rotate && projected.user_unit == current.user_unit
        && projected.struct_parents == current.struct_parents && projected.duration == current.duration
        && projected.content.len() == current.content.len()
}

fn narrow_op(op: &PdfOp) -> bool {
    use PdfOp::*;
    match op {
        SetFont { name, .. } | PaintXObject { name } => name.len() <= TEXT_CAP,
        SetRenderingIntent { intent } => intent.len() <= TEXT_CAP,
        SetDash { array, .. } => array.len() <= TJ_ITEMS,
        ShowText { text } | NextLineShowText { text } | NextLineShowTextSpaced { text, .. } => match text {
            PdfTextString::Codes { codes } => codes.len() <= TEXT_CAP,
            PdfTextString::Text { text } => text.len() <= TEXT_CAP,
        },
        ShowTextArray { items } => items.len() <= TJ_ITEMS && items.iter().all(|item| match item {
            PdfTextArrayItem::Codes { codes } => codes.len() <= TEXT_CAP,
            PdfTextArrayItem::Text { text } => text.len() <= TEXT_CAP,
            PdfTextArrayItem::Adjust { .. } => true,
        }),
        BeginText | EndText | NextLine | SetTextMatrix { .. } | MoveText { .. } | MoveTextSetLeading { .. }
        | SetLeading { .. } | SetWordSpacing { .. } | SetCharSpacing { .. } | SetHorizontalScale { .. }
        | SetTextRise { .. } | SetTextRenderingMode { .. } | Save | Restore | Transform { .. }
        | SetFillGray { .. } | SetFillRgb { .. } | SetFillCmyk { .. }
        | SetStrokeGray { .. } | SetStrokeRgb { .. } | SetStrokeCmyk { .. }
        | SetLineWidth { .. } | SetLineCap { .. } | SetLineJoin { .. } | SetMiterLimit { .. }
        | SetFlatness { .. } | MoveTo { .. } | LineTo { .. } | CurveTo { .. } | CurveToInitial { .. }
        | CurveToFinal { .. } | ClosePath | Rectangle { .. } | Stroke | CloseStroke | Fill | FillEvenOdd
        | FillStroke | FillStrokeEvenOdd | CloseFillStroke | CloseFillStrokeEvenOdd | EndPath | Clip | ClipEvenOdd => true,
        _ => false,
    }
}

struct OverlaySource<'a>(GraphLoan<'a>);

impl ObjectSource for OverlaySource<'_> {
    fn get(&mut self, _reference: ObjRef) -> Option<PdfObject> { None }

    fn inspect(&mut self, reference: ObjRef, read: &mut dyn FnMut(Option<&PdfObject>)) {
        read(self.0.lookup(reference).map(|object| &object.value));
    }
}

use super::classic_graph_quote::GraphLoan;
