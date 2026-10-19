//! 🖊️ Document diff logic and algebra: diffing, applying, inverting and absorbing runs, styles, images and blocks, and the mutation diff and diff algebra implementations of the document diff. See [`super`].

use super::*;

//#region 🔖️DocumentDiffLogic
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_run(old: &DocRun, new: &DocRun) -> Option<DocRunDiff> {
    if old == new {
        return None;
    }
    Some(DocRunDiff { text: (old.text != new.text).then(|| new.text.clone()), style: diff_run_style(&old.style, &new.style) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_run_style(old: &RunStyle, new: &RunStyle) -> Option<RunStyleDiff> {
    if old == new {
        return None;
    }
    Some(RunStyleDiff {
        bold: (old.bold != new.bold).then_some(new.bold),
        italic: (old.italic != new.italic).then_some(new.italic),
        underline: (old.underline != new.underline).then_some(new.underline),
        size: (old.size != new.size).then_some(new.size),
        font: (old.font != new.font).then(|| new.font.clone()),
        color: (old.color != new.color).then(|| new.color.clone()),
        link: (old.link != new.link).then(|| new.link.clone()),
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_style(old: &DocStyle, new: &DocStyle) -> Option<DocStyleDiff> {
    if old == new {
        return None;
    }
    Some(DocStyleDiff { name: (old.name != new.name).then(|| new.name.clone()), based_on: (old.based_on != new.based_on).then(|| new.based_on.clone()) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_image(old: &DocImage, new: &DocImage) -> Option<DocImageDiff> {
    if old == new {
        return None;
    }
    Some(DocImageDiff { mime: (old.mime != new.mime).then(|| new.mime.clone()), bytes: (old.bytes != new.bytes).then(|| new.bytes.clone()) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_run(run: &mut DocRun, diff: &DocRunDiff) {
    if let Some(v) = &diff.text {
        run.text = v.clone();
    }
    if let Some(sd) = &diff.style {
        apply_run_style(&mut run.style, sd);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_run_style(style: &mut RunStyle, diff: &RunStyleDiff) {
    if let Some(v) = diff.bold {
        style.bold = v;
    }
    if let Some(v) = diff.italic {
        style.italic = v;
    }
    if let Some(v) = diff.underline {
        style.underline = v;
    }
    if let Some(v) = &diff.size {
        style.size = *v;
    }
    if let Some(v) = &diff.font {
        style.font = v.clone();
    }
    if let Some(v) = &diff.color {
        style.color = v.clone();
    }
    if let Some(v) = &diff.link {
        style.link = v.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_list_item(item: &mut DocListItem, diff: &DocListItemDiff) {
    if let Some(bd) = &diff.blocks {
        apply_indexed(&mut item.blocks, bd, apply_block);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_cell(cell: &mut DocTableCell, diff: &DocTableCellDiff) {
    if let Some(bd) = &diff.blocks {
        apply_indexed(&mut cell.blocks, bd, apply_block);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_row(row: &mut DocTableRow, diff: &DocTableRowDiff) {
    if let Some(cd) = &diff.cells {
        apply_indexed(&mut row.cells, cd, apply_cell);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_style(style: &mut DocStyle, diff: &DocStyleDiff) {
    if let Some(v) = &diff.name {
        style.name = v.clone();
    }
    if let Some(v) = &diff.based_on {
        style.based_on = v.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_image(image: &mut DocImage, diff: &DocImageDiff) {
    if let Some(v) = &diff.mime {
        image.mime = v.clone();
    }
    if let Some(v) = &diff.bytes {
        image.bytes = v.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_block(block: &mut DocBlock, diff: &DocBlockDiff) {
    match diff {
        DocBlockDiff::Replace { block: new } => *block = new.clone(),
        DocBlockDiff::Paragraph(pd) => {
            if let DocBlock::Paragraph { style_id, runs } = block {
                if let Some(s) = &pd.style_id {
                    *style_id = s.clone();
                }
                if let Some(rd) = &pd.runs {
                    apply_indexed(runs, rd, apply_run);
                }
            }
        }
        DocBlockDiff::Heading(hd) => {
            if let DocBlock::Heading { level, style_id, runs } = block {
                if let Some(l) = hd.level {
                    *level = l;
                }
                if let Some(s) = &hd.style_id {
                    *style_id = s.clone();
                }
                if let Some(rd) = &hd.runs {
                    apply_indexed(runs, rd, apply_run);
                }
            }
        }
        DocBlockDiff::List(ld) => {
            if let DocBlock::List { ordered, items } = block {
                if let Some(o) = ld.ordered {
                    *ordered = o;
                }
                if let Some(id) = &ld.items {
                    apply_indexed(items, id, apply_list_item);
                }
            }
        }
        DocBlockDiff::Table(td) => {
            if let DocBlock::Table { rows } = block {
                if let Some(rd) = &td.rows {
                    apply_indexed(rows, rd, apply_row);
                }
            }
        }
        DocBlockDiff::Code(cd) => {
            if let DocBlock::Code { language, text } = block {
                if let Some(l) = &cd.language {
                    *language = l.clone();
                }
                if let Some(t) = &cd.text {
                    *text = t.clone();
                }
            }
        }
        DocBlockDiff::Quote(qd) => {
            if let DocBlock::Quote { blocks } = block {
                if let Some(bd) = &qd.blocks {
                    apply_indexed(blocks, bd, apply_block);
                }
            }
        }
        DocBlockDiff::Image(id) => {
            if let DocBlock::Image { image_id, alt, width, height } = block {
                if let Some(v) = &id.image_id {
                    *image_id = v.clone();
                }
                if let Some(v) = &id.alt {
                    *alt = v.clone();
                }
                if let Some(v) = &id.width {
                    *width = *v;
                }
                if let Some(v) = &id.height {
                    *height = *v;
                }
            }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_block_to_copy(block: &DocBlock, diff: &DocBlockDiff) -> DocBlock {
    let mut out = block.clone();
    apply_block(&mut out, diff);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_run_to_copy(run: &DocRun, diff: &DocRunDiff) -> DocRun {
    let mut out = run.clone();
    apply_run(&mut out, diff);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_list_item_to_copy(item: &DocListItem, diff: &DocListItemDiff) -> DocListItem {
    let mut out = item.clone();
    apply_list_item(&mut out, diff);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_row_to_copy(row: &DocTableRow, diff: &DocTableRowDiff) -> DocTableRow {
    let mut out = row.clone();
    apply_row(&mut out, diff);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_cell_to_copy(cell: &DocTableCell, diff: &DocTableCellDiff) -> DocTableCell {
    let mut out = cell.clone();
    apply_cell(&mut out, diff);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_run(base: &DocRun, diff: &DocRunDiff) -> DocRunDiff {
    DocRunDiff {
        text: diff.text.as_ref().map(|_| base.text.clone()),
        style: diff.style.as_ref().map(|_| RunStyleDiff {
            bold: Some(base.style.bold),
            italic: Some(base.style.italic),
            underline: Some(base.style.underline),
            size: Some(base.style.size),
            font: Some(base.style.font.clone()),
            color: Some(base.style.color.clone()),
            link: Some(base.style.link.clone()),
        }),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_list_item(base: &DocListItem, diff: &DocListItemDiff) -> DocListItemDiff {
    DocListItemDiff { blocks: diff.blocks.as_ref().map(|bd| inverse_indexed(&base.blocks, bd, inverse_block)) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_cell(base: &DocTableCell, diff: &DocTableCellDiff) -> DocTableCellDiff {
    DocTableCellDiff { blocks: diff.blocks.as_ref().map(|bd| inverse_indexed(&base.blocks, bd, inverse_block)) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_row(base: &DocTableRow, diff: &DocTableRowDiff) -> DocTableRowDiff {
    DocTableRowDiff { cells: diff.cells.as_ref().map(|cd| inverse_indexed(&base.cells, cd, inverse_cell)) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_style(base: &DocStyle, diff: &DocStyleDiff) -> DocStyleDiff {
    DocStyleDiff { name: diff.name.as_ref().map(|_| base.name.clone()), based_on: diff.based_on.as_ref().map(|_| base.based_on.clone()) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_image(base: &DocImage, diff: &DocImageDiff) -> DocImageDiff {
    DocImageDiff { mime: diff.mime.as_ref().map(|_| base.mime.clone()), bytes: diff.bytes.as_ref().map(|_| base.bytes.clone()) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_block(base: &DocBlock, diff: &DocBlockDiff) -> DocBlockDiff {
    match diff {
        DocBlockDiff::Replace { .. } => DocBlockDiff::Replace { block: base.clone() },
        DocBlockDiff::Paragraph(pd) => {
            let DocBlock::Paragraph { style_id, runs } = base else { return DocBlockDiff::Replace { block: base.clone() } };
            DocBlockDiff::Paragraph(DocParagraphDiff { style_id: pd.style_id.as_ref().map(|_| style_id.clone()), runs: pd.runs.as_ref().map(|rd| inverse_indexed(runs, rd, inverse_run)) })
        }
        DocBlockDiff::Heading(hd) => {
            let DocBlock::Heading { level, style_id, runs } = base else { return DocBlockDiff::Replace { block: base.clone() } };
            DocBlockDiff::Heading(DocHeadingDiff { level: hd.level.map(|_| *level), style_id: hd.style_id.as_ref().map(|_| style_id.clone()), runs: hd.runs.as_ref().map(|rd| inverse_indexed(runs, rd, inverse_run)) })
        }
        DocBlockDiff::List(ld) => {
            let DocBlock::List { ordered, items } = base else { return DocBlockDiff::Replace { block: base.clone() } };
            DocBlockDiff::List(DocListDiff { ordered: ld.ordered.map(|_| *ordered), items: ld.items.as_ref().map(|id| inverse_indexed(items, id, inverse_list_item)) })
        }
        DocBlockDiff::Table(td) => {
            let DocBlock::Table { rows } = base else { return DocBlockDiff::Replace { block: base.clone() } };
            DocBlockDiff::Table(DocTableDiff { rows: td.rows.as_ref().map(|rd| inverse_indexed(rows, rd, inverse_row)) })
        }
        DocBlockDiff::Code(cd) => {
            let DocBlock::Code { language, text } = base else { return DocBlockDiff::Replace { block: base.clone() } };
            DocBlockDiff::Code(DocCodeDiff { language: cd.language.as_ref().map(|_| language.clone()), text: cd.text.as_ref().map(|_| text.clone()) })
        }
        DocBlockDiff::Quote(qd) => {
            let DocBlock::Quote { blocks } = base else { return DocBlockDiff::Replace { block: base.clone() } };
            DocBlockDiff::Quote(DocQuoteDiff { blocks: qd.blocks.as_ref().map(|bd| inverse_indexed(blocks, bd, inverse_block)) })
        }
        DocBlockDiff::Image(id) => {
            let DocBlock::Image { image_id, alt, width, height } = base else { return DocBlockDiff::Replace { block: base.clone() } };
            DocBlockDiff::Image(DocImageBlockDiff { image_id: id.image_id.as_ref().map(|_| image_id.clone()), alt: id.alt.as_ref().map(|_| alt.clone()), width: id.width.as_ref().map(|_| *width), height: id.height.as_ref().map(|_| *height) })
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_run_style_diff(a: RunStyleDiff, b: RunStyleDiff) -> RunStyleDiff {
    RunStyleDiff { bold: b.bold.or(a.bold), italic: b.italic.or(a.italic), underline: b.underline.or(a.underline), size: b.size.or(a.size), font: b.font.or(a.font), color: b.color.or(a.color), link: b.link.or(a.link) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_run_diff(a: DocRunDiff, b: DocRunDiff) -> DocRunDiff {
    DocRunDiff {
        text: b.text.or(a.text),
        style: match (a.style, b.style) {
            (None, x) => x,
            (x, None) => x,
            (Some(sa), Some(sb)) => Some(absorb_run_style_diff(sa, sb)),
        },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_list_item_diff(mut a: DocListItemDiff, b: DocListItemDiff) -> DocListItemDiff {
    a.blocks = match (a.blocks.take(), b.blocks) {
        (None, x) => x,
        (x, None) => x,
        (Some(ba), Some(bb)) => Some(absorb_indexed(ba, &bb, absorb_block_diff, apply_block_to_copy)),
    };
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_cell_diff(mut a: DocTableCellDiff, b: DocTableCellDiff) -> DocTableCellDiff {
    a.blocks = match (a.blocks.take(), b.blocks) {
        (None, x) => x,
        (x, None) => x,
        (Some(ba), Some(bb)) => Some(absorb_indexed(ba, &bb, absorb_block_diff, apply_block_to_copy)),
    };
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_row_diff(mut a: DocTableRowDiff, b: DocTableRowDiff) -> DocTableRowDiff {
    a.cells = match (a.cells.take(), b.cells) {
        (None, x) => x,
        (x, None) => x,
        (Some(ca), Some(cb)) => Some(absorb_indexed(ca, &cb, absorb_cell_diff, apply_cell_to_copy)),
    };
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_style_diff(mut a: DocStyleDiff, b: DocStyleDiff) -> DocStyleDiff {
    if b.name.is_some() {
        a.name = b.name;
    }
    if b.based_on.is_some() {
        a.based_on = b.based_on;
    }
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_image_diff(mut a: DocImageDiff, b: DocImageDiff) -> DocImageDiff {
    if b.mime.is_some() {
        a.mime = b.mime;
    }
    if b.bytes.is_some() {
        a.bytes = b.bytes;
    }
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_block_diff(a: DocBlockDiff, b: DocBlockDiff) -> DocBlockDiff {
    match (a, b) {
        (_, DocBlockDiff::Replace { block }) => DocBlockDiff::Replace { block },
        (DocBlockDiff::Replace { block }, b) => DocBlockDiff::Replace { block: apply_block_to_copy(&block, &b) },
        (DocBlockDiff::Paragraph(mut pa), DocBlockDiff::Paragraph(pb)) => {
            if pb.style_id.is_some() {
                pa.style_id = pb.style_id;
            }
            pa.runs = match (pa.runs.take(), pb.runs) {
                (None, x) => x,
                (x, None) => x,
                (Some(ra), Some(rb)) => Some(absorb_indexed(ra, &rb, absorb_run_diff, apply_run_to_copy)),
            };
            DocBlockDiff::Paragraph(pa)
        }
        (DocBlockDiff::Heading(mut ha), DocBlockDiff::Heading(hb)) => {
            if hb.level.is_some() {
                ha.level = hb.level;
            }
            if hb.style_id.is_some() {
                ha.style_id = hb.style_id;
            }
            ha.runs = match (ha.runs.take(), hb.runs) {
                (None, x) => x,
                (x, None) => x,
                (Some(ra), Some(rb)) => Some(absorb_indexed(ra, &rb, absorb_run_diff, apply_run_to_copy)),
            };
            DocBlockDiff::Heading(ha)
        }
        (DocBlockDiff::List(mut la), DocBlockDiff::List(lb)) => {
            if lb.ordered.is_some() {
                la.ordered = lb.ordered;
            }
            la.items = match (la.items.take(), lb.items) {
                (None, x) => x,
                (x, None) => x,
                (Some(ia), Some(ib)) => Some(absorb_indexed(ia, &ib, absorb_list_item_diff, apply_list_item_to_copy)),
            };
            DocBlockDiff::List(la)
        }
        (DocBlockDiff::Table(mut ta), DocBlockDiff::Table(tb)) => {
            ta.rows = match (ta.rows.take(), tb.rows) {
                (None, x) => x,
                (x, None) => x,
                (Some(ra), Some(rb)) => Some(absorb_indexed(ra, &rb, absorb_row_diff, apply_row_to_copy)),
            };
            DocBlockDiff::Table(ta)
        }
        (DocBlockDiff::Code(mut ca), DocBlockDiff::Code(cb)) => {
            if cb.language.is_some() {
                ca.language = cb.language;
            }
            if cb.text.is_some() {
                ca.text = cb.text;
            }
            DocBlockDiff::Code(ca)
        }
        (DocBlockDiff::Quote(mut qa), DocBlockDiff::Quote(qb)) => {
            qa.blocks = match (qa.blocks.take(), qb.blocks) {
                (None, x) => x,
                (x, None) => x,
                (Some(ba), Some(bb)) => Some(absorb_indexed(ba, &bb, absorb_block_diff, apply_block_to_copy)),
            };
            DocBlockDiff::Quote(qa)
        }
        (DocBlockDiff::Image(mut ia), DocBlockDiff::Image(ib)) => {
            if ib.image_id.is_some() {
                ia.image_id = ib.image_id;
            }
            if ib.alt.is_some() {
                ia.alt = ib.alt;
            }
            if ib.width.is_some() {
                ia.width = ib.width;
            }
            if ib.height.is_some() {
                ia.height = ib.height;
            }
            DocBlockDiff::Image(ia)
        }
        (_, b) => b,
    }
}

impl MutationDiff<SemioDocumentSnapshot> for SemioDocumentDiff {
    fn apply(&self, base: &SemioDocumentSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SemioDocumentSnapshot> {
        let mut out = base.clone();
        if let Some(sd) = &self.styles {
            semio_s_artifact_stdio_semio_base::schema::triples::validate_named_triple(&out.styles, sd, |item| item.id.clone(), |added| added.item.id.clone(), ["styles"])?;
            apply_named(&mut out.styles, sd, |s| s.id.clone(), apply_style);
        }
        if let Some(id) = &self.images {
            semio_s_artifact_stdio_semio_base::schema::triples::validate_named_triple(&out.images, id, |item| item.id.clone(), |added| added.item.id.clone(), ["images"])?;
            apply_named(&mut out.images, id, |i| i.id.clone(), apply_image);
        }
        if let Some(bd) = &self.blocks {
            semio_s_artifact_stdio_semio_base::schema::triples::validate_indexed_triple(bd, out.blocks.len(), ["blocks"])?;
            apply_indexed(&mut out.blocks, bd, apply_block);
        }
        Ok(out)
    }

    fn absorb(&mut self, other: Self) {
        let styles = std::mem::take(&mut self.styles);
        self.styles = match (styles, other.styles) {
            (None, x) => x,
            (x, None) => x,
            (Some(sa), Some(sb)) => Some(absorb_named(sa, &sb, |s| s.id.clone(), absorb_style_diff, apply_style)),
        };
        let images = std::mem::take(&mut self.images);
        self.images = match (images, other.images) {
            (None, x) => x,
            (x, None) => x,
            (Some(ia), Some(ib)) => Some(absorb_named(ia, &ib, |i| i.id.clone(), absorb_image_diff, apply_image)),
        };
        let blocks = std::mem::take(&mut self.blocks);
        self.blocks = match (blocks, other.blocks) {
            (None, x) => x,
            (x, None) => x,
            (Some(ba), Some(bb)) => Some(absorb_indexed(ba, &bb, absorb_block_diff, apply_block_to_copy)),
        };
    }
}

impl DiffAlgebra<SemioDocumentSnapshot> for SemioDocumentDiff {
    fn inverse(&self, base: &SemioDocumentSnapshot) -> Self {
        SemioDocumentDiff {
            styles: self.styles.as_ref().map(|sd| inverse_named(&base.styles, sd, |s| s.id.clone(), inverse_style)),
            images: self.images.as_ref().map(|id| inverse_named(&base.images, id, |i| i.id.clone(), inverse_image)),
            blocks: self.blocks.as_ref().map(|bd| inverse_indexed(&base.blocks, bd, inverse_block)),
        }
    }
    fn is_empty(&self) -> bool {
        self.styles.is_none() && self.images.is_none() && self.blocks.is_none()
    }
}
//#endregion 🔖️DocumentDiffLogic
