//! 🌱️ Component preparation runs only an explicit constructor with no guest host authority.

use super::*;

const PREPARE_ACTOR: &str = "semio_owned_prepare_actor_v1";
const PREPARATION_SLICE: u64 = 1 << 20;
const OWNED_PREPARATION_STORAGE_BYTES: usize = 512 * 1024 * 1024;

impl OwnedSemioArtifact {
    /// 🧊️ Runs the declared context-free constructor to produce a sealed input for paid actor opening.
    pub fn prepare_actor(&self, fuel: u64, progress: impl FnMut(u64) -> bool) -> Result<Option<PreparedCoreInstance>, CoreError> {
        self.actor_module.prepare_actor_image(fuel, OWNED_PREPARATION_STORAGE_BYTES, progress)
    }

    /// 📥️ Restores a receipt-verified image against this freshly parsed exact component and its authored constructor.
    pub fn restore_actor_preparation(&self, component_sha256: [u8; 32], image: &[u8], maximum_wire_bytes: usize, maximum_storage_bytes: usize, checkpoint: impl FnMut() -> bool) -> Result<PreparedCoreInstance, CoreError> {
        if !self.actor_module.has_actor_preparation()? {
            return Err(CoreError::Validation("component does not declare actor preparation".into()));
        }
        PreparedCoreInstance::read_image(Arc::clone(&self.actor_module), component_sha256, image, maximum_wire_bytes,
            maximum_storage_bytes.min(OWNED_PREPARATION_STORAGE_BYTES), checkpoint)
    }
}

impl CoreModule {
    /// 🌱️ Seals the result of this exact module's import-free actor constructor.
    pub fn prepare_actor_image(self: &Arc<Self>, fuel: u64, storage_bytes: usize, progress: impl FnMut(u64) -> bool) -> Result<Option<PreparedCoreInstance>, CoreError> {
        if !self.has_actor_preparation()? { return Ok(None) }
        prepare_core(Arc::clone(self), fuel, storage_bytes, progress).map(Some)
    }

    fn has_actor_preparation(&self) -> Result<bool, CoreError> {
        let Some(export) = self.export(PREPARE_ACTOR) else { return Ok(false) };
        let ExportKind::Function(function) = export else {
            return Err(CoreError::Validation("actor preparation export is not a function".into()));
        };
        let signature = self.function_type(*function)?;
        if !signature.parameters.is_empty() || signature.results.as_slice() != [ValueType::I32] {
            return Err(CoreError::Validation("actor preparation must return its scalar readiness".into()));
        }
        Ok(true)
    }
}

fn prepare_core(module: Arc<CoreModule>, fuel: u64, storage_bytes: usize, mut progress: impl FnMut(u64) -> bool) -> Result<PreparedCoreInstance, CoreError> {
    if !progress(0) {
        return Err(CoreError::State("actor preparation cancelled".into()));
    }
    if fuel == 0 {
        return Err(CoreError::State("actor preparation exhausted its instruction budget".into()));
    }
    let mut remaining = fuel;
    let mut core = CoreInstance::instantiate_preparing(module, storage_bytes)?;
    if core.active() {
        run_import_free(&mut core, fuel, &mut remaining, &mut progress)?;
    }
    core.begin_export(PREPARE_ACTOR, Vec::new())?;
    let result = run_import_free(&mut core, fuel, &mut remaining, &mut progress)?;
    if result.as_slice() != [Value::I32(1)] {
        return Err(CoreError::State("component refused actor preparation".into()));
    }
    if !progress(fuel - remaining) {
        return Err(CoreError::State("actor preparation cancelled before publication".into()));
    }
    PreparedCoreInstance::seal(core)
}

fn run_import_free(core: &mut CoreInstance, fuel: u64, remaining: &mut u64, progress: &mut impl FnMut(u64) -> bool) -> Result<Vec<Value>, CoreError> {
    loop {
        if !progress(fuel - *remaining) {
            return Err(CoreError::State("actor preparation cancelled".into()));
        }
        if *remaining == 0 {
            return Err(CoreError::State("actor preparation exhausted its instruction budget".into()));
        }
        match core.step((*remaining).min(PREPARATION_SLICE), StepControl::default()) {
            CoreStepOutcome::Yield { fuel_used } if fuel_used > 0 => *remaining -= fuel_used,
            CoreStepOutcome::Complete { fuel_used, values } => {
                *remaining -= fuel_used;
                return Ok(values);
            }
            CoreStepOutcome::HostCall { call, .. } => {
                if let Some(values) = preparation_host_reply(core, &call)? {
                    core.resume_host(call.id, Ok(values))?;
                    continue;
                }
                return Err(CoreError::Host(format!("actor preparation requested {}::{}", call.module, call.name)));
            },
            CoreStepOutcome::Fault { error, .. } => return Err(error),
            _ => return Err(CoreError::State("actor preparation made no progress".into())),
        }
    }
}

fn preparation_host_reply(core: &mut CoreInstance, call: &HostCall) -> Result<Option<Vec<Value>>, CoreError> {
    if call.name == "insecure-seed" {
        let Some(Value::I32(pointer)) = call.arguments.first() else { return Ok(None) };
        core.write_preparation_seed(*pointer)?;
        return Ok(Some(Vec::new()));
    }
    if call.name == "get-random-u64" { return Ok(Some(vec![Value::I64(0x5345_4d49_4f50_5245)])); }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_ready_import_free_execution_can_publish_prepared_actor_state() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let bytes = case["component"].as_str().unwrap().as_bytes().chunks_exact(2)
                .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect::<Vec<_>>();
            let module = Arc::new(CoreModule::parse(&bytes).unwrap());
            let outcome = module.prepare_actor_image(case["fuel"].as_u64().unwrap(), OWNED_PREPARATION_STORAGE_BYTES, |_| case["continue"].as_bool().unwrap());
            assert_eq!(matches!(&outcome, Ok(Some(_))), case["ready"].as_bool().unwrap(), "{}", case["name"]);
            if let Some(expected) = case["hostImport"].as_str() {
                assert!(matches!(outcome, Err(CoreError::Host(message)) if message.ends_with(expected)));
            }
        }
    }

    #[test]
    fn prepared_storage_rejects_growth_without_issuing_more_authority() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
        for case in fixture["storageCases"].as_array().unwrap() {
            let bytes = case["component"].as_str().unwrap().as_bytes().chunks_exact(2)
                .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect::<Vec<_>>();
            let module = Arc::new(CoreModule::parse(&bytes).unwrap());
            let limit = case["maximumStorageBytes"].as_u64().unwrap() as usize;
            let outcome = module.prepare_actor_image(16, limit, |_| true);
            assert!(matches!(outcome, Ok(Some(_))), "{}", case["name"]);
            assert!(module.prepare_actor_image(16, 0, |_| true).is_err(), "{}", case["name"]);
        }
    }
}
