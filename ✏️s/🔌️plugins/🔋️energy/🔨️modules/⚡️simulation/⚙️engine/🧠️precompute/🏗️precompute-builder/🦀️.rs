//! 🏗️ The staged `PrecomputeBuilder` cursor that derives geometry, constructions, glazing optics, shading casters and enclosures.

use super::*;

// #region 🔖️PrecomputeBuilder
/// 🧮️ Persistent one-record-at-a-time precomputation stage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub enum PrecomputeStage {
    ReserveBacking,
    IndexMaterials,
    IndexGlazingMaterials,
    IndexGasMaterials,
    IndexConstructions,
    IndexSurfaces,
    Zones,
    Surfaces,
    Thermostats,
    Windows,
    ShadingSurfaces,
    Projections,
    ReceiverCasters,
    Enclosures,
    SkyShading,
    Faults,
    Complete,
}

pub const P7C1_PRECOMPUTE_STAGES: [PrecomputeStage; 17] = [
    PrecomputeStage::ReserveBacking,
    PrecomputeStage::IndexMaterials,
    PrecomputeStage::IndexGlazingMaterials,
    PrecomputeStage::IndexGasMaterials,
    PrecomputeStage::IndexConstructions,
    PrecomputeStage::IndexSurfaces,
    PrecomputeStage::Zones,
    PrecomputeStage::Surfaces,
    PrecomputeStage::Thermostats,
    PrecomputeStage::Windows,
    PrecomputeStage::ShadingSurfaces,
    PrecomputeStage::Projections,
    PrecomputeStage::ReceiverCasters,
    PrecomputeStage::Enclosures,
    PrecomputeStage::SkyShading,
    PrecomputeStage::Faults,
    PrecomputeStage::Complete,
];

/// 🧮️ Cursor state for deterministic, resumable model precomputation.
#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct PrecomputeBuilder {
    output: PrecomputedModel,
    stage: PrecomputeStage,
    cursor: usize,
    reserve_cursor: u8,
    backing_rejected: bool,
    projection_cursor: u8,
}

impl PrecomputeBuilder {
    pub fn retained_wire_signature(&self) -> [u64; 8] {
        [
            self.stage as u64,
            self.cursor as u64,
            self.reserve_cursor as u64,
            self.backing_rejected as u64,
            self.projection_cursor as u64,
            self.output.casters.len() as u64,
            self.output.maximum_enclosure_faces as u64,
            self.output.zone_order.len().wrapping_add(self.output.surface_order.len()).wrapping_add(self.output.window_order.len()) as u64,
        ]
    }

    pub fn new(zone_timestep_minutes: u32, system_timestep_minutes: u32) -> Self {
        Self {
            output: PrecomputedModel { zone_timestep_s: zone_timestep_minutes.max(1) as f64 * 60.0, system_timestep_s: system_timestep_minutes.max(1) as f64 * 60.0, ..PrecomputedModel::default() },
            stage: PrecomputeStage::ReserveBacking,
            cursor: 0,
            reserve_cursor: 0,
            backing_rejected: false,
            projection_cursor: 0,
        }
    }

    pub fn stage(&self) -> PrecomputeStage {
        self.stage
    }

    pub fn set_stage_for_gate(&mut self, stage: PrecomputeStage) {
        self.stage = stage;
    }

    pub fn is_complete(&self) -> bool {
        self.stage == PrecomputeStage::Complete
    }

    pub fn backing_rejected(&self) -> bool {
        let tables = [
            self.output.zone_geometry.faulted(),
            self.output.surfaces.faulted(),
            self.output.windows.faulted(),
            self.output.enclosures.faulted(),
            self.output.default_setpoints.faulted(),
            self.output.zone_indices.faulted(),
            self.output.surface_indices.faulted(),
            self.output.material_indices.faulted(),
            self.output.glazing_indices.faulted(),
            self.output.gas_indices.faulted(),
            self.output.construction_indices.faulted(),
            self.output.fault_severity.faulted(),
        ];
        self.backing_rejected || tables.into_iter().any(|faulted| faulted)
    }

