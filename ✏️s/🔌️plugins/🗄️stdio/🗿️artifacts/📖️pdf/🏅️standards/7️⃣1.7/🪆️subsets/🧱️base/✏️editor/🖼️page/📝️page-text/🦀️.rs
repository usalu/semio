//! 📝️ PDF page editor text projection: Unicode replacement of the page text and the agent revision guard.

use super::*;

//#region ✏️PageText
fn replace_page_unicode_text(page: &PdfPage, text: &str) -> Option<PdfContent> {
    fn replace_text_string(value: &mut PdfTextString, replacement: &mut Option<String>) {
        let PdfTextString::Text { text } = value else { return };
        *text = replacement.take().unwrap_or_default();
    }

    let mut content = page.content.clone();
    let mut replacement = Some(text.to_string());
    for operation in &mut content {
        match operation {
            PdfOp::ShowText { text } | PdfOp::NextLineShowText { text } | PdfOp::NextLineShowTextSpaced { text, .. } => replace_text_string(text, &mut replacement),
            PdfOp::ShowTextArray { items } => {
                for item in items {
                    if let PdfTextArrayItem::Text { text } = item {
                        *text = replacement.take().unwrap_or_default();
                    }
                }
            }
            _ => {}
        }
    }
    replacement.is_none().then_some(content)
}

/// 🔐️ The token an agent's omitted `set-page` revision is admitted against: the addressed page text's own token, as its draft
/// binding carries it; a missing page is refused exactly as the edit itself would refuse it.
pub fn page_text_agent_revision(snapshot: &PdfSnapshot, args: &semio_framework_value::DslValue) -> Result<Option<String>, Fault> {
    let page_index = semio_s_artifact_stdio_contract::window_kit_required_index_argument(Some(args), "page")? as usize;
    let target = snapshot.pages.get(page_index).ok_or_else(|| {
        refuse(PdfFault::StaleTarget, format!("PDF page {page_index} no longer exists"))
    })?;
    Ok(Some(semio_plugin::app::DocumentWindowKit::text_revision(&target.text())))
}

/// 📄️ Builds a reversible page-text edit and supplies a font when the page has no editable text.
pub fn page_text_edit_mutations(snapshot: &PdfSnapshot, page: u32, item: u32, revision: &str, text: &str) -> Result<Vec<PdfMutation>, Fault> {
    if item != 0 {
        return Err(refuse(PdfFault::InvalidItem, "PDF page drafts require item zero"));
    }
    let page_index = page as usize;
    let target = snapshot.pages.get(page_index).ok_or_else(|| {
        refuse(PdfFault::StaleTarget, format!("PDF page {page_index} no longer exists"))
    })?;
    semio_s_artifact_stdio_contract::require_window_kit_document_revision(&target.text(), revision, "stdio.pdf.set-page.conflict")?;
    if target.text() == text {
        return Ok(Vec::new());
    }
    if let Some(content) = replace_page_unicode_text(target, text) {
        return Ok(vec![PdfMutation::SetPageContent(SetPageContent { index: page_index, content })]);
    }
    let [left, bottom, right, top] = visible_page_box(target);
    let x = (left + 12.0).min(right);
    let y = (top - 24.0).max(bottom);
    insert_object(snapshot, "insert-text", page_index, x, y, 0.0, 12.0, 0.0, 0.0, 0.0, text)
}

#[cfg(test)]
mod document_text_edit_tests {
    use super::{page_text_edit_mutations, replace_page_unicode_text};
    use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
    use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::{pdf_content, PdfOp, PdfPage, PdfSnapshot, PdfTextString};

    #[test]
    fn replacement_preserves_non_text_operations_and_replaces_instead_of_appending() {
        let mut page = PdfPage::default();
        page.content = pdf_content([
            PdfOp::Save,
            PdfOp::BeginText,
            PdfOp::ShowText { text: PdfTextString::text("old") },
            PdfOp::NextLineShowText { text: PdfTextString::text("tail") },
            PdfOp::EndText,
            PdfOp::Restore,
        ]);
        page.content = replace_page_unicode_text(&page, "new\ncontent").unwrap();
        assert_eq!(page.text(), "new\ncontent");
        assert!(matches!(page.content.first(), Some(PdfOp::Save)));
        assert!(matches!(page.content.last(), Some(PdfOp::Restore)));
        assert_eq!(page.content.len(), 6);
    }

    #[test]
    fn page_text_edit_rejects_missing_stale_and_non_text_addresses() {
        let mut snapshot = PdfSnapshot::default();
        snapshot.pages.push(PdfPage::default());
        let revision = semio_plugin::app::DocumentWindowKit::text_revision("");
        assert!(page_text_edit_mutations(&snapshot, 1, 0, &revision, "draft").is_err());
        assert!(page_text_edit_mutations(&snapshot, 0, 1, &revision, "draft").is_err());
        assert!(page_text_edit_mutations(&snapshot, 0, 0, "0000000000000000", "draft").is_err());
        let edits = page_text_edit_mutations(&snapshot, 0, 0, &revision, "draft").expect("addressed edit");
        let [PdfMutation::SetFont(_), PdfMutation::SetPageContent(edit)] = edits.as_slice() else { panic!("install font and set page content") };
        let mut page = snapshot.pages[0].clone();
        page.content.clone_from(&edit.content);
        assert_eq!(page.text(), "draft");
    }
}
//#endregion ✏️PageText
