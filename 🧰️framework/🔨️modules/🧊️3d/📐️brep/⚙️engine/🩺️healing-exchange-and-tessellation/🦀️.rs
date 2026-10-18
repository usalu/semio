//! 🩺️ Healing, exchange and tessellation: the solid healing and NURBS conversion, the deconstruction, the mesh import and export, the kind lookup, the solid face loops and the tessellation jobs. See [`super`].

use super::*;

// #region 🔖️SyncApi
impl Brep {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn heal_solid_sync(&mut self, shape: &GeometryHandle, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let mut rec = OpRecorder::new();
        let _ = heal_solid(&mut self.body, solid, tolerance, &mut rec).map_err(|error| map_err(&error))?;
        Ok(shape.clone())
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn convert_to_nurbs_sync(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let mut rec = OpRecorder::new();
        let _ = convert_to_nurbs(&mut self.body, solid, &mut rec).map_err(|error| map_err(&error))?;
        Ok(shape.clone())
    }
    /// 🏷️ Uses the same shape reach as validation and retirement, sorted by arena identity.
    /// [`Brep::mint`] retains each component's [`PersistentLabel`] across repeated decomposition.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn deconstruct_sync(&mut self, shape: &GeometryHandle) -> Result<BrepTopology, BrepError> {
        let reach = self.body.reachable_from(&entity_roots(self.entity(shape)?));
        let mut vertices: Vec<_> = reach.vertices.into_iter().collect(); vertices.sort_unstable_by_key(|id| id.raw_index());
        let mut edges: Vec<_> = reach.edges.into_iter().collect(); edges.sort_unstable_by_key(|id| id.raw_index());
        let mut faces: Vec<_> = reach.faces.into_iter().collect(); faces.sort_unstable_by_key(|id| id.raw_index());
        let mut shells: Vec<_> = reach.shells.into_iter().collect(); shells.sort_unstable_by_key(|id| id.raw_index());
        Ok(BrepTopology {
            vertices: vertices.into_iter().map(|id| self.mint(GeometryKind::Vertex, Entity::Vertex(id))).collect(),
            edges: edges.into_iter().map(|id| self.mint(GeometryKind::Edge, Entity::Edge(id))).collect(),
            faces: faces.into_iter().map(|id| self.register_face(id)).collect(),
            shells: shells.into_iter().map(|id| self.register_shell(id)).collect(),
        })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn export_stl_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        let solid = self.solid_id(shapes.first().ok_or_else(|| BrepError::InvalidInput("empty".into()))?)?;
        export_solid_stl(&self.body, solid, deflection).map_err(|error| map_err(&error))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn export_obj_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<String, BrepError> {
        let solid = self.solid_id(shapes.first().ok_or_else(|| BrepError::InvalidInput("empty".into()))?)?;
        export_solid_obj(&self.body, solid, deflection).map_err(|error| map_err(&error))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn export_gltf_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        self.export_glb_sync(shapes, deflection)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn export_glb_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        let solid = self.solid_id(shapes.first().ok_or_else(|| BrepError::InvalidInput("empty".into()))?)?;
        export_solid_glb(&self.body, solid, deflection).map_err(|error| map_err(&error))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn import_glb_sync(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = import_glb_to_body(&mut self.body, data, tolerance).map_err(|error| map_err(&error))?;
        self.register_import_entity(solid)
    }
    /// 🧩️ Merges the imported body into `self.body` (see [`Body::merge`]) rather than replacing
    /// it — handles minted before this call stay resolvable, since nothing already in `self.body`
    /// is renumbered or dropped; only the freshly imported solids get new handles.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn import_stl_sync(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = import_stl_to_body(&mut self.body, data, tolerance).map_err(|error| map_err(&error))?;
        self.register_import_entity(solid)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn import_obj_sync(&mut self, text: &str, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = import_obj_to_body(&mut self.body, text, tolerance).map_err(|error| map_err(&error))?;
        self.register_import_entity(solid)
    }
    fn register_import_entity(&mut self,entity:EntityRef)->Result<GeometryHandle,BrepError> {match entity {EntityRef::Solid(solid)=>Ok(self.register_solid(solid)),EntityRef::Shell(shell)=>Ok(self.register_shell(shell)),_=>Err(BrepError::InvalidInput("mesh import requires shell or solid".into()))}}
    /// 📦 Advances original production work and publishes its completed original entity.
    pub fn step_mesh_import_sync(&mut self,cursor:&mut MeshImportCursor,budget:usize)->Result<Option<GeometryHandle>,BrepError> {
        if budget>0 && !cursor.is_active() && !cursor.retirement_complete() {cursor.set_active(true);self.pending_mutations+=1;}
        let result=cursor.step(&mut self.body,budget).map_err(|error|map_err(&error));
        if cursor.cleanup_complete() && cursor.is_active() {cursor.set_active(false);self.pending_mutations=self.pending_mutations.checked_sub(1).expect("owned import mutation");}
        match result? {MeshImportStep::Done(entity)=>self.register_import_entity(entity).map(Some),_=>Ok(None)}
    }
    /// 🎟️ Forwards every original cleanup currency and its physical receipt without publishing.
    pub fn close_mesh_import_sync(&mut self,cursor:&mut MeshImportCursor,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError> {
        let step=cursor.close_step(&mut self.body,grant)?;
        if cursor.cleanup_complete() && cursor.is_active() {cursor.set_active(false);self.pending_mutations=self.pending_mutations.checked_sub(1).expect("owned import mutation");}
        Ok(step)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn export_mesh_sync(&self, shapes: &[GeometryHandle], deflection: f64, exporter: &dyn semio_framework_mesh_engine::io::MeshExporter) -> Result<Vec<u8>, BrepError> {
        let solid = self.solid_id(shapes.first().ok_or_else(|| BrepError::InvalidInput("empty".into()))?)?;
        export_solid_mesh(&self.body, solid, deflection, exporter).map_err(|error| map_err(&error))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn import_mesh_sync(&mut self, data: &[u8], tolerance: f64, importer: &dyn semio_framework_mesh_engine::io::MeshImporter) -> Result<GeometryHandle, BrepError> {
        let solid = import_mesh_to_body(&mut self.body, data, tolerance, importer).map_err(|error| map_err(&error))?;
        self.register_import_entity(solid)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn kind_sync(&self, shape: &GeometryHandle) -> Result<GeometryKind, BrepError> {
        Ok(match self.entity(shape)? {
            Entity::Vertex(_) => GeometryKind::Vertex,
            Entity::Edge(_) => GeometryKind::Edge,
            Entity::Wire(_, _) => GeometryKind::Wire,
            Entity::Face(_) => GeometryKind::Face,
            Entity::Shell(_) => GeometryKind::Shell,
            Entity::Solid(_) => GeometryKind::Solid,
            Entity::Compound(_, _) => GeometryKind::Compound,
            Entity::Curve(_, _) => GeometryKind::Curve,
            Entity::Surface(_, _) => GeometryKind::Surface,
        })
    }
    /// 🧠 Face outer/hole loops as position indices into the returned vertex buffer.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn solid_face_loops_sync(&self, shape: &GeometryHandle) -> Result<SolidFaceLoops, BrepError> {
        let solid = self.solid_id(shape)?;
        let mut vertex_to_index: HashMap<u32, u32> = HashMap::new();
        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut face_loops = Vec::new();
        for face in self.body.solid_faces(solid) {
            let loops = self.body.face_loops(face);
            if loops.is_empty() {
                continue;
            }
            let mut indexed_loops: Vec<Vec<u32>> = Vec::new();
            for loop_id in loops {
                let mut loop_indices = Vec::new();
                for cid in self.body.loop_coedges(loop_id) {
                    let Some((start, _)) = self.body.coedge_endpoints(cid) else { continue };
                    let key = start.raw_index();
                    let index = if let Some(&existing) = vertex_to_index.get(&key) {
                        existing
                    } else {
                        let Some(vertex) = self.body.vertices.get(start) else {
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn tessellate_sync(&self,shape:&GeometryHandle,deflection:f64)->Result<MeshTransfer,BrepError> {
        self.tessellate_job_sync(shape,deflection)?.run_to_completion(&self.body,self.tessellation_input(shape)?).map(|(mesh,_)|mesh).map_err(|error|map_err(&error))
    }
    /// ⏱️ A resumable, budgetable tessellation of `shape` — the interactive twin of
    /// [`Brep::tessellate_sync`]. The caller drives [`Brep::step_tessellation_job_sync`]
    /// with the same original shape and an interactive work grant,
    /// reads `progress()` between steps, and `cancel()`s it when a newer request supersedes it.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn tessellate_job_sync(&self, shape: &GeometryHandle, deflection: f64) -> Result<TessellationJob, BrepError> {
        match self.entity(shape)? {
            Entity::Vertex(id)=>TessellationJob::for_vertex(&self.body,*id,deflection).map_err(|error|map_err(&error)),
            Entity::Solid(_)|Entity::Face(_)|Entity::Shell(_)|Entity::Wire(_,_)|Entity::Compound(_,_)=>Ok(TessellationJob::new(deflection)),
            other=>Err(BrepError::InvalidInput(format!("cannot tessellate {}",entity_tag(other)))),
        }
    }
    /// 🧊️ Quotes the original borrowed handle index before cold query admission.
    pub fn tessellation_source_depth_demand(&self,source:&str)->Result<usize,semio_framework_value::ValueError>{self.live.next_contains_depth_demand(source)}
    /// 🎟️ Admits the original cold query without allocating an owned error or reparsing its source.
    pub fn admit_tessellation_job(&self,source:&str,deflection:f64)->Result<(TessellationJob,bool),semio_framework_value::ValueError>{
        use semio_framework_value::{ValueError,ValueRefusalKind};
        let entity=self.live.get(source).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original tessellation source handle is not live"))?;
        match entity{
            Entity::Vertex(id)=>{if self.body.vertices.get(*id).is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original tessellation source vertex is missing"));}TessellationJob::for_vertex(&self.body,*id,deflection).map(|job|(job,false)).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"original tessellation vertex admission failed"))},
            Entity::Solid(_)|Entity::Shell(_)|Entity::Compound(_,_)=>Ok((TessellationJob::new(deflection),true)),
            Entity::Face(_)|Entity::Wire(_,_)=>Ok((TessellationJob::new(deflection),false)),
            _=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original tessellation source kind is unsupported")),
        }
    }
    fn tessellation_input<'a>(&'a self,shape:&GeometryHandle)->Result<semio_framework_3d_brep_tessellation::TessellationInput<'a>,BrepError> {
        use semio_framework_3d_brep_tessellation::TessellationInput;
        match self.entity(shape)? {
            Entity::Vertex(id)=>Ok(TessellationInput::Vertex(*id)),Entity::Solid(id)=>Ok(TessellationInput::Solid(*id)),Entity::Face(id)=>Ok(TessellationInput::Face(*id)),Entity::Shell(id)=>Ok(TessellationInput::Shell(*id)),Entity::Wire(wire,_)=>Ok(TessellationInput::Wire(&wire.members)),Entity::Compound(solids,_)=>Ok(TessellationInput::Solids(solids)),other=>Err(BrepError::InvalidInput(format!("cannot tessellate {}",entity_tag(other)))),
        }
    }
    /// ⏱️ Presents the same original entity to one granted topology/mesh turn.
    pub fn step_tessellation_job_sync(&self,shape:&GeometryHandle,job:&mut TessellationJob,budget:usize)->Result<semio_framework_3d_brep_tessellation::TessellationStep,BrepError> {
        job.step(&self.body,self.tessellation_input(shape)?,budget).map_err(|error|map_err(&error))
    }

    /// 🧬 The topology a [`TessellationJob`] steps against — the job borrows nothing, so a host can
    /// retain it across turns and re-present the body on every step.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn tessellation_body(&self) -> &Body {
        &self.body
    }

}
// #endregion 🔖️SyncApi
