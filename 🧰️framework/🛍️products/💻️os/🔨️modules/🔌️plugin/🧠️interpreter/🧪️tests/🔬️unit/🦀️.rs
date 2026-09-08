
use super::*;

const MEMORY_COPY_RANGES_JSON: &str = include_str!("../../🧪️fixtures/🔣️.json");

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct MemoryCopyFixture {
    version: u32,
    cases: Vec<MemoryCopyCase>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct MemoryCopyCase {
    name: String,
    destination: u32,
    source: u32,
    length: u32,
    grow_pages: u32,
    outcome: String,
    owned_error: Option<String>,
    expected_bytes: Option<Vec<u8>>,
}

fn module(sections: &[(u8, Vec<u8>)]) -> Vec<u8> {
    let mut bytes = b"\0asm\x01\0\0\0".to_vec();
    for (id, payload) in sections {
        bytes.push(*id);
        leb(payload.len() as u32, &mut bytes);
        bytes.extend(payload);
    }
    bytes
}

fn leb(mut value: u32, output: &mut Vec<u8>) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        output.push(byte);
        if value == 0 {
            return;
        }
    }
}

fn function_type(parameters: &[u8], results: &[u8]) -> Vec<u8> {
    let mut payload = vec![1, 0x60, parameters.len() as u8];
    payload.extend(parameters);
    payload.push(results.len() as u8);
    payload.extend(results);
    payload
}

fn one_function(body: &[u8]) -> Vec<u8> {
    let mut payload = vec![1];
    leb(body.len() as u32, &mut payload);
    payload.extend(body);
    payload
}

fn exported_function(name: &str, index: u8) -> Vec<u8> {
    let mut payload = vec![1, name.len() as u8];
    payload.extend(name.as_bytes());
    payload.extend([0, index]);
    payload
}

fn add_module() -> Vec<u8> {
    module(&[(1, function_type(&[0x7f, 0x7f], &[0x7f])), (3, vec![1, 0]), (7, exported_function("add", 0)), (10, one_function(&[0, 0x20, 0, 0x20, 1, 0x6a, 0x0b]))])
}

fn loop_module() -> Vec<u8> {
    let body = [1, 1, 0x7f, 0x41, 0, 0x21, 1, 0x02, 0x40, 0x03, 0x40, 0x20, 0, 0x45, 0x0d, 1, 0x20, 1, 0x20, 0, 0x6a, 0x21, 1, 0x20, 0, 0x41, 1, 0x6b, 0x21, 0, 0x0c, 0, 0x0b, 0x0b, 0x20, 1, 0x0b];
    module(&[(1, function_type(&[0x7f], &[0x7f])), (3, vec![1, 0]), (7, exported_function("sum", 0)), (10, one_function(&body))])
}

fn host_module() -> Vec<u8> {
    let mut imports = vec![1, 3];
    imports.extend(b"env");
    imports.push(6);
    imports.extend(b"double");
    imports.extend([0, 0]);
    module(&[(1, function_type(&[0x7f], &[0x7f])), (2, imports), (3, vec![1, 0]), (7, exported_function("call", 1)), (10, one_function(&[0, 0x20, 0, 0x10, 0, 0x0b]))])
}

fn memory_module() -> Vec<u8> {
    let mut exports = vec![2, 6];
    exports.extend(b"memory");
    exports.extend([2, 0, 5]);
    exports.extend(b"round");
    exports.extend([0, 0]);
    module(&[(1, function_type(&[0x7f], &[0x7f])), (3, vec![1, 0]), (5, vec![1, 1, 1, 2]), (7, exports), (10, one_function(&[0, 0x20, 0, 0x41, 0xfb, 0, 0x36, 2, 0, 0x20, 0, 0x28, 2, 0, 0x0b]))])
}

fn memory_copy_module() -> Vec<u8> {
    let mut exports = vec![2, 6];
    exports.extend(b"memory");
    exports.extend([2, 0, 4]);
    exports.extend(b"copy");
    exports.extend([0, 0]);
    module(&[(1, function_type(&[0x7f, 0x7f, 0x7f, 0x7f], &[0x7f])), (3, vec![1, 0]), (5, vec![1, 1, 1, 2]), (7, exports), (10, one_function(&[0, 0x20, 3, 0x40, 0, 0x1a, 0x20, 0, 0x20, 1, 0x20, 2, 0xfc, 10, 0, 0, 0x3f, 0, 0x0b]))])
}

