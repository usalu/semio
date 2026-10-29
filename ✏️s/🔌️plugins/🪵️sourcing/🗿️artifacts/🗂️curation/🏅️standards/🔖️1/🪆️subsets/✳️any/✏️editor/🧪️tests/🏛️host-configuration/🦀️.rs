//! 🏛️ Actual domain classification and owned native host configuration projection.
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin_host_configuration::{with_original_host_configuration,testkit::HeapWitness};
use semio_framework_value::DslValue;

#[global_allocator]
static HOST_CONFIGURATION_HEAP: HeapWitness = HeapWitness;

use semio_s_artifact_sourcing_curation::editor::sourcing::{SourcingCurationApp,config::{SourcingCurationConfigMutation,SetContributionsEdit}};

#[test]
fn original_host_configuration_preserves_sourcing_installable_config_lane() {
    let declaration=SourcingCurationApp::original_host_configuration("setContributions").expect("declared original host source");
    let foreign=r#"[{"id":"sourcing"}]"#;
    with_original_host_configuration(declaration,Some(DslValue::object([("json".to_owned(),DslValue::String(foreign.to_owned()))])),|mutation|{
        assert_eq!(mutation,&SourcingCurationConfigMutation::SetContributions(SetContributionsEdit{json:"[]".to_owned()}));
    });
    let installable=r#"[{"pluginId":"sourcing-salvaged","topicContribution":{"topic":"sourcing.module","payload":{"appId":"sourcing-curation","moduleId":"salvaged","label":"Salvaged","typologyJson":"{}","kindsJson":"[]"}}}]"#;
    with_original_host_configuration(declaration,Some(DslValue::object([("json".to_owned(),DslValue::String(installable.to_owned()))])),|mutation|{
        assert_eq!(mutation,&SourcingCurationConfigMutation::SetContributions(SetContributionsEdit{json:installable.to_owned()}));
    });
    assert!(SourcingCurationApp::original_host_configuration("setFilterQuery").is_none());
}
