extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

use semio_s_plugin_norm_app_surface::{apply_remedy_edit, get_value_at_path, parse_path, set_value_at_path};
use semio_s_plugin_norm_compliance::{CheckReport, CheckStatus, NormFamily, RemedyBound};

/// 🌍️ Families with no annex axis, or with documented identical EN/DE recommended values under evaluate.
const ANNEX_IDENTICAL_ALLOWLIST: &[&str] = &["din4108", "din18599", "iso16757", "vdi3805"];

struct FamilyReport {
    id: &'static str,
    errors: Vec<String>,
}

impl FamilyReport {
    fn new(id: &'static str) -> Self {
        Self { id, errors: Vec::new() }
    }

    fn fail(&mut self, message: impl Into<String>) {
        self.errors.push(message.into());
    }

    fn ok(&self) -> bool {
        self.errors.is_empty()
    }

    fn summary(&self) -> String {
        if self.ok() {
            format!("{}: PASS", self.id)
        } else {
            format!("{}: FAIL — {}", self.id, self.errors.join("; "))
        }
    }
}

fn decode_dsl<D: pack::snapshot::ArtifactDsl>(label: &str, text: &str, report: &mut FamilyReport) -> Option<D> {
    match D::parse_dsl(text) {
        Ok(doc) => Some(doc),
        Err(error) => {
            report.fail(format!("{label}: decode failed: {error}"));
            None
        }
    }
}

fn assert_localized_copy(report: &mut FamilyReport, check_id: &str, field: &str, en: &str, de: &str) {
    if en.trim().is_empty() {
        report.fail(format!("{check_id}: {field}.en empty"));
    }
    if de.trim().is_empty() {
        report.fail(format!("{check_id}: {field}.de empty"));
    }
    if !en.trim().is_empty() && en == de {
        report.fail(format!("{check_id}: {field}.en identical to {field}.de"));
    }
}

fn assert_report_copy_and_paths<D: dsl::ToValue>(report: &mut FamilyReport, doc: &D, check_report: &CheckReport) {
    let root = dsl::ToValue::to_value(doc);
    let en = protocol::Locale::En;
    let de = protocol::Locale::De;
    for check in &check_report.checks {
        let title_en = check.title.resolve(&en);
        let title_de = check.title.resolve(&de);
        let explanation_en = check.explanation.resolve(&en);
        let explanation_de = check.explanation.resolve(&de);
        assert_localized_copy(report, &check.id, "title", title_en, title_de);
        assert_localized_copy(report, &check.id, "explanation", explanation_en, explanation_de);
        if title_en == explanation_en {
            report.fail(format!("{}: title.en identical to explanation.en", check.id));
        }
        if title_de == explanation_de {
            report.fail(format!("{}: title.de identical to explanation.de", check.id));
        }
        if !check.subject.path.is_empty() {
            if let Err(error) = parse_path(&check.subject.path) {
                report.fail(format!("{}: subject.path parse '{}': {error}", check.id, check.subject.path));
            } else if let Err(error) = get_value_at_path(&root, &check.subject.path) {
                report.fail(format!("{}: subject.path resolve '{}': {error}", check.id, check.subject.path));
            }
        }
        if matches!(check.status, CheckStatus::NotApplicable) {
            if explanation_en.trim().is_empty() || explanation_de.trim().is_empty() {
                report.fail(format!("{}: NotApplicable missing localized reason", check.id));
            }
        }
        if matches!(check.status, CheckStatus::Fail) {
            if check.remedies.is_empty() {
                report.fail(format!("{}: Fail without remedies", check.id));
            }
        }
        for (index, remedy) in check.remedies.iter().enumerate() {
            if remedy.target.path.is_empty() {
                continue;
            }
            if let Err(error) = parse_path(&remedy.target.path) {
                report.fail(format!("{}: remedy[{index}].target.path parse '{}': {error}", check.id, remedy.target.path));
            } else if let Err(error) = get_value_at_path(&root, &remedy.target.path) {
                report.fail(format!("{}: remedy[{index}].target.path resolve '{}': {error}", check.id, remedy.target.path));
            }
        }
    }
}

