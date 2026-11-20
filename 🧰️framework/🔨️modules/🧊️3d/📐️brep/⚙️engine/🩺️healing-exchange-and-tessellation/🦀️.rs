//! 🩺️ Healing, exchange and tessellation: the solid healing and NURBS conversion, the deconstruction, the mesh import and export, the kind lookup, the solid face loops and the tessellation jobs. See [`super`].

use super::*;
use semio_framework_value::ValueError;

// #region 🔖️SyncApi
impl Brep {
    pub fn heal_solid_sync(&mut self, shape: &GeometryHandle, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.native.solid_id(shape)?;
        self.native.invalidate_solid_primitive(shape);
        let mut rec = OpRecorder::new();
        let _ = heal_solid(&mut self.native.body, solid, tolerance, &mut rec).map_err(|error| map_err(&error))?;
        Ok(shape.clone())
    }
    pub fn convert_to_nurbs_sync(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let solid = self.native.solid_id(shape)?;
        self.native.invalidate_solid_primitive(shape);
        let mut rec = OpRecorder::new();
        let _ = convert_to_nurbs(&mut self.native.body, solid, &mut rec).map_err(|error| map_err(&error))?;
        Ok(shape.clone())
    }
    /// 🏷️ Uses the same shape reach as validation and retirement, sorted by arena identity.
    /// [`Brep::mint`] retains each component's [`PersistentLabel`] across repeated decomposition.
    pub fn deconstruct_sync(&mut self, shape: &GeometryHandle) -> Result<BrepTopology, BrepError> {
        let reach = self.native.body.reachable_from(&entity_roots(self.native.entity(shape)?));
        let mut vertices: Vec<_> = reach.vertices.into_iter().collect(); vertices.sort_unstable_by_key(|id| id.raw_index());
        let mut edges: Vec<_> = reach.edges.into_iter().collect(); edges.sort_unstable_by_key(|id| id.raw_index());
        let mut faces: Vec<_> = reach.faces.into_iter().collect(); faces.sort_unstable_by_key(|id| id.raw_index());
        let mut shells: Vec<_> = reach.shells.into_iter().collect(); shells.sort_unstable_by_key(|id| id.raw_index());
        Ok(BrepTopology {
            vertices: vertices.into_iter().map(|id| self.native.mint(GeometryKind::Vertex, Entity::Vertex(id))).collect(),
            edges: edges.into_iter().map(|id| self.native.mint(GeometryKind::Edge, Entity::Edge(id))).collect(),
            faces: faces.into_iter().map(|id| self.native.register_face(id)).collect(),
            shells: shells.into_iter().map(|id| self.native.register_shell(id)).collect(),
        })
    }
    