fn passive_data_module() -> Vec<u8> {
    module(&[
        (1, function_type(&[], &[0x7f])),
        (3, vec![1, 0]),
        (5, vec![1, 0, 1]),
        (7, exported_function("load", 0)),
        (12, vec![1]),
        (10, one_function(&[0, 0x41, 0, 0x41, 0, 0x41, 4, 0xfc, 8, 0, 0, 0xfc, 9, 0, 0x41, 0, 0x28, 2, 0, 0x0b])),
        (11, vec![1, 1, 4, 1, 2, 3, 4]),
    ])
}

fn drive(instance: &mut CoreInstance, fuel: u64) -> Result<(Vec<Value>, u64), CoreError> {
    let mut total = 0;
    loop {
        match instance.step(fuel, StepControl::default()) {
            CoreStepOutcome::Yield { fuel_used } => total += fuel_used,
            CoreStepOutcome::Complete { fuel_used, values } => return Ok((values, total + fuel_used)),
            CoreStepOutcome::Fault { error, .. } => return Err(error),
            other => return Err(CoreError::State(format!("unexpected outcome {other:?}"))),
        }
    }
}

#[test]
fn add_executes_one_instruction_per_unit_of_fuel() {
    let module = Arc::new(CoreModule::parse(&add_module()).expect("parse add"));
    let mut instance = CoreInstance::instantiate(module).expect("instantiate add");
    instance.begin_export("add", vec![Value::I32(20), Value::I32(22)]).expect("begin add");
    let (values, fuel) = drive(&mut instance, 1).expect("drive add");
    assert_eq!(values, vec![Value::I32(42)]);
    assert_eq!(fuel, 4);
}

#[test]
fn loop_checkpoint_resumes_byte_identically_at_every_instruction() {
    let module = Arc::new(CoreModule::parse(&loop_module()).expect("parse loop"));
    let mut baseline = CoreInstance::instantiate(Arc::clone(&module)).expect("instantiate loop");
    baseline.begin_export("sum", vec![Value::I32(100)]).expect("begin loop");
    let mut checkpoints = Vec::new();
    loop {
        checkpoints.push(baseline.checkpoint());
        match baseline.step(1, StepControl::default()) {
            CoreStepOutcome::Yield { .. } => {}
            CoreStepOutcome::Complete { values, .. } => {
                assert_eq!(values, vec![Value::I32(5050)]);
                break;
            }
            other => panic!("unexpected loop outcome {other:?}"),
        }
    }
    for checkpoint in checkpoints {
        let mut resumed = CoreInstance::restore(Arc::clone(&module), &checkpoint).expect("restore loop");
        assert_eq!(drive(&mut resumed, 7).expect("finish resumed loop").0, vec![Value::I32(5050)]);
    }
}

#[test]
fn host_call_is_an_explicit_resumable_boundary() {
    let module = Arc::new(CoreModule::parse(&host_module()).expect("parse host module"));
    let mut instance = CoreInstance::instantiate(Arc::clone(&module)).expect("instantiate host module");
    instance.begin_export("call", vec![Value::I32(21)]).expect("begin host call");
    let call = loop {
        match instance.step(1, StepControl::default()) {
            CoreStepOutcome::Yield { .. } => {}
            CoreStepOutcome::HostCall { call, .. } => break call,
            other => panic!("unexpected pre-host outcome {other:?}"),
        }
    };
    assert_eq!((call.module.as_str(), call.name.as_str(), call.arguments.as_slice()), ("env", "double", [Value::I32(21)].as_slice()));
    let checkpoint = instance.checkpoint();
    let mut resumed = CoreInstance::restore(module, &checkpoint).expect("restore pending host call");
    assert_eq!(resumed.pending_host_call(), Some(&call));
    resumed.resume_host(call.id, Ok(vec![Value::I32(42)])).expect("resume host");
    assert_eq!(drive(&mut resumed, 1).expect("finish host call").0, vec![Value::I32(42)]);
}

