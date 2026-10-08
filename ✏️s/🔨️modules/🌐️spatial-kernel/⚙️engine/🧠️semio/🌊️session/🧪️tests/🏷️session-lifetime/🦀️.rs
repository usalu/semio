//! 🌐️ Explicit geometry instances, authority isolation and neutral host lifecycle laws.
use semio_s_spatial_kernel_semio_session::{Session};
use semio_framework_os_flow::{geometry::{GeometryPort, GeometryStep,GeometryPortRetirement}, FlowEvalSession};
use serde_json::{json,Value};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
fn transition_grant()->RetainedCloneGrant {RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1}}
fn session_grant(source:&Session)->RetainedCloneGrant {let copy=source.next_close_copy_byte_demand().unwrap();RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:source.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:source.next_close_release_byte_demand().unwrap(),maximum_depth:source.next_close_depth_demand().unwrap()}}
fn capture_grant(source:&semio_s_spatial_kernel_semio_session::SessionCapture)->RetainedCloneGrant {let copy=source.next_close_copy_byte_demand().unwrap();RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:source.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:source.next_close_release_byte_demand().unwrap(),maximum_depth:source.next_close_depth_demand().unwrap()}}
fn retire_port(port:Box<dyn GeometryPort>) { GeometryPortRetirement::new(port).retire_cold(); }
fn fixture() -> Value { serde_json::from_str(include_str!("../../🧫️fixtures/🏷️session-lifetime/🔣️.json")).unwrap() }
fn invoke(session: &Session, method: &str, args: Value) -> Value {
    let result: Value = serde_json::from_str(&session.brep_invoke_json(method, &args.to_string())).unwrap();
    assert!(result.get("error").is_none(), "{method}: {result}");
    result
}
#[test]
fn independent_geometry_instances_keep_their_own_kernel_and_match_parry_volume() {
    let fixture = fixture();
    let sessions = [Session::new(), Session::new()];
    for (session, row) in sessions.iter().zip(fixture["independentBoxes"].as_array().unwrap()) {
        let handle = invoke(session, "box", row.clone())["handle"].as_str().unwrap().to_owned();
        assert_eq!(invoke(session, "volume", json!({ "shape": handle }))["value"].as_f64(), row["volume"].as_f64());
        let mesh = session.tessellate_geometry(&handle, 0.05).unwrap();
        let vertices = mesh.positions.chunks_exact(3).map(|p| parry3d::na::Point3::new(p[0], p[1], p[2])).collect::<Vec<_>>();
        let triangles = mesh.indices.chunks_exact(3).map(|v| [v[0],v[1],v[2]]).collect::<Vec<_>>();
        let oracle = parry3d::mass_properties::MassProperties::from_trimesh(1.0, &vertices, &triangles);
        assert!((oracle.mass() as f64 - row["volume"].as_f64().unwrap()).abs() < 1e-5);
    }
    sessions[0].close();
    sessions[0].close();
    assert!(sessions[0].is_closed());
    assert!(serde_json::from_str::<Value>(&sessions[0].export_solid_json(&[], "step", 0.05)).unwrap().get("error").is_some());
    assert!(serde_json::from_str::<Value>(&sessions[0].import_solid_json("obj", "", 0.05)).unwrap().get("error").is_some());
    assert!(serde_json::from_str::<Value>(&sessions[0].brep_invoke_json("box", "{\"width\":1,\"depth\":1,\"height\":1}")).unwrap().get("error").is_some());
    let row = &fixture["independentBoxes"][1];
    let handle = invoke(&sessions[1], "box", row.clone())["handle"].as_str().unwrap().to_owned();
    assert_eq!(invoke(&sessions[1], "volume", json!({ "shape": handle }))["value"].as_f64(), row["volume"].as_f64());
    sessions[1].close();
}
#[test]
fn cancelling_one_authority_preserves_another_job_for_the_same_handle() {
    let source = Session::new();
    let fixture = fixture();
    let args = &fixture["sharedHandle"];
    let handle = invoke(&source, "sphere", args.clone())["handle"].as_str().unwrap().to_owned();
    let first = source.port();
    let second = source.port();
    let tolerance = args["tolerance"].as_f64().unwrap();
    assert!(matches!(first.tessellate_step(&handle, tolerance, 0), GeometryStep::Working { .. }));
    assert!(matches!(second.tessellate_step(&handle, tolerance, 0), GeometryStep::Working { .. }));
    assert_eq!(first.cancel(),1);
    assert_eq!(first.cancel(),0);
    assert_eq!(second.cancel(),1);
    let isolated = Session::new();
    let own_handle = invoke(&isolated, "sphere", args.clone())["handle"].as_str().unwrap().to_owned();
    assert!(matches!(isolated.tessellate_step(&own_handle, tolerance, 0), semio_s_spatial_kernel_semio_session::TessellationStepOutcome::Working { .. }));
    isolated.dispose_geometry(&own_handle).unwrap();
    assert_eq!(isolated.cancel_all_tessellations(), 0);
    isolated.close();
    first.begin_close();
    first.begin_close();
    assert!(matches!(first.tessellate_step(&handle, tolerance, 1), GeometryStep::Failed(_)));
    retire_port(first);
    retire_port(second);
    source.close();
}
#[test]
fn closing_one_authority_preserves_other_live_claim_and_refuses_foreign_disposal() {
    let boundary:Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();
    let boundary = &boundary["portBoundary"];
    let source = Session::new();
    let handle = invoke(&source, "box", json!({ "width":2,"depth":3,"height":4 }))["handle"].as_str().unwrap().to_owned();
    let mut unused = source.port();
    assert_eq!(unused.close_step(RetainedCloneGrant {maximum_items:0,..transition_grant()}).unwrap().progress(),RetainedCloneProgress::default());
    assert_eq!(unused.close_step(RetainedCloneGrant {maximum_depth:0,..transition_grant()}).unwrap().progress(),RetainedCloneProgress::default());
    assert_eq!(!source.is_closed(),boundary["producerRemainsOpen"].as_bool().unwrap());
    source.cancel_close();assert_eq!(unused.close_step(transition_grant()).unwrap().progress(),RetainedCloneProgress::default());source.resume_close();
    assert_eq!(unused.close_step(transition_grant()).unwrap().progress(),RetainedCloneProgress {copied_items:1,..Default::default()});
    retire_port(unused);
    assert_eq!(invoke(&source, "volume", json!({ "shape":handle }))["value"].as_f64(),boundary["producerVolume"].as_f64());
    let first = source.port();
    let second = source.port();
    first.retain(&[handle.clone()]);
    second.retain(&[handle.clone()]);
    assert!(first.dispose(&handle).is_err());
    assert!(source.dispose_geometry(&handle).is_err());
    assert!(serde_json::from_str::<Value>(&source.brep_invoke_json("dispose", &json!({ "handle":handle }).to_string())).unwrap().get("error").is_some());
    invoke(&source, "retain", json!({ "handles":[] }));
    assert_eq!(invoke(&source, "volume", json!({ "shape":handle }))["value"].as_f64(),boundary["producerVolume"].as_f64());
    retire_port(first);
    assert_eq!(invoke(&source, "volume", json!({ "shape":handle }))["value"].as_f64(),boundary["producerVolume"].as_f64());
    retire_port(second);
    invoke(&source,"retain",json!({"handles":[]}));
    assert!(serde_json::from_str::<Value>(&source.brep_invoke_json("volume", &json!({ "shape":handle }).to_string())).unwrap().get("error").is_some());
    source.close();
    let producer = Session::new();
    invoke(&producer,"box",json!({ "width":2,"depth":3,"height":4 }));
    let orphan = producer.port();
    producer.begin_close();
    drop(producer);
    let mut orphan = GeometryPortRetirement::new(orphan);
    let mut turns = 0;
    while !orphan.terminal_is_empty() {
        let copy=orphan.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:orphan.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:orphan.next_release_byte_demand().unwrap(),maximum_depth:orphan.next_depth_demand().unwrap()};
        assert!(orphan.close_step(grant).unwrap().progress().fits(grant));
        turns += 1;
        assert!(turns <= 100000);
    }
    assert!(turns > 3);
}
struct RecordingPort { calls:std::sync::Arc<std::sync::Mutex<Vec<&'static str>>>, sealed:std::sync::atomic::AtomicBool,terminal:bool,inline:[u8;8192] }
impl GeometryPort for RecordingPort {
    fn retain(&self, _: &[String]) { self.calls.lock().unwrap().push("retain"); }
    fn tessellate_step(&self, _: &str, _: f64, _: usize) -> GeometryStep { GeometryStep::Cancelled }
    fn dispose(&self, _: &str) -> Result<(), String> { Ok(()) }
    fn cancel(&self) -> usize { self.calls.lock().unwrap().push("cancel"); 0 }
    fn begin_close(&self) { if !self.sealed.swap(true,std::sync::atomic::Ordering::AcqRel) { self.calls.lock().unwrap().push("close"); } }
    fn terminal_is_empty(&self) -> bool { self.terminal }
    fn next_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError> {Ok(usize::from(!self.terminal))}
    fn next_capacity_byte_demand(&self,_copy:usize)->Result<usize,semio_framework_value::ValueError> {Ok(0)}
    fn next_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError> {Ok(0)}
    fn next_depth_demand(&self)->Result<usize,semio_framework_value::ValueError> {Ok(usize::from(!self.terminal))}
    fn close_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep as Step,RetainedCloneProgress};
        let empty=RetainedCloneProgress::default();
        if self.terminal {return Ok(Step::Complete(empty));}
        if grant.maximum_items==0||grant.maximum_copy_bytes<1||grant.maximum_depth<1 {return Ok(Step::Progress(empty));}
        self.terminal=true;self.calls.lock().unwrap().push("retire");
        Ok(Step::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:1,..empty}))
    }
}
impl Drop for RecordingPort { fn drop(&mut self) { self.calls.lock().unwrap().push("drop"); assert!(self.terminal); } }
#[test]
fn a_neutral_flow_session_closes_its_explicitly_supplied_geometry_authority() {
    let calls = std::sync::Arc::new(std::sync::Mutex::new(Vec::with_capacity(16)));
    let mut flow = FlowEvalSession::new().with_geometry_port(Box::new(RecordingPort { calls:calls.clone(),sealed:std::sync::atomic::AtomicBool::new(false),terminal:false,inline:[0;8192] }));
    flow.cancel_preview_evaluation("window");
    flow.begin_close();
    assert_eq!(&*calls.lock().unwrap(),&["cancel","close"]);
    assert_eq!(flow.close_step(0,3),semio_framework_job::InteractiveJobCloseStep::Blocked);
    assert_eq!(flow.close_step(1,0),semio_framework_job::InteractiveJobCloseStep::Blocked);
    assert_eq!(&*calls.lock().unwrap(),&["cancel","close"]);
    flow.retire_cold();
    assert_eq!(serde_json::to_value(&*calls.lock().unwrap()).unwrap(), fixture()["lifecycle"]);
    let calls = std::sync::Arc::new(std::sync::Mutex::new(Vec::with_capacity(16)));
    let mut retirement = GeometryPortRetirement::new(Box::new(RecordingPort { calls:calls.clone(),sealed:std::sync::atomic::AtomicBool::new(false),terminal:false,inline:[0;8192] }));
    let fixture:Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();
    let boundary = &fixture["portBoundary"];
    use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
    let grant=|row:&Value|RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap() as usize,maximum_copy_bytes:row["copyBytes"].as_u64().unwrap() as usize,maximum_capacity_bytes:row["capacityBytes"].as_u64().unwrap() as usize,maximum_release_bytes:row["releaseBytes"].as_u64().unwrap() as usize,maximum_depth:row["depth"].as_u64().unwrap() as usize};
    for row in boundary["zeroGrants"].as_array().unwrap() {assert_eq!(retirement.close_step(grant(row)).unwrap().progress(),RetainedCloneProgress::default());}
    assert_eq!(&*calls.lock().unwrap(),&["close"]);
    assert_eq!(retirement.close_step(grant(&boundary["childGrant"])).unwrap(),RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()}));
    let layout=retirement.next_release_byte_demand().unwrap();
    assert!(layout>=boundary["inlineBytes"].as_u64().unwrap() as usize);
    let release=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:layout/boundary["smallGrantDivisor"].as_u64().unwrap() as usize,maximum_depth:retirement.next_depth_demand().unwrap()};
    assert_eq!(retirement.close_step(release).unwrap().progress(),RetainedCloneProgress::default());
    assert_eq!(&*calls.lock().unwrap(),&["close","retire"]);
    assert!(!retirement.terminal_is_empty());
    assert_eq!(retirement.close_step(RetainedCloneGrant{maximum_release_bytes:layout,..release}).unwrap(),RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes:layout,..Default::default()}));
    assert!(retirement.terminal_is_empty());
    assert_eq!(&*calls.lock().unwrap(),&["close","retire","drop"]);
    let calls = std::sync::Arc::new(std::sync::Mutex::new(Vec::with_capacity(16)));
    let host = semio_framework_os_flow::FlowHost::default().with_geometry_port(Box::new(RecordingPort { calls:calls.clone(),sealed:std::sync::atomic::AtomicBool::new(false),terminal:false,inline:[0;8192] }));
    let mut host = semio_framework_os_flow::FlowHostRetirement::new(host);
    assert!(host.close_page(0,3).is_err());
    assert!(host.close_page(1,0).is_err());
    assert_eq!(&*calls.lock().unwrap(),&["close"]);
    while !host.close_page(1,usize::MAX).unwrap() {}
    assert!(host.terminal_nonopaque_is_empty());
    assert_eq!(&*calls.lock().unwrap(),&["close","retire","drop"]);
    let calls = std::sync::Arc::new(std::sync::Mutex::new(Vec::with_capacity(16)));
    semio_framework_os_flow::FlowHost::default().with_geometry_port(Box::new(RecordingPort { calls:calls.clone(),sealed:std::sync::atomic::AtomicBool::new(false),terminal:false,inline:[0;8192] })).retire_cold();
    assert_eq!(&*calls.lock().unwrap(),&["close","retire","drop"]);
}

