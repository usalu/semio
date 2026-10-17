//! 📄️ The per-page patch, the document information patch and the pages triple with its apply, absorb and inverse laws.

use super::*;

//#region 🔖️PageDiff
/// 📄️ Sparse patch for one `PdfPage`: scalar fields are set/tri-state, `content` and
/// `annotations` are index-keyed triples, `extra` is a name-keyed dictionary patch.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPageDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub media_box: Option<PdfRect>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub crop_box: Option<PdfSet<PdfRect>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bleed_box: Option<PdfSet<PdfRect>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub trim_box: Option<PdfSet<PdfRect>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub art_box: Option<PdfSet<PdfRect>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rotate: Option<i32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub user_unit: Option<PdfSet<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<PdfIndexedDiff<PdfOp>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<PdfIndexedDiff<PdfAnnotation>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<PdfSet<PdfTransparencyGroup>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PdfSet<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub struct_parents: Option<PdfSet<u32>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub transition: Option<PdfSet<Vec<PdfDictEntry>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<PdfSet<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PdfSet<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub additional_actions: Option<Vec<PdfDictEntry>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<PdfDictDiff>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn apply_page_diff(mut page: PdfPage, diff: &PdfPageDiff) -> PdfPage {
    if let Some(v) = diff.media_box {
        page.media_box = v;
    }
    apply_tri(&mut page.crop_box, &diff.crop_box);
    apply_tri(&mut page.bleed_box, &diff.bleed_box);
    apply_tri(&mut page.trim_box, &diff.trim_box);
    apply_tri(&mut page.art_box, &diff.art_box);
    if let Some(v) = diff.rotate {
        page.rotate = v;
    }
    apply_tri(&mut page.user_unit, &diff.user_unit);
    if let Some(content) = &diff.content {
        page.content = content.apply(&page.content);
    }
    if let Some(annotations) = &diff.annotations {
        page.annotations = annotations.apply(&page.annotations);
    }
    apply_tri(&mut page.group, &diff.group);
    apply_tri(&mut page.thumbnail, &diff.thumbnail);
    apply_tri(&mut page.struct_parents, &diff.struct_parents);
    apply_tri(&mut page.transition, &diff.transition);
    apply_tri(&mut page.duration, &diff.duration);
    apply_tri(&mut page.metadata, &diff.metadata);
    if let Some(v) = &diff.additional_actions {
        page.additional_actions = v.clone();
    }
    if let Some(extra) = &diff.extra {
        page.extra = apply_dict_diff(extra, &page.extra);
    }
    page
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn validate_page_diff(diff: &PdfPageDiff, base: &PdfPage) -> MutationApplyResult<()> {
    if let Some(content) = &diff.content {
        content.validate(base.content.len(), "content")?;
    }
    if let Some(annotations) = &diff.annotations {
        annotations.validate(base.annotations.len(), "annotations")?;
    }
    if let Some(extra) = &diff.extra {
        validate_dict_diff(extra, &base.extra)?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn absorb_page_diff(base: PdfPageDiff, other: PdfPageDiff) -> PdfPageDiff {
    PdfPageDiff {
        media_box: other.media_box.or(base.media_box),
        crop_box: other.crop_box.or(base.crop_box),
        bleed_box: other.bleed_box.or(base.bleed_box),
        trim_box: other.trim_box.or(base.trim_box),
        art_box: other.art_box.or(base.art_box),
        rotate: other.rotate.or(base.rotate),
        user_unit: other.user_unit.or(base.user_unit),
        content: match (base.content, other.content) {
            (None, later) => later,
            (earlier, None) => earlier,
            (Some(earlier), Some(later)) => Some(earlier.absorb(later)).filter(|merged| !merged.is_empty()),
        },
        annotations: match (base.annotations, other.annotations) {
            (None, later) => later,
            (earlier, None) => earlier,
            (Some(earlier), Some(later)) => Some(earlier.absorb(later)).filter(|merged| !merged.is_empty()),
        },
        group: other.group.or(base.group),
        thumbnail: other.thumbnail.or(base.thumbnail),
        struct_parents: other.struct_parents.or(base.struct_parents),
        transition: other.transition.or(base.transition),
        duration: other.duration.or(base.duration),
        metadata: other.metadata.or(base.metadata),
        additional_actions: other.additional_actions.or(base.additional_actions),
        extra: match (base.extra, other.extra) {
            (None, later) => later,
            (earlier, None) => earlier,
            (Some(earlier), Some(later)) => Some(absorb_dict_diff(earlier, later)).filter(|merged| !merged.is_empty()),
        },
    }
}
/// ↩️ The negative patch for `diff` over `page`: every field the patch names is set back to the value `page` holds.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn inverse_page_diff(diff: &PdfPageDiff, page: &PdfPage) -> PdfPageDiff {
    PdfPageDiff {
        media_box: diff.media_box.map(|_| page.media_box),
        crop_box: diff.crop_box.as_ref().map(|_| PdfSet::from_option(&page.crop_box)),
        bleed_box: diff.bleed_box.as_ref().map(|_| PdfSet::from_option(&page.bleed_box)),
        trim_box: diff.trim_box.as_ref().map(|_| PdfSet::from_option(&page.trim_box)),
        art_box: diff.art_box.as_ref().map(|_| PdfSet::from_option(&page.art_box)),
        rotate: diff.rotate.map(|_| page.rotate),
        user_unit: diff.user_unit.as_ref().map(|_| PdfSet::from_option(&page.user_unit)),
        content: diff.content.as_ref().map(|content| content.inverse(&page.content)),
        annotations: diff.annotations.as_ref().map(|annotations| annotations.inverse(&page.annotations)),
        group: diff.group.as_ref().map(|_| PdfSet::from_option(&page.group)),
        thumbnail: diff.thumbnail.as_ref().map(|_| PdfSet::from_option(&page.thumbnail)),
        struct_parents: diff.struct_parents.as_ref().map(|_| PdfSet::from_option(&page.struct_parents)),
        transition: diff.transition.as_ref().map(|_| PdfSet::from_option(&page.transition)),
        duration: diff.duration.as_ref().map(|_| PdfSet::from_option(&page.duration)),
        metadata: diff.metadata.as_ref().map(|_| PdfSet::from_option(&page.metadata)),
        additional_actions: diff.additional_actions.as_ref().map(|_| page.additional_actions.clone()),
        extra: diff.extra.as_ref().map(|extra| inverse_dict_diff(extra, &page.extra)),
    }
}
//#endregion 🔖️PageDiff

//#region 🔖️InfoDiff
/// 📇️ Sparse patch for the document `/Info` dictionary: every scalar field is tri-state (`Clear`/`Set`), `extra` is replaced
/// whole when it moves.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfInfoDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<PdfSet<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<PdfSet<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<PdfSet<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<PdfSet<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<PdfSet<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub producer: Option<PdfSet<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<PdfSet<PdfDate>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub modification_date: Option<PdfSet<PdfDate>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub trapped: Option<PdfSet<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Vec<PdfDictEntry>>,
}

