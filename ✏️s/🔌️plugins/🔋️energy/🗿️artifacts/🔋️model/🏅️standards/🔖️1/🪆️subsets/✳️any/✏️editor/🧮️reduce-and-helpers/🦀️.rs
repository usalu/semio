//! 🧮️ The editor reducer, its run-period, ordering and lookup helpers and the camera emit.

use super::*;
use semio_framework_artifact_reference::io::text::artifact_reference::ArtifactReferenceText;

/// 📅️ The run-period field leaves that carry `was` to `now`, ordered so every intermediate period is a calendar interval
/// whenever such an order exists — a field leaf refuses a period whose bounds disagree (`mutation.target-mismatch`), so
/// moving a whole period later sets its end before its start.
fn run_period_steps(was: crate::calendar::RunPeriod, now: crate::calendar::RunPeriod) -> Vec<EnergyModelMutation> {
    let take = |period: &mut crate::calendar::RunPeriod, field: usize| match field {
        0 => period.start_month = now.start_month,
        1 => period.start_day = now.start_day,
        2 => period.end_month = now.end_month,
        3 => period.end_day = now.end_day,
        _ => period.year = now.year,
    };
    let changed: Vec<usize> = (0..5)
        .filter(|field| {
            let mut probe = was;
            take(&mut probe, *field);
            probe != was
        })
        .collect();
    let valid = |order: &Vec<usize>| {
        let mut period = was;
        order.iter().all(|field| {
            take(&mut period, *field);
            period.is_interval()
        })
    };
    let order = orderings(&changed).into_iter().find(valid).unwrap_or(changed);
    order
        .into_iter()
        .map(|field| match field {
            0 => crate::run_mutations::change_run_start_month(now.start_month),
            1 => crate::run_mutations::change_run_start_day(now.start_day),
            2 => crate::run_mutations::change_run_end_month(now.end_month),
            3 => crate::run_mutations::change_run_end_day(now.end_day),
            _ => crate::run_mutations::change_run_year(now.year),
        })
        .collect()
}

/// 🔀️ Every ordering of `items`, the given order first (at most 5! = 120 for the run period's fields).
fn orderings(items: &[usize]) -> Vec<Vec<usize>> {
    if items.len() < 2 {
        return vec![items.to_vec()];
    }
    (0..items.len())
        .flat_map(|index| {
            let rest: Vec<usize> = items.iter().enumerate().filter(|(other, _)| *other != index).map(|(_, item)| *item).collect();
            orderings(&rest).into_iter().map(move |tail| std::iter::once(items[index]).chain(tail).collect::<Vec<usize>>())
        })
        .collect()
}

/// 🚧️ The interzone partner an `OutsideBoundary` carries — the boundary mutation names the tag
/// through `OutsideBoundaryKind` and the partner through this separate optional id, because
/// `dsl::DslScalar` binds unit variants only.
pub(super) fn interzone_partner(boundary: OutsideBoundary) -> Option<EntityId> {
    match boundary {
        OutsideBoundary::Interzone(partner) => Some(partner),
        _ => None,
    }
}

/// 📂️ The sanctioned whole-document load: a `kernel::Effect::LoadDocument` carrying a genesis
/// pack+spr the host swaps into the live store through `ArtifactStore::reset`, OUTSIDE undo history.
/// This is why `📚️examples` need no `replace-model` kind — whole-document replace has no mutation
/// representative in this artifact's vocabulary at all (`📓️derivation-rules.md` rule 6), exactly as
/// in `📐️cad`'s `reset_document_effect` and `🔱️trinity`'s. A freshly minted envelope has no edits,
/// so its spr encode is infallible.
fn load_document_effect(model: &crate::model::Model) -> semio_framework_plugin::kernel::Effect {
    let snapshot = EnergyModelSnapshot::from_model(model).expect("a registered engine model is an admissible native document");
    let pack = <EnergyModelSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&snapshot);
    let envelope = store::os_store::document_materialize::create_document_envelope::<EnergyModelSnapshot, EnergyModelMutation>(ENERGY_MODEL_DOCUMENT_SCHEMA, "model", snapshot, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).into_owners();
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::print_document_spr(&envelope)).expect("energy model document spr encode is infallible for a fresh, edit-free envelope");
    semio_framework_plugin::kernel::Effect::LoadDocument { pack, spr }
}

