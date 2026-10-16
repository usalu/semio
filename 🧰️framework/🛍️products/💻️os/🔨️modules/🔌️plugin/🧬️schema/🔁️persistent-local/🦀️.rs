//! 🔖️ Component-instance persistent locals: the wasip2 linear-memory global holder and the `component_persistent_local!` macro, whose `$crate` paths stay inside this crate.

//#region 🔖️ComponentPersistentLocal
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
#[doc(hidden)]
pub mod component_persistent_local {
    use std::{cell::UnsafeCell, mem::MaybeUninit, sync::Once};

    /// 🧵️ Component-async calls receive fresh Rust TLS contexts, so actor state that must survive
    /// from one exported call to the next lives in the component instance's linear-memory global.
    pub struct ComponentPersistentLocal<T> {
        once: Once,
        value: UnsafeCell<MaybeUninit<T>>,
        initialize: fn() -> T,
    }

    impl<T> ComponentPersistentLocal<T> {
        pub const fn new(initialize: fn() -> T) -> Self {
            Self { once: Once::new(), value: UnsafeCell::new(MaybeUninit::uninit()), initialize }
        }

        pub fn with<R>(&'static self, use_value: impl FnOnce(&'static T) -> R) -> R {
            self.once.call_once(|| unsafe {
                (*self.value.get()).write((self.initialize)());
            });
            use_value(unsafe { (&*self.value.get()).assume_init_ref() })
        }
    }

    unsafe impl<T> Sync for ComponentPersistentLocal<T> {}
}

