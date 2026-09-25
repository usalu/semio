use super::super::apply_pdf_mutation;
use super::*;
use protocol::Severity;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<SetPattern as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "set-pattern");
}

fn shading(id: &str) -> PdfShading {
    PdfShading { id: id.into(), color_space: PdfColorSpace::DeviceRgb, kind: PdfShadingKind::Axial { coords: [0.0, 0.0, 100.0, 0.0], domain: None, function: PdfFunction::Exponential { domain: vec![0.0, 1.0], range: None, c0: vec![1.0, 0.0, 0.0], c1: vec![0.0, 0.0, 1.0], n: 1.0 }, extend: [true, true] }, background: None, bbox: None, anti_alias: false, extra: Vec::new() }
}

fn shading_pattern(shading: &str, ext_g_state: Option<&str>) -> PdfMutation {
    PdfMutation::SetPattern(SetPattern { pattern: PdfPattern { id: "P1".into(), matrix: PDF_IDENTITY_MATRIX, kind: PdfPatternKind::Shading { shading: shading.into(), ext_g_state: ext_g_state.map(Into::into) }, extra: Vec::new() } })
}

fn assert_refused(snapshot: &mut PdfSnapshot, mutation: &PdfMutation) {
    let before = snapshot.clone();
    let outcome = apply_pdf_mutation(snapshot, mutation);
    let [message] = outcome.messages() else { panic!("{mutation:?} must raise exactly one message, raised {:?}", outcome.messages()) };
    assert_eq!((message.level, message.code.0.as_str(), message.target.as_slice()), (Severity::Error, "mutation.target-missing", ["P1".to_string()].as_slice()), "{mutation:?} is refused on the pattern it sets");
    assert_eq!(*snapshot, before, "a refused {mutation:?} leaves the document untouched");
}

/// ⚖️ F3: a shading pattern whose `/Shading` names no shading of the document is `rejected` with
/// `mutation.target-missing` and leaves the document untouched — it used to be `applied`, written with an
/// unresolvable reference and re-read as `shading: ""`.
#[test]
fn a_shading_pattern_naming_an_absent_shading_is_refused_and_changes_nothing() {
    assert_refused(&mut PdfSnapshot::default(), &shading_pattern("Sh1", None));
}

/// ⚖️ The pattern is set once the document holds everything it names; a missing extended graphics state is
/// refused on its own, after the shading resolved.
#[test]
fn a_shading_pattern_is_set_once_the_document_holds_its_shading_and_state() {
    let mut snapshot = PdfSnapshot::default();
    snapshot.shadings.push(shading("Sh1"));
    let mutation = shading_pattern("Sh1", Some("GS1"));
    assert_refused(&mut snapshot, &mutation);
    snapshot.ext_g_states.push(PdfExtGState { id: "GS1".into(), ..PdfExtGState::default() });
    let outcome = apply_pdf_mutation(&mut snapshot, &mutation);
    assert!(outcome.messages().is_empty(), "a pattern naming held resources applies cleanly, raised {:?}", outcome.messages());
    assert_eq!(snapshot.patterns.iter().map(|pattern| pattern.id.as_str()).collect::<Vec<_>>(), ["P1"]);
}

/// ⚖️ An empty shading id is the reader's spelling of an absent `/Shading`: it names nothing and applies, so
/// the inverse of removing such a read pattern stays applicable.
#[test]
fn an_empty_shading_id_names_nothing_and_applies() {
    let mut snapshot = PdfSnapshot::default();
    let outcome = apply_pdf_mutation(&mut snapshot, &shading_pattern("", None));
    assert!(outcome.messages().is_empty(), "an absent shading is not a dangling reference, raised {:?}", outcome.messages());
    assert_eq!(snapshot.patterns.len(), 1);
}
