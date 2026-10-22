//! 🔌️ Mutation diff and diff algebra: the mutation diff and diff algebra implementations of the presentation diff. See [`super`].

use super::*;

//#region 🔖️Apply
impl MutationDiff<SemioPresentationSnapshot> for SemioPresentationDiff {}

impl protocol::ColdMutationDiff<SemioPresentationSnapshot> for SemioPresentationDiff {
    fn absorb(&mut self, other: Self) {
        self.masters = absorb_opt(self.masters.take(), other.masters, |a, b| absorb_named(a, &b, |m| m.id.clone(), absorb_master_diff, apply_master));
        self.layouts = absorb_opt(self.layouts.take(), other.layouts, |a, b| absorb_named(a, &b, |l| l.id.clone(), absorb_layout_diff, apply_layout));
        self.slides = absorb_opt(self.slides.take(), other.slides, |a, b| absorb_indexed(a, b, absorb_slide_diff, apply_slide_to_copy));
    }

    fn apply(&self, unpublished: &mut SemioPresentationSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let base = &*unpublished;
        let mut next = base.clone();
        if let Some(d) = &self.masters {
            semio_s_artifact_stdio_semio_base::schema::triples::validate_named_triple(&next.masters, d, |item| item.id.clone(), |added| added.item.id.clone(), ["masters"])?;
            apply_named(&mut next.masters, d, |m| m.id.clone(), apply_master);
        }
        if let Some(d) = &self.layouts {
            semio_s_artifact_stdio_semio_base::schema::triples::validate_named_triple(&next.layouts, d, |item| item.id.clone(), |added| added.item.id.clone(), ["layouts"])?;
            apply_named(&mut next.layouts, d, |l| l.id.clone(), apply_layout);
        }
        if let Some(d) = &self.slides {
            semio_s_artifact_stdio_semio_base::schema::triples::validate_indexed_triple(d, next.slides.len(), ["slides"])?;
            apply_indexed(&mut next.slides, d, apply_slide);
        }
        *unpublished = next;
        Ok(())
    }
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<SemioPresentationSnapshot> for SemioPresentationDiff {
    fn is_empty(&self) -> bool {
        self.masters.is_none() && self.layouts.is_none() && self.slides.is_none()
    }
}

impl protocol::ColdDiffAlgebra<SemioPresentationSnapshot> for SemioPresentationDiff {
    fn inverse(&self, base: &SemioPresentationSnapshot) -> Self {
        SemioPresentationDiff {
            masters: self.masters.as_ref().map(|d| inverse_named(&base.masters, d, |m| m.id.clone(), inverse_master)),
            layouts: self.layouts.as_ref().map(|d| inverse_named(&base.layouts, d, |l| l.id.clone(), inverse_layout)),
            slides: self.slides.as_ref().map(|d| inverse_indexed(&base.slides, d, inverse_slide)),
        }
    }
}
//#endregion 🔖️DiffAlgebra
