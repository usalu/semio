//! 🔷️ Shape and table logic: applying, inverting and absorbing shape, table row and table cell diffs with their copy variants. See [`super`].

use super::*;

//#region 🔖️ValueDiffLogic
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn apply_shape(shape: &mut SlideShape, diff: &SlideShapeDiff) {
    match diff {
        SlideShapeDiff::Replace { shape: new } => *shape = new.clone(),
        SlideShapeDiff::TextBox { frame, blocks } => {
            if let SlideShape::TextBox { frame: f, blocks: b } = shape {
                if let Some(fd) = frame {
                    apply_frame(f, fd);
                }
                if let Some(bd) = blocks {
                    apply_indexed(b, bd, apply_doc_block);
                }
            }
        }
        SlideShapeDiff::Picture { frame, image } => {
            if let SlideShape::Picture { frame: f, image: i } = shape {
                if let Some(fd) = frame {
                    apply_frame(f, fd);
                }
                if let Some(id) = image {
                    apply_image(i, id);
                }
            }
        }
        SlideShapeDiff::Table { frame, rows } => {
            if let SlideShape::Table { frame: f, rows: r } = shape {
                if let Some(fd) = frame {
                    apply_frame(f, fd);
                }
                if let Some(rd) = rows {
                    apply_indexed(r, rd, apply_table_row);
                }
            }
        }
        SlideShapeDiff::Placeholder { frame, kind } => {
            if let SlideShape::Placeholder { frame: f, kind: k } = shape {
                if let Some(fd) = frame {
                    apply_frame(f, fd);
                }
                if let Some(kd) = kind {
                    *k = kd.clone();
                }
            }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_table_row(row: &mut SlideTableRow, diff: &SlideTableRowDiff) {
    if let Some(cd) = &diff.cells {
        apply_indexed(&mut row.cells, cd, apply_table_cell);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_table_cell(cell: &mut SlideTableCell, diff: &SlideTableCellDiff) {
    if let Some(bd) = &diff.blocks {
        apply_indexed(&mut cell.blocks, bd, apply_doc_block);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_table_cell_to_copy(cell: &SlideTableCell, diff: &SlideTableCellDiff) -> SlideTableCell {
    let mut out = cell.clone();
    apply_table_cell(&mut out, diff);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn apply_shape_to_copy(shape: &SlideShape, diff: &SlideShapeDiff) -> SlideShape {
    let mut out = shape.clone();
    apply_shape(&mut out, diff);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_table_row_to_copy(row: &SlideTableRow, diff: &SlideTableRowDiff) -> SlideTableRow {
    let mut out = row.clone();
    apply_table_row(&mut out, diff);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn inverse_shape(base: &SlideShape, diff: &SlideShapeDiff) -> SlideShapeDiff {
    match diff {
        SlideShapeDiff::Replace { .. } => SlideShapeDiff::Replace { shape: base.clone() },
        SlideShapeDiff::TextBox { frame, blocks } => {
            let SlideShape::TextBox { frame: bf, blocks: bb } = base else { return SlideShapeDiff::Replace { shape: base.clone() } };
            SlideShapeDiff::TextBox { frame: frame.as_ref().map(|fd| inverse_frame(bf, fd)), blocks: blocks.as_ref().map(|bd| inverse_indexed(bb, bd, inverse_doc_block)) }
        }
        SlideShapeDiff::Picture { frame, image } => {
            let SlideShape::Picture { frame: bf, image: bi } = base else { return SlideShapeDiff::Replace { shape: base.clone() } };
            SlideShapeDiff::Picture { frame: frame.as_ref().map(|fd| inverse_frame(bf, fd)), image: image.as_ref().map(|id| inverse_image(bi, id)) }
        }
        SlideShapeDiff::Table { frame, rows } => {
            let SlideShape::Table { frame: bf, rows: br } = base else { return SlideShapeDiff::Replace { shape: base.clone() } };
            SlideShapeDiff::Table { frame: frame.as_ref().map(|fd| inverse_frame(bf, fd)), rows: rows.as_ref().map(|rd| inverse_indexed(br, rd, inverse_table_row)) }
        }
        SlideShapeDiff::Placeholder { frame, kind } => {
            let SlideShape::Placeholder { frame: bf, kind: bk } = base else { return SlideShapeDiff::Replace { shape: base.clone() } };
            SlideShapeDiff::Placeholder { frame: frame.as_ref().map(|fd| inverse_frame(bf, fd)), kind: kind.as_ref().map(|_| bk.clone()) }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_table_row(base: &SlideTableRow, diff: &SlideTableRowDiff) -> SlideTableRowDiff {
    SlideTableRowDiff { cells: diff.cells.as_ref().map(|cd| inverse_indexed(&base.cells, cd, inverse_table_cell)) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_table_cell(base: &SlideTableCell, diff: &SlideTableCellDiff) -> SlideTableCellDiff {
    SlideTableCellDiff { blocks: diff.blocks.as_ref().map(|bd| inverse_indexed(&base.blocks, bd, inverse_doc_block)) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn absorb_shape(a: SlideShapeDiff, b: SlideShapeDiff) -> SlideShapeDiff {
    match (a, b) {
        (_, SlideShapeDiff::Replace { shape }) => SlideShapeDiff::Replace { shape },
        (SlideShapeDiff::Replace { shape }, b) => SlideShapeDiff::Replace { shape: apply_shape_to_copy(&shape, &b) },
        (SlideShapeDiff::TextBox { frame: fa, blocks: ba }, SlideShapeDiff::TextBox { frame: fb, blocks: bb }) => {
            SlideShapeDiff::TextBox { frame: absorb_opt(fa, fb, |a, b| absorb_frame(a, &b)), blocks: absorb_opt(ba, bb, |x, y| absorb_indexed(x, y, absorb_doc_block, apply_doc_block_to_copy)) }
        }
        (SlideShapeDiff::Picture { frame: fa, image: ia }, SlideShapeDiff::Picture { frame: fb, image: ib }) => SlideShapeDiff::Picture { frame: absorb_opt(fa, fb, |a, b| absorb_frame(a, &b)), image: absorb_opt(ia, ib, absorb_image) },
        (SlideShapeDiff::Table { frame: fa, rows: ra }, SlideShapeDiff::Table { frame: fb, rows: rb }) => {
            SlideShapeDiff::Table { frame: absorb_opt(fa, fb, |a, b| absorb_frame(a, &b)), rows: absorb_opt(ra, rb, |x, y| absorb_indexed(x, y, absorb_table_row_diff, apply_table_row_to_copy)) }
        }
        (SlideShapeDiff::Placeholder { frame: fa, kind: ka }, SlideShapeDiff::Placeholder { frame: fb, kind: kb }) => SlideShapeDiff::Placeholder { frame: absorb_opt(fa, fb, |a, b| absorb_frame(a, &b)), kind: kb.or(ka) },
        (_, b) => b,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_table_cell_diff(mut a: SlideTableCellDiff, b: SlideTableCellDiff) -> SlideTableCellDiff {
    a.blocks = absorb_opt(a.blocks.take(), b.blocks, |x, y| absorb_indexed(x, y, absorb_doc_block, apply_doc_block_to_copy));
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_table_row_diff(mut a: SlideTableRowDiff, b: SlideTableRowDiff) -> SlideTableRowDiff {
    a.cells = absorb_opt(a.cells.take(), b.cells, |x, y| {
        absorb_indexed(x, y, absorb_table_cell_diff, apply_table_cell_to_copy)
    });
    a
}
//#endregion 🔖️StructureDiffLogic
