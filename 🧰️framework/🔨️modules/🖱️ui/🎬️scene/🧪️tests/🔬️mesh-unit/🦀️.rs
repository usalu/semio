//! 🧪️ Unit proofs of the paged mesh authority, native resource custody and exact physical retirement receipts.

use super::testkit::{admit_mesh, allocate_mesh, close_mesh, mesh_test_attempt, mesh_test_attempt_controlled, observe_retirement_allocations, ownership_test_grant, paged_mesh_fixture, test_box_mesh, LegacyMeshOracleData, ObservedAllocator};
use super::*;
use semio_framework_ui_scene_algebra::vec3_new_m;
use std::collections::{BTreeMap, BTreeSet};

fn abort_mesh(token:Mesh3dWriteToken)->Result<bool,Mesh3dFault> {let demand=mesh_test_attempt(||mesh3d_abort_demands(token,MESH3D_PAGE_BYTES))?;let grant=ownership_test_grant(MESH3D_PAGE_BYTES,demand.capacity_bytes,demand.release_bytes,demand.depth);let step=mesh_test_attempt(||mesh3d_abort_step(token,grant))?;assert!(step.progress.fits(grant));Ok(step.complete)}
fn move_references(token:Mesh3dWriteToken,table:&mut ComponentReferenceTable)->Result<(),Mesh3dFault> {let grant=ownership_test_grant(0,mesh_test_attempt(||mesh3d_component_capacity_byte_demand(table))?,0,1);let receipt=mesh_test_attempt(||mesh3d_move_component_references(token,table,grant))?;assert!(receipt.fits(grant));Ok(())}
fn admit_components(token:Mesh3dWriteToken,items:usize,work:usize)->Result<Mesh3dComponentAdmissionStep,Mesh3dFault> {let grant=RetainedCloneGrant{maximum_items:items,..ownership_test_grant(work,0,0,1)};let step=mesh_test_attempt(||mesh3d_component_admission_step(token,grant))?;assert!(step.ownership.fits(grant));Ok(step)}
fn reference_fixture(value:serde_json::Value)->ComponentReferenceTable {let oracle:BTreeMap<String,Vec<String>>=serde_json::from_value(value).unwrap();ComponentReferenceTable::from_entries(oracle.into_iter().collect())}
fn authority_close(authority:&mut Mesh3dAuthority,slot:u16,epoch:u64)->Result<bool,Mesh3dFault> {let demand=authority.close_demands(slot,epoch,MESH3D_PAGE_BYTES)?;let grant=ownership_test_grant(MESH3D_PAGE_BYTES,demand.capacity_bytes,demand.release_bytes,demand.depth);let step=authority.close_step(slot,epoch,grant)?;assert!(step.progress.fits(grant));Ok(step.complete)}
fn authority_move(authority:&mut Mesh3dAuthority,token:Mesh3dWriteToken,original:&mut ComponentReferenceTable)->Result<(),Mesh3dFault> {let grant=ownership_test_grant(0,mesh_test_attempt(||mesh3d_component_capacity_byte_demand(original))?,0,1);let progress=authority.move_component_references(token,original,grant)?;assert!(progress.fits(grant));Ok(())}
fn authority_admit(authority:&mut Mesh3dAuthority,token:Mesh3dWriteToken,items:usize,work:usize)->Result<Mesh3dComponentAdmissionStep,Mesh3dFault> {let grant=RetainedCloneGrant{maximum_items:items,..ownership_test_grant(work,0,0,1)};let step=authority.component_admission_step(token,grant)?;assert!(step.ownership.fits(grant));Ok(step)}
fn authority_begin(authority:&mut Mesh3dAuthority,generation:u64,revision:u64,schema:Mesh3dSchema)->Result<Mesh3dWriteToken,Mesh3dFault> {let grant=ownership_test_grant(0,mesh3d_begin_capacity_byte_demand(),0,1);let(token,receipt)=authority.begin(generation,revision,schema,grant)?;assert!(receipt.fits(grant));Ok(token)}