    pub fn finish(self) -> PrecomputedModel {
        debug_assert!(self.is_complete());
        self.output
    }

    fn advance(&mut self, stage: PrecomputeStage) {
        self.stage = stage;
        self.cursor = 0;
        self.projection_cursor = 0;
    }

    fn reject(&mut self) {
        self.backing_rejected = true;
        self.stage = PrecomputeStage::Complete;
    }

    pub fn step(&mut self, model: &Model) {
        match self.stage {
            PrecomputeStage::ReserveBacking => self.step_reserve(model),
            PrecomputeStage::IndexMaterials => self.step_index(model.materials.get(self.cursor).map(|m| m.id), PrecomputeStage::IndexGlazingMaterials, |output| &mut output.material_indices),
            PrecomputeStage::IndexGlazingMaterials => self.step_index(model.glazing_materials.get(self.cursor).map(|m| m.id), PrecomputeStage::IndexGasMaterials, |output| &mut output.glazing_indices),
            PrecomputeStage::IndexGasMaterials => self.step_index(model.gas_materials.get(self.cursor).map(|m| m.id), PrecomputeStage::IndexConstructions, |output| &mut output.gas_indices),
            PrecomputeStage::IndexConstructions => self.step_index(model.constructions.get(self.cursor).map(|c| c.id), PrecomputeStage::IndexSurfaces, |output| &mut output.construction_indices),
            PrecomputeStage::IndexSurfaces => self.step_index(model.surfaces.get(self.cursor).map(|s| s.id), PrecomputeStage::Zones, |output| &mut output.surface_indices),
            PrecomputeStage::Zones => {
                if let Some(zone) = model.zones.get(self.cursor) {
                    let _ = self.output.zone_geometry.insert(zone.id, ZoneGeometry::default());
                    let _ = self.output.default_setpoints.insert(zone.id, ResolvedSetpoints { heating_c: 20.0, cooling_c: 26.0, heating_throttle_k: 2.0, cooling_throttle_k: 2.0 });
                    let _ = self.output.zone_indices.insert(zone.id, self.cursor);
                    self.output.zone_order.push(zone.id);
                    self.cursor += 1;
                } else {
                    self.advance(PrecomputeStage::Surfaces);
                }
            }
            PrecomputeStage::Surfaces => {
                if self.cursor < model.surfaces.len() {
                    self.step_surface(model);
                } else {
                    self.advance(PrecomputeStage::Thermostats);
                }
            }
            PrecomputeStage::Thermostats => {
                if let Some(thermostat) = model.thermostats.get(self.cursor) {
                    let _ = self.output.default_setpoints.insert(thermostat.zone_id, ResolvedSetpoints { heating_c: 20.0, cooling_c: 26.0, heating_throttle_k: thermostat.heating_throttle_range_k, cooling_throttle_k: thermostat.cooling_throttle_range_k });
                    self.cursor += 1;
                } else {
                    self.advance(PrecomputeStage::Windows);
                }
            }
            PrecomputeStage::Windows => {
                if self.cursor < model.fenestrations.len() {
                    self.step_window(model);
                } else {
                    self.advance(PrecomputeStage::ShadingSurfaces);
                }
            }
            PrecomputeStage::ShadingSurfaces => {
                if let Some(shade) = model.shading_surfaces.get(self.cursor) {
                    if !self.push_caster(shade.vertices_m.iter().map(|vertex| rotate(*vertex, model.site.north_axis_deg))) {
                        return self.reject();
                    }
                    self.cursor += 1;
                } else {
                    self.advance(PrecomputeStage::Projections);
                }
            }
            PrecomputeStage::Projections => self.step_projection(model),
            PrecomputeStage::ReceiverCasters => self.step_receiver_casters(),
            PrecomputeStage::Enclosures => {
                if let Some(zone_id) = self.output.zone_order.get(self.cursor).copied() {
                    if !self.step_enclosure(model, zone_id) {
                        return self.reject();
                    }
                    self.cursor += 1;
                } else {
                    self.advance(PrecomputeStage::SkyShading);
                }
            }
            PrecomputeStage::SkyShading => self.step_sky_shading(),
            PrecomputeStage::Faults => {
                if let Some(fault) = model.faults.get(self.cursor) {
                    let _ = self.output.fault_severity.insert(fault.target_equipment_id, fault.severity);
                    self.cursor += 1;
                } else {
                    self.advance(PrecomputeStage::Complete);
                }
            }
            PrecomputeStage::Complete => {}
        }
    }