#[test]
fn bounded_family_retirement_preserves_siblings_waits_readers_and_can_pause() {
    let fixture:Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();
    assert!(std::panic::catch_unwind(||drop(Session::new())).is_err());
    let shared=Session::new();let mut capture=shared.capture();
    assert_eq!(capture.close_step(RetainedCloneGrant {maximum_items:0,..transition_grant()}).unwrap().progress(),RetainedCloneProgress::default());
    assert_eq!(capture.close_step(transition_grant()).unwrap().progress(),RetainedCloneProgress {copied_items:1,..Default::default()});
    let layout=capture.next_close_release_byte_demand().unwrap();assert!(layout>8);
    assert_eq!(capture.close_step(RetainedCloneGrant {maximum_release_bytes:8,..transition_grant()}).unwrap().progress(),RetainedCloneProgress::default());
    assert_eq!(capture.close_step(RetainedCloneGrant {maximum_release_bytes:layout,..transition_grant()}).unwrap(),RetainedCloneStep::Complete(RetainedCloneProgress {copied_items:1,..Default::default()}));
    assert!(capture.terminal_is_empty());assert_eq!(!shared.is_closed(),fixture["capture"]["rootRemainsOpen"].as_bool().unwrap());
    let claimed=invoke(&shared,"box",fixture["box"].clone())["handle"].as_str().unwrap().to_owned();
    assert_eq!(invoke(&shared,"volume",json!({"shape":claimed}))["value"].as_f64(),Some(24.0));
    let mut last=shared.capture();drop(shared);
    assert_eq!(last.close_step(transition_grant()).unwrap().progress(),RetainedCloneProgress {copied_items:1,..Default::default()});
    let mut turns=0;let mut shell_releases=0;
    while !last.terminal_is_empty() {
        let grant=capture_grant(&last);
        if grant.maximum_release_bytes>8 {assert_eq!(last.close_step(RetainedCloneGrant {maximum_release_bytes:8,..grant}).unwrap().progress(),RetainedCloneProgress::default());assert!(!last.terminal_is_empty());}
        let native_terminal=last.native_terminal_is_empty();let step=last.close_step(grant).unwrap();assert!(step.progress().fits(grant));
        if native_terminal && step.progress().released_bytes!=0 {shell_releases+=1;}
        turns+=1;assert!(turns<1000000);
    }
    assert_eq!(shell_releases,2);assert!(turns>1);
    let terminal_source=Session::new();terminal_source.close();let terminal_peer=terminal_source.clone();let mut terminal_capture=terminal_source.capture();drop(terminal_source);
    terminal_capture.retire_cold();assert!(terminal_capture.terminal_is_empty());assert!(terminal_peer.terminal_is_empty());let mut final_peer=terminal_peer.capture();drop(terminal_peer);final_peer.retire_cold();assert!(final_peer.terminal_is_empty());
    let source=Session::new();let handle=invoke(&source,"box",fixture["box"].clone())["handle"].as_str().unwrap().to_owned();let mesh=source.tessellate_geometry(&handle,0.05).unwrap();
    let vertices=mesh.positions.chunks_exact(3).map(|p|parry3d::na::Point3::new(p[0],p[1],p[2])).collect::<Vec<_>>();let indices=mesh.indices.chunks_exact(3).map(|p|[p[0],p[1],p[2]]).collect::<Vec<_>>();assert_eq!(parry3d::mass_properties::MassProperties::from_trimesh(1.0,&vertices,&indices).mass() as f64,fixture["box"]["volume"].as_f64().unwrap());
    assert!(matches!(source.tessellate_step(&handle,0.01,0),semio_s_spatial_kernel_semio_session::TessellationStepOutcome::Working {..}));let progress=source.tessellation_progress(&handle,0.01);
    assert_eq!(source.close_step(RetainedCloneGrant {maximum_items:0,..transition_grant()}).unwrap().progress(),RetainedCloneProgress::default());assert_eq!(source.close_step(RetainedCloneGrant {maximum_depth:0,..transition_grant()}).unwrap().progress(),RetainedCloneProgress::default());assert!(!source.is_closed());assert_eq!(source.tessellation_progress(&handle,0.01),progress);
    let sibling=source.port();sibling.retain(&[handle.clone()]);
    let (entered_tx,entered_rx)=std::sync::mpsc::channel();let (release_tx,release_rx)=std::sync::mpsc::channel();let reader=source.clone();let reader_task=std::thread::spawn(move||reader.with_kernel_read(|_|{entered_tx.send(()).unwrap();release_rx.recv().unwrap();Ok(())}).unwrap());entered_rx.recv().unwrap();
    source.begin_close();source.cancel_close();assert_eq!(source.close_step(transition_grant()).unwrap().progress(),RetainedCloneProgress::default());assert!(!source.terminal_is_empty());assert!(matches!(sibling.tessellate_step(&handle,0.05,1),GeometryStep::Ready(_)));source.resume_close();
    while !source.close_step(session_grant(&source)).unwrap().progress().eq(&RetainedCloneProgress::default()) {}
    assert!(!source.terminal_is_empty());retire_port(sibling);assert!(!source.terminal_is_empty());
    assert!(source.close_step(session_grant(&source)).is_err());release_tx.send(()).unwrap();reader_task.join().unwrap();
    let mut turns=0;while !source.terminal_is_empty() {let grant=session_grant(&source);let step=source.close_step(grant).unwrap();assert!(step.progress().fits(grant));turns+=1;assert!(turns<1000000);}
    assert_eq!(source.close_step(RetainedCloneGrant {maximum_items:0,maximum_depth:0,..transition_grant()}).unwrap(),RetainedCloneStep::Complete(Default::default()));
    let sealed=source.port();assert!(matches!(sealed.tessellate_step(&handle,0.05,1),GeometryStep::Failed(_)));retire_port(sealed);
}