/// 🪜️ The one mutation `step` when the edit `differs` from the stored value, none when it restates it — an inspector edit that
/// repeats the current value is a quiet no-op, never a history row.
pub(super) fn step_if(differs: bool, step: impl FnOnce() -> EnergyModelMutation) -> Vec<EnergyModelMutation> {
    if differs {
        vec![step()]
    } else {
        Vec::new()
    }
}

/// 🆔️ Mints the next free `EntityId` for a collection addressed by its own ids — `max + 1`, so an
/// id is never reused after a delete and every reference in the document stays unambiguous.
fn next_entity_id(existing: impl Iterator<Item = EntityId>) -> EntityId {
    EntityId(existing.map(|id| id.0).max().unwrap_or(0).saturating_add(1))
}

pub(super) fn surface_class_from_id(id: &str) -> Option<SurfaceClass> {
    Some(match id {
        "exteriorWall" => SurfaceClass::ExteriorWall,
        "interiorWall" => SurfaceClass::InteriorWall,
        "roof" => SurfaceClass::Roof,
        "ceiling" => SurfaceClass::Ceiling,
        "floor" => SurfaceClass::Floor,
        "interzone" => SurfaceClass::Interzone,
        "adiabatic" => SurfaceClass::Adiabatic,
        "ground" => SurfaceClass::Ground,
        _ => return None,
    })
}

pub(super) fn target_missing(entity: &str, id: u32) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("mutation.target-missing"), format!("the energy model has no {entity} with id {id}"))
}

fn target_in_use(entity: &str, id: u32, blocker: &str) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("app.command.target-in-use"), format!("{entity} {id} is still referenced by at least one {blocker}"))
}
//#endregion 🧬️MutationSeam

