//! 🔍️ The inspector property setters for surfaces, fenestrations, materials, zones and constructions.

use super::*;

//#region 🔍️InspectorProperties
/// ⛔️ `'{property}'` is not a field of `{entity}`.
fn unknown_property(entity: &str, property: &str) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), format!("'{property}' is not a {entity} property"))
}

/// ⛔️ `'{value}'` cannot be read as `{property}`, or lies outside its SI range.
fn invalid_value(property: &str, value: &str) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), format!("'{value}' is outside the admissible range of property '{property}'"))
}

pub(super) fn as_f64(property: &str, value: &str, admits: impl Fn(f64) -> bool) -> Result<f64, Fault> {
    let parsed: f64 = value.trim().parse().map_err(|_| invalid_value(property, value))?;
    if !parsed.is_finite() || !admits(parsed) {
        return Err(invalid_value(property, value));
    }
    Ok(parsed)
}

fn as_u32(property: &str, value: &str, admits: impl Fn(u32) -> bool) -> Result<u32, Fault> {
    let parsed: u32 = value.trim().parse().map_err(|_| invalid_value(property, value))?;
    if !admits(parsed) {
        return Err(invalid_value(property, value));
    }
    Ok(parsed)
}

pub(super) fn as_bool(property: &str, value: &str) -> Result<bool, Fault> {
    match value.trim() {
        "true" | "1" | "on" | "yes" => Ok(true),
        "false" | "0" | "off" | "no" | "" => Ok(false),
        _ => Err(invalid_value(property, value)),
    }
}

/// 🚧️ The wire spelling of an [`OutsideBoundaryKind`] — camelCase, the same vocabulary
/// [`surface_class_from_id`] uses for [`SurfaceClass`].
fn outside_boundary_kind_from_id(id: &str) -> Option<crate::model::OutsideBoundaryKind> {
    use crate::model::OutsideBoundaryKind as Kind;
    Some(match id {
        "outdoorAir" => Kind::OutdoorAir,
        "ground" => Kind::Ground,
        "otherSideTemperature" => Kind::OtherSideTemperature,
        "adiabatic" => Kind::Adiabatic,
        "interzone" => Kind::Interzone,
        _ => return None,
    })
}

/// 🟫️ Every `Surface` field the inspector addresses. `construction`/`boundary` validate their
/// references against the live model FIRST, so a dangling id is a refusal and never a document that
/// points at nothing.
pub(super) fn set_surface_property(model: &crate::model::Model, surface: u32, property: &str, value: &str, partner_surface: u32) -> Result<Vec<EnergyModelMutation>, Fault> {
    let boundary = match property {
        "boundary" => {
            let kind = outside_boundary_kind_from_id(value.trim()).ok_or_else(|| invalid_value(property, value))?;
            // 🚧️ An interzone boundary needs its other half. A control that carries only the kind can
            // name the partner nowhere, so a partner already on the surface is CARRIED FORWARD rather
            // than dropped — re-picking `interzone` on a surface that is already interzone keeps its
            // neighbour instead of refusing.
            let held = model.surfaces.iter().find(|entry| entry.id.0 == surface).and_then(|entry| interzone_partner(entry.outside_boundary_condition));
            let partner = (partner_surface != 0).then_some(EntityId(partner_surface)).or(held);
            if let Some(partner) = partner {
                if !model.surfaces.iter().any(|entry| entry.id == partner) {
                    return Err(target_missing("surface", partner.0));
                }
            }
            Some(OutsideBoundary::from_parts(kind, partner).ok_or_else(|| invalid_value(property, value))?)
        }
        // 🚧️ The one control that can MAKE a surface interzone: `value` is the partner surface, and the
        // boundary becomes `Interzone(partner)` in the same step. `partner_surface` stays the palette's
        // spelling; the inspector's flat `{field, id, value}` shape has only one slot to carry it in.
        "interzonePartner" => {
            let partner = EntityId(as_u32(property, value, |parsed| parsed != 0)?);
            if partner.0 == surface {
                return Err(invalid_value(property, value));
            }
            if !model.surfaces.iter().any(|entry| entry.id == partner) {
                return Err(target_missing("surface", partner.0));
            }
            Some(OutsideBoundary::Interzone(partner))
        }
        _ => None,
    };
    let construction = match property {
        "construction" => {
            let id = EntityId(as_u32(property, value, |_| true)?);
            if !model.constructions.iter().any(|entry| entry.id == id) {
                return Err(target_missing("construction", id.0));
            }
            Some(id)
        }
        _ => None,
    };
    let target = model.surfaces.iter().find(|entry| entry.id.0 == surface).ok_or_else(|| target_missing("surface", surface))?;
    let id = target.id;
    Ok(match property {
        "name" => step_if(target.name != value, || crate::surface_mutations::rename_surface(id, value.to_string())),
        "class" => {
            let class = surface_class_from_id(value.trim()).ok_or_else(|| invalid_value(property, value))?;
            step_if(target.class != class, || crate::surface_mutations::change_surface_class(id, class))
        }
        "boundary" | "interzonePartner" => {
            let boundary = boundary.expect("the boundary arm parsed its payload above");
            step_if(target.outside_boundary_condition != boundary, || crate::boundary_mutations::change_surface_boundary_condition(id, boundary.kind(), interzone_partner(boundary)))
        }
        "construction" => {
            let construction = construction.expect("the construction arm parsed its payload above");
            step_if(target.construction_id != construction, || crate::surface_mutations::change_surface_construction(id, construction))
        }
        "sunExposed" => {
            let exposed = as_bool(property, value)?;
            step_if(target.sun_exposed != exposed, || crate::boundary_mutations::change_surface_sun_exposed(id, exposed))
        }
        "windExposed" => {
            let exposed = as_bool(property, value)?;
            step_if(target.wind_exposed != exposed, || crate::boundary_mutations::change_surface_wind_exposed(id, exposed))
        }
        "multiplier" => {
            let multiplier = as_u32(property, value, |parsed| parsed >= 1)?;
            step_if(target.multiplier != multiplier, || crate::surface_mutations::change_surface_multiplier(id, multiplier))
        }
        _ => return Err(unknown_property("surface", property)),
    })
}

