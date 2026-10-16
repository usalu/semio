//! 🏠️ Component surface close bridge: the request, terminal and acknowledgement cells between a component scene and the host that closes its surface, lifted out of the OS host for the `scene-state` crate of the renderer partition (plan section 5.2).
use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComponentSurfaceCloseToken(u64);

#[derive(Clone)]
pub struct ComponentSurfaceCloseRequest {
    pub token: ComponentSurfaceCloseToken,
    pub owner: crate::element_contracts::ScenePointerTarget,
    pub engine_token: Option<semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::EngineSurfaceToken>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentSurfaceCloseBridgeState {
    Pending,
    Active,
    Terminal,
}

pub struct ComponentSurfaceCloseBridge {
    pub generation: u64,
    pub request: Option<ComponentSurfaceCloseRequest>,
    pub state: ComponentSurfaceCloseBridgeState,
}

impl Default for ComponentSurfaceCloseBridge {
    fn default() -> Self {
        Self { generation: 0, request: None, state: ComponentSurfaceCloseBridgeState::Terminal }
    }
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    pub static COMPONENT_SURFACE_CLOSE_BRIDGE: std::cell::RefCell<ComponentSurfaceCloseBridge> = std::cell::RefCell::new(ComponentSurfaceCloseBridge::default());
}

#[cfg(not(target_arch = "wasm32"))]
pub static COMPONENT_SURFACE_CLOSE_BRIDGE: std::sync::OnceLock<std::sync::Mutex<ComponentSurfaceCloseBridge>> = std::sync::OnceLock::new();

#[cfg(target_arch = "wasm32")]
pub fn with_component_surface_close_bridge<R>(f: impl FnOnce(&mut ComponentSurfaceCloseBridge) -> R) -> R {
    COMPONENT_SURFACE_CLOSE_BRIDGE.with(|bridge| f(&mut bridge.borrow_mut()))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn with_component_surface_close_bridge<R>(f: impl FnOnce(&mut ComponentSurfaceCloseBridge) -> R) -> R {
    let mut bridge = COMPONENT_SURFACE_CLOSE_BRIDGE.get_or_init(|| std::sync::Mutex::new(ComponentSurfaceCloseBridge::default())).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    f(&mut bridge)
}

pub fn request_component_surface_close(
    owner: crate::element_contracts::ScenePointerTarget,
    engine_token: Option<semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::EngineSurfaceToken>,
) -> Result<ComponentSurfaceCloseToken, (crate::element_contracts::ScenePointerTarget, Option<semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::EngineSurfaceToken>)> {
    with_component_surface_close_bridge(|bridge| {
        if bridge.request.is_some() {
            return Err((owner, engine_token));
        }
        let Some(generation) = bridge.generation.checked_add(1).filter(|generation| *generation != 0) else {
            return Err((owner, engine_token));
        };
        bridge.generation = generation;
        let token = ComponentSurfaceCloseToken(generation);
        bridge.request = Some(ComponentSurfaceCloseRequest { token, owner, engine_token });
        bridge.state = ComponentSurfaceCloseBridgeState::Pending;
        Ok(token)
    })
}

pub fn component_surface_close_terminal(token: ComponentSurfaceCloseToken) -> bool {
    with_component_surface_close_bridge(|bridge| bridge.request.as_ref().is_some_and(|request| request.token == token) && bridge.state == ComponentSurfaceCloseBridgeState::Terminal)
}

pub fn acknowledge_component_surface_close(token: ComponentSurfaceCloseToken) -> bool {
    with_component_surface_close_bridge(|bridge| {
        if bridge.request.as_ref().is_none_or(|request| request.token != token) || bridge.state != ComponentSurfaceCloseBridgeState::Terminal {
            return false;
        }
        bridge.request = None;
        true
    })
}

pub fn take_component_surface_close_request() -> Option<ComponentSurfaceCloseRequest> {
    with_component_surface_close_bridge(|bridge| {
        if bridge.state != ComponentSurfaceCloseBridgeState::Pending {
            return None;
        }
        bridge.state = ComponentSurfaceCloseBridgeState::Active;
        bridge.request.clone()
    })
}

pub fn publish_component_surface_close_terminal(token: ComponentSurfaceCloseToken) -> bool {
    with_component_surface_close_bridge(|bridge| {
        if bridge.request.as_ref().is_none_or(|request| request.token != token) || bridge.state != ComponentSurfaceCloseBridgeState::Active {
            return false;
        }
        bridge.state = ComponentSurfaceCloseBridgeState::Terminal;
        true
    })
}

pub fn component_surface_close_pending() -> bool {
    with_component_surface_close_bridge(|bridge| bridge.request.is_some() && bridge.state != ComponentSurfaceCloseBridgeState::Terminal)
}

pub fn component_surface_close_occupied() -> bool {
    with_component_surface_close_bridge(|bridge| bridge.request.is_some())
}