impl PdfInfoDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self == &PdfInfoDiff::default()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &PdfInfo) -> PdfInfo {
        let mut next = base.clone();
        apply_tri(&mut next.title, &self.title);
        apply_tri(&mut next.author, &self.author);
        apply_tri(&mut next.subject, &self.subject);
        apply_tri(&mut next.keywords, &self.keywords);
        apply_tri(&mut next.creator, &self.creator);
        apply_tri(&mut next.producer, &self.producer);
        apply_tri(&mut next.creation_date, &self.creation_date);
        apply_tri(&mut next.modification_date, &self.modification_date);
        apply_tri(&mut next.trapped, &self.trapped);
        if let Some(extra) = &self.extra {
            next.extra = extra.clone();
        }
        next
    }

    /// ↩️ The negative patch: every field this one names is set back to what `base` holds.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn inverse(&self, base: &PdfInfo) -> Self {
        Self {
            title: self.title.as_ref().map(|_| PdfSet::from_option(&base.title)),
            author: self.author.as_ref().map(|_| PdfSet::from_option(&base.author)),
            subject: self.subject.as_ref().map(|_| PdfSet::from_option(&base.subject)),
            keywords: self.keywords.as_ref().map(|_| PdfSet::from_option(&base.keywords)),
            creator: self.creator.as_ref().map(|_| PdfSet::from_option(&base.creator)),
            producer: self.producer.as_ref().map(|_| PdfSet::from_option(&base.producer)),
            creation_date: self.creation_date.as_ref().map(|_| PdfSet::from_option(&base.creation_date)),
            modification_date: self.modification_date.as_ref().map(|_| PdfSet::from_option(&base.modification_date)),
            trapped: self.trapped.as_ref().map(|_| PdfSet::from_option(&base.trapped)),
            extra: self.extra.as_ref().map(|_| base.extra.clone()),
        }
    }

    /// ➕️ Sequential coalesce: a later write of a field wins.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn absorb(&mut self, other: Self) {
        macro_rules! later {
            ($($field:ident),*) => {
                $(if other.$field.is_some() {
                    self.$field = other.$field;
                })*
            };
        }
        later!(title, author, subject, keywords, creator, producer, creation_date, modification_date, trapped, extra);
    }
}
//#endregion 🔖️InfoDiff