fn assert_remedies_flip_fails<F>(report: &mut FamilyReport, doc: &F::Document, check_report: &CheckReport)
where
    F: NormFamily,
    F::Document: Clone + dsl::ToValue + dsl::FromValue,
{
    for check in check_report.checks.iter().filter(|check| matches!(check.status, CheckStatus::Fail)) {
        if check.remedies.is_empty() {
            continue;
        }
        let applicables: Vec<(usize, &semio_s_plugin_norm_compliance::Remedy)> = check
            .remedies
            .iter()
            .enumerate()
            .filter(|(_, remedy)| remedy.applicable)
            .collect();
        if applicables.is_empty() {
            continue;
        }
        let mut cleared = false;
        let mut partials = Vec::new();
        let mut tried = 0usize;
        for &(remedy_index, remedy) in &applicables {
            let has_scalar = !matches!(remedy.bound, RemedyBound::OneOf) || !remedy.options.is_empty();
            if !has_scalar {
                continue;
            }
            let mut probe = dsl::ToValue::to_value(doc);
            match get_value_at_path(&probe, &remedy.target.path) {
                Ok(dsl::DslValue::Array(_)) | Ok(dsl::DslValue::Object(_)) => {
                    partials.push(format!("remedy[{remedy_index}] targets non-scalar path (skipped for flip)"));
                    continue;
                }
                Err(error) => {
                    partials.push(format!("remedy[{remedy_index}] path: {error}"));
                    continue;
                }
                Ok(_) => {}
            }
            tried += 1;
            let mut tree = probe;
            if let Err(fault) = apply_remedy_edit(check_report, &check.id, remedy_index, 0, &mut tree) {
                partials.push(format!("remedy[{remedy_index}] apply failed: {fault:?}"));
                continue;
            }
            let fixed = match dsl::FromValue::from_value(tree) {
                Ok(doc) => doc,
                Err(error) => {
                    partials.push(format!("remedy[{remedy_index}] decode: {error}"));
                    continue;
                }
            };
            let after = F::evaluate(&fixed);
            let Some(updated) = after.checks.iter().find(|item| item.id == check.id) else {
                cleared = true;
                break;
            };
            if !matches!(updated.status, CheckStatus::Fail) {
                cleared = true;
                break;
            }
            if updated.utilization < check.utilization {
                partials.push(format!(
                    "remedy[{remedy_index}] only lowered utilization {:.4}→{:.4}",
                    check.utilization, updated.utilization
                ));
            } else if updated.utilization > check.utilization + 1e-9 {
                partials.push(format!(
                    "remedy[{remedy_index}] raised utilization {:.4}→{:.4} (still Fail)",
                    check.utilization, updated.utilization
                ));
            } else {
                partials.push(format!("remedy[{remedy_index}] left check failing (u={:.4})", updated.utilization));
            }
        }
        if tried == 0 {
            continue;
        }
        if !cleared && applicables.len() > 1 {
            let mut tree = dsl::ToValue::to_value(doc);
            let mut sequential_ok = true;
            for (remedy_index, remedy) in &applicables {
                let has_scalar = !matches!(remedy.bound, RemedyBound::OneOf) || !remedy.options.is_empty();
                if !has_scalar {
                    sequential_ok = false;
                    break;
                }
                if apply_remedy_edit(check_report, &check.id, *remedy_index, 0, &mut tree).is_err() {
                    sequential_ok = false;
                    break;
                }
            }
            if sequential_ok {
                if let Ok(fixed) = dsl::FromValue::from_value(tree) {
                    let after = F::evaluate(&fixed);
                    match after.checks.iter().find(|item| item.id == check.id) {
                        None => cleared = true,
                        Some(updated) if !matches!(updated.status, CheckStatus::Fail) => cleared = true,
                        Some(updated) => partials.push(format!(
                            "sequential applicables left failing (u={:.4})",
                            updated.utilization
                        )),
                        _ => {}
                    }
                }
            }
        }
        if !cleared {
            report.fail(format!(
                "{}: no applicable remedy cleared the Fail ({})",
                check.id,
                partials.join("; ")
            ));
        }
    }
}

