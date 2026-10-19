//! 📚️ Set active example work: the staged switch of the active example document in fixed steps and bounded chunks.

use super::*;
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dCommand;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dSetActiveExampleStage {
    DeleteAttractions,
    DeleteObjects,
    DeleteVolumes,
    DeleteReferences,
    DeleteCompatibility,
    Domain,
    Catalogs,
    CreateObjects,
    CreateAttractions,
    CreateVolumes,
    CreateReferences,
    CreateCompatibility,
    Publish,
    Complete,
    Closing,
}

/// 🔢️ Steps `Puzzle3dSetActiveExampleWork` spends on stage TRANSITIONS rather than on document items:
/// each of the five delete stages and each of the five create stages answers its own exhaustion with one
/// `Progress`, `Domain` and `Catalogs` are one step each, and `Publish` is the final `Complete` call —
/// 5 + 1 + 1 + 5 + 1 = 13. The extent is an upper bound on `step()` calls (the sibling `patchInspector`
/// law asserts `iterations <= extent`), so it has to carry all of them; it declared `3`, which is short
/// by ten and made a real example load overrun its own declared envelope.
const PUZZLE3D_SET_ACTIVE_EXAMPLE_FIXED_STEPS: usize = 13;

const PUZZLE3D_SET_ACTIVE_EXAMPLE_CHUNK: usize = 8;

/// ♻️ The owners one `Puzzle3dSetActiveExampleWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct Puzzle3dSetActiveExampleWorkOwners {
    pub(super) mutations: Vec<Puzzle3dMutation>,
}

pub struct Puzzle3dSetActiveExampleWork {
    pub(super) stage: Puzzle3dSetActiveExampleStage,
    pub(super) cursor: usize,
    pub(super) mutations: Vec<Puzzle3dMutation>,
    pub(super) close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dSetActiveExampleWorkOwners>,
}

impl Default for Puzzle3dSetActiveExampleWork {
    fn default() -> Self {
        Self { close_owners: Default::default(), stage: Puzzle3dSetActiveExampleStage::DeleteAttractions, cursor: 0, mutations: Vec::with_capacity(semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS) }
    }
}

impl Puzzle3dSetActiveExampleWork {
    pub(super) fn target(command: &Puzzle3dCommand) -> Option<&'static Puzzle3dSceneSnapshot> {
        match command.args().and_then(|args| args.get("exampleId")).and_then(Value::as_str).unwrap_or("") {
            "" => Some(&EMPTY_EXAMPLE_SNAPSHOT),
            PUZZLE3D_EXAMPLE_CONCRETE_FOREST | "concrete" => Some(&CONCRETE_FOREST_EXAMPLE_SNAPSHOT),
            PUZZLE3D_EXAMPLE_NAKAGIN | "nakagin" => Some(&NAKAGIN_EXAMPLE_SNAPSHOT),
            _ => None,
        }
    }

    /// 🏷️ The CANONICAL example id this command loads, for the alias the picker may have sent
    /// (`concrete`/`nakagin`). Persisted on the shared config so `export_snapshot` can name its
    /// download after the example instead of one constant filename.
    fn canonical_example_id(command: &Puzzle3dCommand) -> Option<&'static str> {
        match command.args().and_then(|args| args.get("exampleId")).and_then(Value::as_str).unwrap_or("") {
            "" => Some(""),
            PUZZLE3D_EXAMPLE_CONCRETE_FOREST | "concrete" => Some(PUZZLE3D_EXAMPLE_CONCRETE_FOREST),
            PUZZLE3D_EXAMPLE_NAKAGIN | "nakagin" => Some(PUZZLE3D_EXAMPLE_NAKAGIN),
            _ => None,
        }
    }

    fn compatibility_rows(target: &Puzzle3dSceneSnapshot) -> &[semio_framework_value::DslValue] {
        target.meta.kind_compatibility.as_ref().and_then(semio_framework_value::DslValue::as_array).unwrap_or_default()
    }

    pub(super) fn push(&mut self, mutation: Puzzle3dMutation) -> Result<(), Fault> {
        if self.mutations.len() >= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle3d-set-active-example-output-capacity"));
        }
        self.mutations.push(mutation);
        Ok(())
    }

    fn take_chunk(cursor: &mut usize, len: usize) -> std::ops::Range<usize> {
        if *cursor >= len {
            return *cursor..*cursor;
        }
        let end = cursor.saturating_add(PUZZLE3D_SET_ACTIVE_EXAMPLE_CHUNK).min(len);
        let range = *cursor..end;
        *cursor = end;
        range
    }

    fn advance(&mut self, stage: Puzzle3dSetActiveExampleStage) {
        self.cursor = 0;
        self.stage = stage;
    }
}

