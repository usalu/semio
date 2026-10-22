//! 🛎️ The per-instance app cell of the plugin runtime: host and background holds, refusals and the cell itself.

use semio_framework_plugin_plugin_program::{AppInstance, PluginProgram};
use semio_framework_plugin_decl_app::ArtifactApp;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicU8, Ordering};
use semio_framework::FaultFrom;
use semio_framework_value::{FromValue, ToValue};
use semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1;
use semio_framework_plugin_reactor_app::PluginApp;
use crate::life_cleanup_pumps::RuntimeLiveCleanupPump;
use crate::result_publication;
use crate::life_cleanup_vocabulary::RuntimeMaintenanceStatus;
/// 🛎️ Why a lane did not get [`RuntimeAppCell::instance`]. `Contended` is a hold that releases
/// on its own — another lane is mid-step; `Poisoned` is a hold that never will.
#[doc(hidden)]
#[derive(Debug)]
pub enum RuntimeInstanceRefusal {
    Contended,
    Poisoned,
}
/// 🛎️ The liveness backstop on a host crossing's wait for a background lane, counted in
/// scheduler yields. One background turn is priced at [`RUNTIME_CLOSE_INNER_GRANT_US`] and
/// every yield hands the core to whoever holds the app, so a well-behaved lane releases within
/// a handful of them; this ceiling exists only so a lane that breaks its own step budget turns
/// into the refusal the host already knows how to report rather than an unbounded host stall.
const RUNTIME_HOST_CONTENTION_YIELDS: u32 = 4_096;
/// 🛎️ A host crossing's exclusive hold on one app, which keeps every background lane off the
/// app for as long as it lives. It is deliberately held across `.await`: a host crossing's
/// local future retains exclusive app ownership while suspended (a submitted media export
/// spans several awaits), which is what makes the crossing atomic against the maintenance and
/// close lanes; those lanes never wait on it — they read [`RuntimeAppCell::host_crossings`] and
/// yield their whole turn.
#[doc(hidden)]
pub struct HostInstanceGuard<'a, PA: PluginApp> {
    pub cell: &'a RuntimeAppCell<PA>,
    pub instance: Option<std::sync::MutexGuard<'a, AppInstance<PA>>>,
}
impl<PA: PluginApp> std::ops::Deref for HostInstanceGuard<'_, PA> {
    type Target = AppInstance<PA>;
    fn deref(&self) -> &AppInstance<PA> {
        self.instance.as_ref().expect("a live host crossing holds its app")
    }
}
impl<PA: PluginApp> std::ops::DerefMut for HostInstanceGuard<'_, PA> {
    fn deref_mut(&mut self) -> &mut AppInstance<PA> {
        self.instance.as_mut().expect("a live host crossing holds its app")
    }
}
/// 🛎️ The app is released BEFORE the crossing is unregistered, so no lane can ever read a free
/// counter while the mutex is still held — the reading a waiting lane makes its decision on is
/// never stale in the direction that would refuse a crossing.
impl<PA: PluginApp> Drop for HostInstanceGuard<'_, PA> {
    fn drop(&mut self) {
        drop(self.instance.take());
        self.cell.host_crossings.fetch_sub(1, Ordering::SeqCst);
    }
}
/// 🔧️ A background maintenance or close turn's hold on one app, counted while it lives so a
/// host crossing can tell a transient hold from one it must refuse.
#[doc(hidden)]
pub struct BackgroundInstanceGuard<'a, PA: PluginApp> {
    pub cell: &'a RuntimeAppCell<PA>,
    pub instance: Option<std::sync::MutexGuard<'a, AppInstance<PA>>>,
}
impl<PA: PluginApp> std::ops::Deref for BackgroundInstanceGuard<'_, PA> {
    type Target = AppInstance<PA>;
    fn deref(&self) -> &AppInstance<PA> {
        self.instance.as_ref().expect("a live background turn holds its app")
    }
}
impl<PA: PluginApp> std::ops::DerefMut for BackgroundInstanceGuard<'_, PA> {
    fn deref_mut(&mut self) -> &mut AppInstance<PA> {
        self.instance.as_mut().expect("a live background turn holds its app")
    }
}
/// 🔧️ The app is released BEFORE the hold is unregistered. The other order has a window in
/// which a waiting host crossing reads zero background holds while the mutex is still taken,
/// and refuses the very turn it was waiting out.
impl<PA: PluginApp> Drop for BackgroundInstanceGuard<'_, PA> {
    fn drop(&mut self) {
        drop(self.instance.take());
        self.cell.background_holds.fetch_sub(1, Ordering::SeqCst);
    }
}
#[doc(hidden)]
pub struct RuntimeAppCell<PA: PluginApp> {
    pub id: u32,
    pub mounted_policy: MountedOwnerPolicyV1,
    pub instance: std::sync::Mutex<AppInstance<PA>>,
    pub maintenance_pump: std::sync::Mutex<RuntimeLiveCleanupPump<PA>>,
    pub result_pages: std::sync::Mutex<result_publication::NativeResultPages>,
    pub maintenance_status: AtomicU8,
    pub maintenance_fault_us: AtomicU64,
    pub maintenance_generation: AtomicU64,
    pub maintenance_stalled_steps: AtomicU32,
    /// 🛎️ How many HOST crossings currently want or hold [`Self::instance`]. A background lane
    /// reads it before it touches the mutex and yields its whole turn while it is non-zero, so
    /// the host lane — which has nothing to yield to — is never made to wait on maintenance it
    /// did not ask for.
    pub host_crossings: AtomicU32,
    /// 🔧️ How many BACKGROUND lane turns currently want or hold [`Self::instance`]. It is
    /// raised before the try-lock and lowered on release, so a non-zero reading is the host's
    /// proof that a refused try-lock is a transient maintenance or close step (which is priced
    /// at [`RUNTIME_CLOSE_INNER_GRANT_US`] and will release) rather than an app the host lane
    /// itself already owns.
    pub background_holds: AtomicU32,
    /// 🌡️ [`PluginApp::maintenance_under_pressure`] as of the last maintenance step — read by the
    /// reactor turn to decide whether this instance gets a burst of steps instead of one.
    pub maintenance_pressure: AtomicBool,
    #[cfg(any(test, target_arch = "wasm32"))]
    pub maintenance_probe_entries: AtomicU64,
    #[cfg(test)]
    pub maintenance_probe_input_waits: AtomicU64,
}
impl<PA: PluginApp> RuntimeAppCell<PA> {
    pub fn new(instance: AppInstance<PA>, mounted_policy: MountedOwnerPolicyV1) -> Self {
        Self {
            id: instance.id,
            mounted_policy,
            instance: std::sync::Mutex::new(instance),
            maintenance_pump: std::sync::Mutex::new(RuntimeLiveCleanupPump::new()),
            result_pages: std::sync::Mutex::new(result_publication::NativeResultPages::new()),
            maintenance_status: AtomicU8::new(RuntimeMaintenanceStatus::Ready.repr()),
            maintenance_fault_us: AtomicU64::new(0),
            maintenance_generation: AtomicU64::new(0),
            maintenance_stalled_steps: AtomicU32::new(0),
            host_crossings: AtomicU32::new(0),
            background_holds: AtomicU32::new(0),
            maintenance_pressure: AtomicBool::new(false),
            #[cfg(any(test, target_arch = "wasm32"))]
            maintenance_probe_entries: AtomicU64::new(0),
            #[cfg(test)]
            maintenance_probe_input_waits: AtomicU64::new(0),
        }
    }
    /// 🛎️ Takes the app for a HOST crossing, which outranks every background lane. Registering
    /// the crossing BEFORE the try-lock stops a background lane from starting a new turn on
    /// this app at all; a turn that was already running is waited out, because it is priced at
    /// [`RUNTIME_CLOSE_INNER_GRANT_US`] and releases on its own, and refusing the host over it
    /// would turn a scheduling accident into a `plugin.internal` fault the caller has no way to
    /// retry. `None` keeps the one meaning the bare try-lock always had at a host site: the app
    /// is owned by something that will NOT release for this crossing — the host lane itself (a
    /// reentrant crossing, or a submitted media export holding the app across its await) or a
    /// poisoned mutex.
    pub fn host_instance(&self) -> Option<HostInstanceGuard<'_, PA>> {
        self.host_crossings.fetch_add(1, Ordering::SeqCst);
        for _ in 0..RUNTIME_HOST_CONTENTION_YIELDS {
            match self.instance.try_lock() {
                Ok(instance) => return Some(HostInstanceGuard { cell: self, instance: Some(instance) }),
                Err(std::sync::TryLockError::Poisoned(_)) => break,
                Err(std::sync::TryLockError::WouldBlock) => {
                    if self.background_holds.load(Ordering::SeqCst) == 0 {
                        break;
                    }
                    std::thread::yield_now();
                }
            }
        }
        self.host_crossings.fetch_sub(1, Ordering::SeqCst);
        None
    }
    /// 🔧️ Takes the app for a BACKGROUND maintenance or close turn. It never waits: a host
    /// crossing in flight, or a lock already taken, is a contended turn, and the lane's own
    /// `Yield` is exactly what a contended turn costs.
    pub fn background_instance(&self) -> Result<BackgroundInstanceGuard<'_, PA>, RuntimeInstanceRefusal> {
        if self.host_crossings.load(Ordering::SeqCst) > 0 {
            return Err(RuntimeInstanceRefusal::Contended);
        }
        self.background_holds.fetch_add(1, Ordering::SeqCst);
        match self.instance.try_lock() {
            Ok(instance) => Ok(BackgroundInstanceGuard { cell: self, instance: Some(instance) }),
            Err(refusal) => {
                self.background_holds.fetch_sub(1, Ordering::SeqCst);
                Err(match refusal {
                    std::sync::TryLockError::Poisoned(_) => RuntimeInstanceRefusal::Poisoned,
                    std::sync::TryLockError::WouldBlock => RuntimeInstanceRefusal::Contended,
                })
            }
        }
    }
}