/// 🪟️ Every `Fenestration` field the inspector addresses — the fifteen scalars plus the optional
/// glazing construction, whose EMPTY value clears the binding rather than naming a construction.
pub(super) fn set_fenestration_property(model: &crate::model::Model, fenestration: u32, property: &str, value: &str) -> Result<Vec<EnergyModelMutation>, Fault> {
    let glazing = match property {
        "glazingConstruction" => {
            let trimmed = value.trim();
            if trimmed.is_empty() || trimmed == "none" {
                Some(None)
            } else {
                let id = EntityId(as_u32(property, value, |_| true)?);
                if !model.constructions.iter().any(|entry| entry.id == id) {
                    return Err(target_missing("construction", id.0));
                }
                Some(Some(id))
            }
        }
        _ => None,
    };
    let target = model.fenestrations.iter().find(|entry| entry.id.0 == fenestration).ok_or_else(|| target_missing("fenestration", fenestration))?;
    let id = target.id;
    let positive = |parsed: f64| parsed > 0.0;
    let non_negative = |parsed: f64| parsed >= 0.0;
    let fraction = |parsed: f64| (0.0..=1.0).contains(&parsed);
    Ok(match property {
        "name" => step_if(target.name != value, || crate::fenestration_mutations::rename_fenestration(id, value.to_string())),
        "uValueWM2K" => {
            let parsed = as_f64(property, value, positive)?;
            step_if(target.u_value_w_m2k != parsed, || crate::glazing_mutations::change_fenestration_u_value(id, parsed))
        }
        "shgc" => {
            let parsed = as_f64(property, value, fraction)?;
            step_if(target.shgc != parsed, || crate::glazing_mutations::change_fenestration_shgc(id, parsed))
        }
        "vlt" => {
            let parsed = as_f64(property, value, fraction)?;
            step_if(target.vlt != parsed, || crate::glazing_mutations::change_fenestration_vlt(id, parsed))
        }
        "areaM2" => {
            let parsed = as_f64(property, value, positive)?;
            step_if(target.area_m2 != parsed, || crate::fenestration_mutations::change_fenestration_area(id, parsed))
        }
        "heightM" => {
            let parsed = as_f64(property, value, positive)?;
            step_if(target.height_m != parsed, || crate::fenestration_mutations::change_fenestration_height(id, parsed))
        }
        "sillHeightM" => {
            let parsed = as_f64(property, value, non_negative)?;
            step_if(target.sill_height_m != parsed, || crate::fenestration_mutations::change_fenestration_sill_height(id, parsed))
        }
        "frameConductanceWK" => {
            let parsed = as_f64(property, value, non_negative)?;
            step_if(target.frame_conductance_w_k != parsed, || crate::glazing_mutations::change_fenestration_frame_conductance(id, parsed))
        }
        "dividerConductanceWK" => {
            let parsed = as_f64(property, value, non_negative)?;
            step_if(target.divider_conductance_w_k != parsed, || crate::glazing_mutations::change_fenestration_divider_conductance(id, parsed))
        }
        "overhangDepthM" => {
            let parsed = as_f64(property, value, non_negative)?;
            step_if(target.overhang_depth_m != parsed, || crate::shading_device_mutations::change_fenestration_overhang_depth(id, parsed))
        }
        "overhangOffsetM" => {
            let parsed = as_f64(property, value, non_negative)?;
            step_if(target.overhang_offset_m != parsed, || crate::shading_device_mutations::change_fenestration_overhang_offset(id, parsed))
        }
        "finDepthM" => {
            let parsed = as_f64(property, value, non_negative)?;
            step_if(target.fin_depth_m != parsed, || crate::shading_device_mutations::change_fenestration_fin_depth(id, parsed))
        }
        "finOffsetM" => {
            let parsed = as_f64(property, value, non_negative)?;
            step_if(target.fin_offset_m != parsed, || crate::shading_device_mutations::change_fenestration_fin_offset(id, parsed))
        }
        "glazingConstruction" => {
            let glazing = glazing.expect("the glazing arm parsed its payload above");
            step_if(target.glazing_construction_id != glazing, || match glazing {
                Some(construction) => crate::glazing_mutations::bind_fenestration_glazing_construction(id, construction),
                None => crate::glazing_mutations::clear_fenestration_glazing_construction(id),
            })
        }
        _ => return Err(unknown_property("fenestration", property)),
    })
}

