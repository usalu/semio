use super::*;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<ReplacePage as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "replace-page");
}

#[test]
fn a_missing_page_is_refused_and_has_no_inverse() {
    let base = PdfSnapshot::default();
    let mutation = ReplacePage { index: 0, page: PdfPage::new(10.0, 20.0) };
    assert_eq!(<ReplacePage as MutationKind<PdfSnapshot, PdfMutation>>::diff(&mutation, &base).messages().len(), 1);
    assert!(<ReplacePage as MutationKind<PdfSnapshot, PdfMutation>>::inverse(&mutation, &base).expect("an unaddressable page has nothing to undo").is_empty());
}

#[test]
fn the_diff_carries_only_the_fields_the_pages_differ_in() {
    let base = PdfSnapshot { pages: vec![PdfPage::new(10.0, 20.0)], ..PdfSnapshot::default() };
    let mut page = base.pages[0].clone();
    page.rotate = 90;
    let outcome = <ReplacePage as MutationKind<PdfSnapshot, PdfMutation>>::diff(&ReplacePage { index: 0, page }, &base);
    let pages = outcome.diff().pages.as_ref().expect("the page moved");
    assert_eq!((pages.modified.len(), pages.added.len(), pages.removed.len()), (1, 0, 0));
    assert_eq!(pages.modified[0].diff.rotate, Some(90));
    assert_eq!(pages.modified[0].diff.media_box, None);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let mut page = PdfPage::new(10.0, 20.0);
    page.rotate = 90;
    page.thumbnail = Some("thumb".to_string());
    let base = PdfSnapshot { pages: vec![PdfPage::new(10.0, 20.0), PdfPage::new(30.0, 40.0)], ..PdfSnapshot::default() };
    assert_mutation_inverse_sum_law(&PdfMutation::ReplacePage(ReplacePage { index: 0, page }), &base).await;
}