#[doc(hidden)]
#[macro_export]
macro_rules! component_persistent_local {
    ($( $(#[$attribute:meta])* static $name:ident: $value_type:ty = $initialize:expr; )+) => {
        $(
            #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
            $(#[$attribute])*
            static $name: $crate::component_persistent_local::ComponentPersistentLocal<$value_type> =
                $crate::component_persistent_local::ComponentPersistentLocal::new(|| $initialize);
        )+

        #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
        std::thread_local! {
            $(
                $(#[$attribute])*
                static $name: $value_type = $initialize;
            )+
        }
    };
}
//#endregion 🔖️ComponentPersistentLocal

//#region 📦️OwnedAbi
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
#[doc(hidden)]
pub mod owned_abi {
    /// 📋️ Fixed stack authority preserves all original scalar fields and refuses malformed extents.
    pub fn original_job_budget(arguments:[u64;19])->Result<semio_framework::kernel::OriginalJobBudget,semio_framework_value::ValueRefusalKind>{semio_framework::kernel::OriginalJobBudget::from_scalar_arguments(arguments).map_err(|error|error.kind)}

    use semio_framework_value_derive::{FromValue, ToValue};
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    pub struct PollInput {
        pub events: Vec<semio_framework::kernel::Event>,
        pub command_page: Option<(semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage)>,
        pub cold_pair_page: Option<semio_framework::kernel::ColdDocumentPairPage>,
        pub budget: semio_framework::kernel::Budget,
    }

    /// 🌉️ Hand-written, not derived — `events`/`command_page`/`budget` are foreign kernel types
    /// (`semio_framework::kernel::*`, owned by the `semio-framework`/`semio-framework-os-kernel`
    /// crates), so neither `#[derive(ToValue, FromValue)]` nor a direct `impl … for
    /// semio_framework::kernel::Event` is legal here (orphan rule: neither the trait nor the type
    /// is local to this crate). Bridges through the already-derived `Serialize`/`Deserialize` via
    /// `serde_json` instead, reusing the existing `DslValue <-> serde_json::Value` conversion.
    impl semio_framework_value::ToValue for PollInput {
        fn to_value(&self) -> semio_framework_value::DslValue {
            semio_framework_value::DslValue::from(&serde_json::to_value(self).expect("PollInput always serializes"))
        }
    }
    impl semio_framework_value::FromValue for PollInput {
        fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
            serde_json::from_value(serde_json::Value::from(value)).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))
        }
    }

    #[derive(Deserialize, FromValue, Serialize, ToValue)]
    pub struct StartJobInput {
        pub source: Option<semio_framework::kernel::OriginalJobSource>,
        pub budget: semio_framework::kernel::OriginalJobBudget,
    }

    #[derive(Deserialize, FromValue, Serialize, ToValue)]
    pub struct StepJobInput {
        pub job: u64,
        pub budget: semio_framework::kernel::OriginalJobBudget,
    }

    #[derive(Deserialize, FromValue, Serialize, ToValue)]
    pub struct CancelJobInput {
        pub job: u64,
        pub budget: semio_framework::kernel::OriginalJobBudget,
    }

    #[derive(Deserialize, FromValue, Serialize, ToValue)]
    pub struct RestoreInput {
        pub state: Vec<u8>,
    }

    /// 🧬️ Input of the owned `codec` core exports. `document_id` is only read by `genesis`, `ops`
    /// only by `apply-ops` (an `encode_ops_vec` batch) and `replay-envelopes` (an `encode_envelopes`
    /// stream), and the pair only by `print-mirror`/`apply-ops`/`replay-envelopes` — one shape for
    /// all five keeps the owned ABI's JSON envelope single-valued per export, exactly as
    /// `RestoreInput` does.
    #[derive(Deserialize, FromValue, Serialize, ToValue)]
    pub struct CodecInput {
        pub artifact_schema: String,
        #[serde(default)]
        pub document_id: String,
        #[serde(default)]
        pub pack: Vec<u8>,
        #[serde(default)]
        pub spr: Vec<u8>,
        #[serde(default)]
        pub ops: Vec<u8>,
    }

    /// 📦️ Output of the owned `codec.genesis`/`codec.apply-ops`/`codec.replay-envelopes` core exports.
    #[derive(Deserialize, FromValue, Serialize, ToValue)]
    pub struct CodecPair {
        pub pack: Vec<u8>,
        pub spr: Vec<u8>,
    }

    /// 📥️ Output of the owned `codec.print-mirror` core export.
    #[derive(Deserialize, FromValue, Serialize, ToValue)]
    pub struct CodecMirror {
        pub dsl: String,
        pub ops: String,
    }

    #[derive(Deserialize, FromValue, Serialize, ToValue)]
    #[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
    #[value(tag = "kind", rename_all = "camelCase")]
    pub enum JobStep {
        Running { progress: Option<Vec<u8>> },
        Done { output: Vec<u8> },
        Failed { error: Vec<u8> },
    }

    pub fn allocate(length: u32) -> u32 {
        let bytes = vec![0_u8; length as usize].into_boxed_slice();
        Box::into_raw(bytes) as *mut u8 as usize as u32
    }

    pub unsafe fn deallocate(pointer: u32, length: u32) {
        let slice = std::ptr::slice_from_raw_parts_mut(pointer as usize as *mut u8, length as usize);
        drop(unsafe { Box::from_raw(slice) });
    }

    pub unsafe fn take_json<T: serde::de::DeserializeOwned>(pointer: u32, length: u32) -> Result<T, Vec<u8>> {
        let slice = std::ptr::slice_from_raw_parts_mut(pointer as usize as *mut u8, length as usize);
        let bytes = unsafe { Box::from_raw(slice) };
        serde_json::from_slice(&bytes).map_err(|error| error.to_string().into_bytes())
    }

    unsafe fn take_actor_json<T: serde::de::DeserializeOwned>(pointer:u32,length:u32)->Result<T,semio_framework_value::ValueRefusalKind>{
        let slice=std::ptr::slice_from_raw_parts_mut(pointer as usize as *mut u8,length as usize);
        let bytes=unsafe{Box::from_raw(slice)};
        serde_json::from_slice(&bytes).map_err(|_|semio_framework_value::ValueRefusalKind::InvalidValue)
    }

    /// 🌱️ The owned actor admission port never creates dynamic diagnostic prose on malformed input.
    pub unsafe fn take_actor_admission_input(pointer:u32,length:u32)->Result<(semio_framework::kernel::RetainedTurnInput,semio_framework_value::RetainedCloneGrant),semio_framework_value::ValueRefusalKind>{unsafe{take_actor_json(pointer,length)}}

    /// 📬️ Retains the original declared poll fields without an error-text carrier.
    pub unsafe fn take_actor_poll_input(pointer:u32,length:u32)->Result<PollInput,semio_framework_value::ValueRefusalKind>{unsafe{take_actor_json(pointer,length)}}

    pub fn return_json<T: Serialize>(value: &T) -> u64 {
        match serde_json::to_vec(value) {
            Ok(bytes) => return_bytes(bytes),
            Err(error) => return_bytes(error.to_string().into_bytes()),
        }
    }

    pub fn return_bytes(bytes: Vec<u8>) -> u64 {
        let bytes = bytes.into_boxed_slice();
        let length = bytes.len() as u32;
        let pointer = Box::into_raw(bytes) as *mut u8 as usize as u32;
        u64::from(pointer) | (u64::from(length) << 32)
    }
}
//#endregion 📦️OwnedAbi