/// 🧩️ The one pure reducer both `ArtifactEditor::handle` and the retained bounded work step run —
/// identical semantics on the interactive path and on the retained path by construction. Each command is read against the live
/// snapshot and answers the concrete semantic mutations that carry it, one per field it changes; nothing is copied or differenced.
pub(super) fn reduce(command: &EnergyModelEditorCommand, doc: &ArtifactView<'_, EnergyModelSnapshot>) -> Result<Emit<EnergyModelMutation, EnergyModelConfigMutation>, Fault> {
    let model = &doc.snapshot.model;
    let steps = match command {
        EnergyModelEditorCommand::SetStructureField { field, value } => match field.as_str() {
            "name" => step_if(model.name != *value, || crate::model_mutations::rename_model(value.clone())),
            "version" => step_if(model.version != *value, || crate::model_mutations::change_model_version(value.clone())),
            _ => return Ok(Emit::default()),
        },
        EnergyModelEditorCommand::SetZoneCell { row, column, value } => {
            let Some(zone) = model.zones.get(*row as usize) else { return Ok(Emit::default()) };
            let id = zone.id;
            match column.as_str() {
                "name" => step_if(zone.name != *value, || crate::zone_mutations::rename_zone(id, value.clone())),
                "volumeM3" => match value.parse::<f64>() {
                    Ok(parsed) => step_if(zone.volume_m3 != parsed, || crate::zone_mutations::change_zone_volume(id, parsed)),
                    Err(_) => return Ok(Emit::default()),
                },
                "multiplier" => match value.parse::<u32>() {
                    Ok(parsed) => step_if(zone.multiplier != parsed, || crate::zone_mutations::change_zone_multiplier(id, parsed)),
                    Err(_) => return Ok(Emit::default()),
                },
                "conditioned" => match value.parse::<bool>() {
                    Ok(parsed) => step_if(zone.conditioned != parsed, || crate::zone_mutations::change_zone_conditioned(id, parsed)),
                    Err(_) => return Ok(Emit::default()),
                },
                "partOfTotalFloorArea" => match value.parse::<bool>() {
                    Ok(parsed) => step_if(zone.part_of_total_floor_area != parsed, || crate::zone_mutations::change_zone_floor_area_participation(id, parsed)),
                    Err(_) => return Ok(Emit::default()),
                },
                _ => return Ok(Emit::default()),
            }
        }
        EnergyModelEditorCommand::CreateZone { name, volume_m3, multiplier, conditioned } => {
            if *volume_m3 <= 0.0 {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), "a zone volume must be strictly positive"));
            }
            let id = next_entity_id(model.zones.iter().map(|zone| zone.id));
            vec![crate::zone_mutations::create_zone(id, name.clone(), *volume_m3, (*multiplier).max(1), *conditioned, true, None)]
        }
        EnergyModelEditorCommand::RenameZone { zone, new_name } => {
            let target = model.zones.iter().find(|entry| entry.id.0 == *zone).ok_or_else(|| target_missing("zone", *zone))?;
            step_if(target.name != *new_name, || crate::zone_mutations::rename_zone(target.id, new_name.clone()))
        }
        EnergyModelEditorCommand::DeleteZone { zone } => {
            let id = EntityId(*zone);
            if !model.zones.iter().any(|entry| entry.id == id) {
                return Err(target_missing("zone", *zone));
            }
            if model.spaces.iter().any(|space| space.zone_id == id) {
                return Err(target_in_use("zone", *zone, "space"));
            }
            if model.surfaces.iter().any(|surface| surface.zone_id == id) {
                return Err(target_in_use("zone", *zone, "surface"));
            }
            if model.thermostats.iter().any(|thermostat| thermostat.zone_id == id) {
                return Err(target_in_use("zone", *zone, "thermostat"));
            }
            vec![crate::zone_mutations::delete_zone(id)]
        }
        EnergyModelEditorCommand::CreateSurface { name, zone, construction, class } => {
            let zone_id = EntityId(*zone);
            let construction_id = EntityId(*construction);
            if !model.zones.iter().any(|entry| entry.id == zone_id) {
                return Err(target_missing("zone", *zone));
            }
            if !model.constructions.iter().any(|entry| entry.id == construction_id) {
                return Err(target_missing("construction", *construction));
            }
            let class = surface_class_from_id(class).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), format!("'{class}' is not a surface class")))?;
            let id = next_entity_id(model.surfaces.iter().map(|surface| surface.id));
            let vertices = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0], [0.0, 0.0, 1.0]];
            vec![crate::surface_mutations::create_surface(id, name.clone(), zone_id, class, vertices, construction_id, OutsideBoundary::OutdoorAir.kind(), None, true, true, 1, None)]
        }
        EnergyModelEditorCommand::DeleteSurface { surface } => {
            let id = EntityId(*surface);
            if !model.surfaces.iter().any(|entry| entry.id == id) {
                return Err(target_missing("surface", *surface));
            }
            vec![crate::surface_mutations::delete_surface(id)]
        }
        EnergyModelEditorCommand::AssignSurfaceConstruction { surface, construction } => {
            let construction_id = EntityId(*construction);
            if !model.constructions.iter().any(|entry| entry.id == construction_id) {
                return Err(target_missing("construction", *construction));
            }
            let target = model.surfaces.iter().find(|entry| entry.id.0 == *surface).ok_or_else(|| target_missing("surface", *surface))?;
            step_if(target.construction_id != construction_id, || crate::surface_properties_mutations::change_surface_construction(target.id, construction_id))
        }
        EnergyModelEditorCommand::SetMaterialProperty { material, property, value } => {
            let target = model.materials.iter().find(|entry| entry.id.0 == *material).ok_or_else(|| target_missing("material", *material))?;
            set_material_property(target, property, value)?
        }
        EnergyModelEditorCommand::SetConstructionProperty { construction, property, value } => set_construction_property(model, *construction, property, value)?,
        EnergyModelEditorCommand::SetSurfaceProperty { surface, property, value, partner_surface } => set_surface_property(model, *surface, property, value, *partner_surface)?,
        EnergyModelEditorCommand::SetFenestrationProperty { fenestration, property, value } => set_fenestration_property(model, *fenestration, property, value)?,
        EnergyModelEditorCommand::SetGlazingMaterialProperty { material, property, value } => {
            let target = model.glazing_materials.iter().find(|entry| entry.id.0 == *material).ok_or_else(|| target_missing("glazing material", *material))?;
            set_glazing_material_property(target, property, value)?
        }
        EnergyModelEditorCommand::SelectWeatherFile { value } => {
            let target=semio_framework_artifact_reference::ArtifactRef::parse_uri(value).map_err(|_|Fault::from("Invalid EPW artifact address. / Ungültige EPW-Artefaktadresse."))?;
            if target.dialect.artifact_kind!="s.stdio.epw"||target.dialect.standard!="energyplus"||target.dialect.subset!="*"{return Err(Fault::from("Select an EPW EnergyPlus artifact. / Ein EPW-EnergyPlus-Artefakt auswählen."));}
            vec![crate::model_mutations::bind_weather_file(store::ArtifactLink{target,pin:store::LinkPin::Head,role:crate::model_mutations::WEATHER_LINK_ROLE.into()})]
        }
        EnergyModelEditorCommand::ClearWeatherFile => step_if(doc.snapshot.weather_link.is_some(),crate::model_mutations::unbind_weather_file),
        EnergyModelEditorCommand::ApplyNativeMutation { .. } => return Err(Fault::from("app.command.original-admission-required")),
        EnergyModelEditorCommand::SetHvacProperty { system, property, value } => set_hvac_property(model, *system, property, value)?,
        EnergyModelEditorCommand::SetSpaceProperty { space, property, value } => set_space_property(model, *space, property, value)?,
        EnergyModelEditorCommand::SetLoadProperty { load, property, value } => set_load_property(model, *load, property, value)?,
        EnergyModelEditorCommand::SetShadingProperty { shading, property, value } => set_shading_property(model, *shading, property, value)?,
        EnergyModelEditorCommand::SetGasMaterialProperty { material, property, value } => {
            let target = model.gas_materials.iter().find(|entry| entry.id.0 == *material).ok_or_else(|| target_missing("gas material", *material))?;
            set_gas_material_property(target, property, value)?
        }
        EnergyModelEditorCommand::SetZoneProperty { zone, property, value } => {
            let target = model.zones.iter().find(|entry| entry.id.0 == *zone).ok_or_else(|| target_missing("zone", *zone))?;
            set_zone_property(target, property, value)?
        }
        EnergyModelEditorCommand::SetThermostatSetpoints { thermostat, zone, heating_schedule, cooling_schedule, heating_throttle_range_k, cooling_throttle_range_k } => {
            if !model.zones.iter().any(|entry| entry.id.0 == *zone) { return Err(target_missing("zone", *zone)); }
            let schedules = &model.schedules;
            if !schedule_exists(schedules, *heating_schedule) {
                return Err(target_missing("schedule", *heating_schedule));
            }
            if !schedule_exists(schedules, *cooling_schedule) {
                return Err(target_missing("schedule", *cooling_schedule));
            }
            let target = model.thermostats.iter().find(|entry| entry.id.0 == *thermostat).ok_or_else(|| target_missing("thermostat", *thermostat))?;
            let id = target.id;
            [
                step_if(target.zone_id.0 != *zone, || crate::thermostat_mutations::change_thermostat_zone(id, EntityId(*zone))),
                step_if(target.heating_setpoint_schedule_id != ScheduleId(*heating_schedule), || crate::thermostat_mutations::change_thermostat_heating_setpoint_schedule(id, ScheduleId(*heating_schedule))),
                step_if(target.cooling_setpoint_schedule_id != ScheduleId(*cooling_schedule), || crate::thermostat_mutations::change_thermostat_cooling_setpoint_schedule(id, ScheduleId(*cooling_schedule))),
                step_if(target.heating_throttle_range_k != *heating_throttle_range_k, || crate::thermostat_mutations::change_thermostat_heating_throttle_range(id, *heating_throttle_range_k)),
                step_if(target.cooling_throttle_range_k != *cooling_throttle_range_k, || crate::thermostat_mutations::change_thermostat_cooling_throttle_range(id, *cooling_throttle_range_k)),
            ]
            .concat()
        }
        EnergyModelEditorCommand::SetSite { latitude_deg, longitude_deg, elevation_m, time_zone_hours, north_axis_deg } => {
            if latitude_deg.is_some_and(|value| !(-90.0..=90.0).contains(&value)) || longitude_deg.is_some_and(|value| !(-180.0..=180.0).contains(&value)) {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), "the site latitude/longitude are outside their SI ranges"));
            }
            let site = &model.site;
            [
                latitude_deg.filter(|value| *value != site.latitude_deg).map(crate::site_mutations::change_site_latitude),
                longitude_deg.filter(|value| *value != site.longitude_deg).map(crate::site_mutations::change_site_longitude),
                elevation_m.filter(|value| *value != site.elevation_m).map(crate::site_mutations::change_site_elevation),
                time_zone_hours.filter(|value| *value != site.time_zone_hours).map(crate::site_mutations::change_site_time_zone),
                north_axis_deg.filter(|value| *value != site.north_axis_deg).map(crate::site_mutations::change_site_north_axis),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
        EnergyModelEditorCommand::SetRunPeriod { start_month, start_day, end_month, end_day } => {
            let valid = (1..=12).contains(start_month) && (1..=12).contains(end_month) && (1..=31).contains(start_day) && (1..=31).contains(end_day);
            if !valid {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), "the run period must address real calendar months and days"));
            }
            let now = crate::calendar::RunPeriod { start_month: *start_month as u8, start_day: *start_day as u8, end_month: *end_month as u8, end_day: *end_day as u8, year: model.run_period.year };
            run_period_steps(model.run_period, now)
        }
        EnergyModelEditorCommand::SetActiveExample { example_id } => {
            let loaded = example_model(example_id).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("mutation.target-missing"), format!("this artifact bundles no example {example_id:?}")))?;
            return Ok(Emit { effects: vec![load_document_effect(&loaded)], ..Default::default() });
        }
        EnergyModelEditorCommand::SetSimulationSettings { zone_timestep_minutes, system_timestep_minutes, warmup_days } => {
            let settings = ChangeSimulationSettings { zone_timestep_minutes: *zone_timestep_minutes, system_timestep_minutes: *system_timestep_minutes, warmup_days: *warmup_days };
            if !settings.config().is_valid() {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), "the simulation settings are outside the engine's admissible timestep and warmup ranges"));
            }
            return Ok(Emit { config_mutations: vec![EnergyModelConfigMutation::ChangeSimulationSettings(settings)], ..Default::default() });
        }
        EnergyModelEditorCommand::SetResultField { field } => {
            let Some(selected) = crate::editor_config::ResultField::from_id(field) else {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), format!("'{field}' is not a published per-surface result field")));
            };
            return Ok(Emit {
                config_mutations: vec![EnergyModelConfigMutation::ChangeResultField(ChangeResultField { field: selected.id().to_string() })],
                ui_scope: UiDirtyScope::Partial {
                    window_bodies: vec![crate::session::ENERGY_MODEL_3D_WINDOW_KIND_ID.to_string(), simulation::BODY_KEY.to_string()],
                    panel_bodies: Vec::new(),
                    utilities: false,
                    tools: false,
                    engagements: false,
                    measures: false,
                    labels: false,
                },
                ..Default::default()
            });
        }
        EnergyModelEditorCommand::SetCamera { .. } => {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("energy.model.3d.window-required"), "a camera change requires a concrete 3d model window"));
        }
    };
    Ok(Emit { artifact_mutations: steps, ..Default::default() })
}