    pub fn export_stl_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        let solid = self.native.solid_id(shapes.first().ok_or_else(|| BrepError::InvalidInput("empty".into()))?)?;
        export_solid_stl(&self.native.body, solid, deflection).map_err(|error| map_err(&error))
    }
    pub fn export_obj_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<String, BrepError> {
        let solid = self.native.solid_id(shapes.first().ok_or_else(|| BrepError::InvalidInput("empty".into()))?)?;
        export_solid_obj(&self.native.body, solid, deflection).map_err(|error| map_err(&error))
    }
    pub fn export_gltf_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        self.export_glb_sync(shapes, deflection)
    }
    pub fn export_glb_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        let solid = self.native.solid_id(shapes.first().ok_or_else(|| BrepError::InvalidInput("empty".into()))?)?;
        export_solid_glb(&self.native.body, solid, deflection).map_err(|error| map_err(&error))
    }
    pub fn import_glb_sync(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = import_glb_to_body(&mut self.native.body, data, tolerance).map_err(|error| map_err(&error))?;
        self.register_import_entity(solid)
    }
    /// 🧩️ Merges the imported body into `self.native.body` (see [`Body::merge`]) rather than replacing
    /// it — handles minted before this call stay resolvable, since nothing already in `self.native.body`
    /// is renumbered or dropped; only the freshly imported solids get new handles.
    
    pub fn import_stl_sync(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = import_stl_to_body(&mut self.native.body, data, tolerance).map_err(|error| map_err(&error))?;
        self.register_import_entity(solid)
    }
    pub fn import_obj_sync(&mut self, text: &str, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = import_obj_to_body(&mut self.native.body, text, tolerance).map_err(|error| map_err(&error))?;
        self.register_import_entity(solid)
    }
    fn register_import_entity(&mut self,entity:EntityRef)->Result<GeometryHandle,BrepError> {match entity {EntityRef::Solid(solid)=>Ok(self.native.register_solid(solid)),EntityRef::Shell(shell)=>Ok(self.native.register_shell(shell)),_=>Err(BrepError::InvalidInput("mesh import requires shell or solid".into()))}}
    /// 📦 Advances original production work and publishes its completed original entity.
    pub fn step_mesh_import_sync(&mut self,cursor:&mut MeshImportCursor,budget:usize)->Result<Option<GeometryHandle>,BrepError> {
        if budget>0 && !cursor.is_active() && !cursor.cleanup_complete() {cursor.activate(&mut self.native.pending_mutations);}
        let result=cursor.step(&mut self.native.body,budget).map_err(|error|map_err(&error));
        if cursor.cleanup_complete() {cursor.release();}
        match result? {MeshImportStep::Done(entity)=>self.register_import_entity(entity).map(Some),_=>Ok(None)}
    }
    /// 🎟️ Forwards every original cleanup currency and its physical receipt without publishing.
    /// 🧾️ Forwards the exact import close work before either original result branch.
    pub fn close_mesh_import_original(&mut self,cursor:&mut MeshImportCursor,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::RetainedWorkOutcome{
        let turn=cursor.close_original_step(&mut self.native.body,effect);
        if cursor.cleanup_complete() {cursor.release();}
        turn
    }
    pub fn close_mesh_import_sync(&mut self,cursor:&mut MeshImportCursor,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>{self.close_mesh_import_original(cursor,effect).result}
    pub fn export_mesh_sync(&self, shapes: &[GeometryHandle], deflection: f64, exporter: &dyn semio_framework_mesh_engine::io::MeshExporter) -> Result<Vec<u8>, BrepError> {
        let solid = self.native.solid_id(shapes.first().ok_or_else(|| BrepError::InvalidInput("empty".into()))?)?;
        export_solid_mesh(&self.native.body, solid, deflection, exporter).map_err(|error| map_err(&error))
    }
    pub fn import_mesh_sync(&mut self, data: &[u8], tolerance: f64, importer: &dyn semio_framework_mesh_engine::io::MeshImporter) -> Result<GeometryHandle, BrepError> {
        let solid = import_mesh_to_body(&mut self.native.body, data, tolerance, importer).map_err(|error| map_err(&error))?;
        self.register_import_entity(solid)
    }
    pub fn kind_sync(&self, shape: &GeometryHandle) -> Result<GeometryKind, BrepError> {
        Ok(Self::entity_kind(self.native.entity(shape)?))
    }

    /// 🎭️ Reads original geometry kind using its borrowed canonical key.
    pub fn original_topology_kind(&self, handle: &str) -> Result<GeometryKind, ValueError> {
        Ok(Self::entity_kind(self.native.original_query_entity(handle)?))
    }

    fn entity_kind(entity: &Entity) -> GeometryKind {
        match entity {
            Entity::Vertex(_) => GeometryKind::Vertex,
            Entity::Edge(_) => GeometryKind::Edge,
            Entity::Wire(_, _) => GeometryKind::Wire,
            Entity::Face(_) => GeometryKind::Face,
            Entity::Shell(_) => GeometryKind::Shell,
            Entity::Solid(_, _) => GeometryKind::Solid,
            Entity::Compound(_, _) => GeometryKind::Compound,
            Entity::Curve(_, _) => GeometryKind::Curve,
            Entity::Surface(_, _) => GeometryKind::Surface,
        }
    }
    /// 🧠 Face outer/hole loops as position indices into the returned vertex buffer.
    pub fn solid_face_loops_sync(&self, shape: &GeometryHandle) -> Result<SolidFaceLoops, BrepError> {
        let solid = self.native.solid_id(shape)?;
        let mut vertex_to_index: HashMap<u32, u32> = HashMap::new();
        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut face_loops = Vec::new();
        for face in self.native.body.solid_faces(solid) {
            let loops = self.native.body.face_loops(face);
            if loops.is_empty() {
                continue;
            }
            let mut indexed_loops: Vec<Vec<u32>> = Vec::new();
            for loop_id in loops {
                let mut loop_indices = Vec::new();
                for cid in self.native.body.loop_coedges(loop_id) {
                    let Some((start, _)) = self.native.body.coedge_endpoints(cid) else { continue };
                    let key = start.raw_index();
                    let index = if let Some(&existing) = vertex_to_index.get(&key) {
                        existing
                    } else {
                        let Some(vertex) = self.native.body.vertices.get(start) else {
                            return Err(BrepError::MissingHandle(format!("vertex {start}")));
                        };
                        let next = positions.len() as u32;
                        positions.push([vertex.position.x as f32, vertex.position.y as f32, vertex.position.z as f32]);
                        vertex_to_index.insert(key, next);
                        next
                    };
                    loop_indices.push(index);
                }
                if loop_indices.len() >= 3 {
                    indexed_loops.push(loop_indices);
                }
            }
            let mut iter = indexed_loops.into_iter();
            let Some(outer) = iter.next() else { continue };
            face_loops.push((outer, iter.collect()));
        }
        Ok((positions, face_loops))
    }
    pub fn tessellate_sync(&self,shape:&GeometryHandle,deflection:f64)->Result<MeshTransfer,BrepError> {
        self.tessellate_job_sync(shape,deflection)?.run_to_completion(&self.native.body,self.tessellation_input(shape)?).map(|(mesh,_)|mesh).map_err(|error|map_err(&error))
    }
    /// ⏱️ A resumable, budgetable tessellation of `shape` — the interactive twin of
    /// [`Brep::tessellate_sync`]. The caller drives [`Brep::step_tessellation_job_sync`]
    /// with the same original shape and an interactive work grant,
    /// reads `progress()` between steps, and `cancel()`s it when a newer request supersedes it.
    pub fn tessellate_job_sync(&self, shape: &GeometryHandle, deflection: f64) -> Result<TessellationJob, BrepError> {
        match self.native.entity(shape)? {
            Entity::Vertex(id)=>TessellationJob::for_vertex(&self.native.body,*id,deflection).map_err(|error|map_err(&error)),
            Entity::Curve(_,_)|Entity::Solid(_, _)|Entity::Face(_)|Entity::Shell(_)|Entity::Edge(_)|Entity::Wire(_,_)|Entity::Compound(_,_)=>Ok(TessellationJob::new(deflection)),
            Entity::Surface(Surface::Plane { .. }, _) => Ok(TessellationJob::for_analytic_plane(deflection)),
            other=>Err(BrepError::InvalidInput(format!("cannot tessellate {}",entity_tag(other)))),
        }
    }
    /// 💎️ Identifies the source revision borrowed by original validation.
    pub fn preview_validation_revision(&self)->u64 {self.native.shape_mutation_revision}

    /// 🌳️ Returns one original shape root without collecting sibling geometry.
    pub fn preview_validation_root(&self,handle:&str,index:usize)->Result<Option<EntityRef>,semio_framework_value::ValueError>{
        let entity=self.native.original_query_entity(handle)?;
        match entity{
            Entity::Solid(id,_)=>Ok((index==0).then_some(EntityRef::Solid(*id))),
            Entity::Shell(id)=>Ok((index==0).then_some(EntityRef::Shell(*id))),
            Entity::Compound(solids,_)=>Ok(solids.get(index).copied().map(EntityRef::Solid)),
            Entity::Face(id)=>Ok((index==0).then_some(EntityRef::Face(*id))),
            Entity::Edge(id)=>Ok((index==0).then_some(EntityRef::Edge(*id))),
            Entity::Vertex(id)=>Ok((index==0).then_some(EntityRef::Vertex(*id))),
            Entity::Wire(wire,_)=>{
                if let Some((edge,_))=wire.members.get(index){return Ok(Some(EntityRef::Edge(*edge)));}
                Ok(wire.vertices.get(index.saturating_sub(wire.members.len())).copied().map(EntityRef::Vertex))
            }
            Entity::Curve(..)|Entity::Surface(..)=>Ok(None),
        }
    }

    /// 🧪️ Verifies whether original construction already certified the current solid.
    pub fn original_validation_is_certified(&self,handle:&str)->Result<bool,semio_framework_value::ValueError>{
        Ok(matches!(self.native.original_query_entity(handle)?,Entity::Solid(_,Some(_))))
    }

    /// 🎟️ Admits the original cold query without allocating an owned error or reparsing its source.
    pub fn admit_tessellation_job(&self,source:&str,deflection:f64)->Result<(TessellationJob,bool),semio_framework_value::ValueError>{
        use semio_framework_value::{ValueError,ValueRefusalKind};
        use semio_framework_3d_brep_tessellation::PlanarPreviewRoot;
        let entity=self.native.live.get(source).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original tessellation source handle is not live"))?;
        match entity{
            Entity::Solid(id,provenance)=>Ok((TessellationJob::for_planar_shape(PlanarPreviewRoot::Solid(*id),self.native.shape_mutation_revision,deflection),!matches!(provenance,Some(SolidProvenance::PlanarBoundary)))),
            Entity::Shell(id)=>Ok((TessellationJob::for_planar_shape(PlanarPreviewRoot::Shell(*id),self.native.shape_mutation_revision,deflection),true)),
            Entity::Compound(_,label)=>Ok((TessellationJob::for_planar_shape(PlanarPreviewRoot::Compound(label.0),self.native.shape_mutation_revision,deflection),true)),
            Entity::Face(face)=>{
                if self.native.body.faces.get(*face).is_some_and(|row|
                    matches!(self.native.body.surfaces.get(row.surface), Some(Surface::Cylinder { .. } | Surface::Cone { .. }))) {
                    return Ok((TessellationJob::for_axial_face_preview(*face, self.native.shape_mutation_revision, deflection), false));
                }
                if let Some((curve,_,_))=self.conic_face_preview_source(*face) {
                    let labels=self.conic_preview_labels(*face).ok_or_else(||ValueError::invariant("original conic component labels are missing"))?;
                    return TessellationJob::for_conic_face_components(curve,deflection,self.native.shape_mutation_revision,labels).map(|job|(job,false))
                        .map_err(|_|ValueError::literal(ValueRefusalKind::InvalidValue,"conic face cannot be previewed at this tolerance"));
                }
                Ok((TessellationJob::for_planar_shape(PlanarPreviewRoot::Face(*face),self.native.shape_mutation_revision,deflection),false))
            }
            Entity::Wire(..) | Entity::Edge(_) | Entity::Vertex(_) => {
                let input = Self::wire_preview_source(entity).expect("selected entity is native boundary topology");
                Ok((TessellationJob::for_wire_preview(input, self.native.shape_mutation_revision, deflection), false))
            },
            Entity::Curve(curve,label)=>TessellationJob::for_curve_component(curve,label.0,deflection).map(|job|(job,false))
                .map_err(|_| ValueError::literal(ValueRefusalKind::InvariantViolated, "analytic curve cannot be admitted at this tolerance")),
            Entity::Surface(Surface::Plane{..},label)=>Ok((TessellationJob::for_plane_component(deflection,label.0),false)),
            Entity::Surface(surface,label)=>TessellationJob::for_surface_patch(surface,label.0,deflection).map(|job|(job,false)),
        }
    }
    fn wire_preview_source(entity: &Entity) -> Option<semio_framework_3d_brep_tessellation::WirePreviewSource<'_>> {
        use semio_framework_3d_brep_tessellation::WirePreviewSource;
        match entity {
            Entity::Vertex(id) => Some(WirePreviewSource::Vertex(*id)),
            Entity::Edge(id) => Some(WirePreviewSource::Edge(*id)),
            Entity::Wire(wire, label) => Some(WirePreviewSource::Wire { label: label.0, members: &wire.members }),
            _ => None,
        }
    }

    /// 📐️ Quotes the current funded preview and its unchanged borrowed native source.
    pub fn analytic_preview_demand(&self, handle: &str, job: &TessellationJob) -> Result<Option<semio_framework_value::RetirementDemand>, semio_framework_value::ValueError> {
        let body=&self.native.body;
        let mut quote=if job.is_wire_preview(){
            let input=self.native.live.get(handle).and_then(Self::wire_preview_source)
                .ok_or_else(||semio_framework_value::ValueError::invariant("original wire preview source disappeared"))?;
            Some(job.wire_preview_demand(body,input,self.native.shape_mutation_revision)?)
        }else if job.is_axial_face_preview() {
            let Some(Entity::Face(face)) = self.native.live.get(handle) else {
                return Err(semio_framework_value::ValueError::invariant("original axial face preview source disappeared"));
            };
            Some(job.axial_face_preview_demand(body, *face, self.native.shape_mutation_revision)?)
        }else if job.is_planar_preview(){
            let members=match self.native.live.get(handle){Some(Entity::Compound(items,_))=>Some(items.as_slice()),_=>None};
            Some(job.planar_preview_demand(body,members)?)
        }else{job.analytic_demand()};
        if let Some(demand)=quote.as_mut(){
            demand.depth=demand.depth.max(self.native.tessellation_source_depth_demand(handle)?);
            if !job.is_planar_preview() && !job.is_axial_face_preview(){
                if let Some(Entity::Face(id))=self.native.live.get(handle){
                    demand.depth=demand.depth.max(self.conic_preview_depth(*id)?);
                }
            }
        }
        Ok(quote)
    }

    /// ➰️ Borrows the admitted analytic entity for one funded preview step.
    pub fn step_analytic_tessellation(&self, handle: &str, job: &mut TessellationJob, grant: semio_framework_value::RetainedCloneGrant)
        -> Result<(Option<semio_framework_mesh_engine::MeshData>, semio_framework_value::RetainedCloneProgress), semio_framework_value::ValueError> {
        use semio_framework_3d_brep_tessellation::PlanarPreviewRoot;
        match self.native.live.get(handle) {
            Some(entity @ (Entity::Wire(..) | Entity::Edge(_) | Entity::Vertex(_))) if job.is_wire_preview() => {
                let input = Self::wire_preview_source(entity).expect("admitted entity is native boundary topology");
                job.step_wire_preview(&self.native.body, input, self.native.shape_mutation_revision, grant)
            }
            Some(Entity::Curve(curve,label))=>job.step_curve_component(curve,label.0,grant),
            Some(Entity::Surface(Surface::Plane{frame},label))=>job.step_plane_component(frame,label.0,grant),
            Some(Entity::Surface(surface,label))=>job.step_surface_patch_funded(surface,label.0,grant),
            Some(Entity::Solid(id,_)) if job.is_planar_preview()=>job.step_planar_funded(&self.native.body,PlanarPreviewRoot::Solid(*id),None,self.native.shape_mutation_revision,grant),
            Some(Entity::Shell(id)) if job.is_planar_preview()=>job.step_planar_funded(&self.native.body,PlanarPreviewRoot::Shell(*id),None,self.native.shape_mutation_revision,grant),
            Some(Entity::Face(face)) if job.is_axial_face_preview() =>
                job.step_axial_face_preview(&self.native.body, *face, self.native.shape_mutation_revision, grant),
            Some(Entity::Face(id)) if job.is_planar_preview()=>job.step_planar_funded(&self.native.body,PlanarPreviewRoot::Face(*id),None,self.native.shape_mutation_revision,grant),
            Some(Entity::Compound(members,label)) if job.is_planar_preview()=>
                job.step_planar_funded(&self.native.body,PlanarPreviewRoot::Compound(label.0),Some(members),self.native.shape_mutation_revision,grant),
            Some(Entity::Face(face)) => {
                let (curve,normal,flip)=self.conic_face_preview_source(*face).ok_or_else(||semio_framework_value::ValueError::invariant("original conic face preview source changed"))?;
                job.step_conic_face_components(curve,normal,flip,self.native.shape_mutation_revision,grant)
            }
            _ => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "analytic preview source disappeared")),
        }
    }

    pub(super) fn conic_preview_depth(&self,id:FaceId)->Result<usize,semio_framework_value::ValueError>{
        let body=&self.native.body;
        let invalid=||semio_framework_value::ValueError::invariant("original conic preview topology disappeared");
        let face=body.faces.get(id).ok_or_else(invalid)?;
        let ring=face.outer.ok_or_else(invalid)?;
        let coedge=body.loops.get(ring).ok_or_else(invalid)?.first;
        let use_=body.coedges.get(coedge).ok_or_else(invalid)?;
        let edge=body.edges.get(use_.edge).ok_or_else(invalid)?;
        let trim=use_.pcurve.ok_or_else(invalid)?;
        Ok(body.faces.next_remove_depth_demand(id)?
            .max(body.surfaces.next_remove_depth_demand(face.surface)?)
            .max(body.loops.next_remove_depth_demand(ring)?)
            .max(body.coedges.next_remove_depth_demand(coedge)?)
            .max(body.curves2.next_remove_depth_demand(trim)?)
            .max(body.edges.next_remove_depth_demand(use_.edge)?)
            .max(body.curves3.next_remove_depth_demand(edge.curve)?))
    }

    fn conic_preview_labels(&self,id:FaceId)->Option<[u64;3]>{
        let body=&self.native.body;
        let face=body.faces.get(id)?;
        let ring=body.loops.get(face.outer?)?;
        let edge=body.edges.get(body.coedges.get(ring.first)?.edge)?;
        let vertex=body.vertices.get(edge.v0)?;
        Some([face.label.0,edge.label.0,vertex.label.0])
    }

    pub(super) fn conic_face_preview_source(&self, id: FaceId) -> Option<(&Curve3, EVec3, bool)> {
        self.native.conic_face_boundary(id)
    }

    fn tessellation_input<'a>(&'a self,shape:&GeometryHandle)->Result<semio_framework_3d_brep_tessellation::TessellationInput<'a>,BrepError> {
        use semio_framework_3d_brep_tessellation::TessellationInput;
        match self.native.entity(shape)? {
            Entity::Surface(Surface::Plane { frame }, _) => Ok(TessellationInput::Plane(frame)),
            Entity::Curve(curve,_)=>Ok(TessellationInput::Curve(curve)),Entity::Vertex(id)=>Ok(TessellationInput::Vertex(*id)),Entity::Edge(id)=>Ok(TessellationInput::Edge(*id)),Entity::Solid(id, _)=>Ok(TessellationInput::Solid(*id)),Entity::Face(id)=>Ok(TessellationInput::Face(*id)),Entity::Shell(id)=>Ok(TessellationInput::Shell(*id)),Entity::Wire(wire,_)=>Ok(TessellationInput::Wire(&wire.members)),Entity::Compound(solids,_)=>Ok(TessellationInput::Solids(solids)),other=>Err(BrepError::InvalidInput(format!("cannot tessellate {}",entity_tag(other)))),
        }
    }
    /// ⏱️ Presents the same original entity to one granted topology/mesh turn.
    pub fn step_tessellation_job_sync(&self,shape:&GeometryHandle,job:&mut TessellationJob,budget:usize)->Result<semio_framework_3d_brep_tessellation::TessellationStep,BrepError> {
        job.step(&self.native.body,self.tessellation_input(shape)?,budget).map_err(|error|map_err(&error))
    }

    /// 🧬 The topology a [`TessellationJob`] steps against — the job borrows nothing, so a host can
    /// retain it across turns and re-present the body on every step.
    pub fn tessellation_body(&self) -> &Body {
        &self.native.body
    }

}
// #endregion 🔖️SyncApi