//#region 🔖️PagesTriple
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPageModified {
    pub index: usize,
    pub diff: PdfPageDiff,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPageAdded {
    pub index: usize,
    pub page: PdfPage,
}

/// 📦️ Index-keyed `pages` triple (positional -- the recipe's "index usize" key kind).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPagesDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<PdfPageModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<PdfPageAdded>,
}

impl PdfPagesDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// ▶️ Apply semantics (normative): `removed`/`modified` indices refer to BASE state (removals
/// processed descending); `added` indices refer to FINAL state (ascending insert).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn apply_pages_diff(diff: &PdfPagesDiff, base: &[PdfPage]) -> Vec<PdfPage> {
    let mut next: Vec<PdfPage> = base.to_vec();
    for modified in &diff.modified {
        if let Some(slot) = next.get_mut(modified.index) {
            *slot = apply_page_diff(std::mem::take(slot), &modified.diff);
        }
    }
    let mut removed = diff.removed.clone();
    removed.sort_unstable_by(|a, b| b.cmp(a));
    for index in removed {
        if index < next.len() {
            next.remove(index);
        }
    }
    let mut added = diff.added.clone();
    added.sort_by_key(|item| item.index);
    for item in added {
        next.insert(item.index.min(next.len()), item.page);
    }
    next
}