#[test]
fn memory_access_and_growth_are_bounded() {
    let module = Arc::new(CoreModule::parse(&memory_module()).expect("parse memory module"));
    let mut instance = CoreInstance::instantiate(module).expect("instantiate memory module");
    instance.begin_export("round", vec![Value::I32(64)]).expect("begin roundtrip");
    assert_eq!(drive(&mut instance, 2).expect("drive roundtrip").0, vec![Value::I32(123)]);
    assert_eq!(&instance.memory(0).expect("memory")[64..68], &[123, 0, 0, 0]);
    assert_eq!(instance.grow_memory(0, 1), Some(1));
    assert_eq!(instance.grow_memory(0, 1), None);
}

#[test]
fn memory_copy_ranges_match_the_language_neutral_fixture_and_wasmtime() {
    let fixture: MemoryCopyFixture = serde_json::from_str(MEMORY_COPY_RANGES_JSON).expect("memory.copy fixture must match its language-neutral schema");
    assert_eq!(fixture.version, 1);
    let bytes = memory_copy_module();
    let module = Arc::new(CoreModule::parse(&bytes).expect("parse memory.copy module"));
    let engine = wasmtime::Engine::default();
    let oracle_module = wasmtime::Module::new(&engine, &bytes).expect("Wasmtime accepts memory.copy fixture module");
    let seed = [1, 2, 3, 4, 5, 6, 7, 8];
    for case in fixture.cases {
        let mut owned = CoreInstance::instantiate(Arc::clone(&module)).expect("instantiate owned memory.copy module");
        owned.memory_mut(0).expect("owned memory export")[..seed.len()].copy_from_slice(&seed);
        owned.begin_export("copy", vec![Value::I32(case.destination as i32), Value::I32(case.source as i32), Value::I32(case.length as i32), Value::I32(case.grow_pages as i32)]).expect("begin owned memory.copy");
        let owned_result = drive(&mut owned, 1);

        let mut oracle_store = wasmtime::Store::new(&engine, ());
        let oracle_instance = wasmtime::Instance::new(&mut oracle_store, &oracle_module, &[]).expect("instantiate Wasmtime memory.copy module");
        let oracle_memory = oracle_instance.get_memory(&mut oracle_store, "memory").expect("Wasmtime memory export");
        oracle_memory.write(&mut oracle_store, 0, &seed).expect("seed Wasmtime memory");
        let oracle_copy = oracle_instance.get_typed_func::<(i32, i32, i32, i32), i32>(&mut oracle_store, "copy").expect("Wasmtime typed memory.copy export");
        let oracle_result = oracle_copy.call(&mut oracle_store, (case.destination as i32, case.source as i32, case.length as i32, case.grow_pages as i32));

        match case.outcome.as_str() {
            "ok" => {
                assert_eq!(owned_result.unwrap_or_else(|error| panic!("owned {} unexpectedly trapped: {error}", case.name)).0, vec![Value::I32((1 + case.grow_pages) as i32)]);
                assert_eq!(oracle_result.unwrap_or_else(|error| panic!("Wasmtime {} unexpectedly trapped: {error}", case.name)), (1 + case.grow_pages) as i32);
                let expected = case.expected_bytes.as_deref().expect("successful fixture owns exact destination bytes");
                assert_eq!(expected.len(), case.length as usize, "fixture length differs for {}", case.name);
                let start = case.destination as usize;
                let end = start + case.length as usize;
                assert_eq!(&owned.memory(0).expect("owned memory")[start..end], expected, "owned bytes differ for {}", case.name);
                assert_eq!(&oracle_memory.data(&oracle_store)[start..end], expected, "Wasmtime bytes differ for {}", case.name);
            }
            "trap" => {
                let error = match owned_result.expect_err("owned interpreter must reject hostile memory.copy range") {
                    CoreError::Trap(message) => message,
                    other => panic!("owned {} returned non-trap error: {other}", case.name),
                };
                assert_eq!(error, case.owned_error.expect("trap fixture owns an exact diagnostic"), "owned diagnostic differs for {}", case.name);
                assert!(oracle_result.is_err(), "Wasmtime accepted hostile memory.copy range {}", case.name);
            }
            other => panic!("unknown memory.copy fixture outcome {other:?}"),
        }
    }
}

