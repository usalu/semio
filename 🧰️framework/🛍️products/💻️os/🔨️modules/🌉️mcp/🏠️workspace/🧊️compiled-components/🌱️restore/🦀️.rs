//! 🌱️ Restores the image retained by the exact staged component publication.

use super::*;
use semio_os_mcp_catalog::plugin_paths_and_descriptor::StagedActorPreparation;
use std::io::Read;

pub(super) fn restore_compiled_actor(
    compiled: &mut semio_plugin_host::CompiledHandle,
    component: &[u8],
    preparation: &StagedActorPreparation,
    scope: &ActivationScope,
) -> Result<(), GatewayError> {
    let refused = |detail: &str| GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("retained actor {}: {detail}", preparation.image.display()));
    scope.enter(ActivationPhase::CompilingComponent).map_err(activation_fault)?;
    let mut component_hash = framework_hash::Sha256::new();
    for chunk in component.chunks(COMPONENT_HASH_CHUNK_BYTES) {
        scope.checkpoint().map_err(activation_fault)?;
        component_hash.update(chunk);
    }
    let expected = preparation.image_bytes;
    let metadata = std::fs::symlink_metadata(&preparation.image).map_err(|error| refused(&error.to_string()))?;
    if !metadata.is_file() || metadata.len() != expected as u64 {
        return Err(refused("published extent differs"));
    }
    let mut file = std::fs::File::open(&preparation.image).map_err(|error| refused(&error.to_string()))?;
    let mut image = Vec::new();
    image.try_reserve_exact(expected).map_err(|error| refused(&error.to_string()))?;
    let mut buffer = [0; 64 * 1024];
    let mut image_hash = framework_hash::Sha256::new();
    while image.len() < expected {
        let count = (expected - image.len()).min(buffer.len());
        file.read_exact(&mut buffer[..count]).map_err(|error| refused(&error.to_string()))?;
        image_hash.update(&buffer[..count]);
        image.extend_from_slice(&buffer[..count]);
        scope.advance(ActivationPhase::CompilingComponent, 0.5 * image.len() as f64 / expected as f64).map_err(activation_fault)?;
    }
    if file.read(&mut buffer[..1]).map_err(|error| refused(&error.to_string()))? != 0 || image_hash.finalize() != preparation.image_sha256 {
        return Err(refused("published digest differs"));
    }
    let restored = compiled.restore_owned_actor(component_hash.finalize(), &image, preparation.maximum_bytes, || !scope.cancel().is_cancelled_now());
    scope.checkpoint().map_err(activation_fault)?;
    restored.map_err(|error| refused(&error.to_string()))?;
    scope.advance(ActivationPhase::CompilingComponent, 1.0).map_err(activation_fault)
}