#[test]
fn mesh3d_constructor_pages_and_directory_require_exact_physical_grants() {
    use protocol::value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../📐️math/♻️retirement/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    use std::io::Write;
    let mut oracle=std::process::Command::new("bun").args(["-e","const x=JSON.parse(await Bun.stdin.text());console.log(JSON.stringify(x.fixture.copyGrants.map(work=>({work,pageBytes:x.pageBytes,directoryBytes:x.directoryBytes}))));"]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn().unwrap();
    oracle.stdin.take().unwrap().write_all(serde_json::json!({"fixture":fixture,"pageBytes":MESH3D_PAGE_BYTES,"directoryBytes":MESH3D_OWNER_PAGE_CAPACITY*std::mem::size_of::<Option<Mesh3dPage>>()}).to_string().as_bytes()).unwrap();
    let result=oracle.wait_with_output().unwrap();assert!(result.status.success());let rows:serde_json::Value=serde_json::from_slice(&result.stdout).unwrap();
    for row in rows.as_array().unwrap() {
        let work=row["work"].as_u64().unwrap()as usize;let birth=mesh3d_begin_capacity_byte_demand();assert_eq!(birth,row["directoryBytes"].as_u64().unwrap()as usize);
        let mut grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work,maximum_capacity_bytes:birth-1,maximum_release_bytes:0,maximum_depth:1};
        let(answer,heap)=observe_retirement_allocations(||mesh_test_attempt(||mesh3d_begin(700,800,Mesh3dSchema::triangle_mesh(3,3),grant)));assert_eq!(answer,Err(Mesh3dFault::ByteCapacity));assert_eq!(heap,(0,0));
        grant.maximum_capacity_bytes=birth;let(answer,heap)=observe_retirement_allocations(||mesh_test_attempt(||mesh3d_begin(700,800,Mesh3dSchema::triangle_mesh(3,3),grant)));let(token,receipt)=answer.unwrap();assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));assert_eq!(heap,(birth,0));let mut born=birth;let mut physical=0;
        let page=mesh_test_attempt(||mesh3d_allocate_capacity_byte_demand(token)).unwrap();assert_eq!(page,row["pageBytes"].as_u64().unwrap()as usize);grant.maximum_capacity_bytes=page-1;
        let(step,heap)=observe_retirement_allocations(||mesh_test_attempt(||mesh3d_allocate_step(token,grant)).unwrap());assert_eq!(step.progress,RetainedCloneProgress::default());assert!(!step.complete);assert_eq!(heap,(0,0));
        grant.maximum_capacity_bytes=page;let(step,heap)=observe_retirement_allocations(||mesh_test_attempt(||mesh3d_allocate_step(token,grant)).unwrap());assert!(step.complete);assert_eq!(heap,(step.progress.retained_capacity_bytes,step.progress.released_bytes));born+=heap.0;
        mesh_test_attempt(||mesh3d_abort(token)).unwrap();let demand=mesh_test_attempt(||mesh3d_abort_demands(token,work)).unwrap();assert_eq!(demand.release_bytes,page);grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:page-1,maximum_depth:demand.depth};
        let(step,heap)=observe_retirement_allocations(||mesh_test_attempt(||mesh3d_abort_step(token,grant)).unwrap());assert_eq!(step.progress,RetainedCloneProgress::default());assert!(!step.complete);assert_eq!(heap,(0,0));
        let mut directory_refused=false;
        for _ in 0..100000 {let demand=mesh_test_attempt(||mesh3d_abort_demands(token,work)).unwrap();let mut grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};if demand.release_bytes==birth&&!directory_refused{grant.maximum_release_bytes=birth-1;let(step,heap)=observe_retirement_allocations(||mesh_test_attempt(||mesh3d_abort_step(token,grant)).unwrap());assert!(!step.complete);assert_eq!(step.progress,RetainedCloneProgress::default());assert_eq!(heap,(0,0));directory_refused=true;grant.maximum_release_bytes=birth;}let(step,heap)=observe_retirement_allocations(||mesh_test_attempt(||mesh3d_abort_step(token,grant)).unwrap());assert!(step.progress.fits(grant));assert_eq!(heap,(step.progress.retained_capacity_bytes,step.progress.released_bytes));born+=heap.0;physical+=heap.1;if step.complete {break;}}
        assert!(directory_refused);
        assert!(mesh_test_attempt(||mesh3d_authority().try_lock().map_err(|_|Mesh3dFault::Busy)).unwrap().slots[usize::from(token.slot)].is_none());assert_eq!(born,physical);
    }
}

/// 🔐️ Measures the system lock separately from the original mesh owner constructor.
#[test]
fn mesh3d_system_mutex_physical_boundary_has_exact_terminal_release() {
    let mutex=std::sync::Mutex::new(());
    let (_,first)=observe_retirement_allocations(||drop(mutex.lock().unwrap()));
    let (_,repeat)=observe_retirement_allocations(||drop(mutex.lock().unwrap()));
    let (_,terminal)=observe_retirement_allocations(||drop(mutex));
    assert_eq!(repeat,(0,0));
    assert_eq!(first.1,0);
    assert_eq!(terminal.0,0);
    assert_eq!(first.0,terminal.1);
}

/// 🚥️ Preserves the System oracle's exclusion while the admitted owner gate allocates no backing.
#[test]
fn mesh3d_retained_owner_gate_matches_system_exclusion_without_heap_birth() {
    let system=std::sync::Mutex::new(7u32);
    let system_guard=system.try_lock().unwrap();
    assert!(system.try_lock().is_err());
    assert_eq!(*system_guard,7);
    drop(system_guard);
    *system.try_lock().unwrap()=11;
    let gate=protocol::value::retirement::controlled::RetainedOwnerGate::new(7u32);
    let (_,heap)=observe_retirement_allocations(||{
        let mut owner=gate.try_lock().unwrap();
        assert!(gate.try_lock().is_err());
        assert_eq!(*owner,7);
        *owner=11;
    });
    assert_eq!(heap,(0,0));
    assert_eq!(*gate.try_lock().unwrap(),*system.try_lock().unwrap());
    let (_,heap)=observe_retirement_allocations(||drop(gate));
    assert_eq!(heap,(0,0));
}

