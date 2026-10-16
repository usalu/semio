//! 🧪️ The instance extension list honours every registered entry, in registration order, and an empty list routes nothing.
use super::*;

/// 🅰️ A probe extension that owns `probe.alpha` and admits it while the document loads.
struct ProbeAlpha;

impl InstanceExtension for ProbeAlpha {
    fn claims_action(action: &str) -> bool {
        action == "probe.alpha"
    }

    fn admitted_while_loading(action: &str) -> bool {
        action == "probe.alpha"
    }
}

/// 🅱️ A probe extension that owns `probe.beta` and `probe.alpha`, and needs history for `probe.beta`.
struct ProbeBeta;

impl InstanceExtension for ProbeBeta {
    fn claims_action(action: &str) -> bool {
        matches!(action, "probe.alpha" | "probe.beta")
    }

    fn reads_history(action: &str) -> bool {
        action == "probe.beta"
    }
}

#[test]
fn empty_list_claims_no_route() {
    assert!(!<() as InstanceExtension>::claims_action("probe.alpha"));
    assert!(!<() as InstanceExtension>::reads_history("probe.beta"));
    assert!(!<() as InstanceExtension>::admitted_while_loading("probe.alpha"));
}

#[test]
fn registered_entry_is_honoured() {
    type Registered = (ProbeAlpha, ());
    assert!(<Registered as InstanceExtension>::claims_action("probe.alpha"));
    assert!(<Registered as InstanceExtension>::admitted_while_loading("probe.alpha"));
    assert!(!<Registered as InstanceExtension>::claims_action("probe.beta"));
}

#[test]
fn every_entry_of_a_list_contributes() {
    type Registered = (ProbeAlpha, (ProbeBeta, ()));
    assert!(<Registered as InstanceExtension>::claims_action("probe.alpha"));
    assert!(<Registered as InstanceExtension>::claims_action("probe.beta"));
    assert!(<Registered as InstanceExtension>::reads_history("probe.beta"));
    assert!(!<Registered as InstanceExtension>::reads_history("probe.alpha"));
    assert!(!<Registered as InstanceExtension>::claims_action("probe.gamma"));
}