/// 🧊️ The glazing-material fields this artifact's vocabulary can actually name. The seven other
/// optical scalars (`solar_reflectance_*`, `visible_reflectance_*`, `infrared_transmittance`) have no
/// mutation kind, so they are refused here rather than written and then faulted by the probe.
pub(super) fn set_glazing_material_property(material: &crate::model::GlazingMaterial, property: &str, value: &str) -> Result<Vec<EnergyModelMutation>, Fault> {
    let id = material.id;
    let positive = |parsed: f64| parsed > 0.0;
    let fraction = |parsed: f64| (0.0..=1.0).contains(&parsed);
    Ok(match property {
        "name" => step_if(material.name != value, || crate::window_layer_mutations::rename_glazing_material(id, value.to_string())),
        "thicknessM" => {
            let parsed = as_f64(property, value, positive)?;
            step_if(material.thickness_m != parsed, || crate::window_layer_mutations::change_glazing_material_thickness(id, parsed))
        }
        "conductivityWMK" => {
            let parsed = as_f64(property, value, positive)?;
            step_if(material.conductivity_w_m_k != parsed, || crate::window_layer_mutations::change_glazing_material_conductivity(id, parsed))
        }
        "solarTransmittance" => {
            let parsed = as_f64(property, value, fraction)?;
            step_if(material.solar_transmittance != parsed, || crate::window_layer_mutations::change_glazing_material_solar_transmittance(id, parsed))
        }
        "visibleTransmittance" => {
            let parsed = as_f64(property, value, fraction)?;
            step_if(material.visible_transmittance != parsed, || crate::window_layer_mutations::change_glazing_material_visible_transmittance(id, parsed))
        }
        "infraredEmissivityFront" => {
            let front = as_f64(property, value, fraction)?;
            step_if(material.infrared_emissivity_front != front, || crate::window_layer_mutations::change_glazing_material_infrared_emissivity(id, front, material.infrared_emissivity_back))
        }
        "infraredEmissivityBack" => {
            let back = as_f64(property, value, fraction)?;
            step_if(material.infrared_emissivity_back != back, || crate::window_layer_mutations::change_glazing_material_infrared_emissivity(id, material.infrared_emissivity_front, back))
        }
        _ => return Err(unknown_property("glazing material", property)),
    })
}

