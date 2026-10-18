/// 🧰️ Fixture session, operators and registry host: the session grant and fixture drain helpers, the preview phase projection law, the fixture operator and dispatch observer, the registry host and the math bridge.

const FIXTURE_SESSION_GRANT:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096};
fn prepare_fixture_window(session:&mut FlowEvalSession){
    for _ in 0..1000{let step=session.prepare_window_tick_latch("preview-1",FIXTURE_SESSION_GRANT).unwrap();if matches!(step,RetainedCloneStep::Complete(_)){return}}
    panic!("original fixture window preparation stalled");
}
fn drain_fixture_cancellation(session:&mut FlowEvalSession){
    for _ in 0..1_000_000{if session.preview_cancellation_terminal_is_empty(){return}session.preview_cancellation_step(FIXTURE_SESSION_GRANT).unwrap();}
    panic!("original fixture cancellation stalled");
}
fn capture_fixture_baseline(session:&mut FlowEvalSession,host:&FlowHost){
    session.capture_baseline_from(host,FIXTURE_SESSION_GRANT).unwrap();
    if !session.baseline_capture_terminal_is_empty(){
        assert!(session.geometry_port.is_none());session.begin_retain_preview_meshes(Vec::new()).unwrap();
        for _ in 0..1_000_000{if session.preview_retention_terminal_is_empty(){return}session.retain_preview_meshes_step(FIXTURE_SESSION_GRANT).unwrap();}
        panic!("original empty fixture membership stalled");
    }
}

const NUMBER_OPS: &[&str] = &["core.number"];


#[test]
fn retained_preview_phase_projection_preserves_original_status_and_cancellation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⏱️cold-preview/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let phase = row["phase"].as_str().unwrap();
        let envelope = serde_json::to_string(row).unwrap();
        let independent: serde_json::Value = serde_json::from_str(&envelope).unwrap();
        let mut session = FlowEvalSession::new();
    prepare_fixture_window(&mut session);
        assert!(session.note_pending_tessellate(7, "cold-preview".into()));
        session.tessellate_progress_by_hash.insert(7,PreviewTessellateProgress{phase:PreviewTessellatePhase::from_tag(phase),units_done:independent["unitsDone"].as_u64().unwrap(),units_total:independent["unitsTotal"].as_u64().unwrap(),..Default::default()});
        let status = session.preview_tessellate_status();
        let labels = status.phase.labels();
        session.retire_cold();
        assert_eq!(status.phase.tag(), independent["phase"].as_str().unwrap(), "{phase}");
        assert_eq!(status.phase.is_cancellable(), independent["cancellable"].as_bool().unwrap(), "{phase}");
        assert_eq!(status.units_done as u64, independent["unitsDone"].as_u64().unwrap());
        assert_eq!(status.units_total as u64, independent["unitsTotal"].as_u64().unwrap());
        assert!(!labels.0.is_empty() && !labels.1.is_empty());
    }
}

#[test]
fn tree_from_dag_builds_neurons_and_synapses() {
    let nodes = vec![WireNode { id: "a".into(), kind: "core.number".into(), port: None, properties: PropertyBag::new() }, WireNode { id: "b".into(), kind: "math.add".into(), port: None, properties: PropertyBag::new() }];
    let edges = vec![WireEdge { from: "a".into(), from_port: "number".into(), to: "b".into(), to_port: "a".into(), directed: true, properties: PropertyBag::new() }];
    let tree = FlowHost::tree_from_dag(&nodes, &edges);
    assert_eq!(tree.neurons.len(), 2);
    assert_eq!(tree.synapses.len(), 1);
}