/// 🎥️ The window-addressed half of the command set: `setCamera` writes the addressed
/// `energy.model.3d` window's own retained pose, never the document and never the app config. Both
/// dispatch routes (`ArtifactEditor::handle` and the retained `energy_model_reduce`) call this FIRST
/// and only fall through to [`reduce`] when it answers `None`, so the two can never diverge.
///
/// 🐢️ `UiDirtyScope::Partial` with nothing listed: the pane that sent the pose already holds it, and
/// republishing the 3d body on every debounced orbit tick would be pure churn. A window that opens
/// later reads the stored pose on its own first render.
pub(super) fn camera_emit(command: &EnergyModelEditorCommand, view_state: Option<&semio_framework_plugin::ViewModel>) -> Option<Result<Emit<EnergyModelMutation, EnergyModelConfigMutation, NoDraftMutation>, Fault>> {
    let EnergyModelEditorCommand::SetCamera { camera } = command else { return None };
    Some((|| {
        let view = view_state.ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("energy.model.3d.window-required"), "a camera change requires a concrete 3d model window"))?;
        if camera.is_empty() {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), "setCamera carries no {position,target,zoom} pose"));
        }
        let value = semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(camera, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), format!("the camera pose is not a value: {error}")))?;
        let pose = <model_window::config::EnergyModelCameraPose as semio_framework_value::FromValue>::from_value(value).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), format!("the camera pose is malformed: {error}")))?;
        if !pose.is_valid() {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), "the camera pose is not finite, or its zoom is not positive"));
        }
        let mutation = model_window::config::EnergyModelWindowConfigMutation::SetCamera(model_window::config::SetCamera { camera: pose });
        Ok(Emit {
            window_config_mutations: vec![model_window::config::addressed(view, mutation)?],
            ui_scope: UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false },
            ..Default::default()
        })
    })())
}

/// 🗓️ A thermostat setpoint reference must resolve inside the model's own `ScheduleSet` — the five
/// schedule families are addressed by one shared `ScheduleId` space, so any of them may own it.
pub(super) fn schedule_exists(schedules: &crate::schedule::ScheduleSet, id: u32) -> bool {
    let id = ScheduleId(id);
    schedules.constants.iter().any(|schedule| schedule.id == id)
        || schedules.daily.iter().any(|schedule| schedule.id == id)
        || schedules.weekly.iter().any(|schedule| schedule.id == id)
        || schedules.annual.iter().any(|schedule| schedule.id == id)
        || schedules.time_series.iter().any(|schedule| schedule.id == id)
}
