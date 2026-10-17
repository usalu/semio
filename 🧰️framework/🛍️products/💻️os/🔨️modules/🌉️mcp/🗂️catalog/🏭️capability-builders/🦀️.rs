//! 🏭️ Capability builders of the catalog: capabilities from actions, commands, contributions and OS commands, the contribution rows, the humanizer and the dialog and artifact creation capabilities. See [`super`].

use super::*;

//#region 🔖️CapabilityBuilders
/// 🏭️ Shared action→capability projection used by both the per-plugin walk and the framework dedup
/// pass — `owner`/`artifact_kind`/`source` differ by caller, everything else (`title`/`description`/
/// `input_schema`/`effects`/`policy`/`execution`/`presentation`/`examples`) is derived from `action`
/// identically either way.
pub(super) fn capability_from_action(id: &str, owner: CapabilityOwner, artifact_kind: Option<String>, action: &manifest::ActionDefinition, source: CapabilitySource, locale: Locale, terminology: Terminology) -> CapabilityDefinition {
    let title = action.label.resolve(terminology, locale).to_string();
    let description = action.semantics.description.as_ref().map(|label| label.resolve(terminology, locale).to_string()).unwrap_or_default();
    CapabilityDefinition {
        id: CapabilityRef(id.to_string()),
        version: 1,
        owner,
        kind: action.kind.into(),
        audience: manifest::resolve_audience(action).into(),
        title,
        description,
        artifact_kind,
        use_when: action.semantics.use_when.clone(),
        input_schema: action_input_schema(id, &action.args, locale, terminology),
        output_schema: capability_generic_output_schema(id),
        effects: action.semantics.effects.clone(),
        policy: action.semantics.policy.clone(),
        execution: action.semantics.execution.clone(),
        exposure: ToolExposure::CatalogOnly,
        presentation: CapabilityPresentation {
            icon_id: Some(action.icon_id.as_str().to_string()),
            category: action.category.clone(),
            keys: action.keys.clone(),
            in_palette: action.in_palette,
            args: action.args.iter().map(|arg| CapabilityArgSummary { id: arg.id.clone(), label: arg.label.resolve(terminology, locale).to_string(), required: arg.required, definition: Some(arg.clone()) }).collect(),
        },
        examples: action.semantics.examples.iter().map(|request| CapabilityExample { request: request.clone(), input: serde_json::Value::Null }).collect(),
        source,
    }
}

/// 🏭️ `CommandDefinition`→`CapabilityDefinition` projection — mirrors `capability_from_action`, one
/// tier up the owner hierarchy (`app_id`/`mode_id` are `None` for a plugin-scope command).
#[allow(clippy::too_many_arguments)]
pub(super) fn capability_from_command(id: &str, owner: CapabilityOwner, artifact_kind: Option<String>, command: &manifest::CommandDefinition, source: CapabilitySource, locale: Locale, terminology: Terminology) -> CapabilityDefinition {
    let title = command.label.resolve(terminology, locale).to_string();
    let description = command.semantics.description.as_ref().map(|label| label.resolve(terminology, locale).to_string()).unwrap_or_default();
    CapabilityDefinition {
        id: CapabilityRef(id.to_string()),
        version: 1,
        owner,
        kind: command.kind.into(),
        audience: manifest::resolve_command_audience(command).into(),
        title,
        description,
        artifact_kind,
        use_when: command.semantics.use_when.clone(),
        input_schema: action_input_schema(id, &command.args, locale, terminology),
        output_schema: capability_generic_output_schema(id),
        effects: command.semantics.effects.clone(),
        policy: command.semantics.policy.clone(),
        execution: command.semantics.execution.clone(),
        exposure: ToolExposure::CatalogOnly,
        presentation: CapabilityPresentation {
            icon_id: Some(command.icon_id.as_str().to_string()),
            category: Some(command.category.clone()),
            keys: command.keybindings.first().map(|kb| kb.chord.clone()),
            in_palette: command.in_palette,
            args: command.args.iter().map(|arg| CapabilityArgSummary { id: arg.id.clone(), label: arg.label.resolve(terminology, locale).to_string(), required: arg.required, definition: Some(arg.clone()) }).collect(),
        },
        examples: command.semantics.examples.iter().map(|request| CapabilityExample { request: request.clone(), input: serde_json::Value::Null }).collect(),
        source,
    }
}