impl<A: Puzzle3dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle3dSetActiveExampleWork {
    fn tool_id(&self) -> &'static str {
        "setActiveExample"
    }

    fn extent(&self, command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let target = Self::target(command)?;
        let document = snapshot.typed();
        let items = document
            .attractions
            .len()
            .checked_add(document.objects.len())?
            .checked_add(document.target_volumes.len())?
            .checked_add(document.references.len())?
            .checked_add(document.meta.kind_compatibility.len())?
            .checked_add(target.attractions.len())?
            .checked_add(target.objects.len())?
            .checked_add(target.target_volumes.len())?
            .checked_add(target.references.len())?
            .checked_add(Self::compatibility_rows(target).len())?
            .checked_add(PUZZLE3D_SET_ACTIVE_EXAMPLE_FIXED_STEPS)?;
        (items <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_plugin_puzzle_retained::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let Some(target) = Self::target(command) else { return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::default())) };
        match self.stage {
            Puzzle3dSetActiveExampleStage::DeleteAttractions => {
                let items = &snapshot.typed().attractions;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for attraction in &items[range] {
                        self.push(semio_s_artifact_puzzle_3d_mutations_vortex::disconnect_vortices::mutation::disconnect_vortices(attraction.id.clone()))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-delete-attraction", "Removing old attraction", "Alte Anziehung wird entfernt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::DeleteObjects);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-delete-object", "Removing old object", "Altes Objekt wird entfernt"))
            }
            Puzzle3dSetActiveExampleStage::DeleteObjects => {
                let items = &snapshot.typed().objects;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for object in &items[range] {
                        self.push(semio_s_artifact_puzzle_3d_mutations_object::delete_object::mutation::delete_object(object.id.clone()))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-delete-object", "Removing old object", "Altes Objekt wird entfernt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::DeleteVolumes);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-delete-volume", "Removing old target volume", "Altes Zielvolumen wird entfernt"))
            }
            Puzzle3dSetActiveExampleStage::DeleteVolumes => {
                let items = &snapshot.typed().target_volumes;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for volume in &items[range] {
                        self.push(semio_s_artifact_puzzle_3d_mutations_volume::delete_target_volume::mutation::delete_target_volume(volume.id.clone()))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-delete-volume", "Removing old target volume", "Altes Zielvolumen wird entfernt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::DeleteReferences);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-delete-reference", "Removing old reference", "Alte Referenz wird entfernt"))
            }
            Puzzle3dSetActiveExampleStage::DeleteReferences => {
                let items = &snapshot.typed().references;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for reference in &items[range] {
                        self.push(semio_s_artifact_puzzle_3d_mutations_reference::delete_reference::mutation::delete_reference(reference.id.clone()))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-delete-reference", "Removing old reference", "Alte Referenz wird entfernt"))
                }
                self.advance(Puzzle3dSetActiveExampleStage::DeleteCompatibility);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-delete-compatibility", "Removing old compatibility", "Alte Kompatibilität wird entfernt"))
            }
            Puzzle3dSetActiveExampleStage::DeleteCompatibility => {
                let items = &snapshot.typed().meta.kind_compatibility;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for row in &items[range] {
                        self.push(semio_s_artifact_puzzle_3d_mutations_catalog::disconnect_kind_compatibility::mutation::disconnect_kind_compatibility(row.source.clone(), row.target.clone()))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-delete-compatibility", "Removing old compatibility", "Alte Kompatibilität wird entfernt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::Domain);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-domain", "Updating document domain", "Dokumentdomäne wird aktualisiert"))
            }
            Puzzle3dSetActiveExampleStage::Domain => {
                self.push(semio_s_artifact_puzzle_3d_mutations_catalog::change_domain::mutation::change_domain(target.domain.clone()))?;
                self.advance(Puzzle3dSetActiveExampleStage::Catalogs);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-catalogs", "Updating kind catalogs", "Artenkataloge werden aktualisiert"))
            }
            Puzzle3dSetActiveExampleStage::Catalogs => {
                let catalogs = target.meta.kind_catalogs.as_ref().map(|catalogs| FromValue::from_value(catalogs.clone())).transpose().map_err(|_| Fault::from("puzzle3d-set-active-example-catalogs-malformed"))?;
                self.push(semio_s_artifact_puzzle_3d_mutations_catalog::replace_kind_catalogs::mutation::replace_kind_catalogs(catalogs))?;
                self.advance(Puzzle3dSetActiveExampleStage::CreateObjects);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-create-object", "Adding example object", "Beispielobjekt wird hinzugefügt"))
            }
            Puzzle3dSetActiveExampleStage::CreateObjects => {
                let items = &target.objects;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for object in &items[range] {
                        let value = semio_framework_value::ToValue::to_value(object);
                        let object = FromValue::from_value(value).map_err(|_| Fault::from("puzzle3d-set-active-example-object-malformed"))?;
                        self.push(semio_s_artifact_puzzle_3d_mutations_object::create_object::mutation::create_object(object, None))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-create-object", "Adding example object", "Beispielobjekt wird hinzugefügt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::CreateAttractions);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-create-attraction", "Adding example attraction", "Beispielanziehung wird hinzugefügt"))
            }
            Puzzle3dSetActiveExampleStage::CreateAttractions => {
                let items = &target.attractions;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for attraction in &items[range] {
                        self.push(semio_s_artifact_puzzle_3d_mutations_vortex::connect_vortices::mutation::connect_vortices(
                            attraction.id.clone(),
                            attraction.attracting.clone(),
                            attraction.attracted.clone(),
                            attraction.gap,
                            attraction.shift,
                            attraction.rise,
                            attraction.rotation,
                            attraction.turn,
                            attraction.tilt,
                            0.0,
                            0.0, None,
                        ))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-create-attraction", "Adding example attraction", "Beispielanziehung wird hinzugefügt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::CreateVolumes);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-create-volume", "Adding example target volume", "Beispielzielvolumen wird hinzugefügt"))
            }
            Puzzle3dSetActiveExampleStage::CreateVolumes => {
                let items = &target.target_volumes;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for volume in &items[range] {
                        let value = semio_framework_value::ToValue::to_value(volume);
                        let volume = FromValue::from_value(value).map_err(|_| Fault::from("puzzle3d-set-active-example-volume-malformed"))?;
                        self.push(semio_s_artifact_puzzle_3d_mutations_volume::create_target_volume::mutation::create_target_volume(volume, None))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-create-volume", "Adding example target volume", "Beispielzielvolumen wird hinzugefügt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::CreateReferences);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-create-reference", "Adding example reference", "Beispielreferenz wird hinzugefügt"))
            }
            Puzzle3dSetActiveExampleStage::CreateReferences => {
                let items = &target.references;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for reference in &items[range] {
                        let value = semio_framework_value::ToValue::to_value(reference);
                        let reference = FromValue::from_value(value).map_err(|_| Fault::from("puzzle3d-set-active-example-reference-malformed"))?;
                        self.push(semio_s_artifact_puzzle_3d_mutations_reference::create_reference::mutation::create_reference(reference, None))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-create-reference", "Adding example reference", "Beispielreferenz wird hinzugefügt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::CreateCompatibility);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-create-compatibility", "Adding example compatibility", "Beispielkompatibilität wird hinzugefügt"))
            }
            Puzzle3dSetActiveExampleStage::CreateCompatibility => {
                let items = Self::compatibility_rows(target);
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for row in items[range].iter().cloned() {
                        let row: semio_s_artifact_puzzle_3d_model::Puzzle3dKindCompatibility = FromValue::from_value(row).map_err(|_| Fault::from("puzzle3d-set-active-example-compatibility-malformed"))?;
                        self.push(semio_s_artifact_puzzle_3d_mutations_catalog::connect_kind_compatibility::mutation::connect_kind_compatibility(row.source, row.target, row.bidirectional, row.important, row.specificity, None))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-create-compatibility", "Adding example compatibility", "Beispielkompatibilität wird hinzugefügt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::Publish);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-publish", "Publishing example", "Beispiel wird veröffentlicht"))
            }
            Puzzle3dSetActiveExampleStage::Publish => {
                self.stage = Puzzle3dSetActiveExampleStage::Complete;
                let example_id = Self::canonical_example_id(command).unwrap_or_default();
                let config_mutations = if config.active_example_id == example_id {
                    Vec::new()
                } else {
                    vec![Puzzle3dConfigMutation::SetActiveExampleId(Puzzle3dConfigSetActiveExampleId{ value: example_id.to_string() })]
                };
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit {
                    artifact_mutations: std::mem::take(&mut self.mutations),
                    config_mutations,
                    ui_scope: puzzle3d_scope(Puzzle3dScopeClass::Chrome),
                    ..Default::default()
                }))
            }
            Puzzle3dSetActiveExampleStage::Complete => Err(Fault::from("puzzle3d-set-active-example-complete-repolled")),
            Puzzle3dSetActiveExampleStage::Closing => Err(Fault::from("puzzle3d-set-active-example-closing")),
        }
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dSetActiveExampleStage::Closing;
        self.close_owners.stage(Puzzle3dSetActiveExampleWorkOwners { mutations: std::mem::take(&mut self.mutations) });
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|{
            self.close_owners.close_step(grant)
        })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle3dSetActiveExampleStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
