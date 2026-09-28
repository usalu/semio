//! 📥️ Exact-request GLB delivery through the renderer's existing paged asset lane.
use std::sync::{Mutex, OnceLock};
use infinite_world::world::{World3dMeshAsset, WorldAssetRequestKind, WorldAssetRequestToken};

const CAPACITY: usize = 16;

struct Slot {
    token: WorldAssetRequestToken,
    asset: Option<World3dMeshAsset>,
    fault: Option<String>,
    cancelled: bool,
}

static SLOTS: OnceLock<Mutex<[Option<Slot>; CAPACITY]>> = OnceLock::new();

fn slots() -> &'static Mutex<[Option<Slot>; CAPACITY]> {
    SLOTS.get_or_init(|| Mutex::new(std::array::from_fn(|_| None)))
}

pub(crate) struct IconExportAssetRequest {
    token: Option<WorldAssetRequestToken>,
}

impl IconExportAssetRequest {
    pub(crate) fn new(url: &str) -> Result<Self, String> {
        let mut slots = slots().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let index = slots.iter().position(Option::is_none).ok_or("icon export asset requests are at capacity")?;
        let token = crate::reserve_renderer_asset_request(WorldAssetRequestKind::Glb, url).map_err(|fault| format!("icon export asset request: {fault:?}"))?;
        slots[index] = Some(Slot { token, asset: None, fault: None, cancelled: false });
        Ok(Self { token: Some(token) })
    }

    pub(crate) fn take_ready(&mut self) -> Option<Result<World3dMeshAsset, String>> {
        let token = self.token?;
        let mut slots = slots().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let index = slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.token == token))?;
        let slot = slots[index].as_mut()?;
        if slot.cancelled {
            return None;
        }
        let result = if let Some(asset) = slot.asset.take() {
            Ok(asset)
        } else if let Some(fault) = slot.fault.take() {
            Err(fault)
        } else {
            return None;
        };
        slots[index] = None;
        self.token = None;
        Some(result)
    }

    pub(crate) fn cancel(&mut self) {
        let Some(token) = self.token else { return };
        if let Ok(mut slots) = slots().lock() {
            if let Some(slot) = slots.iter_mut().flatten().find(|slot| slot.token == token) {
                slot.cancelled = true;
            }
        }
        if let Ok(mut authority) = crate::renderer_asset_io().lock() {
            authority.cancel_request(token);
        }
    }

    pub(crate) fn close_step(&mut self) -> bool {
        self.cancel();
        let Some(token) = self.token else { return true };
        let mut slots = slots().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let Some(index) = slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.token == token)) else {
            self.token = None;
            return true;
        };
        if close_slot(&mut slots[index]) {
            self.token = None;
            true
        } else {
            false
        }
    }
}

impl Drop for IconExportAssetRequest {
    fn drop(&mut self) {
        self.cancel();
    }
}

fn close_slot(slot: &mut Option<Slot>) -> bool {
    if let Some(asset) = slot.as_mut().and_then(|slot| slot.asset.as_mut()) {
        if !asset.close_step() {
            return false;
        }
    }
    *slot = None;
    true
}

pub(crate) fn publish(token: WorldAssetRequestToken, asset: World3dMeshAsset) -> Result<(), World3dMeshAsset> {
    let mut slots = slots().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(slot) = slots.iter_mut().flatten().find(|slot| slot.token == token) else { return Err(asset) };
    if slot.cancelled || slot.asset.is_some() || slot.fault.is_some() {
        return Err(asset);
    }
    slot.asset = Some(asset);
    Ok(())
}

pub(crate) fn reject(token: WorldAssetRequestToken, fault: &str) {
    let mut slots = slots().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(slot) = slots.iter_mut().flatten().find(|slot| slot.token == token) {
        if !slot.cancelled && slot.asset.is_none() && slot.fault.is_none() {
            slot.fault = Some(fault.to_string());
        }
    }
}

pub(crate) fn retire_cancelled_step() -> bool {
    let mut slots = slots().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(slot) = slots.iter_mut().find(|slot| slot.as_ref().is_some_and(|slot| slot.cancelled)) else { return false };
    close_slot(slot);
    true
}

pub(crate) fn close_all_step() -> bool {
    let mut slots = slots().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(slot) = slots.iter_mut().find(|slot| slot.is_some()) else { return true };
    if let Some(slot) = slot.as_mut() {
        slot.cancelled = true;
        if let Ok(mut authority) = crate::renderer_asset_io().lock() {
            authority.cancel_request(slot.token);
        }
    }
    close_slot(slot);
    false
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../../🧪️tests/📤️asset-delivery/🦀️.rs"]
mod tests;