#[test]
fn mesh3d_original_component_backings_and_cancelled_hash_index_close_with_exact_receipts() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../📐️math/♻️retirement/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["sourceTables"].as_array().unwrap(){for work in fixture["copyGrants"].as_array().unwrap(){let work=work.as_u64().unwrap()as usize;
        let mut table=reference_fixture(row.clone());let original=table.original_capacity_bytes().unwrap();let pointers=table.iter().map(|(_,labels)|labels.iter().map(|label|label.as_ptr()).collect::<Vec<_>>()).collect::<Vec<_>>();
        let grant=ownership_test_grant(0,mesh3d_begin_capacity_byte_demand(),0,1);let(answer,heap)=observe_retirement_allocations(||mesh_test_attempt(||mesh3d_begin(900,1000,Mesh3dSchema::triangle_mesh(3,3),grant)));let(token,receipt)=answer.unwrap();assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));let mut born=heap.0;let mut physical=heap.1;
        let grant=ownership_test_grant(0,mesh_test_attempt(||mesh3d_allocate_capacity_byte_demand(token)).unwrap(),0,1);let(step,heap)=observe_retirement_allocations(||mesh_test_attempt(||mesh3d_allocate_step(token,grant)).unwrap());assert!(step.complete);assert_eq!(heap,(step.progress.retained_capacity_bytes,step.progress.released_bytes));born+=heap.0;physical+=heap.1;
        let capacity=mesh_test_attempt(||mesh3d_component_capacity_byte_demand(&table)).unwrap();if capacity>0 {let grant=ownership_test_grant(0,capacity-1,0,1);let(answer,heap)=observe_retirement_allocations(||mesh_test_attempt(||mesh3d_move_component_references(token,&mut table,grant)));assert_eq!(answer,Err(Mesh3dFault::ByteCapacity));assert_eq!(heap,(0,0));assert_eq!(table.iter().map(|(_,labels)|labels.iter().map(|label|label.as_ptr()).collect::<Vec<_>>()).collect::<Vec<_>>(),pointers);}
        let grant=ownership_test_grant(0,capacity,0,1);let(answer,heap)=observe_retirement_allocations(||mesh_test_attempt(||mesh3d_move_component_references(token,&mut table,grant)));let receipt=answer.unwrap();assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));assert!(table.is_empty());born+=heap.0;physical+=heap.1;let authority=mesh_test_attempt(||mesh3d_authority().try_lock().map_err(|_|Mesh3dFault::Busy)).unwrap();assert_eq!(authority.writing_ref(token).unwrap().component_references.as_ref().unwrap().iter().map(|(_,labels)|labels.iter().map(|label|label.as_ptr()).collect::<Vec<_>>()).collect::<Vec<_>>(),pointers);drop(authority);
        for _ in 0..2 {let grant=ownership_test_grant(64,0,0,1);let(step,heap)=observe_retirement_allocations(||mesh_test_attempt(||mesh3d_component_admission_step(token,grant)).unwrap());assert!(step.ownership.fits(grant));assert_eq!(heap,(step.ownership.retained_capacity_bytes,step.ownership.released_bytes));born+=heap.0;physical+=heap.1;if step.complete{break;}}
        mesh_test_attempt(||mesh3d_abort(token)).unwrap();let mut closed=false;for _ in 0..100000 {let demand=mesh_test_attempt(||mesh3d_abort_demands(token,work)).unwrap();let grant=ownership_test_grant(work,demand.capacity_bytes,demand.release_bytes,demand.depth);let(step,heap)=observe_retirement_allocations(||mesh_test_attempt(||mesh3d_abort_step(token,grant)).unwrap());assert!(step.progress.fits(grant));assert_eq!(heap,(step.progress.retained_capacity_bytes,step.progress.released_bytes));born+=heap.0;physical+=heap.1;if step.complete{closed=true;break;}}
        assert!(closed);assert_eq!(original+born,physical);
    }}
}