/// 💨️ The gas-gap fields: thickness, fill gas and name.
pub(super) fn set_gas_material_property(material: &crate::model::GasMaterial, property: &str, value: &str) -> Result<Vec<EnergyModelMutation>, Fault> {
    let id = material.id;
    Ok(match property {
        "name" => step_if(material.name != value, || crate::window_layer_mutations::rename_gas_material(id, value.to_string())),
        "thicknessM" => {
            let parsed = as_f64(property, value, |parsed| parsed > 0.0)?;
            step_if(material.thickness_m != parsed, || crate::window_layer_mutations::change_gas_material_thickness(id, parsed))
        }
        "gas" => {
            let gas = gas_kind_from_id(value.trim()).ok_or_else(|| invalid_value(property, value))?;
            step_if(material.gas != gas, || crate::window_layer_mutations::change_gas_material_gas(id, gas))
        }
        _ => return Err(unknown_property("gas material", property)),
    })
}


fn gas_kind_from_id(id: &str) -> Option<crate::model::GasKind> {
    use crate::model::GasKind as Kind;
    Some(match id {
        "air" => Kind::Air,
        "argon" => Kind::Argon,
        "krypton" => Kind::Krypton,
        "xenon" => Kind::Xenon,
        _ => return None,
    })
}


/// 🏘️ Every `Zone` field, addressed by the zone's own `EntityId` rather than by its table row.
pub(super) fn set_zone_property(zone: &Zone, property: &str, value: &str) -> Result<Vec<EnergyModelMutation>, Fault> {
    let id = zone.id;
    Ok(match property {
        "name" => step_if(zone.name != value, || crate::zone_mutations::rename_zone(id, value.to_string())),
        "volumeM3" => {
            let parsed = as_f64(property, value, |parsed| parsed > 0.0)?;
            step_if(zone.volume_m3 != parsed, || crate::zone_mutations::change_zone_volume(id, parsed))
        }
        "multiplier" => {
            let parsed = as_u32(property, value, |parsed| parsed >= 1)?;
            step_if(zone.multiplier != parsed, || crate::zone_mutations::change_zone_multiplier(id, parsed))
        }
        "conditioned" => {
            let parsed = as_bool(property, value)?;
            step_if(zone.conditioned != parsed, || crate::zone_mutations::change_zone_conditioned(id, parsed))
        }
        "partOfTotalFloorArea" => {
            let parsed = as_bool(property, value)?;
            step_if(zone.part_of_total_floor_area != parsed, || crate::zone_mutations::change_zone_floor_area_participation(id, parsed))
        }
        _ => return Err(unknown_property("zone", property)),
    })
}
//#endregion 🔍️InspectorProperties

/// 🧱️ The material record's WHOLE addressable surface: its name, its roughness class and its seven
/// SI scalars. `value` is text like every other inspector verb, so one control shape carries a name,
/// an enum spelling and a number alike; the numeric properties parse it and refuse a non-finite or
/// out-of-range reading rather than writing it.
pub(super) fn set_material_property(material: &Material, property: &str, value: &str) -> Result<Vec<EnergyModelMutation>, Fault> {
    let id = material.id;
    let positive = |parsed: f64| parsed > 0.0;
    let fraction = |parsed: f64| (0.0..=1.0).contains(&parsed);
    Ok(match property {
        "name" => step_if(material.name != value, || crate::material_mutations::rename_material(id, value.to_string())),
        "roughness" => {
            let roughness = surface_roughness_from_id(value.trim()).ok_or_else(|| invalid_value(property, value))?;
            step_if(material.roughness != roughness, || crate::finish_mutations::change_material_roughness(id, roughness))
        }
        "thicknessM" => {
            let parsed = as_f64(property, value, positive)?;
            step_if(material.thickness_m != parsed, || crate::material_mutations::change_material_thickness(id, parsed))
        }
        "conductivityWMK" => {
            let parsed = as_f64(property, value, positive)?;
            step_if(material.conductivity_w_m_k != parsed, || crate::material_mutations::change_material_conductivity(id, parsed))
        }
        "densityKgM3" => {
            let parsed = as_f64(property, value, positive)?;
            step_if(material.density_kg_m3 != parsed, || crate::material_mutations::change_material_density(id, parsed))
        }
        "specificHeatJKgK" => {
            let parsed = as_f64(property, value, positive)?;
            step_if(material.specific_heat_j_kg_k != parsed, || crate::material_mutations::change_material_specific_heat(id, parsed))
        }
        "thermalAbsorptance" => {
            let parsed = as_f64(property, value, fraction)?;
            step_if(material.thermal_absorptance != parsed, || crate::finish_mutations::change_material_thermal_absorptance(id, parsed))
        }
        "solarAbsorptance" => {
            let parsed = as_f64(property, value, fraction)?;
            step_if(material.solar_absorptance != parsed, || crate::finish_mutations::change_material_solar_absorptance(id, parsed))
        }
        "visibleAbsorptance" => {
            let parsed = as_f64(property, value, fraction)?;
            step_if(material.visible_absorptance != parsed, || crate::finish_mutations::change_material_visible_absorptance(id, parsed))
        }
        _ => return Err(unknown_property("material", property)),
    })
}