    fn step_reserve(&mut self, model: &Model) {
        let projections = model.fenestrations.iter().map(|fen| usize::from(fen.overhang_depth_m > 0.0) + 2 * usize::from(fen.fin_depth_m > 0.0)).sum::<usize>();
        let rejected = match self.reserve_cursor {
            0 => self.output.zone_geometry.admit(model.zones.len()).is_err(),
            1 => self.output.surfaces.admit(model.surfaces.len()).is_err(),
            2 => self.output.windows.admit(model.fenestrations.len()).is_err(),
            3 => self.output.enclosures.admit(model.zones.len()).is_err(),
            4 => self.output.default_setpoints.admit(model.zones.len()).is_err(),
            5 => self.output.zone_indices.admit(model.zones.len()).is_err(),
            6 => self.output.surface_indices.admit(model.surfaces.len()).is_err(),
            7 => self.output.material_indices.admit(model.materials.len()).is_err(),
            8 => self.output.glazing_indices.admit(model.glazing_materials.len()).is_err(),
            9 => self.output.gas_indices.admit(model.gas_materials.len()).is_err(),
            10 => self.output.construction_indices.admit(model.constructions.len()).is_err(),
            11 => self.output.fault_severity.admit(model.faults.len()).is_err(),
            12 => self.output.zone_order.try_reserve_exact(model.zones.len()).is_err(),
            13 => self.output.surface_order.try_reserve_exact(model.surfaces.len()).is_err(),
            14 => self.output.window_order.try_reserve_exact(model.fenestrations.len()).is_err(),
            15 => self.output.casters.try_reserve_exact(model.shading_surfaces.len() + projections).is_err(),
            _ => {
                self.advance(PrecomputeStage::IndexMaterials);
                return;
            }
        };
        if rejected {
            self.reject();
        } else {
            self.reserve_cursor += 1;
        }
    }

    fn step_index(&mut self, id: Option<EntityId>, next: PrecomputeStage, table: impl FnOnce(&mut PrecomputedModel) -> &mut FixedTable<EntityId, usize>) {
        if let Some(id) = id {
            let _ = table(&mut self.output).insert(id, self.cursor);
            self.cursor += 1;
        } else {
            self.advance(next);
        }
    }

    fn push_caster(&mut self, polygon: impl Iterator<Item = [f64; 3]>) -> bool {
        let mut owned = Vec::new();
        let (lower, _) = polygon.size_hint();
        if owned.try_reserve_exact(lower.max(3)).is_err() {
            return false;
        }
        owned.extend(polygon);
        self.output.casters.push(owned);
        true
    }