#[test]
fn semio_describe_host_preserves_context_backpressure_and_bounded_result() {
    let module = Arc::new(CoreModule::parse(&memory_module()).expect("parse memory module"));
    let core = CoreInstance::instantiate(module).expect("instantiate memory module");
    let mut actor = SemioActorInstance { component_fingerprint: 1, core };
    let mut host = SemioDescribeHost::new(4);
    let set = HostCall { id: 1, module: "$root".into(), name: "[context-set-0]".into(), arguments: vec![Value::I32(77)], results: vec![] };
    assert_eq!(host.reply(&mut actor, &set).expect("set context").results, vec![]);
    let get = HostCall { id: 2, module: "$root".into(), name: "[context-get-0]".into(), arguments: vec![], results: vec![ValueType::I32] };
    assert_eq!(host.reply(&mut actor, &get).expect("get context").results, vec![Value::I32(77)]);
    let check_write = HostCall { id: 3, module: "wasi:io/streams@0.2.0".into(), name: "[method]output-stream.check-write".into(), arguments: vec![Value::I32(1), Value::I32(0)], results: vec![] };
    host.reply(&mut actor, &check_write).expect("check write");
    assert_eq!(&actor.memory().expect("memory")[8..16], &65_536u64.to_le_bytes());
    actor.memory_mut().expect("memory")[16..20].copy_from_slice(b"test");
    let returned = HostCall { id: 4, module: SEMIO_DESCRIBE_RETURN_MODULE.into(), name: SEMIO_DESCRIBE_RETURN_NAME.into(), arguments: vec![Value::I32(16), Value::I32(4)], results: vec![] };
    assert_eq!(host.reply(&mut actor, &returned).expect("describe result").descriptor, Some(b"test".to_vec()));
}

#[test]
fn passive_data_initialization_and_drop_execute_in_section_order() {
    let module = Arc::new(CoreModule::parse(&passive_data_module()).expect("parse passive data module"));
    let mut instance = CoreInstance::instantiate(module).expect("instantiate passive data module");
    instance.begin_export("load", Vec::new()).expect("begin passive load");
    assert_eq!(drive(&mut instance, 1).expect("drive passive load").0, vec![Value::I32(0x0403_0201)]);
    assert!(instance.data[0].is_none());
}

#[test]
fn cancellation_discards_the_active_machine_before_an_instruction_runs() {
    let module = Arc::new(CoreModule::parse(&loop_module()).expect("parse loop"));
    let mut instance = CoreInstance::instantiate(module).expect("instantiate loop");
    instance.begin_export("sum", vec![Value::I32(1_000_000)]).expect("begin loop");
    assert_eq!(instance.step(50, StepControl { cancelled: true }), CoreStepOutcome::Cancelled { fuel_used: 0 });
    assert!(!instance.active());
}

#[test]
fn checkpoints_are_deterministic_and_module_bound() {
    let module = Arc::new(CoreModule::parse(&add_module()).expect("parse add"));
    let mut instance = CoreInstance::instantiate(Arc::clone(&module)).expect("instantiate add");
    instance.begin_export("add", vec![Value::I32(1), Value::I32(2)]).expect("begin add");
    instance.step(2, StepControl::default());
    assert_eq!(instance.checkpoint(), instance.checkpoint());
    let other = Arc::new(CoreModule::parse(&loop_module()).expect("parse other"));
    assert!(CoreInstance::restore(other, &instance.checkpoint()).is_err());
}

#[test]
fn component_artifact_owns_nested_core_modules_without_platform_wasm() {
    let core = add_module();
    let mut component = b"\0asm\x0d\0\x01\0".to_vec();
    component.push(1);
    leb(core.len() as u32, &mut component);
    component.extend(&core);
    component.extend([0, 5, 4]);
    component.extend(b"test");
    let artifact = WasmArtifact::parse(&component).expect("parse component");
    assert_eq!(artifact.kind(), WasmArtifactKind::Component);
    let WasmArtifact::Component(component) = artifact else { unreachable!() };
    assert_eq!(component.core_modules().len(), 1);
    assert_eq!(component.sections().len(), 2);
    assert_eq!(component.sections()[0].kind, ComponentSectionKind::CoreModule);
    assert_eq!(component.sections()[1].custom_name.as_deref(), Some("test"));
    let mut instance = CoreInstance::instantiate(Arc::clone(&component.core_modules()[0])).expect("instantiate nested core");
    instance.begin_export("add", vec![Value::I32(7), Value::I32(8)]).expect("begin nested add");
    assert_eq!(drive(&mut instance, 1).expect("drive nested add").0, vec![Value::I32(15)]);
}