/// 🎯️ The existing paged mesh owner retains original labels without cloning or narrowing.
#[test]
fn mesh3d_original_component_references_and_source_are_exact_and_retired() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎯️component-source/🔣️.json")).unwrap();
    let source = ComponentSource3d::new(fixture["source"]["handle"].as_str().unwrap(), fixture["source"]["revision"].as_str().unwrap()).unwrap();
    assert_eq!(source.handle(), fixture["source"]["handle"].as_str().unwrap());
    assert_eq!(source.revision(), fixture["source"]["revision"].as_str().unwrap());
    assert!(ComponentSource3d::new(&source.handle().to_uppercase(), source.revision()).is_none());
    assert!(ComponentSource3d::new(&source.handle()[..63], source.revision()).is_none());
    let field = |kind| match kind { "face" => Mesh3dField::FaceIds, "edge" => Mesh3dField::EdgeIds, "vertex" => Mesh3dField::VertexIds, _ => panic!("unknown neutral component kind") };
    let publish = |references: &mut ComponentReferenceTable| {
        let token = admit_mesh(77, 88, Mesh3dSchema::triangle_mesh(3, 3)).unwrap();
        while !allocate_mesh(token).unwrap() {}
        for point in [[-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], [0.0, 1.0, 0.0]] { mesh_test_attempt(||mesh3d_write_vec3(token, Mesh3dField::Positions, point)).unwrap(); mesh_test_attempt(||mesh3d_write_vec3(token, Mesh3dField::Normals, [0.0, 0.0, 1.0])).unwrap(); }
        for index in 0..3 { mesh_test_attempt(||mesh3d_write_u32(token, Mesh3dField::Indices, index)).unwrap(); }
        move_references(token, references).unwrap();
        while !admit_components(token, 1, MESH3D_PAGE_BYTES).unwrap().complete {}
        mesh_test_attempt(||mesh3d_seal(token)).unwrap()
    };
    let close = |lease| { mesh_test_attempt(||mesh3d_begin_close(lease)).unwrap(); let mut turns = 0; while !close_mesh(lease).unwrap() { turns += 1; assert!(turns < 256); } assert!(mesh_test_attempt(||if mesh3d_terminal_is_empty(lease){Ok(true)}else{Err(Mesh3dFault::Busy)}).unwrap()); };
    let mut references:ComponentReferenceTable=reference_fixture(fixture["references"].clone());
    let rows_pointer = references["face"].as_ptr();
    let label_pointer = references["face"][1].as_ptr();
    let lease = publish(&mut references);
    assert!(references.is_empty());
    {
        let authority = mesh_test_attempt(||mesh3d_authority().try_lock().map_err(|_|Mesh3dFault::Busy)).unwrap();
        let original = authority.ready(lease).unwrap().component_references.as_ref().unwrap();
        assert_eq!(original["face"].as_ptr(), rows_pointer);
        assert_eq!(original["face"][1].as_ptr(), label_pointer);
    }
    for row in fixture["cases"].as_array().unwrap() {
        let expected: u64 = serde_json::from_str(row["label"].as_str().unwrap()).unwrap();
        assert_eq!(mesh_test_attempt(||lease.component_label(field(row["kind"].as_str().unwrap()), row["group"].as_u64().unwrap() as u32)).unwrap(), Some(expected));
    }
    assert_eq!(mesh_test_attempt(||lease.component_label(Mesh3dField::FaceIds, 2)).unwrap(), None);
    assert_eq!(mesh_test_attempt(||lease.component_label(Mesh3dField::Indices, 0)), Err(Mesh3dFault::Schema));
    close(lease);
    assert_eq!(mesh_test_attempt(||lease.component_label(Mesh3dField::FaceIds, 0)), Err(Mesh3dFault::Stale));
    for label in fixture["invalidLabels"].as_array().unwrap() {
        let mut original = ComponentReferenceTable::from_entries(vec![("face".into(), vec![label.as_str().unwrap().into()])]);
        let lease = publish(&mut original);
        assert_eq!(mesh_test_attempt(||lease.component_label(Mesh3dField::FaceIds, 0)), Err(Mesh3dFault::Schema));
        close(lease);
    }
}

fn component_admission_test_publish(original: &mut ComponentReferenceTable) -> Result<Mesh3dLease, Mesh3dFault> {
    let token = admit_mesh(79, 90, Mesh3dSchema::triangle_mesh(3, 3))?;
    let result = (|| {
        while !allocate_mesh(token)? {}
        for point in [[-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], [0.0, 1.0, 0.0]] { mesh_test_attempt(||mesh3d_write_vec3(token, Mesh3dField::Positions, point))?; mesh_test_attempt(||mesh3d_write_vec3(token, Mesh3dField::Normals, [0.0, 0.0, 1.0]))?; }
        for index in 0..3 { mesh_test_attempt(||mesh3d_write_u32(token, Mesh3dField::Indices, index))?; }
        move_references(token, original)?;
        while !admit_components(token, 1, MESH3D_PAGE_BYTES)?.complete {}
        mesh_test_attempt(||mesh3d_seal(token))
    })();
    if result.is_err() { mesh_test_attempt(||mesh3d_abort(token)).unwrap(); while !abort_mesh(token).unwrap() {} }
    result
}

fn close_component_admission_test_mesh(lease: Mesh3dLease) {
    mesh_test_attempt(||mesh3d_begin_close(lease)).unwrap();
    for _ in 0..100000 { if close_mesh(lease).unwrap() { assert!(mesh_test_attempt(||if mesh3d_terminal_is_empty(lease){Ok(true)}else{Err(Mesh3dFault::Busy)}).unwrap()); return; } }
    panic!("original metadata owner did not retire");
}

#[test]
fn mesh3d_original_component_admission_marks_duplicate_and_invalid_u64_groups_ineligible() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎯️component-source/🔣️.json")).unwrap();
    let law = &fixture["admission"];
    let mut original:ComponentReferenceTable=reference_fixture(law["references"].clone());
    let lease = component_admission_test_publish(&mut original).unwrap();
    let mut answers = Vec::new();
    for row in law["cases"].as_array().unwrap() {
        let kind = row["kind"].as_str().unwrap();
        let labels: Vec<String> = serde_json::from_value(law["references"][kind].clone()).unwrap();
        let label = row["label"].as_str().unwrap();
        let oracle = serde_json::from_str::<u64>(label).ok().filter(|value| *value > 0 && value.to_string() == label);
        let unique = oracle.filter(|value| labels.iter().filter(|text| serde_json::from_str::<u64>(text).ok() == Some(*value)).count() == 1);
        assert_eq!(unique.is_some(), row["eligible"].as_bool().unwrap());
        let field = match kind { "face" => Mesh3dField::FaceIds, "edge" => Mesh3dField::EdgeIds, "vertex" => Mesh3dField::VertexIds, _ => panic!("unknown neutral component kind") };
        answers.push((mesh_test_attempt(||lease.component_label(field, row["group"].as_u64().unwrap() as u32)), unique));
    }
    close_component_admission_test_mesh(lease);
    for (actual, expected) in answers { assert_eq!(actual, expected.map_or(Err(Mesh3dFault::Schema), |label| Ok(Some(label)))); }
}