fn annex_values_differ(left: &CheckReport, right: &CheckReport) -> bool {
    for left_check in &left.checks {
        if let Some(right_check) = right.checks.iter().find(|check| check.id == left_check.id) {
            if (left_check.limit.value - right_check.limit.value).abs() > 1e-12 {
                return true;
            }
            if (left_check.computed.value - right_check.computed.value).abs() > 1e-12 {
                return true;
            }
        }
    }
    false
}

fn assert_annex_divergence<F>(report: &mut FamilyReport, doc: &F::Document)
where
    F: NormFamily,
    F::Document: Clone + dsl::ToValue + dsl::FromValue,
{
    if ANNEX_IDENTICAL_ALLOWLIST.contains(&report.id) {
        return;
    }
    let root = dsl::ToValue::to_value(doc);
    if get_value_at_path(&root, "annex").is_err() {
        report.fail("annex field missing but family not on ANNEX_IDENTICAL_ALLOWLIST");
        return;
    }
    let mut de_tree = root.clone();
    let mut en_tree = root;
    if let Err(error) = set_value_at_path(&mut de_tree, "annex", dsl::DslValue::String("De".into())) {
        report.fail(format!("set annex=De: {error}"));
        return;
    }
    if let Err(error) = set_value_at_path(&mut en_tree, "annex", dsl::DslValue::String("En".into())) {
        report.fail(format!("set annex=En: {error}"));
        return;
    }
    let de_doc = match dsl::FromValue::from_value(de_tree) {
        Ok(doc) => doc,
        Err(error) => {
            report.fail(format!("decode annex=De: {error}"));
            return;
        }
    };
    let en_doc = match dsl::FromValue::from_value(en_tree) {
        Ok(doc) => doc,
        Err(error) => {
            report.fail(format!("decode annex=En: {error}"));
            return;
        }
    };
    let de_report = F::evaluate(&de_doc);
    let en_report = F::evaluate(&en_doc);
    if !annex_values_differ(&de_report, &en_report) {
        report.fail("DE vs EN annex produced identical limit/computed values for all shared check ids");
    }
}

fn gate_family<F>(id: &'static str, examples: &[(&str, &str)]) -> FamilyReport
where
    F: NormFamily,
    F::Document: Clone + Default + pack::snapshot::ArtifactDsl + dsl::ToValue + dsl::FromValue,
{
    let mut report = FamilyReport::new(id);
    let default_doc = F::Document::default();
    let default_report = F::evaluate(&default_doc);
    if default_report.checks.is_empty() {
        report.fail("default evaluate produced zero checks");
    }
    assert_report_copy_and_paths(&mut report, &default_doc, &default_report);
    assert_annex_divergence::<F>(&mut report, &default_doc);

    let mut compliant = 0usize;
    let mut noncompliant = 0usize;
    for (label, text) in examples {
        let Some(doc) = decode_dsl::<F::Document>(label, text, &mut report) else {
            continue;
        };
        let check_report = F::evaluate(&doc);
        if check_report.checks.is_empty() {
            report.fail(format!("{label}: evaluate produced zero checks"));
        }
        assert_report_copy_and_paths(&mut report, &doc, &check_report);
        assert_remedies_flip_fails::<F>(&mut report, &doc, &check_report);
        if check_report.failing().count() == 0 {
            compliant += 1;
        }
        let fail_count = check_report.checks.iter().filter(|check| matches!(check.status, CheckStatus::Fail)).count();
        if fail_count >= 2 {
            noncompliant += 1;
        }
    }
    if examples.is_empty() {
        report.fail("no examples registered for gate");
    }
    if compliant == 0 {
        report.fail("no compliant example (no failing check)");
    }
    if noncompliant == 0 {
        report.fail("no non-compliant example with ≥2 Fail checks");
    }
    report
}
