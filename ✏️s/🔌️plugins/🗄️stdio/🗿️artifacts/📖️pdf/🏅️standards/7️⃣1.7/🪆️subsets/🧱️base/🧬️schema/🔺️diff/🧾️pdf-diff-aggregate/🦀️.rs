//! 🧾️ The `PdfDiff` aggregate: validation, unchecked application, absorption and inversion across every document lane.

use super::*;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.pdf.1.7`. `schema` is an identity field and is never diffed. `info` is a
/// field-wise tri-state patch (@see [`PdfInfoDiff`]). `graph_edit` marks a diff whose `objects`/`trailer` rows edit
/// the retained COS graph directly: applying it carries every typed lane the edited graph reads differently
/// (@see `schema::graph_projection::carry_graph_edit`), so a direct COS edit and the typed model never disagree and the next write spells the edit
/// instead of re-stating a stale typed lane over it.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf.1.7.diff")]
pub struct PdfDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub declared_version: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pages: Option<PdfPagesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fonts: Option<PdfKeyedDiff<PdfFont>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub images: Option<PdfKeyedDiff<PdfImage>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub forms: Option<PdfKeyedDiff<PdfFormXObject>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub ext_g_states: Option<PdfKeyedDiff<PdfExtGState>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shadings: Option<PdfKeyedDiff<PdfShading>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub patterns: Option<PdfKeyedDiff<PdfPattern>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color_spaces: Option<PdfKeyedDiff<PdfNamedColorSpace>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PdfKeyedDiff<PdfNamedProperties>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub outlines: Option<PdfIndexedDiff<PdfOutlineItem>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub named_destinations: Option<PdfIndexedDiff<PdfNamedDestination>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub page_labels: Option<PdfIndexedDiff<PdfPageLabelRange>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub embedded_files: Option<PdfKeyedDiff<PdfEmbeddedFile>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub output_intents: Option<PdfIndexedDiff<PdfOutputIntent>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub acro_form: Option<PdfSet<PdfAcroForm>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub optional_content: Option<PdfSet<PdfOptionalContent>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub page_layout: Option<PdfSet<PdfPageLayout>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub page_mode: Option<PdfSet<PdfPageMode>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub viewer_preferences: Option<PdfSet<PdfViewerPreferences>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub open_action: Option<PdfSet<PdfOpenAction>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<PdfSet<String>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mark_info: Option<PdfSet<PdfMarkInfo>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PdfSet<String>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub document_id: Option<PdfSet<[Vec<u8>; 2]>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<PdfSet<PdfEncryption>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<PdfInfoDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub catalog_extra: Option<PdfDictDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub objects: Option<PdfObjectsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub trailer: Option<PdfDictDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "is_false")]
    pub graph_edit: bool,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub admitted_stream_roles: Option<PdfIndexedDiff<PdfAdmittedStreamRole>>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_false(value: &bool) -> bool {
    !*value
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn validate_pages_diff(diff: &PdfPagesDiff, base: &[PdfPage]) -> MutationApplyResult<()> {
    validate_index_triple(base.len(), &diff.removed, &diff.modified.iter().map(|item| item.index).collect::<Vec<_>>(), &diff.added.iter().map(|item| item.index).collect::<Vec<_>>(), "pages")?;
    for modified in &diff.modified {
        validate_page_diff(&modified.diff, &base[modified.index]).map_err(|error| error.under(vec!["pages".to_string(), modified.index.to_string()]))?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_pdf_diff(diff: &PdfDiff, base: &PdfSnapshot) -> MutationApplyResult<()> {
    if let Some(pages) = &diff.pages {
        validate_pages_diff(pages, &base.pages).map_err(|error| error.under(["pages"]))?;
    }
    macro_rules! keyed {
        ($($field:ident),*) => {
            $(if let Some(lane) = &diff.$field {
                lane.validate(&base.$field, stringify!($field))?;
            })*
        };
    }
    keyed!(fonts, images, forms, ext_g_states, shadings, patterns, color_spaces, properties, embedded_files);
    macro_rules! indexed {
        ($($field:ident),*) => {
            $(if let Some(lane) = &diff.$field {
                lane.validate(base.$field.len(), stringify!($field))?;
            })*
        };
    }
    indexed!(outlines, named_destinations, page_labels, output_intents, admitted_stream_roles);
    if let Some(objects) = &diff.objects {
        validate_objects_diff(objects, &base.objects).map_err(|error| error.under(["objects"]))?;
    }
    if let Some(trailer) = &diff.trailer {
        validate_dict_diff(trailer, &base.trailer).map_err(|error| error.under(["trailer"]))?;
    }
    if let Some(extra) = &diff.catalog_extra {
        validate_dict_diff(extra, &base.catalog_extra).map_err(|error| error.under(["catalogExtra"]))?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_pdf_diff_unchecked(diff: &PdfDiff, base: &PdfSnapshot) -> PdfSnapshot {
    let mut next = base.clone();
    if let Some(v) = &diff.declared_version {
        next.declared_version = v.clone();
    }
    if let Some(info) = &diff.info {
        next.info = info.apply(&base.info);
    }
    if let Some(pd) = &diff.pages {
        next.pages = apply_pages_diff(pd, &base.pages);
    }
    macro_rules! lanes {
        ($($field:ident),*) => {
            $(if let Some(lane) = &diff.$field {
                next.$field = lane.apply(&base.$field);
            })*
        };
    }
    lanes!(fonts, images, forms, ext_g_states, shadings, patterns, color_spaces, properties, embedded_files, outlines, named_destinations, page_labels, output_intents, admitted_stream_roles);
    apply_tri(&mut next.acro_form, &diff.acro_form);
    apply_tri(&mut next.optional_content, &diff.optional_content);
    apply_tri(&mut next.page_layout, &diff.page_layout);
    apply_tri(&mut next.page_mode, &diff.page_mode);
    apply_tri(&mut next.viewer_preferences, &diff.viewer_preferences);
    apply_tri(&mut next.open_action, &diff.open_action);
    apply_tri(&mut next.language, &diff.language);
    apply_tri(&mut next.mark_info, &diff.mark_info);
    apply_tri(&mut next.metadata, &diff.metadata);
    apply_tri(&mut next.document_id, &diff.document_id);
    apply_tri(&mut next.encryption, &diff.encryption);
    if let Some(extra) = &diff.catalog_extra {
        next.catalog_extra = apply_dict_diff(extra, &base.catalog_extra);
    }
    if let Some(od) = &diff.objects {
        next.objects = apply_objects_diff(od, &base.objects);
    }
    if let Some(td) = &diff.trailer {
        next.trailer = apply_dict_diff(td, &base.trailer);
    }
    next
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_option<T>(slot: &mut Option<T>, other: Option<T>, merge: impl FnOnce(T, T) -> Option<T>) {
    *slot = match (slot.take(), other) {
        (None, b) => b,
        (a, None) => a,
        (Some(a), Some(b)) => merge(a, b),
    };
}

impl MutationDiff<PdfSnapshot> for PdfDiff {}

impl protocol::ColdMutationDiff<PdfSnapshot> for PdfDiff {
    /// ➕️ Structural, total, base-free sequential-coalesce absorb (`## Absorb` contract).
    /// Scalars and tri-states: LWW. Triples: composed via their own key/index-transported absorb.
    fn absorb(&mut self, other: Self) {
        if other.declared_version.is_some() {
            self.declared_version = other.declared_version;
        }
        absorb_option(&mut self.info, other.info, |mut a, b| {
            a.absorb(b);
            (!a.is_empty()).then_some(a)
        });
        absorb_option(&mut self.pages, other.pages, |a, b| {
            let m = absorb_law_pages_diff(a, &b);
            (!m.is_empty()).then_some(m)
        });
        macro_rules! lanes {
            ($($field:ident),*) => {
                $(absorb_option(&mut self.$field, other.$field, |a, b| {
                    let m = a.absorb(b);
                    (!m.is_empty()).then_some(m)
                });)*
            };
        }
        lanes!(fonts, images, forms, ext_g_states, shadings, patterns, color_spaces, properties, embedded_files, outlines, named_destinations, page_labels, output_intents, admitted_stream_roles);
        macro_rules! lww {
            ($($field:ident),*) => {
                $(if other.$field.is_some() {
                    self.$field = other.$field;
                })*
            };
        }
        lww!(acro_form, optional_content, page_layout, page_mode, viewer_preferences, open_action, language, mark_info, metadata, document_id, encryption);
        absorb_option(&mut self.catalog_extra, other.catalog_extra, |a, b| {
            let m = absorb_dict_diff(a, b);
            (!m.is_empty()).then_some(m)
        });
        absorb_option(&mut self.objects, other.objects, |a, b| {
            let m = absorb_law_objects_diff(a, b);
            (!m.is_empty()).then_some(m)
        });
        absorb_option(&mut self.trailer, other.trailer, |a, b| {
            let m = absorb_dict_diff(a, b);
            (!m.is_empty()).then_some(m)
        });
        self.graph_edit = (self.graph_edit || other.graph_edit) && (self.objects.is_some() || self.trailer.is_some());
    }

    fn apply(&self, unpublished: &mut PdfSnapshot, _capability: ApplyCapability) -> MutationApplyResult<()> {
        let base = &*unpublished;
        validate_pdf_diff(self, base)?;
        let mut next = apply_pdf_diff_unchecked(self, base);
        let replacements: Vec<_> = self.admitted_stream_roles.iter().flat_map(|roles| roles.modified.iter().chain(&roles.added)).map(|item| item.value.clone()).collect();
        validate_role_transition(&base.objects,&next.objects,&base.admitted_stream_roles,&next.admitted_stream_roles,&replacements).map_err(|message| MutationApplyError::new("mutation.apply.stream-role-required", message))?;
        if self.graph_edit || self.admitted_stream_roles.is_some() { carry_graph_edit(base, &mut next).map_err(|message| MutationApplyError::new("mutation.apply.stream-role-required", message))?; }
        *unpublished = next;
        Ok(())
    }
}