/// ➕️ Sequential coalesce of two page triples: `d1`'s modified pages absorb `d2`'s patches
/// where both touch a page that survives, `d2`'s removals of `d1`'s additions cancel, and the
/// rest is transported through `d1`'s index moves via the same virtual-base slot algorithm the
/// generic triple uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn absorb_law_pages_diff(d1: PdfPagesDiff, d2: &PdfPagesDiff) -> PdfPagesDiff {
    #[derive(Clone, PartialEq)]
    struct PageSlotValue {
        page: Option<PdfPage>,
        patch: PdfPageDiff,
    }
    let first: PdfIndexedDiff<PageSlotValue> = PdfIndexedDiff { removed: d1.removed.clone(), modified: d1.modified.iter().map(|m| PdfIndexedItem { index: m.index, value: PageSlotValue { page: None, patch: m.diff.clone() } }).collect(), added: d1.added.iter().map(|a| PdfIndexedItem { index: a.index, value: PageSlotValue { page: Some(a.page.clone()), patch: PdfPageDiff::default() } }).collect() };
    let second: PdfIndexedDiff<PageSlotValue> = PdfIndexedDiff { removed: d2.removed.clone(), modified: d2.modified.iter().map(|m| PdfIndexedItem { index: m.index, value: PageSlotValue { page: None, patch: m.diff.clone() } }).collect(), added: d2.added.iter().map(|a| PdfIndexedItem { index: a.index, value: PageSlotValue { page: Some(a.page.clone()), patch: PdfPageDiff::default() } }).collect() };
    let max_base = first.removed.iter().copied().chain(first.modified.iter().map(|m| m.index)).max().map(|m| m + 1).unwrap_or(0).max(first.added.len() + second.removed.iter().copied().chain(second.modified.iter().map(|m| m.index)).max().map(|m| m + 1).unwrap_or(0));
    let base: Vec<Slot<PageSlotValue>> = (0..max_base + first.added.len() + second.added.len() + 1).map(Slot::Base).collect();
    let mut mid = base.clone();
    for item in &first.modified {
        if let Some(slot) = mid.get_mut(item.index) {
            *slot = match slot {
                Slot::Base(index) | Slot::Modified(index, _) => Slot::Modified(*index, item.value.clone()),
                Slot::Added(_) => Slot::Added(item.value.clone()),
            };
        }
    }
    let mut removed = first.removed.clone();
    removed.sort_unstable_by(|a, b| b.cmp(a));
    for index in removed {
        if index < mid.len() {
            mid.remove(index);
        }
    }
    let mut added = first.added.clone();
    added.sort_by_key(|item| item.index);
    for item in added {
        mid.insert(item.index.min(mid.len()), Slot::Added(item.value));
    }
    let mut end = mid.clone();
    for item in &second.modified {
        if let Some(slot) = end.get_mut(item.index) {
            *slot = match slot {
                Slot::Base(index) => Slot::Modified(*index, item.value.clone()),
                Slot::Modified(index, previous) => {
                    let patch = absorb_page_diff(previous.patch.clone(), item.value.patch.clone());
                    Slot::Modified(*index, PageSlotValue { page: None, patch })
                }
                Slot::Added(previous) => {
                    let page = apply_page_diff(previous.page.clone().unwrap_or_default(), &item.value.patch);
                    Slot::Added(PageSlotValue { page: Some(page), patch: PdfPageDiff::default() })
                }
            };
        }
    }
    let mut removed = second.removed.clone();
    removed.sort_unstable_by(|a, b| b.cmp(a));
    for index in removed {
        if index < end.len() {
            end.remove(index);
        }
    }
    let mut added = second.added.clone();
    added.sort_by_key(|item| item.index);
    for item in added {
        end.insert(item.index.min(end.len()), Slot::Added(item.value));
    }
    let surviving: HashSet<usize> = end
        .iter()
        .filter_map(|slot| match slot {
            Slot::Base(index) | Slot::Modified(index, _) => Some(*index),
            Slot::Added(_) => None,
        })
        .collect();
    let mut out = PdfPagesDiff::default();
    for slot in &base {
        if let Slot::Base(index) = slot {
            if !surviving.contains(index) {
                out.removed.push(*index);
            }
        }
    }
    for (position, slot) in end.iter().enumerate() {
        match slot {
            Slot::Modified(index, value) => out.modified.push(PdfPageModified { index: *index, diff: value.patch.clone() }),
            Slot::Added(value) => out.added.push(PdfPageAdded { index: position, page: value.page.clone().unwrap_or_default() }),
            Slot::Base(_) => {}
        }
    }
    out
}
/// ↩️ The negative page triple over `base`: it removes what `diff` added, patches every modified page back through its own
/// negative patch at the position the page holds in the final state, and re-adds every removed base page at its base index.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn inverse_pages_diff(diff: &PdfPagesDiff, base: &[PdfPage]) -> PdfPagesDiff {
    let added: Vec<usize> = diff.added.iter().map(|item| item.index).collect();
    let mut inverse = PdfPagesDiff::default();
    for (position, origin) in triple_layout(base.len(), &diff.removed, &added).into_iter().enumerate() {
        match origin {
            Origin::Added => inverse.removed.push(position),
            Origin::Base(index) => {
                if let (Some(modified), Some(page)) = (diff.modified.iter().find(|item| item.index == index), base.get(index)) {
                    inverse.modified.push(PdfPageModified { index: position, diff: inverse_page_diff(&modified.diff, page) });
                }
            }
        }
    }
    let mut removed = diff.removed.clone();
    removed.sort_unstable();
    removed.dedup();
    for index in removed {
        if let Some(page) = base.get(index) {
            inverse.added.push(PdfPageAdded { index, page: page.clone() });
        }
    }
    inverse
}
//#endregion 🔖️PagesTriple