#[derive(Clone,Copy)]
enum FixtureOperatorFamily { Math, Merge, Extension }
struct FixtureOperator {kind:&'static str,family:FixtureOperatorFamily}
thread_local! {static FIXTURE_DISPATCH_OBSERVER:std::cell::RefCell<(Option<std::sync::Arc<std::sync::atomic::AtomicUsize>>,Option<std::sync::Arc<Mutex<Vec<String>>>>)>=const {std::cell::RefCell::new((None,None))};}
struct FixtureDispatchObservation;
impl FixtureDispatchObservation {
    fn counter(counter:std::sync::Arc<std::sync::atomic::AtomicUsize>)->Self {FIXTURE_DISPATCH_OBSERVER.with(|observer|observer.borrow_mut().0=Some(counter));Self}
    fn calls(calls:std::sync::Arc<Mutex<Vec<String>>>)->Self {FIXTURE_DISPATCH_OBSERVER.with(|observer|observer.borrow_mut().1=Some(calls));Self}
}
impl Drop for FixtureDispatchObservation {fn drop(&mut self){FIXTURE_DISPATCH_OBSERVER.with(|observer|*observer.borrow_mut()=(None,None));}}
impl neural::Operator for FixtureOperator {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
    fn evaluate(&self,input:&Dictionary)->Result<Dictionary,EvalError> {
        FIXTURE_DISPATCH_OBSERVER.with(|observer|{let observer=observer.borrow();if let Some(counter)=&observer.0{counter.fetch_add(1,std::sync::atomic::Ordering::Relaxed);}if let Some(calls)=&observer.1{calls.lock().unwrap().push(self.kind.into());}});
        match self.family {FixtureOperatorFamily::Math=>test_math_bridge(self.kind,input),FixtureOperatorFamily::Merge=>test_dictionary_merge_bridge(self.kind,input),FixtureOperatorFamily::Extension=>test_extension_bridge(self.kind,input)}
    }
}
struct FixtureRegistryHost {host:Option<FlowHost>,retirement:neural::RegistryRetirement,observation:Option<FixtureDispatchObservation>}
impl FixtureRegistryHost {
    fn new(host:FlowHost,family:FixtureOperatorFamily)->Self {
        let mut registry=neural::Registry::new();
        for kind in ["core.number","core.text","core.image","core.variable","math.add","math.passThrough","dictionary.merge","plugin.left","plugin.right"] {
            registry.register_operator(NeuronKindInfo{id:kind.into(),..Default::default()},vec![neural::OperatorImpl{schemas:vec![],operator:Box::new(FixtureOperator{kind,family})}],&[]);
        }
        let (registry,retirement)=neural::SharedRegistry::new(registry);
        let infos=host.kind_infos.clone();
        let mut host=host.with_operator_registry(registry);host.set_neuron_kind_info_map(infos);
        Self{host:Some(host),retirement,observation:None}
    }
    fn retire_cold(mut self){self.close();}
    fn close(&mut self){
        if let Some(host)=self.host.take(){host.retire_cold();}
        for _ in 0..1_000_000 {
            if self.retirement.terminal_is_empty(){return;}
            let copy=64;let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:self.retirement.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:self.retirement.next_release_byte_demand().unwrap(),maximum_depth:self.retirement.next_depth_demand().unwrap()};
            self.retirement.close_step(grant).unwrap();
        }
        panic!("original fixture registry did not close");
    }
}
impl std::ops::Deref for FixtureRegistryHost {type Target=FlowHost;fn deref(&self)->&FlowHost{self.host.as_ref().unwrap()}}
impl std::ops::DerefMut for FixtureRegistryHost {fn deref_mut(&mut self)->&mut FlowHost{self.host.as_mut().unwrap()}}
impl Drop for FixtureRegistryHost {fn drop(&mut self){self.close();}}

fn test_math_bridge(kind: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
    if kind == "core.number" {
        let value = input.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0);
        return Ok(channel_output("number", Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(value)))));
    }
    if kind == "core.text" {
        let value = input.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap_or_default();
        return Ok(channel_output("text", Dictionary::with_schema("text").insert("value", NeuralValue::Atom(Atom::String(value.into())))));
    }
    if kind == "core.image" {
        let value = input.get("dataUrl").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap_or_default();
        return Ok(channel_output("image", Dictionary::with_schema("image").insert("dataUrl", NeuralValue::Atom(Atom::String(value.into())))));
    }
    if kind == "math.add" {
        let a = input.get("a").or_else(|| input.get("number")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput("a".into()))?;
        let b = input.get("b").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0);
        return Ok(channel_output("sum", Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(a + b)))));
    }
    if kind == "math.passThrough" {
        let n = input.get("number").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput("number".into()))?;
        return Ok(channel_output("number", Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(n)))));
    }
    if kind == "core.variable" {
        let name = input.get("name").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).ok_or_else(|| EvalError::MissingInput("name".into()))?;
        let payload = input.get(name).and_then(|v| v.as_dictionary()).cloned().ok_or_else(|| EvalError::MissingInput(name.into()))?;
        return Ok(channel_output(name, payload));
    }
    Err(EvalError::UnknownKind(kind.into()))
}
//#region 🔌️PortSides
//#endregion 🔌️PortSides
