//! 🧩️ Ordered business-clause provenance, localized diagnostics and exact source addresses.
use super::*;
pub(super) fn field_name(field: u8, layout: usize) -> &'static str {
    match field {
        0 => "RelatingObject",
        1 => "RelatedObjects",
        2 => "DesignParameters",
        _ => LENGTHS[layout],
    }
}
pub(super) fn field_owner(field: u8, layout: usize) -> &'static str {
    match field {
        0 | 1 => "IFCRELNESTS",
        2 => "IFCALIGNMENTSEGMENT",
        _ => PARAMETERS[layout],
    }
}
pub(super) fn empty_report() -> IfcAlignmentTerminalReport {
    IfcAlignmentTerminalReport {
        concept: "Object_Composition/Nesting/Alignment_Layouts/Alignment_Layout_-_Horizontal,_Vertical_and_Cant",
        predicate: "business-terminal-zero",
        source: IfcConceptSource {
            commit: "610ee3f95db9d0380b1eb8de4dc6e17833531518",
            url: "https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/concepts/Object_Composition/Nesting/Alignment_Layouts/Alignment_Layout_-_Horizontal,_Vertical_and_Cant/content.html",
            sha256: "46f10e76e2646fbfd5f3c8e6967c59fb3b522169086dfd9a1af6e0ccfefe7158",
        },
        schema_conformance: "not-evaluated",
        view_conformance: "not-evaluated",
        geometry_conformance: "not-evaluated",
        status: "not-applicable",
        targets: vec![],
        diagnostics: vec![],
    }
}
pub(super) fn labels(code: &str) -> (&'static str, &'static str) {
    match code {
        "mvd.source" => ("The source cannot be admitted for this predicate.", "Die Quelle kann für diese Prüfung nicht eingelesen werden."),
        "mvd.applicability" => ("Predicate applicability is unresolved.", "Die Anwendbarkeit der Prüfung ist ungeklärt."),
        "mvd.target.missing" => ("The selected layout is missing.", "Das ausgewählte Layout fehlt."),
        "mvd.target.ambiguous" => ("The selected layout identifier is ambiguous.", "Die Kennung des ausgewählten Layouts ist mehrdeutig."),
        "mvd.target.type" => ("The target has another layout kind.", "Das Ziel hat eine andere Layoutart."),
        "mvd.target.unknown" => ("The layout declaration or kind is unresolved.", "Die Layoutdeklaration oder Layoutart ist ungeklärt."),
        "mvd.attribute.missing" => ("A required clause attribute is missing.", "Ein erforderliches Prüfungsattribut fehlt."),
        "mvd.attribute.ambiguous" => ("Several constituents declare the required attribute.", "Mehrere Bestandteile deklarieren das erforderliche Attribut."),
        "mvd.attribute.unknown" => ("Attribute ownership is unresolved.", "Die Attributzuordnung ist ungeklärt."),
        "mvd.attribute.shape" => ("A required clause attribute has an unresolved shape.", "Die Wertstruktur eines erforderlichen Prüfungsattributs ist ungeklärt."),
        "mvd.nesting.missing" => ("No ordered segment nesting was resolved.", "Keine geordnete Segmentverschachtelung wurde aufgelöst."),
        "mvd.nesting.ambiguous" => ("Several nesting lists have no unambiguous shared order.", "Mehrere Verschachtelungslisten haben keine eindeutige gemeinsame Reihenfolge."),
        "mvd.nesting.unresolved" => ("Nesting ownership is unresolved.", "Die Zuordnung der Verschachtelung ist ungeklärt."),
        "mvd.member.reference" => ("A segment member is not a reference.", "Ein Segmenteintrag ist keine Referenz."),
        "mvd.member.missing" => ("A referenced segment or design parameter is missing.", "Ein referenziertes Segment oder ein Entwurfsparameter fehlt."),
        "mvd.member.ambiguous" => ("A referenced segment or design parameter is ambiguous.", "Ein referenziertes Segment oder ein Entwurfsparameter ist mehrdeutig."),
        "mvd.member.type" => ("A list member is not an authenticated alignment segment.", "Ein Listeneintrag ist kein bestätigtes Trassierungssegment."),
        "mvd.parameter.type" => ("The design parameter has an unresolved layout domain.", "Der Layoutbereich des Entwurfsparameters ist ungeklärt."),
        "mvd.length.shape" => ("The declared segment length is not a finite numeric value.", "Die deklarierte Segmentlänge ist kein endlicher Zahlenwert."),
        "mvd.terminal.nonzero" => ("The last business segment has nonzero length.", "Das letzte fachliche Segment hat eine Länge ungleich null."),
        _ => ("The business predicate is unresolved.", "Die fachliche Prüfung ist ungeklärt."),
    }
}
pub(super) fn path_count(path: u8) -> usize {
    match path {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        _ => 4,
    }
}
pub(super) fn path_word(path: u8, index: usize, layout: usize, member: usize) -> Result<String, ValueError> {
    if index == 1 && path >= 2 {
        number(member)
    } else {
        owned(match index {
            0 => "RelatedObjects",
            2 => "DesignParameters",
            _ => LENGTHS[layout],
        })
    }
}
pub(super) fn path_word_len(path: u8, index: usize, layout: usize, member: usize) -> usize {
    if index == 1 && path >= 2 {
        digit_length(member)
    } else {
        match index {
            0 => "RelatedObjects".len(),
            2 => "DesignParameters".len(),
            _ => LENGTHS[layout].len(),
        }
    }
}
pub(super) fn pointer_len(owner: Option<Owner>, item: Option<usize>) -> usize {
    owner.map_or(0, |owner| {
        10 + digit_length(owner.entity) + if owner.argument == usize::MAX { 5 } else { (if owner.part.is_some() { 9 + digit_length(owner.part.unwrap()) } else { 0 }) + 6 + digit_length(owner.argument) + item.map_or(0, |item| 7 + digit_length(item)) }
    })
}
pub(super) fn pointer(owner: Owner, item: Option<usize>) -> Result<String, ValueError> {
    let mut output = String::new();
    output.try_reserve_exact(pointer_len(Some(owner), item)).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "mvd.terminal.allocation"))?;
    output.push_str("/entities/");
    append_number(&mut output, owner.entity);
    if owner.argument == usize::MAX {
        output.push_str("/name");
        return Ok(output);
    }
    if let Some(part) = owner.part {
        output.push_str("/complex/");
        append_number(&mut output, part);
    }
    output.push_str("/args/");
    append_number(&mut output, owner.argument);
    if let Some(item) = item {
        output.push_str("/value/");
        append_number(&mut output, item);
    }
    Ok(output)
}