#[test]
fn mesh3d_original_component_admission_refuses_oversized_owned_source_and_retires_it() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎯️component-source/🔣️.json")).unwrap();
    let bytes = fixture["admission"]["oversizedSource"]["labelBytes"].as_u64().unwrap() as usize;
    let mut original = ComponentReferenceTable::from_entries(vec![("face".into(), vec!["7".repeat(bytes)])]);
    assert!(semio_framework_pack_json::from_dsl_value(&protocol::value::ToValue::to_value(&original)).to_string().len() > MESH3D_OWNER_BYTE_CAPACITY);
    let answer = component_admission_test_publish(&mut original);
    if let Ok(lease) = answer { close_component_admission_test_mesh(lease); }
    assert_eq!(answer, Err(Mesh3dFault::ByteCapacity));
    assert!(original.is_empty());
}

#[test]
fn mesh3d_original_zero_index_wire_and_point_domains_preserve_independent_buffers() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎯️component-source/🔣️.json")).unwrap();
    for row in fixture["zeroIndex"]["accepted"].as_array().unwrap() {
        let positions: Vec<[f32; 3]> = serde_json::from_value(row["positions"].clone()).unwrap();
        let vertex_ids: Vec<u32> = serde_json::from_value(row["vertexIds"].clone()).unwrap();
        let edges: Vec<[[f32; 3]; 2]> = serde_json::from_value(row["edges"].clone()).unwrap();
        let edge_ids: Vec<u32> = serde_json::from_value(row["edgeIds"].clone()).unwrap();
        let schema = Mesh3dSchema { vertices: positions.len() as u32, vertex_ids: vertex_ids.len() as u32, edges: edges.len() as u32, edge_ids: edge_ids.len() as u32, ..Mesh3dSchema::triangle_mesh(positions.len() as u32, 0) };
        let token = admit_mesh(80, 91, schema).expect("original nonempty typed zero-index domain");
        while !allocate_mesh(token).unwrap() {}
        for point in &positions { mesh_test_attempt(||mesh3d_write_vec3(token, Mesh3dField::Positions, *point)).unwrap(); mesh_test_attempt(||mesh3d_write_vec3(token, Mesh3dField::Normals, [0.0, 0.0, 1.0])).unwrap(); }
        for id in &vertex_ids { mesh_test_attempt(||mesh3d_write_u32(token, Mesh3dField::VertexIds, *id)).unwrap(); }
        for edge in &edges { mesh_test_attempt(||mesh3d_write_edge(token, *edge)).unwrap(); }
        for id in &edge_ids { mesh_test_attempt(||mesh3d_write_u32(token, Mesh3dField::EdgeIds, *id)).unwrap(); }
        let lease = mesh_test_attempt(||mesh3d_seal(token)).unwrap();
        assert_eq!(mesh_test_attempt(||lease.schema()).unwrap(), schema);
        let bounds: [[f32; 3]; 2] = serde_json::from_value(row["bounds"].clone()).unwrap();
        assert_eq!(mesh_test_attempt(||lease.aabb()).unwrap(), (bounds[0], bounds[1]));
        for (index, point) in positions.iter().enumerate() { assert_eq!(mesh_test_attempt(||lease.vec3(Mesh3dField::Positions, index as u32)).unwrap(), *point); if !vertex_ids.is_empty() { assert_eq!(mesh_test_attempt(||lease.u32(Mesh3dField::VertexIds, index as u32)).unwrap(), vertex_ids[index]); } }
        if vertex_ids.is_empty() { assert_eq!(mesh_test_attempt(||lease.u32(Mesh3dField::VertexIds, 0)), Err(Mesh3dFault::Schema)); }
        for (index, edge) in edges.iter().enumerate() { assert_eq!(mesh_test_attempt(||lease.edge(index as u32)).unwrap(), *edge); assert_eq!(mesh_test_attempt(||lease.u32(Mesh3dField::EdgeIds, index as u32)).unwrap(), edge_ids[index]); }
        close_component_admission_test_mesh(lease);
    }
    for row in fixture["zeroIndex"]["refused"].as_array().unwrap() {
        let schema = Mesh3dSchema { vertices: row["vertices"].as_u64().unwrap() as u32, indices: row["indices"].as_u64().unwrap() as u32, vertex_ids: row["vertexIds"].as_u64().unwrap() as u32, edges: row["edges"].as_u64().unwrap() as u32, edge_ids: row["edgeIds"].as_u64().unwrap() as u32, ..Mesh3dSchema::triangle_mesh(0, 0) };
        assert!(matches!(admit_mesh(80, 91, schema), Err(Mesh3dFault::Schema)));
    }
}

