//! 📮️ Session entry points for the native handle registry and topology publications.

use super::*;
use semio_framework_value::{
    RetainedAllocationEffect, RetainedCloneGrant as Grant, RetainedCloneProgress as Progress,
    RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep as Step, RetirementDemand, ValueError, ValueRefusalKind,
};

impl Brep {
    /// 📌️ Plans an analytic curve from finite parameters and reserves only a monotone label.
    pub fn plan_analytic_curve(&mut self, parameters: AnalyticCurveParameters) -> Result<AnalyticPublication, ValueError> {
        self.native.plan_analytic_curve(parameters)
    }

    /// 🌙️ Opens exact rational arc construction without allocating its control lattice.
    pub fn plan_analytic_arc(&mut self,center:EVec3,normal:EVec3,radius:f64,start:f64,end:f64) -> Result<AnalyticPublication,ValueError> {
        self.native.plan_analytic_arc(center, normal, radius, start, end)
    }

    /// ➿️ Plans bounded original curve interpolation without copying input points.
    pub fn plan_interpolated_curve(&mut self, count: usize, degree: usize) -> Result<AnalyticPublication, ValueError> {
        self.native.plan_interpolated_curve(count, degree)
    }

    /// 📉️ Admits an original endpoint-constrained curve fit.
    pub fn plan_approximated_curve(&mut self, count: usize, degree: usize, controls: usize) -> Result<AnalyticPublication, ValueError> {
        self.native.plan_approximated_curve(count, degree, controls)
    }

    /// 🌈️ Plans the original interpolation and exact blending of four boundary curves.
    pub fn plan_coons_surface(&mut self, boundaries: [usize; 4]) -> Result<AnalyticPublication, ValueError> {
        self.native.plan_coons_surface(boundaries)
    }

    /// 🪡️ Plans an original tensor-product interpolation without copying a grid.
    pub fn plan_interpolated_grid(&mut self, rows: usize, columns: usize, degree_u: usize, degree_v: usize) -> Result<AnalyticPublication, ValueError> {
        self.native.plan_interpolated_grid(rows, columns, degree_u, degree_v)
    }

    /// 📄️ Plans a plane from its finite origin and normal without allocating geometry.
    pub fn plan_analytic_plane(&mut self, origin: EVec3, normal: EVec3) -> Result<AnalyticPublication, ValueError> {
        self.native.plan_analytic_plane(origin, normal)
    }

    /// 📐️ Plans one affine image of an original analytic curve or plane.
    pub fn plan_analytic_transform(&mut self, source: &str, transform: AnalyticTransform) -> Result<AnalyticPublication, ValueError> {
        self.native.plan_analytic_transform(source, transform)
    }

    /// 📏️ Quotes one live-index insertion without expanding the rest of the session.
    pub fn analytic_publication_demand(&self, plan: &AnalyticPublication, copy: usize) -> Result<RetirementDemand, ValueError> {
        self.native.analytic_publication_demand(plan, copy)
    }

    /// 🏗️ Reserves one actual live-index page without publishing its entity.
    pub fn reserve_analytic_publication(&mut self, plan: &AnalyticPublication, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        self.native.reserve_analytic_publication(plan, grant)
    }