impl DiffAlgebra<PdfSnapshot> for PdfDiff {
    fn is_empty(&self) -> bool {
        self == &PdfDiff::default()
    }
}

impl protocol::ColdDiffAlgebra<PdfSnapshot> for PdfDiff {
    /// 🔁️ Diff-level undo, built lane by lane from `base`: every lane this diff names is set back to what `base` holds, so
    /// the negative diff applied after `self` restores `base`. A graph edit's negative diff is a graph edit too.
    fn inverse(&self, base: &PdfSnapshot) -> Self {
        macro_rules! keyed_lanes {
            ($($field:ident),*) => {
                $(let $field = self.$field.as_ref().map(|lane| lane.inverse(&base.$field));)*
            };
        }
        keyed_lanes!(fonts, images, forms, ext_g_states, shadings, patterns, color_spaces, properties, embedded_files, outlines, named_destinations, page_labels, output_intents, admitted_stream_roles);
        macro_rules! set_lanes {
            ($($field:ident),*) => {
                $(let $field = self.$field.as_ref().map(|_| PdfSet::from_option(&base.$field));)*
            };
        }
        set_lanes!(acro_form, optional_content, page_layout, page_mode, viewer_preferences, open_action, language, mark_info, metadata, document_id, encryption);
        PdfDiff {
            declared_version: self.declared_version.as_ref().map(|_| base.declared_version.clone()),
            pages: self.pages.as_ref().map(|pages| inverse_pages_diff(pages, &base.pages)),
            fonts,
            images,
            forms,
            ext_g_states,
            shadings,
            patterns,
            color_spaces,
            properties,
            outlines,
            named_destinations,
            page_labels,
            embedded_files,
            output_intents,
            acro_form,
            optional_content,
            page_layout,
            page_mode,
            viewer_preferences,
            open_action,
            language,
            mark_info,
            metadata,
            document_id,
            encryption,
            info: self.info.as_ref().map(|info| info.inverse(&base.info)),
            catalog_extra: self.catalog_extra.as_ref().map(|extra| inverse_dict_diff(extra, &base.catalog_extra)),
            objects: self.objects.as_ref().map(|objects| inverse_objects_diff(objects, &base.objects)),
            trailer: self.trailer.as_ref().map(|trailer| inverse_dict_diff(trailer, &base.trailer)),
            graph_edit: self.graph_edit,
            admitted_stream_roles,
        }
    }
}
//#endregion 🔖️Diff
