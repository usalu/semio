//! 📝️ PDF page editor text projection: Unicode replacement of the page text and the agent revision guard.

use super::*;

//#region ✏️PageText
/// ✏️ Replaces the Unicode text projection while preserving every non-text page-content operator.
pub fn replace_page_unicode_text(page: &PdfPage, text: &str) -> Vec<PdfOp> {
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
    if replacement.as_deref().is_some_and(|text| !text.is_empty()) {
        content.extend([PdfOp::BeginText, PdfOp::ShowText { text: PdfTextString::text(replacement.take().expect("pending replacement")) }, PdfOp::EndText]);
    }
    content
}

/// 🔐️ The token an agent's omitted `set-page` revision is admitted against: the addressed page text's own token, as its draft
/// binding carries it; a missing page is refused exactly as the edit itself would refuse it.
pub fn page_text_agent_revision(snapshot: &PdfSnapshot, args: &semio_framework_value::DslValue) -> Result<Option<String>, Fault> {
    let page_index = semio_s_artifact_stdio_contract::window_kit_required_index_argument(Some(args), "page")? as usize;
    let target = snapshot.pages.get(page_index).ok_or_else(|| {
        Fault::new(FaultOrigin::App, FaultCode::new("stdio.pdf.set-page.stale-target"), format!("PDF page {page_index} no longer exists"))
    })?;
    Ok(Some(semio_framework_plugin::app::DocumentWindowKit::text_revision(&target.text())))
}

/// 📄️ Builds one exact, reversible PDF page-text replacement or rejects an invalid/stale target.
pub fn page_text_edit_mutation(snapshot: &PdfSnapshot, page: u32, item: u32, revision: &str, text: &str) -> Result<Option<PdfMutation>, Fault> {
    if item != 0 {
        return Err(Fault::new(
            FaultOrigin::App,
            FaultCode::new("stdio.pdf.set-page.invalid-item"),
            "PDF page drafts require item zero",
        ));
    }
    let page_index = page as usize;
    let target = snapshot.pages.get(page_index).ok_or_else(|| {
        Fault::new(
            FaultOrigin::App,
            FaultCode::new("stdio.pdf.set-page.stale-target"),
            format!("PDF page {page_index} no longer exists"),
        )
    })?;
    semio_s_artifact_stdio_contract::require_window_kit_document_revision(&target.text(), revision, "stdio.pdf.set-page.conflict")?;
    if target.text() == text {
        return Ok(None);
    }
    Ok(Some(PdfMutation::SetPageContent(SetPageContent { index: page_index, content: replace_page_unicode_text(target, text) })))
}

#[cfg(test)]
mod document_text_edit_tests {
    use super::{page_text_edit_mutation, replace_page_unicode_text};
    use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfOp, PdfPage, PdfSnapshot, PdfTextString};

    #[test]
    fn replacement_preserves_non_text_operations_and_replaces_instead_of_appending() {
        let mut page = PdfPage::default();
        page.content = vec![
            PdfOp::Save,
            PdfOp::BeginText,
            PdfOp::ShowText { text: PdfTextString::text("old") },
            PdfOp::NextLineShowText { text: PdfTextString::text("tail") },
            PdfOp::EndText,
            PdfOp::Restore,
        ];
        page.content = replace_page_unicode_text(&page, "new\ncontent");
        assert_eq!(page.text(), "new\ncontent");
        assert!(matches!(page.content.first(), Some(PdfOp::Save)));
        assert!(matches!(page.content.last(), Some(PdfOp::Restore)));
        assert_eq!(page.content.len(), 6);
    }

    #[test]
    fn page_text_edit_rejects_missing_stale_and_non_text_addresses() {
        let mut snapshot = PdfSnapshot::default();
        snapshot.pages.push(PdfPage::default());
        let revision = semio_framework_plugin::app::DocumentWindowKit::text_revision("");
        assert!(page_text_edit_mutation(&snapshot, 1, 0, &revision, "draft").is_err());
        assert!(page_text_edit_mutation(&snapshot, 0, 1, &revision, "draft").is_err());
        assert!(page_text_edit_mutation(&snapshot, 0, 0, "0000000000000000", "draft").is_err());
        let Some(PdfMutation::SetPageContent(edit)) = page_text_edit_mutation(&snapshot, 0, 0, &revision, "draft").expect("addressed edit") else { panic!("set page content") };
        let mut page = snapshot.pages[0].clone();
        page.content = edit.content;
        assert_eq!(page.text(), "draft");
    }
}
//#endregion ✏️PageText