/// 🧱️ One construction field by name, or ONE list edit of its layer stack. The layer verbs carry
/// their operand in `value` because a rendered control merges exactly one scalar there:
/// `addLayer` = the material id to append, `removeLayer`/`moveLayerUp`/`moveLayerDown` = the layer
/// index, `replaceLayer:<index>` = the material id that takes that slot (the un-suffixed spelling
/// takes `<index>:<materialId>`, which is what a palette invocation types).
///
/// ⚠️ `add-construction-layer` admits an OPAQUE `Material` only — its own diff refuses a glazing or
/// gas id with `mutation.target-missing`. So an add/replace naming one is refused HERE, loudly and
/// early, instead of reducing cleanly and dying at the store.
pub(super) fn set_construction_property(model: &crate::model::Model, construction: u32, property: &str, value: &str) -> Result<Vec<EnergyModelMutation>, Fault> {
    let opaque: Vec<EntityId> = model.materials.iter().map(|material| material.id).collect();
    let target = model.constructions.iter().find(|entry| entry.id.0 == construction).ok_or_else(|| target_missing("construction", construction))?;
    let (id, layers) = (target.id, target.layer_material_ids.as_slice());
    let layer_material = |raw: &str| -> Result<EntityId, Fault> {
        let id = EntityId(as_u32(property, raw, |_| true)?);
        if !opaque.contains(&id) {
            return Err(Fault::new(
                FaultOrigin::App,
                FaultCode::new("app.command.invalid-payload"),
                format!("layer material {} is not an opaque material — 'add-construction-layer' declares no glazing or gas layer", id.0),
            ));
        }
        Ok(id)
    };
    let slot = |raw: &str, layers: &[EntityId]| -> Result<usize, Fault> {
        let at = as_u32(property, raw, |_| true)? as usize;
        if at >= layers.len() {
            return Err(invalid_value(property, raw));
        }
        Ok(at)
    };
    Ok(match property {
        "name" => step_if(target.name != value, || crate::construction_mutations::rename_construction(id, value.to_string())),
        "addLayer" => {
            let material = layer_material(value.trim())?;
            vec![crate::construction_mutations::add_construction_layer(id, layers.len() as u32, material)]
        }
        "removeLayer" => {
            let at = slot(value.trim(), layers)?;
            vec![crate::construction_mutations::remove_construction_layer(id, at as u32)]
        }
        "moveLayerUp" | "moveLayerDown" => {
            let at = slot(value.trim(), layers)?;
            let other = if property == "moveLayerUp" { at.checked_sub(1) } else { at.checked_add(1) };
            let Some(other) = other.filter(|other| *other < layers.len()) else { return Err(invalid_value(property, value)) };
            let mut swapped = layers.to_vec();
            swapped.swap(at, other);
            step_if(swapped != layers, || crate::construction_mutations::reorder_construction_layers(id, swapped.clone()))
        }
        _ if property == "replaceLayer" || property.starts_with("replaceLayer:") => {
            let (raw_index, raw_material) = match property.strip_prefix("replaceLayer:") {
                Some(at) => (at, value.trim()),
                None => value.trim().split_once(':').ok_or_else(|| invalid_value(property, value))?,
            };
            let at = slot(raw_index.trim(), layers)?;
            let material = layer_material(raw_material.trim())?;
            if layers[at] == material {
                Vec::new()
            } else {
                vec![crate::construction_mutations::remove_construction_layer(id, at as u32), crate::construction_mutations::add_construction_layer(id, at as u32, material)]
            }
        }
        _ => return Err(unknown_property("construction", property)),
    })
}


fn surface_roughness_from_id(id: &str) -> Option<crate::model::SurfaceRoughness> {
    use crate::model::SurfaceRoughness as Kind;
    Some(match id {
        "veryRough" => Kind::VeryRough,
        "rough" => Kind::Rough,
        "mediumRough" => Kind::MediumRough,
        "mediumSmooth" => Kind::MediumSmooth,
        "smooth" => Kind::Smooth,
        "verySmooth" => Kind::VerySmooth,
        _ => return None,
    })
}


//#endregion 🔖️Reduce