    fn step_surface(&mut self, model: &Model) {
        let surface = &model.surfaces[self.cursor];
        let north = model.site.north_axis_deg;
        let mut polygon = Vec::new();
        if polygon.try_reserve_exact(surface.vertices_m.len()).is_err() {
            return self.reject();
        }
        polygon.extend(surface.vertices_m.iter().map(|vertex| rotate(*vertex, north)));
        let normal = polygon_normal(&polygon);
        let orientation = surface_tilt_azimuth(normal, 0.0);
        let gross = surface_area_m2(&polygon);
        let layers: Vec<&model::Material> = self
            .output
            .construction_indices
            .get(&surface.construction_id)
            .and_then(|index| model.constructions.get(*index))
            .map(|construction| construction.layer_material_ids.iter().filter_map(|id| self.output.material_indices.get(id).and_then(|index| model.materials.get(*index))).collect())
            .unwrap_or_default();
        let step_s = self.output.balance_step_s();
        let conduction: Vec<ConductionLayer> = layers.iter().map(|material| ConductionLayer { thickness_m: material.thickness_m, conductivity_w_m_k: material.conductivity_w_m_k, volumetric_heat_capacity_j_m3k: material.density_kg_m3 * material.specific_heat_j_kg_k }).collect();
        let nodes = NodeChain::node_count(&conduction, step_s);
        let mut chain = NodeChain::default();
        if chain.conductance_w_m2k.try_reserve_exact(nodes.saturating_sub(1)).is_err() || chain.capacitance_j_m2k.try_reserve_exact(nodes).is_err() {
            return self.reject();
        }
        for layer in &conduction {
            chain.push_layer(layer, step_s);
        }
        if chain.nodes() < 2 {
            chain.push_layer(&ConductionLayer { thickness_m: 0.001, conductivity_w_m_k: 1000.0, volumetric_heat_capacity_j_m3k: 0.0 }, step_s);
        }
        self.output.maximum_nodes = self.output.maximum_nodes.max(chain.nodes());
        let outside = layers.first();
        let inside = layers.last();
        let Some(geometry) = self.output.zone_geometry.get_mut(&surface.zone_id) else { return self.reject() };
        if matches!(surface.class, SurfaceClass::Floor | SurfaceClass::Ground) {
            geometry.floor_area_m2 += gross;
        }
        if matches!(surface.outside_boundary_condition, OutsideBoundary::OutdoorAir) {
            geometry.exterior_area_m2 += gross;
        }
        if matches!(surface.class, SurfaceClass::Roof | SurfaceClass::Ceiling) {
            geometry.roof_area_m2 += gross;
        }
        let record = SurfacePrecompute {
            zone_id: surface.zone_id,
            class: surface.class,
            boundary: surface.outside_boundary_condition,
            centroid_height_m: polygon.iter().map(|vertex| vertex[2]).sum::<f64>() / polygon.len().max(1) as f64,
            polygon,
            normal,
            gross_area_m2: gross,
            area_m2: gross,
            tilt_deg: orientation.tilt_deg,
            azimuth_deg: orientation.azimuth_deg,
            sun_exposed: surface.sun_exposed && matches!(surface.outside_boundary_condition, OutsideBoundary::OutdoorAir),
            wind_exposed: surface.wind_exposed && matches!(surface.outside_boundary_condition, OutsideBoundary::OutdoorAir),
            chain,
            outside_solar_absorptance: outside.map_or(0.7, |m| m.solar_absorptance),
            outside_emissivity: outside.map_or(0.9, |m| m.thermal_absorptance),
            inside_solar_absorptance: inside.map_or(0.7, |m| m.solar_absorptance),
            inside_emissivity: inside.map_or(0.9, |m| m.thermal_absorptance),
            roughness_multiplier: outside.map_or(1.52, |m| m.roughness.forced_convection_multiplier()),
            sky_isotropic_ratio: 1.0,
            sky_horizon_ratio: 1.0,
            casters: Vec::new(),
            windows: Vec::new(),
        };
        let _ = self.output.surfaces.insert(surface.id, record);
        self.output.surface_order.push(surface.id);
        self.cursor += 1;
    }