    /// 🌱️ Transfers the admitted live key and original analytic geometry into the reserved index.
    pub fn publish_analytic_geometry(&mut self, plan: &mut AnalyticPublication, key: String, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, (ValueError, String)> {
        self.native.publish_analytic_geometry(plan, key, grant)
    }

    /// 📸️ Read-only mathematical topology for an owner-supplied format codec.
    pub fn representation(&self) -> &Body { self.native.representation() }

    /// 🧵️ Borrows the face identity and original topology revision for exact sewing.
    pub fn cuboid_sew_source(&self, key: &str) -> Result<(FaceId, u64, usize), ValueError> { match self.native.original_query_entity(key)? { Entity::Face(id) => Ok((*id, self.native.shape_mutation_revision, self.native.tessellation_source_depth_demand(key)?)), _ => Err(ValueError::literal(ValueRefusalKind::InvalidValue, "sewing requires face inputs")) } }

    /// 🧊️ Resolves a live solid without exposing handle registry mutation.
    pub fn geometry_solid(&self, handle: &GeometryHandle) -> Result<SolidId,BrepError> {
        self.native.geometry_solid(handle)
    }

    /// 📥️ Merges mathematical topology while preserving every prior live handle.
    pub fn merge_representation(&mut self, imported: Body) -> Vec<GeometryHandle> {
        self.native.merge_representation(imported)
    }

    /// 🪪️ Enumerates registered handles for explicit session operation claims.
    pub fn live_handles(&self) -> std::collections::HashSet<String> {
        self.native.live_handles()
    }

    /// 👀️ Borrows the original live registry without constructing a handle owner.
    pub fn contains_live_handle(&self,handle:&str)->bool {
        self.native.contains_live_handle(handle)
    }

    /// 🧊️ Quotes the original borrowed handle index before cold query admission.
    pub fn tessellation_source_depth_demand(&self,source:&str)->Result<usize,ValueError> {
        self.native.tessellation_source_depth_demand(source)
    }

    /// 🧷️ Borrows proof that the original live geometry still has its constructor's exact topology.
    pub fn live_solid_primitive(&self, handle: &str) -> Option<SolidPrimitive> {
        self.native.live_solid_primitive(handle)
    }

    /// ⚖️ Borrows the exact measure certified for the original constructed solid.
    pub fn live_certified_volume(&self, handle: &str) -> Option<f64> {
        self.native.live_certified_volume(handle)
    }

    /// 🔎️ Exposes the exact bore certificate only while the original native solid is live.
    pub fn live_bore_preview(&self, handle: &str) -> Option<(SolidId, BorePreviewSource)> {
        self.native.live_bore_preview(handle)
    }

    /// ✂️ Captures a live certified box and plane parameters without cloning its topology.
    pub fn plan_box_section(&self, source: &str, origin: EVec3, normal: EVec3) -> Result<BoxSectionDraft, ValueError> {
        let primitive = self.native.live_solid_primitive(source).filter(|shape| matches!(shape, SolidPrimitive::Box { .. }))
            .ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "original Section requires a certified box"))?;
        BoxSectionDraft::new(primitive, origin, normal)
            .map_err(|reason| ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, reason))
    }

    /// 🔪️ Stages the exact boundary of rectangular stock less both certified tool intervals.
    pub fn plan_compound_box_cut(&mut self, stock: [[f64; 3]; 2], tools: &[[[f64; 3]; 2]]) -> Result<SolidPublication, ValueError> { self.native.plan_orthogonal_solid(semio_framework_3d_brep_boolean::certified_boundaries::OrthogonalBoundary::cut_tools(stock, tools)?) }

    pub fn plan_box_split(&mut self, source: &str, origin: EVec3, normal: EVec3) -> Result<[SolidPublication; 2], ValueError> { self.native.plan_box_split(source, origin, normal) }
    pub fn solid_pair_demand(&self) -> Result<RetirementDemand, ValueError> { self.native.live.vacant_pair_demand() }
    pub fn reserve_solid_pair(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<Option<Progress>, ValueError> { self.native.live.reserve_vacant_pair(effect) }
    pub fn publish_solid_pair(&mut self, pair: &mut [SolidPublication; 2], keys: &mut [String; 2], grant: Grant) -> Result<Progress, ValueError> { self.native.publish_solid_pair(pair, keys, grant) }

    /// 🎟️ Quotes original native reads for an analytic bounding box.
    pub fn analytic_bounds_depth(&self, handle: &str) -> Result<usize, ValueError> {
        self.native.analytic_bounds_depth(handle)
    }

    /// 🔳️ Plans a funded solid for one exact finite analytic interval.
    pub fn plan_analytic_bounds(&mut self, handle: &str) -> Result<SolidPublication, ValueError> {
        self.native.plan_analytic_bounds(handle)
    }

    /// 🎟️ Quotes the original arena reads of a certified solid.
    pub fn certified_solid_depth(&self, handle: &str) -> Result<usize, ValueError> {
        self.native.certified_solid_depth(handle)
    }

    /// 🪚️ Stages one exact original box finish through the solid owner.
    pub fn plan_box_finish(&mut self, shape: &str, member: &str, finish: BoxFinish) -> Result<SolidPublication, ValueError> {
        self.native.plan_box_finish(shape, member, finish)
    }

    /// 📏️ Quotes the exact source paths of a selected box member.
    pub fn box_finish_depth(&self, shape: &str, member: &str) -> Result<usize, ValueError> {
        self.native.box_finish_depth(shape, member)
    }

    /// 📌️ Classifies one point against a live certified solid.
    pub fn original_solid_classification(&self, handle: &str, point: EVec3) -> Result<PointClassification, ValueError> {
        self.native.original_solid_classification(handle, point)
    }

    /// 📏️ Computes exact closed-set separation of two live certified boxes.
    pub fn original_box_distance(&self, a: &str, b: &str) -> Result<f64, ValueError> {
        self.native.original_box_distance(a, b)
    }

    /// 🥞️ Quotes the paired original Loft profile paths.
    pub fn loft_depth(&self, handles: [&str; 2]) -> Result<usize, ValueError> {
        self.native.loft_depth(handles)
    }

    /// 🧱️ Starts a cylinder or the line correspondence scan for a two-profile Loft.
    pub fn plan_exact_loft(&mut self, handles: [&str; 2]) -> Result<LoftPlan, ValueError> {
        self.native.plan_exact_loft(handles)
    }

    /// 📏️ Quotes one pair of original Loft profile lines.
    pub fn loft_draft_demand(&self, handles: [&str; 2], draft: &LoftDraft) -> Result<RetirementDemand, ValueError> {
        self.native.loft_draft_demand(handles, draft)
    }

    /// 🛠️ Advances one line correspondence or transfers into the original prism draft.
    pub fn step_loft_draft(&self, handles: [&str; 2], draft: &mut LoftDraft, effect: &mut RetainedAllocationEffect<'_>)
        -> Result<(Option<PrismDraft>, Progress), ValueError> {
        self.native.step_loft_draft(handles, draft, effect)
    }

    /// 🎟️ Quotes the original face and path arenas for a straight sweep.
    pub fn straight_sweep_depth(&self, profile: &str, path: &str) -> Result<usize, ValueError> {
        self.native.straight_sweep_depth(profile, path)
    }

    /// 🛤️ Stages a certified cylinder or prism over an original directed line.
    pub fn plan_straight_sweep(&mut self, profile: &str, path: &str) -> Result<PlanarExtrusion, ValueError> {
        self.native.plan_straight_sweep(profile, path)
    }

    /// 📦️ Stages a verified convex point boundary for original solid publication.
    pub fn finish_point_hull(&mut self, draft: &mut HullDraft) -> Result<SolidPublication, ValueError> {
        self.native.finish_point_hull(draft)
    }

    /// 🐚️ Captures the certified box for a retained shell operation.
    pub fn plan_box_shell(&self, handle: &str, thickness: f64) -> Result<BoxShellDraft, ValueError> {
        self.native.plan_box_shell(handle, thickness)
    }

    /// 🎟️ Quotes the original arena paths for one selected shell opening.
    pub fn box_shell_face_depth(&self, draft: &BoxShellDraft, handle: &str) -> Result<usize, ValueError> {
        self.native.box_shell_face_depth(draft, handle)
    }

    /// 🪟️ Adds one original source face to the shell opening mask.
    pub fn add_box_shell_face(&self, draft: &mut BoxShellDraft, handle: &str) -> Result<(), ValueError> {
        self.native.add_box_shell_face(draft, handle)
    }

    /// 🪚️ Stages the exact solid boundary from the completed shell selection.
    pub fn finish_box_shell(&mut self, draft: &BoxShellDraft, handle: &str) -> Result<SolidPublication, ValueError> {
        self.native.finish_box_shell(draft, handle)
    }

    /// ↔️ Plans a signed sharp offset of a certified box.
    pub fn plan_certified_solid_offset(&mut self, handle: &str, distance: f64) -> Result<SolidPublication, ValueError> {
        self.native.plan_certified_solid_offset(handle, distance)
    }

    /// 🫴️ Captures precisely one live certified operand.
    pub fn capture_boolean_operand(&self, handle: &str) -> Result<BooleanOperand, ValueError> {
        self.native.capture_boolean_operand(handle)
    }

    /// 🔲️ Plans an original result without copying source topology or publishing any live row.
    pub fn plan_certified_boolean(&mut self, a: &str, b: &str, first: &BooleanOperand, operation: BooleanOp)
        -> Result<SolidPublication, ValueError> {
        self.native.plan_certified_boolean(a, b, first, operation)
    }

    /// 🎟️ Begins read-only acquisition without entering cold topology cloning.
    pub fn plan_boolean_preparation(&self, a: &str, b: &str) -> Result<BooleanPreparation, ValueError> {
        self.native.plan_boolean_preparation(a, b)
    }

    /// 🪙️ Quotes one source face or one scalar projection boundary.
    pub fn boolean_preparation_demand(&self, a: &str, b: &str, preparation: &BooleanPreparation) -> Result<RetirementDemand, ValueError> {
        self.native.boolean_preparation_demand(a, b, preparation)
    }

    /// ⏱️ Captures one original face under its physical copy and work grant.
    pub fn step_boolean_preparation(&self, a: &str, b: &str, preparation: &mut BooleanPreparation, effect: &mut RetainedAllocationEffect<'_>)
        -> Result<(bool, RetainedCloneProgress), ValueError> {
        self.native.step_boolean_preparation(a, b, preparation, effect)
    }

    pub fn plan_topology_selection(&self, handle: &str, kind: TopologyList) -> Result<TopologySelection, ValueError> {
        self.native.plan_topology_selection(handle, kind)
    }

    pub fn topology_member_depth(&self, plan: &TopologySelection, index: usize) -> Result<usize, ValueError> {
        self.native.topology_member_depth(plan, index)
    }

    pub fn topology_member(&self, plan: &TopologySelection, index: usize) -> Result<TopologyAlias, ValueError> {
        self.native.topology_member(plan, index)
    }

    pub fn reached_topology_alias_depth(&self, member: semio_framework_3d_brep_representation::topology::EntityRef) -> Result<usize, ValueError> {
        self.native.reached_topology_alias_depth(member)
    }

    /// 🧷️ Creates an alias for one member of a retained native reachability scope.
    pub fn reached_topology_alias(&self, member: semio_framework_3d_brep_representation::topology::EntityRef) -> Result<TopologyAlias, ValueError> {
        self.native.reached_topology_alias(member)
    }


    pub fn topology_alias_demand(&self, alias: &TopologyAlias, copy: usize) -> Result<RetirementDemand, ValueError> {
        self.native.topology_alias_demand(alias, copy)
    }

    pub fn reserve_topology_alias(&mut self, alias: &TopologyAlias, grant: Grant) -> Result<Progress, ValueError> {
        self.native.reserve_topology_alias(alias, grant)
    }

    pub fn publish_topology_alias(&mut self, alias: &TopologyAlias, key: &mut String, grant: Grant) -> Result<Progress, ValueError> {
        self.native.publish_topology_alias(alias, key, grant)
    }

    /// 📍️ Admits one native vertex for retained publication.
    pub fn plan_vertex_publication(&mut self, point: EVec3) -> Result<VertexPublication, ValueError> {
        self.native.plan_vertex_publication(point)
    }

    pub fn vertex_publication_demand(&self, vertex: &VertexPublication, copy: usize) -> Result<RetirementDemand, ValueError> {
        self.native.vertex_publication_demand(vertex, copy)
    }

    pub fn step_vertex_topology(&mut self, vertex: &mut VertexPublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        self.native.step_vertex_topology(vertex, effect)
    }

    pub fn reserve_vertex_publication(&mut self, vertex: &VertexPublication, grant: Grant) -> Result<Progress, ValueError> {
        self.native.reserve_vertex_publication(vertex, grant)
    }

    pub fn publish_vertex(&mut self, vertex: &mut VertexPublication, key: String, grant: Grant) -> Result<Progress, (ValueError, String)> {
        self.native.publish_vertex(vertex, key, grant)
    }

    pub fn vertex_close_demand(&self, vertex: &VertexPublication) -> Result<RetirementDemand, ValueError> {
        self.native.vertex_close_demand(vertex)
    }

    pub fn close_vertex(&mut self, vertex: &mut VertexPublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Step, ValueError> {
        self.native.close_vertex(vertex, effect)
    }

    /// 🎟️ Admits unallocated polygonal wire topology and reserves only stable labels.
    pub fn plan_polygon_wire(&mut self, recipe: PolygonWireRecipe) -> Result<PolygonWirePublication, ValueError> {
        self.native.plan_polygon_wire(recipe)
    }

    /// 📐️ Quotes the next owner turn or permanent live index reservation.
    pub fn polygon_wire_demand(&self, wire: &PolygonWirePublication, copy: usize) -> Result<RetirementDemand, ValueError> {
        self.native.polygon_wire_demand(wire, copy)
    }

    /// 🧱️ Constructs one original wire record under the initiating grant.
    pub fn step_polygon_wire_topology(&mut self, wire: &mut PolygonWirePublication, point: Option<EVec3>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        self.native.step_polygon_wire_topology(wire, point, effect)
    }

    /// 🗄️ Reserves the owner index for a completed wire.
    pub fn reserve_polygon_wire(&mut self, wire: &PolygonWirePublication, grant: Grant) -> Result<Progress, ValueError> {
        self.native.reserve_polygon_wire(wire, grant)
    }

    /// 📤️ Publishes the complete wire without cloning topology or geometry.
    pub fn publish_polygon_wire(&mut self, plan: &mut PolygonWirePublication, key: String, grant: Grant) -> Result<Progress, (ValueError, String)> {
        self.native.publish_polygon_wire(plan, key, grant)
    }

    /// 🧹️ Quotes removal of one unpublished arena record or release of an original ID buffer.
    pub fn polygon_wire_close_demand(&self, plan: &PolygonWirePublication) -> Result<RetirementDemand, ValueError> {
        self.native.polygon_wire_close_demand(plan)
    }

    /// ♻️ Releases one unpublished wire record or backing allocation.
    pub fn close_polygon_wire(&mut self, plan: &mut PolygonWirePublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Step, ValueError> {
        self.native.close_polygon_wire(plan, effect)
    }

    /// 🎟️ Admits one closed wire with a stable source address and label.
    pub fn plan_polygon_face(&mut self, source: &str, xy_only: bool) -> Result<Option<PolygonFacePublication>, ValueError> {
        self.native.plan_polygon_face(source, xy_only)
    }

    /// 🔒️ Admits a planar point-list face with a private closed polygon boundary.
    pub fn plan_polygon_face_points(&mut self, vertices: usize) -> Result<PolygonFacePublication, ValueError> {
        self.native.plan_polygon_face_points(vertices)
    }

    /// ↔️ Admits a planar single-ring offset from its native source.
    pub fn plan_polygon_face_offset(&mut self, source: &str, distance: f64) -> Result<Option<PolygonFacePublication>, ValueError> {
        self.native.plan_polygon_face_offset(source, distance)
    }

    /// 🔁️ Admits an affine polygon face from its original line ring.
    pub fn plan_polygon_face_transform(&mut self, source: &str, transform: AnalyticTransform) -> Result<Option<PolygonFacePublication>, ValueError> {
        self.native.plan_polygon_face_transform(source, transform)
    }

    /// 📏️ Quotes the original source and one topology operation.
    pub fn polygon_face_topology_demand(&self, face: &PolygonFacePublication, source: Option<&str>) -> Result<RetirementDemand, ValueError> {
        self.native.polygon_face_topology_demand(face, source)
    }

    /// 🧱️ Advances one source check or one exact planar topology record.
    pub fn step_polygon_face_topology(&mut self, plan: &mut PolygonFacePublication, source: Option<&str>, point: Option<EVec3>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        self.native.step_polygon_face_topology(plan, source, point, effect)
    }

    /// 📦️ Recognizes one axis-aligned horizontal rectangular wire or face without copying topology.
    pub fn plan_rectangular_extrusion(&mut self, source: &str, vector: EVec3) -> Result<Option<SolidPublication>, ValueError> {
        self.native.plan_rectangular_extrusion(source, vector)
    }

    /// 🗄️ Quotes the live index reservation for a completed polygon face.
    pub fn polygon_face_demand(&self, face: &PolygonFacePublication, copy: usize) -> Result<RetirementDemand, ValueError> {
        self.native.polygon_face_demand(face, copy)
    }

    /// 💾️ Reserves the live index for an admitted planar face.
    pub fn reserve_polygon_face(&mut self, face: &PolygonFacePublication, grant: Grant) -> Result<Progress, ValueError> {
        self.native.reserve_polygon_face(face, grant)
    }

    /// 📤️ Transfers the completed face and keeps its referenced boundary live.
    pub fn publish_polygon_face(&mut self, face: &mut PolygonFacePublication, key: String, grant: Grant) -> Result<Progress, (ValueError, String)> {
        self.native.publish_polygon_face(face, key, grant)
    }

    /// 🧹️ Quotes one unpublished native slot or the published cursor's journal release.
    pub fn polygon_face_close_demand(&self, face: &PolygonFacePublication) -> Result<RetirementDemand, ValueError> {
        self.native.polygon_face_close_demand(face)
    }

    /// ♻️ Removes unpublished topology or only releases the published rollback journal.
    pub fn close_polygon_face(&mut self, face: &mut PolygonFacePublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Step, ValueError> {
        self.native.close_polygon_face(face, effect)
    }

    /// 🧭️ Plans the exact fill of one closed conic with optional XY-plane restriction.
    pub fn plan_conic_face(&mut self, handle: &str, xy_only: bool) -> Result<ConicFacePublication, ValueError> {
        self.native.plan_conic_face(handle, xy_only)
    }

    /// ↔️ Plans the translated conic topology for an original planar face.
    pub fn plan_conic_face_offset(&mut self, handle: &str, distance: f64) -> Result<ConicFacePublication, ValueError> {
        self.native.plan_conic_face_offset(handle, distance)
    }

    /// 📏️ Quotes the next original arena slot or the final live index element.
    pub fn conic_face_demand(&self, plan: &ConicFacePublication, copy: usize) -> Result<RetirementDemand, ValueError> {
        self.native.conic_face_demand(plan, copy)
    }

    /// 🧱️ Advances one funded conic face topology action.
    pub fn step_conic_face_topology(&mut self, plan: &mut ConicFacePublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        self.native.step_conic_face_topology(plan, effect)
    }

    /// 🗄️ Reserves the original live index for the closed face.
    pub fn reserve_conic_face(&mut self, plan: &ConicFacePublication, grant: Grant) -> Result<Progress, ValueError> {
        self.native.reserve_conic_face(plan, grant)
    }

    /// 🌱️ Publishes only the completed conic face, preserving the exact original profile.
    pub fn publish_conic_face(&mut self, plan: &mut ConicFacePublication, key: String, grant: Grant) -> Result<Progress, (ValueError, String)> {
        self.native.publish_conic_face(plan, key, grant)
    }

    /// 🧹️ Quotes one unpublished conic face arena removal.
    pub fn conic_face_close_demand(&self, plan: &ConicFacePublication) -> Result<RetirementDemand, ValueError> {
        self.native.conic_face_close_demand(plan)
    }

    /// ♻️ Releases one original unpublished face arena slot in reverse dependency order.
    pub fn close_conic_face(&mut self, plan: &mut ConicFacePublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Step, ValueError> {
        self.native.close_conic_face(plan, effect)
    }

    /// 🌱️ Starts a bound, allocation-free extrusion recipe from one planar face.
    pub fn plan_prism(&self, source: &str, vector: EVec3) -> Result<Option<PrismDraft>, ValueError> {
        self.native.plan_prism(source, vector)
    }

    /// 📏️ Quotes one source lookup and bounded curve/vertex read.
    pub fn prism_draft_demand(&self, source: &str, draft: &PrismDraft) -> Result<RetirementDemand, ValueError> {
        self.native.prism_draft_demand(source, draft)
    }

    /// 🛠️ Advances one face boundary member or transfers the resulting exact solid recipe.
    pub fn step_prism_draft(&mut self, source: &str, draft: &mut PrismDraft, effect: &mut RetainedAllocationEffect<'_>) -> Result<(Option<SolidPublication>, Progress), ValueError> {
        self.native.step_prism_draft(source, draft, effect)
    }

    /// 📏️ Admits a finite analytic primitive and an independent range of stable topology labels.
    pub fn plan_solid_primitive(&mut self, primitive: SolidPrimitive) -> Result<SolidPublication, ValueError> {
        self.native.plan_solid_primitive(primitive)
    }

    /// 🧭️ Plans an exact affine image when the result still fits a certified primitive recipe.
    pub fn plan_certified_solid_transform(&mut self, source: &str, transform: AnalyticTransform) -> Result<Option<SolidPublication>,ValueError> {
        self.native.plan_certified_solid_transform(source, transform)
    }

    /// ⤴️ Plans the exact axial extrusion of one live complete circular curve, wire, or face.
    pub fn plan_circular_extrusion(&mut self, handle: &str, extrusion: EVec3) -> Result<Option<SolidPublication>, ValueError> {
        self.native.plan_circular_extrusion(handle, extrusion)
    }

    /// 🔍️ Quotes the original source trim paths needed by retained planar sweep operators.
    pub fn planar_face_boundary_depth(&self, handle: &str) -> Result<usize, ValueError> {
        self.native.planar_face_boundary_depth(handle)
    }

    /// 🔄️ Plans a full revolution of an original circular profile around its in-plane axis.
    pub fn plan_circular_revolution(&mut self, handle: &str, origin: EVec3, direction: EVec3, angle: f64)
        -> Result<SolidPublication, ValueError> {
        self.native.plan_circular_revolution(handle, origin, direction, angle)
    }

    /// 🧱️ Plans a signed planar thickening against the original live face.
    pub fn plan_planar_thickening(&mut self, handle: &str, thickness: f64) -> Result<PlanarExtrusion, ValueError> {
        self.native.plan_planar_thickening(handle, thickness)
    }

    /// 🎟️ Quotes one native topology turn or the final live-index insertion.
    pub fn solid_primitive_demand(&self, item: &SolidPublication, copy: usize) -> Result<RetirementDemand, ValueError> {
        self.native.solid_primitive_demand(item, copy)
    }

    /// 🧱️ Advances one charged arena reservation or topology action.
    pub fn step_solid_primitive_topology(&mut self, item: &mut SolidPublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        self.native.step_solid_primitive_topology(item, effect)
    }

    /// 🗄️ Reserves only the next original backing allocation for the completed solid.
    pub fn reserve_solid_primitive(&mut self, item: &SolidPublication, grant: Grant) -> Result<Progress, ValueError> {
        self.native.reserve_solid_primitive(item, grant)
    }

    /// 🫴️ Transfers the completed solid into its admitted live slot.
    pub fn publish_solid_primitive(&mut self, item: &mut SolidPublication, key: String, grant: Grant) -> Result<Progress, (ValueError, String)> {
        self.native.publish_solid_primitive(item, key, grant)
    }

    /// 🧹️ Quotes the immediate release of one unpublished arena value under original stock.
    pub fn solid_primitive_close_demand(&self, item: &SolidPublication) -> Result<RetirementDemand, ValueError> {
        self.native.solid_primitive_close_demand(item)
    }

    /// ♻️ Returns an unpublished solid's arena slots in reverse dependency order.
    pub fn close_solid_primitive(&mut self, item: &mut SolidPublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Step, ValueError> {
        self.native.close_solid_primitive(item, effect)
    }
}

impl Brep {
    pub fn plan_compound(&mut self,count:usize)->Result<CompoundPublication,ValueError>{
        self.native.plan_compound(count)
    }

    pub fn compound_preparation_demand(&self,plan:&CompoundPublication,source:Option<&str>)->Result<RetirementDemand,ValueError>{
        self.native.compound_preparation_demand(plan,source)
    }

    pub fn step_compound_preparation(&self,plan:&mut CompoundPublication,source:Option<&str>,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>{
        self.native.step_compound_preparation(plan,source,effect)
    }

    pub fn compound_publication_demand(&self,plan:&CompoundPublication,copy:usize)->Result<RetirementDemand,ValueError>{
        self.native.compound_publication_demand(plan,copy)
    }

    pub fn reserve_compound_publication(&mut self,plan:&CompoundPublication,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        self.native.reserve_compound_publication(plan,grant)
    }

    pub fn publish_compound(&mut self,plan:&mut CompoundPublication,key:String,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,String)>{
        self.native.publish_compound(plan,key,grant)
    }
}