#[test]
fn mesh3d_original_component_admission_reserves_aggregate_authority_metadata_credit() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎯️component-source/🔣️.json")).unwrap();
    let law = &fixture["admission"]["authorityBudget"];
    let page_bytes = law["pageBytes"].as_u64().unwrap() as usize;
    assert_eq!(page_bytes, MESH3D_PAGE_BYTES);
    assert_eq!(law["ceilingPages"].as_u64().unwrap() as usize, MESH3D_AUTHORITY_PAGE_CAPACITY);
    let pages = law["geometryPages"].as_u64().unwrap() as usize;
    let schema = Mesh3dSchema::triangle_mesh(((pages * page_bytes - 12) / 24) as u32, 3);
    let mut authority = Mesh3dAuthority::new();
    let mut tokens = Vec::new();
    for _ in 0..law["filledOwners"].as_u64().unwrap() { let token = authority.begin(82,93,schema,ownership_test_grant(0,mesh3d_begin_capacity_byte_demand(),0,1)).unwrap().0; assert_eq!(usize::from(authority.writing_ref(token).unwrap().layout.page_count), pages); tokens.push(token); }
    let token = component_admission_local_writer(&mut authority);
    let mut original = ComponentReferenceTable::from_entries(vec![("face".into(), vec!["7".repeat(law["labelBytes"].as_u64().unwrap() as usize)])]);
    authority_move(&mut authority,token,&mut original).unwrap();
    let available = (MESH3D_AUTHORITY_PAGE_CAPACITY - authority.reserved_pages) * page_bytes;
    let owned = authority.writing_ref(token).unwrap().component_references.as_ref().unwrap();
    let independent_bytes = semio_framework_pack_json::from_dsl_value(&protocol::value::ToValue::to_value(owned)).to_string().len();
    assert!(independent_bytes > available && independent_bytes < MESH3D_OWNER_BYTE_CAPACITY);
    assert_eq!(authority_admit(&mut authority,token, 1, page_bytes), Err(Mesh3dFault::PageCapacity));
    assert_eq!(authority.seal(token), Err(Mesh3dFault::PageCapacity));
    tokens.push(token);
    for token in tokens { authority.begin_close_write(token).unwrap(); while !authority_close(&mut authority,token.slot, token.epoch).unwrap() {} }
    assert!(original.is_empty());
    assert_eq!(authority.reserved_pages, 0);
}

#[test]
fn mesh3d_original_component_admission_refuses_sentinel_only_point_at_complete_seal() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎯️component-source/🔣️.json")).unwrap();
    let sentinel = fixture["zeroIndex"]["vertexSentinel"].as_u64().unwrap() as u32;
    let mut answers = Vec::new();
    for row in fixture["zeroIndex"]["sentinelCases"].as_array().unwrap() {
        let positions: Vec<[f32; 3]> = serde_json::from_value(row["positions"].clone()).unwrap();
        let ids: Vec<u32> = serde_json::from_value(row["vertexIds"].clone()).unwrap();
        let indices: Vec<u32> = serde_json::from_value(row["indices"].clone()).unwrap();
        let edges: Vec<[[f32; 3]; 2]> = serde_json::from_value(row["edges"].clone()).unwrap();
        let independent = !indices.is_empty() || !edges.is_empty() || ids.iter().any(|id| *id != sentinel);
        assert_eq!(independent, row["eligible"].as_bool().unwrap());
        let schema = Mesh3dSchema { vertex_ids: ids.len() as u32, edges: edges.len() as u32, ..Mesh3dSchema::triangle_mesh(positions.len() as u32, indices.len() as u32) };
        let token = admit_mesh(83, 94, schema).unwrap();
        while !allocate_mesh(token).unwrap() {}
        assert_eq!(mesh_test_attempt(||mesh3d_seal(token)), Err(Mesh3dFault::Incomplete));
        for point in positions { mesh_test_attempt(||mesh3d_write_vec3(token, Mesh3dField::Positions, point)).unwrap(); mesh_test_attempt(||mesh3d_write_vec3(token, Mesh3dField::Normals, [0.0, 0.0, 1.0])).unwrap(); }
        for id in &ids { mesh_test_attempt(||mesh3d_write_u32(token, Mesh3dField::VertexIds, *id)).unwrap(); }
        for index in indices { mesh_test_attempt(||mesh3d_write_u32(token, Mesh3dField::Indices, index)).unwrap(); }
        for edge in edges { mesh_test_attempt(||mesh3d_write_edge(token, edge)).unwrap(); }
        let result = mesh_test_attempt(||mesh3d_seal(token));
        if let Ok(lease) = result { for (group, id) in ids.iter().enumerate() { assert_eq!(mesh_test_attempt(||lease.u32(Mesh3dField::VertexIds, group as u32)).unwrap(), *id); } close_component_admission_test_mesh(lease); }
        else { mesh_test_attempt(||mesh3d_abort(token)).unwrap(); while !abort_mesh(token).unwrap() {} }
        answers.push((result.map(|_| ()), independent));
    }
    for (actual, eligible) in answers { assert_eq!(actual, if eligible { Ok(()) } else { Err(Mesh3dFault::Schema) }); }
}

fn component_admission_local_writer(authority: &mut Mesh3dAuthority) -> Mesh3dWriteToken {
    let token = authority.begin(81,92,Mesh3dSchema::triangle_mesh(3,3),ownership_test_grant(0,mesh3d_begin_capacity_byte_demand(),0,1)).unwrap().0;
    let owner = authority.writing(token).unwrap();
    while !owner.allocate_step(ownership_test_grant(0,MESH3D_PAGE_BYTES,0,1)).complete {}
    for _ in 0..3 { owner.write(Mesh3dField::Positions, &[0; 12]).unwrap(); owner.write(Mesh3dField::Normals, &[0; 12]).unwrap(); }
    for index in 0u32..3 { owner.write(Mesh3dField::Indices, &index.to_le_bytes()).unwrap(); }
    token
}

