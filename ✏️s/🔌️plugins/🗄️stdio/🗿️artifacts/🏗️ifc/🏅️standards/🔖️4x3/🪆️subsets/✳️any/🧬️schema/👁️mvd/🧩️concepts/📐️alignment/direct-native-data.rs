//! 🛤️ Released direct-clause metadata and bounded diagnostic paths share native report conventions.
use super::*;
pub(super) fn decimal_len(mut value: u64) -> usize {
    let mut length = 1;
    while value >= 10 {
        value /= 10;
        length += 1;
    }
    length
}
pub(super) fn decimal(mut value: u64) -> Result<String, ValueError> {
    let mut bytes = [0u8; 20];
    let mut offset = bytes.len();
    loop {
        offset -= 1;
        bytes[offset] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    owned(std::str::from_utf8(&bytes[offset..]).map_err(|_| invalid("mvd.direct.number"))?)
}
use super::super::alignment_roles::{LENGTHS, PARAMETERS};
pub(super) fn field_name(field: u8, layout: usize) -> &'static str {
    match field {
        0 => "DesignParameters",
        1 => LENGTHS[layout],
        2 => "Representation",
        3 => "Representations",
        4 => "Items",
        _ => "SegmentLength",
    }
}
pub(super) fn field_owner(field: u8, layout: usize) -> &'static str {
    match field {
        0 => "IFCALIGNMENTSEGMENT",
        1 => PARAMETERS[layout],
        2 => "IFCPRODUCT",
        3 => "IFCPRODUCTREPRESENTATION",
        4 => "IFCREPRESENTATION",
        _ => "IFCCURVESEGMENT",
    }
}
pub(super) fn path(field: u8, layout: usize) -> &'static [&'static str] {
    match field {
        0 => &["DesignParameters"],
        1 => {
            if layout == 0 {
                &["DesignParameters", "SegmentLength"]
            } else {
                &["DesignParameters", "HorizontalLength"]
            }
        }
        2 => &["Representation"],
        3 => &["Representation", "Representations"],
        4 => &["Representation", "Representations", "0", "Items"],
        _ => &["Representation", "Representations", "0", "Items", "0", "SegmentLength"],
    }
}
pub(super) fn empty_report() -> IfcAlignmentDirectReport {
    IfcAlignmentDirectReport {
        concept: "Product_Shape/Product_Geometric_Representation/Alignment_Geometry/Alignment_Geometry_-_Horizontal,_Vertical_and_Cant",
        predicate: "selected-direct-zero-counterpart",
        source: IfcConceptSource {
            commit: "610ee3f95db9d0380b1eb8de4dc6e17833531518",
            url: "https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/concepts/Product_Shape/Product_Geometric_Representation/Alignment_Geometry/Alignment_Geometry_-_Horizontal,_Vertical_and_Cant/content.html",
            sha256: "052ef264607c967a36123caf90a609a87c263e732cc44220c1f7847d115d07a7",
        },
        schema_conformance: "not-evaluated",
        view_conformance: "not-evaluated",
        geometry_conformance: "not-evaluated",
        layout_terminal_conformance: "not-evaluated",
        parent_curve_conformance: "not-evaluated",
        status: "not-applicable",
        targets: vec![],
        diagnostics: vec![],
    }
}
pub(super) fn labels(code: &str) -> (&'static str, &'static str) {
    match code {
        "mvd.direct.source" => ("The source cannot be admitted for this predicate.", "Die Quelle kann für diese Prüfung nicht eingelesen werden."),
        "mvd.direct.target.missing" => ("The selected business segment is missing.", "Das ausgewählte fachliche Segment fehlt."),
        "mvd.direct.target.ambiguous" => ("The selected segment identifier is ambiguous.", "Die Kennung des ausgewählten Segments ist mehrdeutig."),
        "mvd.direct.target.type" => ("The selected object is not an alignment segment.", "Das ausgewählte Objekt ist kein Trassierungssegment."),
        "mvd.direct.role.unknown" => ("The declared object role is unresolved.", "Die deklarierte Objektrolle ist ungeklärt."),
        "mvd.direct.attribute.missing" => ("A declared clause attribute is missing.", "Ein deklariertes Prüfungsattribut fehlt."),
        "mvd.direct.attribute.ambiguous" => ("Several source slots declare this attribute.", "Mehrere Quellpositionen deklarieren dieses Attribut."),
        "mvd.direct.attribute.unknown" => ("Attribute ownership is unresolved.", "Die Attributzuordnung ist ungeklärt."),
        "mvd.direct.attribute.shape" => ("The declared attribute has an unresolved value shape.", "Die Wertstruktur des deklarierten Attributs ist ungeklärt."),
        "mvd.direct.reference.missing" => ("A directly referenced object is missing.", "Ein direkt referenziertes Objekt fehlt."),
        "mvd.direct.reference.ambiguous" => ("A directly referenced identifier is ambiguous.", "Eine direkt referenzierte Kennung ist mehrdeutig."),
        "mvd.direct.parameter.type" => ("The design parameter belongs to another layout kind.", "Der Entwurfsparameter gehört zu einer anderen Layoutart."),
        "mvd.direct.business.shape" => ("The selected business length is not a finite numeric measure.", "Die ausgewählte fachliche Länge ist kein endliches numerisches Maß."),
        "mvd.direct.business.nonzero" => ("The selected segment does not meet this predicate's zero-length precondition.", "Das ausgewählte Segment erfüllt die Nulllängenbedingung dieser Prüfung nicht."),
        "mvd.direct.path.missing" => ("No direct geometric counterpart path was resolved.", "Kein direkter geometrischer Gegenstückpfad wurde aufgelöst."),
        "mvd.direct.path.ambiguous" => ("Several direct paths require an explicit correspondence policy.", "Mehrere direkte Pfade erfordern eine explizite Zuordnungsregel."),
        "mvd.direct.path.unsupported" => ("The direct representation requires another correspondence evaluator.", "Die direkte Repräsentation erfordert eine andere Zuordnungsprüfung."),
        "mvd.direct.curve.shape" => ("The curve length requires one finite value in a released measure SELECT branch.", "Die Kurvenlänge erfordert einen endlichen Wert in einem freigegebenen Maß-SELECT-Zweig."),
        "mvd.direct.curve.nonzero" => ("The selected direct geometric counterpart has nonzero length.", "Das ausgewählte direkte geometrische Gegenstück hat eine Länge ungleich null."),
        _ => ("The direct predicate is unresolved.", "Die direkte Prüfung ist ungeklärt."),
    }
}
pub(super) fn pointer_len(owner: Option<Owner>, item: bool) -> usize {
    owner.map_or(0, |owner| 10 + digit_length(owner.entity) + if owner.part.is_some() { 9 + digit_length(owner.part.unwrap()) } else { 0 } + 6 + digit_length(owner.argument) + if item { 8 } else { 0 })
}
pub(super) fn pointer(owner: Owner, item: bool) -> Result<String, ValueError> {
    let mut output = String::new();
    output.try_reserve_exact(pointer_len(Some(owner), item)).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "mvd.direct.allocation"))?;
    output.push_str("/entities/");
    append_number(&mut output, owner.entity);
    if let Some(part) = owner.part {
        output.push_str("/complex/");
        append_number(&mut output, part);
    }
    output.push_str("/args/");
    append_number(&mut output, owner.argument);
    if item {
        output.push_str("/value/0");
    }
    Ok(output)
}
