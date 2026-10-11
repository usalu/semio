//! 🧬️ Set Contributions in the imperative.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-contributions")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetContributions {
    pub json: String,
}

impl protocol::MutationKind<ImperativeConfig, ImperativeConfigMutation> for SetContributions {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "contributions_json", kind: "set-contributions", record: "SetContributions" };
    fn diff(&self, base: &ImperativeConfig) -> protocol::MutationOutcome<ImperativeConfigDiff> {
        imperative_engine::sync_imperative_module_contributions(&self.json);
        if base.contributions_json == self.json {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The requested configuration value is already current.");
        }
        protocol::MutationOutcome::new(ImperativeConfigDiff { contributions_json: Some(self.json.clone()), ..Default::default() })
    }
    fn inverse(&self, base: &ImperativeConfig) -> Result<Vec<ImperativeConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![ImperativeConfigMutation::SetContributions(Self { json: base.contributions_json.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Contributions", "Beiträge setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["contributions_json".into()]
    }
}

#[cfg(test)]
mod law_tests {
    use super::*;

    /// ⚖️ The inverse diffs sum to the negative of the forward diff (L3).
    #[semio_framework_async_macros::async_test]
    async fn inverse_diffs_sum_to_the_negative_diff() {
        let base = ImperativeConfig::default();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&ImperativeConfigMutation::SetContributions(SetContributions { json: "[1]".into() }), &base).await;
    }
}