    fn step_window(&mut self, model: &Model) {
        let fenestration = &model.fenestrations[self.cursor];
        self.cursor += 1;
        let Some(host) = self.output.surfaces.get(&fenestration.surface_id) else { return };
        let siblings: Vec<&model::Fenestration> = model.fenestrations.iter().filter(|other| other.surface_id == fenestration.surface_id).collect();
        let position = siblings.iter().position(|other| other.id == fenestration.id).unwrap_or(0);
        // 🔶️ `host.polygon` is already turned into world coordinates by the site's north axis, so
        // an aperture's OWN polygon — stated in the same model frame as its host's vertices — has
        // to make the same turn before the two can share a plane.
        let north = model.site.north_axis_deg;
        let own: Vec<[f64; 3]> = fenestration.vertices_m.iter().map(|vertex| rotate(*vertex, north)).collect();
        let polygon = place_aperture(&host.polygon, &own, fenestration.area_m2, fenestration.height_m, fenestration.sill_height_m, position, siblings.len());
        let glazing = fenestration.glazing_construction_id.and_then(|id| self.output.construction_indices.get(&id).and_then(|index| model.constructions.get(*index))).and_then(|construction| {
            let panes: Vec<Pane> = construction
                .layer_material_ids
                .iter()
                .step_by(2)
                .filter_map(|id| self.output.glazing_indices.get(id).and_then(|index| model.glazing_materials.get(*index)))
                .map(|glass| Pane {
                    thickness_m: glass.thickness_m,
                    conductivity_w_m_k: glass.conductivity_w_m_k,
                    solar_transmittance: glass.solar_transmittance,
                    solar_reflectance_front: glass.solar_reflectance_front,
                    solar_reflectance_back: glass.solar_reflectance_back,
                    infrared_transmittance: glass.infrared_transmittance,
                    infrared_emissivity_front: glass.infrared_emissivity_front,
                    infrared_emissivity_back: glass.infrared_emissivity_back,
                })
                .collect();
            let gaps: Vec<Gap> = construction.layer_material_ids.iter().skip(1).step_by(2).filter_map(|id| self.output.gas_indices.get(id).and_then(|index| model.gas_materials.get(*index))).map(|gas| Gap { width_m: gas.thickness_m, gas: gas.gas }).collect();
            GlazingSystem::layered(&panes, &gaps)
        });
        let glazing = glazing.unwrap_or_else(|| GlazingSystem::simple(fenestration.u_value_w_m2k, fenestration.shgc));
        let normal = host.normal;
        let orientation = surface_tilt_azimuth(normal, 0.0);
        let record = WindowPrecompute {
            zone_id: host.zone_id,
            surface_id: fenestration.surface_id,
            area_m2: surface_area_m2(&polygon),
            height_m: fenestration.height_m.max(1e-3),
            centroid_height_m: polygon.iter().map(|vertex| vertex[2]).sum::<f64>() / polygon.len().max(1) as f64,
            polygon,
            normal,
            tilt_deg: orientation.tilt_deg,
            azimuth_deg: orientation.azimuth_deg,
            sun_exposed: host.sun_exposed,
            wind_exposed: host.wind_exposed,
            coefficient_adjustment: rated_coefficient_adjustment(&glazing),
            glazing,
            sky_isotropic_ratio: 1.0,
            sky_horizon_ratio: 1.0,
            casters: Vec::new(),
        };
        let area = record.area_m2;
        let _ = self.output.windows.insert(fenestration.id, record);
        self.output.window_order.push(fenestration.id);
        if let Some(host) = self.output.surfaces.get_mut(&fenestration.surface_id) {
            host.area_m2 = (host.area_m2 - area).max(0.0);
            if host.windows.try_reserve(1).is_err() {
                return self.reject();
            }
            host.windows.push(fenestration.id);
        }
    }