#[test]
fn mesh3d_original_component_admission_freezes_grants_and_cancels_its_owned_index() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎯️component-source/🔣️.json")).unwrap();
    let law = &fixture["admission"];
    let mut authority = Mesh3dAuthority::new();
    let token = component_admission_local_writer(&mut authority);
    let mut original:ComponentReferenceTable=reference_fixture(law["references"].clone());
    let pointer = original["face"].as_ptr();
    authority_move(&mut authority,token,&mut original).unwrap();
    assert!(original.is_empty());
    assert_eq!(authority.seal(token), Err(Mesh3dFault::Incomplete));
    let credit = authority.writing_ref(token).unwrap().component_credit;
    for grant in [&law["zeroGrant"], &law["insufficientGrant"]] {
        let step = authority_admit(&mut authority,token, grant["items"].as_u64().unwrap() as usize, grant["bytes"].as_u64().unwrap() as usize).unwrap();
        let oracle = serde_json::json!({"complete":step.complete,"processedItems":step.processed_items,"processedBytes":step.processed_bytes,"retiredItems":step.retired_items,"retiredBytes":step.retired_bytes});
        for key in ["complete", "processedItems", "processedBytes", "retiredItems", "retiredBytes"] { assert_eq!(oracle[key], grant[key]); }
        let owner = authority.writing_ref(token).unwrap();
        assert_eq!((owner.component_domain, owner.component_group, owner.component_credit, owner.component_seen.len(), owner.component_ineligible.len()), (0, 0, credit, 0, 0));
        assert_eq!(owner.component_references.as_ref().unwrap()["face"].as_ptr(), pointer);
    }
    for _ in 0..law["cancelAfterItems"].as_u64().unwrap() {
        let step = authority_admit(&mut authority,token, 1, law["grant"]["bytes"].as_u64().unwrap() as usize).unwrap();
        assert_eq!(step.processed_items, 1);
    }
    let owner = authority.writing_ref(token).unwrap();
    assert_eq!(owner.component_seen.len(), 1);
    assert_eq!(owner.component_ineligible.len(), 2);
    assert!(owner.component_credit > credit);
    assert!(authority.reserved_pages > 1);
    authority.begin_close_write(token).unwrap();
    for _ in 0..law["maximumTurns"].as_u64().unwrap() { if authority_close(&mut authority,token.slot, token.epoch).unwrap() { assert_eq!(authority.reserved_pages, 0); assert!(authority.slots[usize::from(token.slot)].is_none());  return; } }
    panic!("original cancelled admission failed to retire");
}

#[test]
fn mesh3d_original_component_admission_retires_typed_scratch_before_publication() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎯️component-source/🔣️.json")).unwrap();
    let law = &fixture["admission"];
    let mut original:ComponentReferenceTable=reference_fixture(law["references"].clone());
    let expected = original.iter().map(|(_,labels)| labels.iter().filter_map(|label| serde_json::from_str::<u64>(label).ok().filter(|value| *value > 0 && value.to_string() == *label)).collect::<BTreeSet<_>>().len()).sum::<usize>();
    let mut authority = Mesh3dAuthority::new();
    let token = component_admission_local_writer(&mut authority);
    authority_move(&mut authority,token,&mut original).unwrap();
    let mut retired = (0, 0);
    let mut completed = false;
    for _ in 0..law["maximumTurns"].as_u64().unwrap() {
        let step = authority_admit(&mut authority,token, 1, law["grant"]["bytes"].as_u64().unwrap() as usize).unwrap();
        assert!(step.processed_items + step.retired_items <= 1);
        assert!(step.processed_bytes + step.retired_bytes <= law["grant"]["bytes"].as_u64().unwrap() as usize);
        retired.0 += step.retired_items; retired.1 += step.retired_bytes;
        if step.complete { completed = true; break; }
    }
    assert!(completed);
    assert_eq!(retired, (expected, expected * law["scratchEntryBytes"].as_u64().unwrap() as usize));
    assert!(authority.writing_ref(token).unwrap().component_seen.is_empty());
    let lease = authority.seal(token).unwrap();
    assert!(authority.ready(lease).unwrap().component_seen.is_empty());
    authority.begin_close(lease).unwrap();
    while !authority_close(&mut authority,lease.slot, lease.epoch).unwrap() {}
    assert_eq!(authority.reserved_pages, 0);
}

#[test]
fn paged_mesh_authority_preserves_order_aba_and_interrupted_close() {
    let mut authority = Mesh3dAuthority::new();
    let schema = Mesh3dSchema::triangle_mesh(3, 3);
    let token = authority_begin(&mut authority,7,11,schema).expect("fixed mesh claim");
    assert!(authority.writing(token).unwrap().allocate_step(ownership_test_grant(0,MESH3D_PAGE_BYTES,0,1)).complete);
    for position in [[0.0f32, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]] {
        let mut bytes = [0; 12];
        for (index, value) in position.into_iter().enumerate() {
            bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
        }
        authority.writing(token).unwrap().write(Mesh3dField::Positions, &bytes).unwrap();
    }
    for _ in 0..3 {
        let mut bytes = [0; 12];
        bytes[8..].copy_from_slice(&1.0f32.to_le_bytes());
        authority.writing(token).unwrap().write(Mesh3dField::Normals, &bytes).unwrap();
    }
    for index in [0u32, 1, 2] {
        authority.writing(token).unwrap().write(Mesh3dField::Indices, &index.to_le_bytes()).unwrap();
    }
    let lease = authority.seal(token).expect("terminal mesh publication");
    let first = authority.ready(lease).unwrap().item_bytes::<12>(Mesh3dField::Positions, 0).unwrap();
    assert_eq!(f32::from_le_bytes(first[..4].try_into().unwrap()), 0.0);
    authority.begin_close(lease).unwrap();
    assert!(!authority_close(&mut authority,lease.slot, lease.epoch).unwrap());
    assert!(authority_close(&mut authority,lease.slot, lease.epoch).unwrap());
    let replacement = authority_begin(&mut authority,8,12,schema).expect("reused fixed slot");
    assert_eq!(replacement.slot, token.slot);
    assert_ne!(replacement.epoch, token.epoch);
    assert!(matches!(authority.ready(lease), Err(Mesh3dFault::Stale)));
    authority.begin_close_write(replacement).unwrap();
    assert!(authority_close(&mut authority,replacement.slot, replacement.epoch).unwrap());
}

