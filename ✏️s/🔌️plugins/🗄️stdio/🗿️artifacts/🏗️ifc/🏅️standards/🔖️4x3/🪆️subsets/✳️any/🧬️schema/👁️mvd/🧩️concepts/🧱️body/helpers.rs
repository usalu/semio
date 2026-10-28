//! 🧭️ Bounded native source, subtype and diagnostic metadata retain actual graph owners.
use super::*;
pub(super) const PREDICATES: [&str; 3] = ["RepresentationIdentifier", "RepresentationType", "Items"];
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct DiagnosticSpec {
    pub code: &'static str,
    pub predicate: Option<usize>,
    pub owner: Option<Owner>,
    pub item: Option<usize>,
}
fn pointer_length(spec: DiagnosticSpec) -> usize {
    let Some(owner) = spec.owner else {
        return 0;
    };
    let mut length = 10 + digit_length(owner.entity);
    if owner.argument == usize::MAX {
        return length + 5;
    }
    if let Some(part) = owner.part {
        length += 9 + digit_length(part);
    }
    length += 6 + digit_length(owner.argument);
    if let Some(item) = spec.item {
        length += 7 + digit_length(item);
    }
    length
}
pub(super) fn pointer(spec: DiagnosticSpec) -> Result<Option<String>, ValueError> {
    let Some(owner) = spec.owner else {
        return Ok(None);
    };
    let mut output = String::new();
    output.try_reserve_exact(pointer_length(spec)).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "mvd.allocation"))?;
    output.push_str("/entities/");
    append_number(&mut output, owner.entity);
    if owner.argument == usize::MAX {
        output.push_str("/name");
        return Ok(Some(output));
    }
    if let Some(part) = owner.part {
        output.push_str("/complex/");
        append_number(&mut output, part);
    }
    output.push_str("/args/");
    append_number(&mut output, owner.argument);
    if let Some(item) = spec.item {
        output.push_str("/value/");
        append_number(&mut output, item);
    }
    Ok(Some(output))
}
pub(super) fn path_length(spec: DiagnosticSpec) -> usize {
    usize::from(spec.predicate.is_some()) + usize::from(spec.item.is_some())
}
pub(super) fn diagnostic_demand(spec: DiagnosticSpec, step: u8, list: &Vec<IfcConceptDiagnostic>) -> Result<RetirementDemand, ValueError> {
    if step == 0 {
        return vector_growth::<IfcConceptDiagnostic>(list.len(), list.capacity());
    }
    let capacity = match step {
        1 => path_length(spec) * size_of::<String>(),
        2 => spec.predicate.map_or(0, |index| PREDICATES[index].len()),
        3 => spec.item.map_or(0, digit_length),
        4 => pointer_length(spec),
        _ => 0,
    };
    Ok(RetirementDemand { copy_bytes: 2 * size_of::<IfcConceptDiagnostic>() + 2 * capacity, capacity_bytes: capacity, depth: 1, ..Default::default() })
}
pub(super) fn empty_diagnostic(policy: BodyPolicy, spec: DiagnosticSpec) -> IfcConceptDiagnostic {
    let (en, de) = labels(policy, spec.code);
    IfcConceptDiagnostic { code: spec.code, path: vec![], pointer: None, label: IfcConceptLabel { en, de } }
}
pub(super) fn empty_report(policy: BodyPolicy) -> IfcBodyConceptReport {
    IfcBodyConceptReport {
        concept: policy.concept(),
        source: policy.source(),
        schema_conformance: "not-evaluated",
        view_conformance: "not-evaluated",
        status: "not-applicable",
        targets: vec![],
        diagnostics: vec![],
    }
}
fn labels(policy: BodyPolicy, code: &str) -> (&'static str, &'static str) {
    match code {
        "mvd.source" => ("The source cannot be admitted for this concept.", "Die Quelle kann für dieses Konzept nicht eingelesen werden."),
        "mvd.applicability" => ("Concept applicability is unresolved.", "Die Anwendbarkeit des Konzepts ist ungeklärt."),
        "mvd.target.missing" => ("The representation target is missing.", "Die angegebene Repräsentation fehlt."),
        "mvd.target.ambiguous" => ("The representation identifier is ambiguous.", "Die Kennung der Repräsentation ist mehrdeutig."),
        "mvd.target.type" => ("The target is not a shape representation.", "Das Ziel ist keine Formrepräsentation."),
        "mvd.target.unknown" => ("The representation declaration is unresolved.", "Die Deklaration der Repräsentation ist ungeklärt."),
        "mvd.attribute.missing" => ("A required concept attribute cannot be resolved.", "Ein benötigtes Konzeptattribut kann nicht aufgelöst werden."),
        "mvd.attribute.unknown" => ("An attribute declaration is unresolved.", "Eine Attributdeklaration ist ungeklärt."),
        "mvd.attribute.ambiguous" => ("Several constituents declare the concept attribute.", "Mehrere Bestandteile deklarieren das Konzeptattribut."),
        "mvd.attribute.shape" => ("The concept attribute has an unresolved value shape.", "Die Wertstruktur des Konzeptattributs ist ungeklärt."),
        "mvd.equals" => ("The attribute differs from the released concept value.", "Das Attribut weicht vom veröffentlichten Konzeptwert ab."),
        "mvd.items.type" => match policy { BodyPolicy::Tessellation => ("A representation item is not a tessellated face set.", "Ein Repräsentationselement ist kein tessellierter Flächensatz."), BodyPolicy::SweptSolid => ("A representation item is not a basic extruded or revolved area solid.", "Ein Repräsentationselement ist kein einfacher extrudierter oder gedrehter Flächenkörper."), BodyPolicy::AdvancedSweptSolid => ("A representation item is neither a swept area solid nor a swept disk solid.", "Ein Repräsentationselement ist weder ein Flächen-Sweep-Körper noch ein Kreis-Sweep-Körper."), BodyPolicy::SurfaceModel => ("A representation item is neither a tessellated item nor a shell-based or face-based surface model.", "Ein Repräsentationselement ist weder ein tesselliertes Element noch ein schalen- oder flächenbasiertes Oberflächenmodell."), BodyPolicy::Clipping => ("A representation item is not a boolean clipping result.", "Ein Repräsentationselement ist kein boolesches Beschneidungsergebnis."), BodyPolicy::Brep => ("A representation item is not a faceted boundary representation solid.", "Ein Repräsentationselement ist kein facettierter BRep-Körper."), BodyPolicy::AdvancedBrep => ("A representation item is neither an advanced nor a faceted boundary representation solid.", "Ein Repräsentationselement ist weder ein erweiterter noch ein facettierter BRep-Körper.") },
        "mvd.items.reference" => ("A representation item is not an entity reference.", "Ein Repräsentationselement ist keine Entitätsreferenz."),
        "mvd.items.missing" => ("A referenced representation item is missing.", "Ein referenziertes Repräsentationselement fehlt."),
        "mvd.items.ambiguous" => ("A referenced representation item is ambiguous.", "Ein referenziertes Repräsentationselement ist mehrdeutig."),
        "mvd.items.unknown" => ("An item declaration is unresolved.", "Die Deklaration eines Elements ist ungeklärt."),
        _ => unreachable!(),
    }
}
