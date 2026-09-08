
use super::{ladder, part21};
use semio_repo_test_host::Json;

/// 🧫️ The real committed AP214 fixture — a real Rhino 8.31 / ST-Developer v19.2 export whose
/// entire DATA section is untouched real data.
fn fixture() -> Vec<u8> {
    include_bytes!("../../../🪆️subsets/🧱️base/🧫️fixtures/🌲️hexagonal-cut-concrete-forest-left-ap214/📐️.stp").to_vec()
}

/// 🧪️ Every ISO 10303-21 §6.4.2 control directive this projection has to survive, read off a
/// literal spelled the way the standard spells it. The two rows that matter most are real: the
/// committed AP214 export carries an ST-Developer line break INSIDE a string literal, and the
/// committed IfcOpenShell IFC4 export carries `\\` for a one-character backslash name. Before
/// this decoder existed the projection compared `ruststep`'s raw lexeme, so this repository's
/// own conformant `\X2\000A\X0\` and the oracle's raw newline read as different VALUES.
#[test]
fn every_control_directive_decodes_to_the_value_it_denotes() {
    let decode = |lexeme: &str| part21::decode_string_literal(lexeme).unwrap_or_else(|error| panic!("decode {lexeme:?}: {error}"));
    assert_eq!(decode("plain text"), "plain text");
    assert_eq!(decode("\n"), "\n", "a raw line break passes through — it is what ST-Developer actually wrote");
    assert_eq!(decode(r"\X2\000A\X0\"), "\n", "and the conformant spelling of the same character decodes to the same value");
    assert_eq!(decode(r"\\"), "\\", "the doubled reverse solidus is ONE backslash");
    assert_eq!(decode(r"c\X2\000D000A\X0\every"), "c\r\nevery", "the real Nakagin description's own run");
    assert_eq!(decode(r"\X\41"), "A", "\\X\\ carries exactly two hex digits and no terminator");
    assert_eq!(decode(r"\X\41\S\A"), "A\u{00C1}");
    assert_eq!(decode(r"\S\A"), "\u{00C1}");
    assert_eq!(decode(r"\PA\\S\A"), "\u{00C1}");
    assert_eq!(decode(r"\X4\0001F600\X0\"), "\u{1F600}");
    assert_eq!(decode(r"\X2\4E2D6587\X0\"), "中文");
}

/// 🧪️ A directive this decoder cannot honour is an ERROR, never a lexeme waved through: a
/// subject that emitted a broken escape has to FAIL the comparison, not slip past it.
#[test]
fn a_malformed_or_unmappable_directive_is_refused() {
    assert!(part21::decode_string_literal(r"\Q").is_err(), "an unknown directive is not passed through");
    assert!(part21::decode_string_literal(r"\X\ZZ").is_err(), "non-hex digits are not passed through");
    assert!(part21::decode_string_literal(r"\X2\4E2D").is_err(), "an unterminated \\X2\\ run is not passed through");
    let page = part21::decode_string_literal(r"\PB\\S\A").expect_err("ISO 8859-2 must not be guessed");
    assert!(page.contains("ISO 8859 page B"), "the error must name the page it refused: {page}");
}

#[test]
fn the_real_fixture_carries_exactly_the_ladder_the_standard_predicts() {
    let exchange = part21::read(&fixture()).expect("ruststep parses the real export");
    let census = ladder::census(&exchange);
    assert_eq!(census.len(), 2, "the real file carries two representations: {census:?}");
    assert_eq!(census[0], (13, "ADVANCED_BREP_SHAPE_REPRESENTATION".to_string(), 6));
    assert_eq!(census[1], (836, "SHAPE_REPRESENTATION".to_string(), 2), "the bare base type classifies as rung 2, the minimal geometry-bearing form");
    assert!(ladder::violations(&exchange, 6).is_empty(), "nothing in a real AP214 export can exceed the top of the ladder");
    assert_eq!(ladder::violations(&exchange, 1).len(), 2, "CC1 admits neither of them");
}

/// 🏭️ `SHAPE_REPRESENTATION_RELATIONSHIP` (`#10` of the real file) must NOT be on the ladder:
/// it does not end in `SHAPE_REPRESENTATION`, and counting it would make every class report a
/// violation the standard does not describe.
#[test]
fn a_relationship_is_not_a_representation() {
    assert_eq!(ladder::rung_of("SHAPE_REPRESENTATION_RELATIONSHIP"), None);
    assert_eq!(ladder::rung_of("SHAPE_REPRESENTATION"), Some(2));
}

#[test]
fn the_real_fixture_carries_the_product_chain_through_the_iso_10303_41_subtype() {
    let exchange = part21::read(&fixture()).expect("ruststep parses the real export");
    assert!(ladder::has_product_chain(&exchange), "the formation rung is PRODUCT_DEFINITION_FORMATION_WITH_SPECIFIED_SOURCE, a real subtype");
    let identity = ladder::product_identity_json(&exchange);
    assert_eq!(identity.get("product"), Some(&Json::Number(827.0)));
    assert_eq!(identity.get("formation"), Some(&Json::Number(822.0)));
    assert_eq!(identity.get("definition"), Some(&Json::Number(821.0)));
}

#[test]
fn the_writer_round_trips_the_real_export_through_the_independent_reader() {
    let input = fixture();
    let exchange = part21::read(&input).expect("ruststep parses the real export");
    let written = part21::write(&exchange);
    assert_ne!(written, input, "a from-scratch writer cannot reproduce another writer's layout -- identical bytes would mean the input was copied");
    let reparsed = part21::read(&written).expect("ruststep parses this module's own output");
    assert_eq!(ladder::census(&reparsed), ladder::census(&exchange));
    assert_eq!(ladder::project(&written, 6).unwrap(), ladder::project(&input, 6).unwrap());
}

#[test]
fn a_demotion_keeps_the_representation_and_only_moves_its_rung() {
    let mut exchange = part21::read(&fixture()).expect("ruststep parses the real export");
    let before = ladder::representation_json(&exchange, 13).expect("#13 is a representation");
    let previous = ladder::demote_representation(&mut exchange, 13, ladder::ceiling_type_of(4).unwrap()).expect("a real representation demotes");
    assert_eq!(previous, "ADVANCED_BREP_SHAPE_REPRESENTATION");
    let after = ladder::representation_json(&exchange, 13).expect("still a representation");
    assert_eq!(after.get("name"), before.get("name"), "a demotion must not rename the representation");
    assert_eq!(after.get("items"), before.get("items"), "a demotion must not discard its items");
    assert_eq!(after.get("typeName"), Some(&Json::String("MANIFOLD_SURFACE_SHAPE_REPRESENTATION".to_string())));
}

#[test]
fn a_ladder_edit_refuses_an_instance_that_is_not_on_the_ladder() {
    let mut exchange = part21::read(&fixture()).expect("ruststep parses the real export");
    assert!(ladder::remove_representation(&mut exchange, 827).is_err(), "#827 is the PRODUCT record");
    assert!(ladder::remove_representation(&mut exchange, 10).is_err(), "#10 is a SHAPE_REPRESENTATION_RELATIONSHIP");
    assert!(ladder::has_product_chain(&exchange), "a refused edit changes nothing");
}

#[test]
fn clearing_the_product_identity_is_what_turns_the_soft_diagnostic_on() {
    let mut exchange = part21::read(&fixture()).expect("ruststep parses the real export");
    ladder::set_product_identity(&mut exchange, None).expect("clearing always succeeds");
    assert!(!ladder::has_product_chain(&exchange));
    assert_eq!(ladder::product_identity_json(&exchange), Json::Null);
}

/// 🧬️ The identical-catalog argument this module's own header makes, as an assertion. Four of
/// the six conformance classes declare the same six kinds, and that is a CONSEQUENCE of where
/// their ceilings sit on the ISO 10303-214 §4.3 ladder rather than a copied list — so it is
/// checked here rather than asserted in prose four times.
#[test]
fn the_four_interior_classes_share_one_vocabulary_because_their_ceilings_share_one_place() {
    use crate::standards::v_ap214::subsets::{cc1, cc2, cc3, cc4, cc5, cc6};

    for (class, rung, kinds) in [("cc2", cc2::MAX_RUNG, cc2::KINDS), ("cc3", cc3::MAX_RUNG, cc3::KINDS), ("cc4", cc4::MAX_RUNG, cc4::KINDS), ("cc5", cc5::MAX_RUNG, cc5::KINDS)] {
        assert!(ladder::ceiling_type_of(rung).is_some(), "{class} sits inside the ladder, so it has a ceiling type to write and to demote onto");
        assert!(rung < 6, "{class} sits inside the ladder, so at least one rung is above it to demote from");
        assert!(kinds.contains(&"set-shape-representation") && kinds.contains(&"demote-shape-representation"), "{class} admits both ladder verbs");
        assert_eq!(kinds, cc2::KINDS, "{class} reads the same three axes as cc2 with the same two ladder verbs, so its vocabulary is the same list");
    }

    assert_eq!(ladder::ceiling_type_of(cc1::MAX_RUNG), None, "cc1 admits no representation, so it has nothing to write and nothing to demote onto");
    assert!(!cc1::KINDS.contains(&"set-shape-representation") && !cc1::KINDS.contains(&"demote-shape-representation"));
    assert!(cc1::KINDS.contains(&"remove-shape-representation"), "deletion is cc1's only ladder repair");
    assert_ne!(cc1::KINDS, cc2::KINDS, "the class below the ladder cannot declare the interior vocabulary");

    assert_eq!(cc6::MAX_RUNG, 6, "cc6 sits on the top rung");
    assert!(!cc6::KINDS.contains(&"demote-shape-representation"), "nothing can be above the top rung, so a demotion has no subject");
    assert!(cc6::KINDS.contains(&"set-shape-representation"));
    assert_ne!(cc6::KINDS, cc2::KINDS, "the class on top of the ladder cannot declare the interior vocabulary");
}

#[test]
fn every_ceiling_type_classifies_back_to_its_own_rung() {
    assert_eq!(ladder::ceiling_type_of(1), None, "CC1 admits no representation, so it has no ceiling type");
    for rung in 2..=6u8 {
        assert_eq!(ladder::rung_of(ladder::ceiling_type_of(rung).unwrap()), Some(rung));
    }
}