#[test]
fn component_limits_reject_oversize_before_section_allocation() {
    let bytes = b"\0asm\x0d\0\x01\0";
    let limits = ComponentLimits { maximum_bytes: 7, maximum_nesting: 0, maximum_core_modules: 0 };
    assert!(matches!(ComponentArtifact::parse_with_limits(bytes, limits), Err(CoreError::Validation(_))));
}

#[test]
fn execution_binary_recursively_removes_only_custom_sections() {
    let raw_core = add_module();
    let mut core = raw_core[..8].to_vec();
    core.extend([0, 6, 1, b'c', 1, 2, 3, 4]);
    core.extend_from_slice(&raw_core[8..]);
    let mut component = b"\0asm\x0d\0\x01\0".to_vec();
    component.push(1);
    leb(core.len() as u32, &mut component);
    component.extend_from_slice(&core);
    component.extend([0, 6, 1, b't', 1, 2, 3, 4]);
    let normalized = wasm_execution_binary(&component).expect("normalize component");
    assert!(normalized.len() < component.len());
    let artifact = ComponentArtifact::parse(&normalized).expect("parse normalized component");
    assert_eq!(artifact.sections().len(), 1);
    let mut instance = CoreInstance::instantiate(Arc::clone(&artifact.core_modules()[0])).expect("instantiate normalized core");
    instance.begin_export("add", vec![Value::I32(19), Value::I32(23)]).expect("begin normalized add");
    assert_eq!(drive(&mut instance, 1).expect("drive normalized add").0, vec![Value::I32(42)]);
}

#[test]
fn configured_component_fixture_is_owned_parseable() {
    let Some(path) = std::env::var_os("SEMIO_OWNED_COMPONENT_FIXTURE") else { return };
    let bytes = std::fs::read(path).expect("read configured component fixture");
    let WasmArtifact::Component(component) = WasmArtifact::parse(&bytes).expect("parse configured component fixture") else { panic!("configured fixture is not a component") };
    assert!(!component.core_modules().is_empty(), "configured component has no executable core module");
    let actor = SemioActorArtifact::from_component(component).expect("validate configured Semio actor ABI");
    assert!(actor.module().export(SEMIO_DESCRIBE_EXPORT).is_some());
    let execution = wasm_execution_binary(&bytes).expect("normalize configured component fixture");
    assert!(execution.len() < bytes.len());
    SemioActorArtifact::parse(&execution).expect("validate normalized Semio actor ABI");
}

mod long {
    use super::*;

    #[test]
    fn configured_component_fixture_owned_describe_runs_bounded_fuel() {
        let Some(path) = std::env::var_os("SEMIO_OWNED_COMPONENT_FIXTURE") else { return };
        let bytes = std::fs::read(path).expect("read configured component fixture");
        let artifact = SemioActorArtifact::parse(&bytes).expect("parse configured Semio actor");
        let mut actor = artifact.instantiate().expect("instantiate configured Semio actor");
        assert!(!actor.startup_active(), "configured Semio actor has an undriven start function");
        actor.begin_describe().expect("begin owned describe");
        let mut host = SemioDescribeHost::new(64 * 1024 * 1024);
        let mut task_return = None;
        let mut fuel_used = 0;
        let mut host_calls = 0;
        while fuel_used < 100_000_000 {
            let grant = (100_000_000 - fuel_used).min(100_000);
            match actor.step(grant, StepControl::default()) {
                CoreStepOutcome::Yield { fuel_used: used } => fuel_used += used,
                CoreStepOutcome::HostCall { fuel_used: used, call } => {
                    fuel_used += used;
                    host_calls += 1;
                    let reply = host.reply(&mut actor, &call).expect("serve owned describe host call");
                    task_return = reply.descriptor.or(task_return);
                    actor.resume_host(call.id, Ok(reply.results)).expect("resume owned describe host call");
                }
                CoreStepOutcome::Complete { fuel_used: used, .. } => {
                    fuel_used += used;
                    break;
                }
                CoreStepOutcome::Cancelled { .. } => panic!("owned describe cancelled"),
                CoreStepOutcome::Fault { error, .. } => panic!("owned describe fault: {error}"),
            }
        }
        assert_eq!(fuel_used, 100_000_000);
        assert!(host_calls > 0);
        assert!(task_return.is_none(), "stdio fixture unexpectedly completed inside the documented probe budget");
    }
}