/// 🧩️ One typed contribution row, reduced to the three things a capability needs: a stable local
/// id, a human title, and the artifact kind it acts on. Implemented once per `ContributionSet`
/// category so {@link capability_from_contribution} stays a single generic function — the four
/// categories carry genuinely different shapes (an inference names schemas, an io entry names a
/// dialect pair), and flattening them back into one untyped row would discard exactly the fields
/// that make a contributed capability findable.
trait ContributionRow {
    /// 🪪️ Stable id segment, unique within `(plugin, category)`.
    fn row_id(&self) -> String;
    /// 🏷️ Human title for search and display.
    fn row_title(&self) -> String;
    /// 💬️ One sentence naming what this contribution does.
    fn row_description(&self) -> String;
    /// 🗿️ Artifact kind this row acts on, when the shape names one.
    fn row_artifact_kind(&self) -> Option<String> {
        None
    }
}

impl ContributionRow for manifest::ContributedInferenceMetadata {
    fn row_id(&self) -> String {
        self.inference_schema.clone()
    }

    fn row_title(&self) -> String {
        humanize(&self.inference_schema)
    }

    fn row_description(&self) -> String {
        format!("Infers {} on {} artifacts (schema {} v{}).", self.inference_schema, self.artifact_kind, self.artifact_schema, self.inference_schema_version)
    }

    fn row_artifact_kind(&self) -> Option<String> {
        Some(self.artifact_kind.clone())
    }
}

impl ContributionRow for manifest::ContributedMutationMetadata {
    fn row_id(&self) -> String {
        self.mutation_id.clone()
    }

    fn row_title(&self) -> String {
        format!("{} {}", humanize(&self.semantics.verb), humanize(&self.semantics.entity))
    }

    fn row_description(&self) -> String {
        format!("Contributed mutation: {} a {} ({}) on {}.", self.semantics.verb, self.semantics.entity, self.semantics.kind, self.semantics.record)
    }
}

impl ContributionRow for manifest::IoEntryDescriptor {
    fn row_id(&self) -> String {
        format!("{}:{}:{}", self.owner.to_coordinate(), direction_word(&self.direction), self.counterpart.to_coordinate())
    }

    fn row_title(&self) -> String {
        match self.direction {
            manifest::IoEntryDirection::Import => format!("Import {} as {}", self.counterpart.to_coordinate(), self.owner.to_coordinate()),
            manifest::IoEntryDirection::Export => format!("Export {} as {}", self.owner.to_coordinate(), self.counterpart.to_coordinate()),
        }
    }

    fn row_description(&self) -> String {
        match self.direction {
            manifest::IoEntryDirection::Import => format!("Reads {} into the {} artifact kind.", self.counterpart.to_coordinate(), self.owner.to_coordinate()),
            manifest::IoEntryDirection::Export => format!("Writes the {} artifact kind out as {}.", self.owner.to_coordinate(), self.counterpart.to_coordinate()),
        }
    }

    fn row_artifact_kind(&self) -> Option<String> {
        Some(self.owner.artifact_kind.clone())
    }
}

/// 🧭️ `Import`/`Export` as the lowercase word used inside a contributed io capability id.
fn direction_word(direction: &manifest::IoEntryDirection) -> &'static str {
    match direction {
        manifest::IoEntryDirection::Import => "import",
        manifest::IoEntryDirection::Export => "export",
    }
}

impl ContributionRow for manifest::ComposerEntryDescriptor {
    fn row_id(&self) -> String {
        self.writes.to_coordinate()
    }

    fn row_title(&self) -> String {
        format!("Compose {}", self.writes.to_coordinate())
    }

    fn row_description(&self) -> String {
        if self.reads.is_empty() {
            format!("Composes {}.", self.writes.to_coordinate())
        } else {
            format!("Composes {} from {}.", self.writes.to_coordinate(), self.reads.iter().map(|dialect| dialect.to_coordinate()).collect::<Vec<_>>().join(", "))
        }
    }

    fn row_artifact_kind(&self) -> Option<String> {
        Some(self.writes.artifact_kind.clone())
    }
}