    fn step_projection(&mut self, model: &Model) {
        let Some(fenestration) = model.fenestrations.get(self.cursor) else {
            return self.advance(PrecomputeStage::ReceiverCasters);
        };
        let Some(window) = self.output.windows.get(&fenestration.id) else {
            self.cursor += 1;
            return;
        };
        let (p0, p1, p3) = (window.polygon[0], window.polygon[1], window.polygon[3]);
        let normal = window.normal;
        let along = |point: [f64; 3], direction: [f64; 3], distance: f64| [point[0] + direction[0] * distance, point[1] + direction[1] * distance, point[2] + direction[2] * distance];
        let unit = |a: [f64; 3], b: [f64; 3]| {
            let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            let length = dot(d, d).sqrt().max(1e-12);
            [d[0] / length, d[1] / length, d[2] / length]
        };
        let (horizontal, vertical) = (unit(p0, p1), unit(p0, p3));
        let width = dot([p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]], horizontal);
        let height = dot([p3[0] - p0[0], p3[1] - p0[1], p3[2] - p0[2]], vertical);
        let polygon: Option<[[f64; 3]; 4]> = match self.projection_cursor {
            0 if fenestration.overhang_depth_m > 0.0 => {
                let edge = along(p0, vertical, height + fenestration.overhang_offset_m.max(0.0));
                let far = along(edge, horizontal, width);
                Some([edge, along(edge, normal, fenestration.overhang_depth_m), along(far, normal, fenestration.overhang_depth_m), far])
            }
            1 | 2 if fenestration.fin_depth_m > 0.0 => {
                let base = if self.projection_cursor == 1 { along(p0, horizontal, -fenestration.fin_offset_m.max(0.0)) } else { along(p0, horizontal, width + fenestration.fin_offset_m.max(0.0)) };
                let top = along(base, vertical, height);
                Some([base, top, along(top, normal, fenestration.fin_depth_m), along(base, normal, fenestration.fin_depth_m)])
            }
            _ => None,
        };
        if let Some(polygon) = polygon {
            if !self.push_caster(polygon.into_iter()) {
                return self.reject();
            }
        }
        if self.projection_cursor >= 2 {
            self.projection_cursor = 0;
            self.cursor += 1;
        } else {
            self.projection_cursor += 1;
        }
    }

    fn step_receiver_casters(&mut self) {
        let surfaces = self.output.surface_order.len();
        let windows = self.output.window_order.len();
        if self.cursor >= surfaces + windows {
            return self.advance(PrecomputeStage::Enclosures);
        }
        let (origin, normal, sun) = if self.cursor < surfaces {
            let surface = self.output.surfaces.get(&self.output.surface_order[self.cursor]).expect("indexed surface");
            (surface.polygon[0], surface.normal, surface.sun_exposed)
        } else {
            let window = self.output.windows.get(&self.output.window_order[self.cursor - surfaces]).expect("indexed window");
            (window.polygon[0], window.normal, window.sun_exposed)
        };
        let mut casters = Vec::new();
        if sun {
            for (index, caster) in self.output.casters.iter().enumerate() {
                if caster.iter().any(|vertex| dot([vertex[0] - origin[0], vertex[1] - origin[1], vertex[2] - origin[2]], normal) > 1e-6) {
                    if casters.try_reserve(1).is_err() {
                        return self.reject();
                    }
                    casters.push(index);
                }
            }
        }
        if self.cursor < surfaces {
            let id = self.output.surface_order[self.cursor];
            if let Some(surface) = self.output.surfaces.get_mut(&id) {
                surface.casters = casters;
            }
        } else {
            let id = self.output.window_order[self.cursor - surfaces];
            if let Some(window) = self.output.windows.get_mut(&id) {
                window.casters = casters;
            }
        }
        self.cursor += 1;
    }

    fn step_enclosure(&mut self, model: &Model, zone_id: EntityId) -> bool {
        let mut faces = Vec::new();
        for id in &self.output.surface_order {
            if self.output.surfaces.get(id).is_some_and(|surface| surface.zone_id == zone_id) {
                if faces.try_reserve(1).is_err() {
                    return false;
                }
                faces.push(EnclosureFace::Opaque(*id));
            }
        }
        for id in &self.output.window_order {
            if self.output.windows.get(id).is_some_and(|window| window.zone_id == zone_id) {
                if faces.try_reserve(1).is_err() {
                    return false;
                }
                faces.push(EnclosureFace::Window(*id));
            }
        }
        let n = faces.len();
        let describe = |face: &EnclosureFace| match face {
            EnclosureFace::Opaque(id) => {
                let s = self.output.surfaces.get(id).expect("enclosure surface");
                (s.area_m2, s.inside_emissivity, s.inside_solar_absorptance, s.azimuth_deg, s.tilt_deg, matches!(s.class, SurfaceClass::Floor | SurfaceClass::Ground))
            }
            EnclosureFace::Window(id) => {
                let w = self.output.windows.get(id).expect("enclosure window");
                (w.area_m2, w.glazing.emissivity_back[w.glazing.panes - 1], w.glazing.diffuse_transmittance + w.glazing.diffuse_back_absorptance_total(), w.azimuth_deg, w.tilt_deg, false)
            }
        };
        let mut areas = Vec::new();
        let mut emissivities = Vec::new();
        let mut descriptors = Vec::new();
        if areas.try_reserve_exact(n).is_err() || emissivities.try_reserve_exact(n).is_err() || descriptors.try_reserve_exact(n).is_err() {
            return false;
        }
        let mut solar_weight = 0.0;
        let mut radiant_weight = 0.0;
        for face in &faces {
            let d = describe(face);
            areas.push(d.0);
            emissivities.push(d.1);
            solar_weight += d.0 * d.2;
            radiant_weight += d.0 * d.1;
            descriptors.push((d.3, d.4, d.5));
        }
        let _ = model;
        let radiation = if n <= EXACT_ENCLOSURE_FACES {
            let Some((view_factors, exchange_factors)) = enclosure_factors(&areas, &emissivities, &descriptors) else { return false };
            EnclosureRadiation::Exchange { view_factors, exchange_factors }
        } else {
            let Some(participation) = mean_radiant_participation(&areas, &emissivities) else { return false };
            EnclosureRadiation::MeanRadiant { participation }
        };
        self.output.maximum_enclosure_faces = self.output.maximum_enclosure_faces.max(n);
        let _ = self.output.enclosures.insert(zone_id, EnclosurePrecompute { faces, areas_m2: areas, emissivities, radiation, diffuse_solar_multiplier: if solar_weight > 0.01 { 1.0 / solar_weight } else { 0.0 }, radiant_weight_total_m2: radiant_weight });
        true
    }

    fn step_sky_shading(&mut self) {
        let surfaces = self.output.surface_order.len();
        let windows = self.output.window_order.len();
        if self.cursor >= surfaces + windows {
            return self.advance(PrecomputeStage::Faults);
        }
        let casters = &self.output.casters;
        if self.cursor < surfaces {
            let id = self.output.surface_order[self.cursor];
            let Some(surface) = self.output.surfaces.get(&id) else { return };
            if surface.sun_exposed && !surface.casters.is_empty() {
                let openings: Vec<&[[f64; 3]]> = surface.windows.iter().filter_map(|window| self.output.windows.get(window)).map(|window| window.polygon.as_slice()).collect();
                let (isotropic, horizon) = sky_diffuse_shading_ratios(&surface.polygon, surface.normal, openings.iter().copied(), surface.casters.iter().map(|index| casters[*index].as_slice()));
                if let Some(surface) = self.output.surfaces.get_mut(&id) {
                    surface.sky_isotropic_ratio = isotropic;
                    surface.sky_horizon_ratio = horizon;
                }
            }
        } else {
            let id = self.output.window_order[self.cursor - surfaces];
            let Some(window) = self.output.windows.get(&id) else { return };
            if window.sun_exposed && !window.casters.is_empty() {
                let (isotropic, horizon) = sky_diffuse_shading_ratios(&window.polygon, window.normal, std::iter::empty(), window.casters.iter().map(|index| casters[*index].as_slice()));
                if let Some(window) = self.output.windows.get_mut(&id) {
                    window.sky_isotropic_ratio = isotropic;
                    window.sky_horizon_ratio = horizon;
                }
            }
        }
        self.cursor += 1;
    }
}
// #endregion 🔖️PrecomputeBuilder
