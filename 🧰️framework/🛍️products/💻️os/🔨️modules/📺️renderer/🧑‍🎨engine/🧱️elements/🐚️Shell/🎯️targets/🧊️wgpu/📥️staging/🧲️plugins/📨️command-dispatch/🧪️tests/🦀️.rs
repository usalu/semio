use super::scoped_contribution_command;
use semio_framework::manifest::{CommandOwnerAddress, PackageDescriptor, TopicContribution};
use semio_framework_value::DslValue;
use std::{fs, path::Path};

#[derive(serde::Deserialize)]
struct ScopeFixture {
    receiver: String,
    app: String,
    cases: Vec<ScopeCase>,
}

#[derive(serde::Deserialize)]
struct ScopeCase {
    consumes: Vec<String>,
    providers: Vec<(String, String)>,
}

#[test]
fn native_scope_delivers_only_admitted_workpiece_contributions() {
    let fixture: ScopeFixture = serde_json::from_str(include_str!("🧫️scope.json")).unwrap();
    let path = "✏️s/🔌️plugins/🏭️process/🧩️extensions";
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().find(|path| path.join("AGENTS.md").is_file() && path.join("✏️s").is_dir()).unwrap();
    for case in fixture.cases {
        let providers = case.providers.iter().map(|(folder, plugin_id)| {
            let descriptor: PackageDescriptor = serde_json::from_slice(&fs::read(workspace.join(path).join(folder).join("🧩️component/🔣️.json")).unwrap()).unwrap();
            assert_eq!(&descriptor.manifest.plugin_id, plugin_id);
            descriptor.manifest
        }).collect::<Vec<_>>();
        let borrowed = || providers.iter().map(|manifest| (manifest.plugin_id.as_str(), manifest.topic_contributions.as_slice()));
        let unrelated = [TopicContribution::new("flow.extension", DslValue::Object(Default::default()))];
        let (command, selected) = scoped_contribution_command(&fixture.receiver, &fixture.app, &case.consumes, None, borrowed().chain([("foreign", unrelated.as_slice())])).unwrap();
        let expected = case.providers.iter().map(|(_, id)| id.clone()).collect::<Vec<_>>();
        assert_eq!(selected, expected);
        assert_eq!(command.address.command_id, "setContributions");
        assert!(matches!(command.address.owner, CommandOwnerAddress::App { ref plugin_id, ref app_id } if plugin_id == &fixture.receiver && app_id == &fixture.app));
        let json = command.arguments["json"].as_str().unwrap();
        let entries = semio_framework::parse_contributions(json);
        assert_eq!(entries.len(), expected.len());
        for (entry, provider) in entries.iter().zip(&providers) {
            assert_eq!(entry.plugin_id, provider.plugin_id);
            assert_eq!(entry.topic_contribution.as_ref(), provider.topic_contributions.first());
        }
        let (_, enabled) = scoped_contribution_command(&fixture.receiver, &fixture.app, &case.consumes, None, borrowed().skip(1)).unwrap();
        assert_eq!(enabled, expected[1..]);
        assert!(scoped_contribution_command(&fixture.receiver, &fixture.app, &case.consumes, Some(&[]), borrowed()).is_none());
    }
}