/// 🏭️ One typed contribution row → a `Query`/`Job` capability. Generic over {@link ContributionRow}
/// so every `ContributionSet` category is projected by the same rules while keeping its own
/// identity and wording.
pub(super) fn capability_from_contribution<Row: ContributionRow>(plugin_id: &str, plugin_label: &str, category: &str, entry: &Row, kind: CapabilityKind) -> CapabilityDefinition {
    let row_id = entry.row_id();
    let id = format!("{plugin_id}.{category}.{row_id}");
    CapabilityDefinition {
        id: CapabilityRef(id.clone()),
        version: 1,
        owner: CapabilityOwner::Plugin { plugin_id: plugin_id.to_string(), label: Some(plugin_label.to_string()), app_id: None, window_kind_id: None, mode_id: None },
        kind,
        audience: CapabilityAudience::Agent,
        title: entry.row_title(),
        description: entry.row_description(),
        artifact_kind: entry.row_artifact_kind(),
        use_when: Vec::new(),
        input_schema: capability_generic_input_schema(&id),
        output_schema: capability_generic_output_schema(&id),
        effects: manifest::CapabilityEffects::default(),
        policy: manifest::CapabilityPolicy::default(),
        execution: manifest::CapabilityExecution { class: manifest::ExecutionClass::Job, interactive_job: manifest::InteractiveJobClassification::Migrated, ..Default::default() },
        exposure: ToolExposure::CatalogOnly,
        presentation: CapabilityPresentation { icon_id: None, category: Some(category.to_string()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Descriptor { category: category.to_string(), id: row_id },
    }
}

/// 🏭️ `CommandDefinition`→capability for one real `💻️os/🎮️commands/*` module — `owner: Os`, `source:
/// OsCommand`. The `id` is already fully qualified (`os.<slug>`, see every `pub const ID` in
/// `💻️os/🎮️commands/*/🦀️.rs`) — used as-is, never re-prefixed.
pub(super) fn capability_from_os_command(command: &manifest::CommandDefinition, locale: Locale, terminology: Terminology) -> CapabilityDefinition {
    capability_from_command(&command.id.clone(), CapabilityOwner::Os, None, command, CapabilitySource::OsCommand { id: command.id.clone() }, locale, terminology)
}

/// 🔤️ `"open-artifact"`/`"myThing"` → `"Open Artifact"`/`"My Thing"` — a last-resort title for
/// untyped `DescriptorEntry` rows that have no `LocalizedLabel` to resolve.
fn humanize(id: &str) -> String {
    let spaced: String = id.chars().flat_map(|character| if character.is_uppercase() { vec![' ', character] } else { vec![character] }).collect();
    spaced
        .replace(['-', '_'], " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
//#endregion 🔖️CapabilityBuilders

//#region 🔖️UiDialogAndArtifactCreate
/// 🗨️ `📋️master.md` §3.2: "`dialogs` → `ui.dialog.open`" — every app's declared `DialogDefinition`s
/// fold into ONE gateway-owned `ui.dialog.open` capability (never one capability per dialog), whose
/// `dialogId` argument enumerates every dialog id collected across every walked app.
pub fn ui_dialog_open_capability(dialog_ids: &[String]) -> CapabilityDefinition {
    let input_schema = ui_dialog_open_input_schema(dialog_ids);
    CapabilityDefinition {
        id: CapabilityRef("ui.dialog.open".to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Ui,
        audience: CapabilityAudience::Agent,
        title: "Open Dialog".to_string(),
        description: "Opens one of the workspace's declared modal form dialogs by id.".to_string(),
        artifact_kind: None,
        use_when: vec!["open a dialog".to_string(), "show a form".to_string()],
        input_schema,
        output_schema: capability_generic_output_schema("ui.dialog.open"),
        effects: manifest::CapabilityEffects { reads: vec![manifest::ResourceSelector::new("ui:window")], ..Default::default() },
        policy: manifest::CapabilityPolicy { scopes: vec![kernel::CapabilityId("ui.dialog".into())], ..Default::default() },
        execution: manifest::CapabilityExecution::default(),
        exposure: ToolExposure::CatalogOnly,
        presentation: CapabilityPresentation { icon_id: None, category: Some("ui".into()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Gateway,
    }
}

/// 📖️ `📋️master.md` §3.2: "`examples` → the `artifact.create` template enum" — every plugin's
/// declared `ExampleDefinition`s fold into ONE gateway-owned `artifact.create` capability, whose
/// `template` argument enumerates every `<plugin_id>:<example_id>` collected across every descriptor.
pub(super) fn artifact_create_capability(template_ids: &[String]) -> CapabilityDefinition {
    let input_schema = artifact_create_template_input_schema(template_ids);
    CapabilityDefinition {
        id: CapabilityRef("artifact.create".to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Job,
        audience: CapabilityAudience::Agent,
        title: "Create Artifact".to_string(),
        description: "Creates a new artifact, optionally seeded from a declared playground example/template.".to_string(),
        artifact_kind: None,
        use_when: vec!["create a new artifact".to_string(), "start a new document".to_string()],
        input_schema,
        output_schema: capability_generic_output_schema("artifact.create"),
        effects: manifest::CapabilityEffects { writes: vec![manifest::ResourceSelector::new("artifact:{self}")], reversible: false, ..Default::default() },
        policy: manifest::CapabilityPolicy { scopes: vec![kernel::CapabilityId("artifacts.write".into())], ..Default::default() },
        execution: manifest::CapabilityExecution { class: manifest::ExecutionClass::Job, interactive_job: manifest::InteractiveJobClassification::Migrated, ..Default::default() },
        exposure: ToolExposure::CatalogOnly,
        presentation: CapabilityPresentation { icon_id: None, category: Some("artifact".into()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Gateway,
    }
}
//#endregion 🔖️UiDialogAndArtifactCreate
