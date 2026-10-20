//! 🎟️ The real launcher supplies independent startup and host-operation stock before native execution.
use crate::native_caller::{NativeFrameCaller, NativeFrameRootPolicyV1};
use semio_framework_job::Generation;
use semio_framework_value::{ValueError, ValueRefusalKind};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct NativeLaunchDeclarationV1<'a> {
    version: u8,
    #[serde(borrow)]
    owner: &'a str,
    startup: NativeFrameRootPolicyV1,
    host_io: NativeFrameRootPolicyV1,
    compiler_task: NativeFrameRootPolicyV1,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct NativeLaunchAdmissionV1<'a> {
    version: u8,
    #[serde(borrow)]
    declaration: NativeLaunchDeclarationV1<'a>,
    #[serde(borrow)]
    script_owner: &'a str,
    #[serde(deserialize_with = "required_deadline")]
    script_deadline_epoch_milliseconds: Option<u64>,
}

fn required_deadline<'de,D:serde::Deserializer<'de>>(decoder:D)->Result<Option<u64>,D::Error>{<Option<u64> as serde::Deserialize>::deserialize(decoder)}

#[derive(Clone, Copy,PartialEq,Eq)]
pub struct NativeLaunchPolicy {
    startup: NativeFrameRootPolicyV1,
    host_io: NativeFrameRootPolicyV1,
    compiler_task: NativeFrameRootPolicyV1,
    script_deadline_epoch_milliseconds: Option<u64>,
}

static NATIVE_LAUNCH_POLICY: std::sync::OnceLock<NativeLaunchPolicy> = std::sync::OnceLock::new();

fn refused(detail: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, detail)
}

impl NativeLaunchPolicy {
    fn read(wire: &str) -> Result<Self, ValueError> {
        let admission: NativeLaunchAdmissionV1<'_> = serde_json::from_str(wire)
            .map_err(|_| refused("complete native launch admission required"))?;
        let declaration = admission.declaration;
        if admission.version != 1 || declaration.version != 1
            || declaration.owner.is_empty() || declaration.owner.chars().count() > 128
            || admission.script_owner.is_empty() || admission.script_owner.chars().count() > 128
            || admission.script_deadline_epoch_milliseconds.is_some_and(|value| value > 9_007_199_254_740_991)
        { return Err(refused("native launch owner, version or original deadline refused")); }
        for policy in [declaration.startup, declaration.host_io, declaration.compiler_task] {
            policy.validate()?;
            let grant = policy.grant;
            let reserve = policy.retirement_reserve;
            if [policy.fuel, policy.maximum_elapsed_microseconds, reserve.fuel].iter().any(|value| *value > 9_007_199_254_740_991)
                || [grant.maximum_items, grant.maximum_copy_bytes, grant.maximum_capacity_bytes, grant.maximum_release_bytes, grant.maximum_depth,
                    reserve.grant.maximum_items, reserve.grant.maximum_copy_bytes, reserve.grant.maximum_capacity_bytes,
                    reserve.grant.maximum_release_bytes, reserve.grant.maximum_depth].iter().any(|value| *value as u128 > 9_007_199_254_740_991)
            { return Err(refused("native launch independent currency exceeds schema range")); }
        }
        Ok(Self { startup: declaration.startup, host_io: declaration.host_io, compiler_task:declaration.compiler_task, script_deadline_epoch_milliseconds: admission.script_deadline_epoch_milliseconds })
    }

    fn original_deadline(self, policy: NativeFrameRootPolicyV1) -> Result<u64, ValueError> {
        let original_now = semio_framework_job::default_now_us()
            .ok_or_else(|| refused("native launch original source clock refused"))?;
        let mut duration = policy.maximum_elapsed_microseconds;
        if let Some(deadline) = self.script_deadline_epoch_milliseconds {
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
                .map_err(|_| refused("native launch original wall clock refused"))?.as_micros();
            let remaining = (deadline as u128).checked_mul(1000).and_then(|deadline| deadline.checked_sub(now)).filter(|remaining| *remaining != 0)
                .ok_or_else(|| ValueError::literal(ValueRefusalKind::WorkLimit, "native launch original Script deadline exhausted"))?;
            let microseconds = u64::try_from(remaining).ok()
                .ok_or_else(|| refused("native launch duration range refused"))?;
            duration = duration.min(microseconds);
        }
        original_now.checked_add(duration).ok_or_else(|| refused("native launch original deadline overflow"))
    }

    pub fn caller(self, startup: bool) -> Result<NativeFrameCaller, ValueError> {
        let policy = if startup { self.startup } else { self.host_io };
        let deadline = self.original_deadline(policy)?;
        NativeFrameCaller::from_original_deadline(semio_framework_trace::allocate_operation_id(), Generation(1),
            semio_framework_job::default_now_us, policy, deadline)
    }
    fn compiler_caller(self)->Result<NativeFrameCaller,ValueError>{let deadline=self.original_deadline(self.compiler_task)?;NativeFrameCaller::from_original_deadline(semio_framework_trace::allocate_operation_id(),Generation(1),semio_framework_job::default_now_us,self.compiler_task,deadline)}
}

/// 🚪️ Missing launch metadata is refused instead of borrowing later UI-event stock.
pub fn install_native_launch_admission() -> Result<(), ValueError> {
    let wire = std::env::var("SEMIO_NATIVE_LAUNCH_ADMISSION")
        .map_err(|_| refused("native entrypoint requires its repository launch admission"))?;
    let policy = NativeLaunchPolicy::read(&wire)?;
    policy.original_deadline(policy.startup)?;
    policy.original_deadline(policy.host_io)?;
    policy.original_deadline(policy.compiler_task)?;
    NATIVE_LAUNCH_POLICY.set(policy).map_err(|_| refused("native launch admission already installed"))
}

/// 📨️ Only an actual submitted host operation obtains its declared immutable lifetime.
pub fn native_launch_caller(startup: bool) -> Result<NativeFrameCaller, ValueError> {
    NATIVE_LAUNCH_POLICY.get().copied().ok_or_else(|| refused("native launch admission is not installed"))?.caller(startup)
}

/// 🧩️ Actual CreateApp submission issues one finite compiler source from its independent required launch declaration.
pub fn native_compiler_task_caller()->Result<NativeFrameCaller,ValueError>{NATIVE_LAUNCH_POLICY.get().copied().ok_or_else(||refused("native launch admission is not installed"))?.compiler_caller()}

/// 🧪️ A native fixture installs its explicit complete launch wire once before any real invocation, never replacing live process policy.
#[cfg(any(test, feature = "testkit"))]
pub fn install_native_launch_fixture(wire:&str)->Result<(),ValueError>{
    let policy=NativeLaunchPolicy::read(wire)?;
    match NATIVE_LAUNCH_POLICY.set(policy){Ok(())=>Ok(()),Err(policy)=>if NATIVE_LAUNCH_POLICY.get()==Some(&policy){Ok(())}else{Err(refused("native fixture cannot replace an installed original launch admission"))}}
}
