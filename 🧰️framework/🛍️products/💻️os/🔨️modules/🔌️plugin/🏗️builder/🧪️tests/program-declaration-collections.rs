use super::*;
use serde_json::Value;
use semio_ui_locale::LocalizedLabel;

#[test]
fn plugin_manifest_keeps_authored_command_order() {
    let fixture: Value = serde_json::from_str(include_str!("🧫️fixtures/declaration-collections.json")).unwrap();
    let mut plugin = Plugin::<NoPluginApp>::new("declarations", "Declarations", "0.1.0");
    for value in fixture["commands"].as_array().unwrap() {
        let id = value.as_str().unwrap();
        let command = CommandDefinition::new(id, LocalizedLabel::data(id), "fixture", "pencil", ActionKind::View);
        plugin = plugin.plugin_command(command, Box::new(|_, _| unreachable!()));
    }
    let expected: Vec<_> = fixture["expectedCommands"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect();
    assert_eq!(plugin.manifest.commands.iter().map(|command| command.id.as_str()).collect::<Vec<_>>(), expected);
    assert!(expected.iter().all(|id| plugin.command_handlers.contains_key(*id)));
}