#[test]
fn paged_mesh_authority_rejects_aggregate_page_plus_one_before_allocation() {
    let mut authority = Mesh3dAuthority::new();
    let vertices = ((MESH3D_OWNER_BYTE_CAPACITY / 28) / 3 * 3) as u32;
    let indices = vertices;
    let schema = Mesh3dSchema::triangle_mesh(vertices, indices);
    let mut tokens = Vec::new();
    for generation in 1..=MESH3D_AUTHORITY_PAGE_CAPACITY / MESH3D_OWNER_PAGE_CAPACITY {
        tokens.push(authority_begin(&mut authority,generation as u64,1,schema).expect("aggregate admitted mesh"));
    }
    assert_eq!(authority_begin(&mut authority,99,1,Mesh3dSchema::triangle_mesh(3,3)).unwrap_err(), Mesh3dFault::PageCapacity);
    for token in tokens {
        authority.begin_close_write(token).unwrap();
        assert!(authority_close(&mut authority,token.slot, token.epoch).unwrap());
    }
    assert_eq!(authority.reserved_pages, 0);
}

#[test]
fn mesh_schema_has_vertex_colors_only_for_a_complete_vertex_field() {
    let without = test_box_mesh().schema().unwrap();
    assert_eq!(without.colors, 0);
    let mut data = LegacyMeshOracleData::triangle();
    data.colors = vec![[1.0; 4]; data.positions.len()];
    let with = paged_mesh_fixture(data).schema().unwrap();
    assert_eq!(with.colors, with.vertices);
}

#[test]
fn instance_model_from_trs_translates_and_scales_point() {
    let model = Instance3d::model_from_trs([1.0, 2.0, 3.0], [0.0, 0.0, 0.0, 1.0], [2.0, 2.0, 2.0]);
    let point = model.transform_point_m(vec3_new_m(1.0, 0.0, 0.0));
    assert!((point.x - 3.0).abs() < 1e-5, "x={}", point.x);
    assert!((point.y - 2.0).abs() < 1e-5, "y={}", point.y);
    assert!((point.z - 3.0).abs() < 1e-5, "z={}", point.z);
}
#[global_allocator]
static OBSERVED_ALLOCATOR:ObservedAllocator=ObservedAllocator;

#[test]
fn mesh3d_owner_contention_is_an_explicit_temporary_refusal(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../📐️math/🚦️ownership/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(fixture["ownerBusy"],"Busy");assert_eq!(fixture["semanticClosing"],"Closing");
    let authority=mesh_test_attempt(||mesh3d_authority().try_lock().map_err(|_|Mesh3dFault::Busy)).expect("bounded test owner admitted");
    let grant=ownership_test_grant(0,fixture["retry"]["constructorCapacityBytes"].as_u64().unwrap() as usize,0,1);
    let (result,heap)=observe_retirement_allocations(||mesh3d_begin(71,83,Mesh3dSchema::triangle_mesh(3,3),grant));
    assert_eq!(result,Err(Mesh3dFault::Busy));assert_eq!(heap,(0,0));
    drop(authority);
}
#[test]
fn mesh3d_original_busy_caller_has_finite_attempt_deadline_and_cancellation(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../📐️math/🚦️ownership/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(fixture["retry"]["maximumAttempts"],4096);assert_eq!(fixture["retry"]["maximumDurationMs"],5000);
    let mut calls=0;
    let cancel=std::sync::atomic::AtomicBool::new(false);
    let deadline=std::time::Instant::now()+std::time::Duration::from_millis(5000);
    let result=mesh_test_attempt_controlled(3,deadline,&cancel,||{calls+=1;Err::<(),_>(Mesh3dFault::Busy)});
    assert_eq!(result,Err(Mesh3dFault::Busy));assert_eq!(calls,3);
    for (attempts,deadline,canceled) in [(0,deadline,false),(3,std::time::Instant::now(),false),(3,deadline,true)]{
        calls=0;let cancel=std::sync::atomic::AtomicBool::new(canceled);
        assert_eq!(mesh_test_attempt_controlled(attempts,deadline,&cancel,||{calls+=1;Ok(())}),Err(Mesh3dFault::Busy));assert_eq!(calls,0);
    }
    calls=0;
    assert_eq!(mesh_test_attempt_controlled(3,deadline,&cancel,||{calls+=1;Err::<(),_>(Mesh3dFault::Closing)}),Err(Mesh3dFault::Closing));assert_eq!(calls,1);
}
