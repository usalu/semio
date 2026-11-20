//! 📒️ The `norm-energy` plugin component: the one cdylib of `semio-hub-norm-energy`, holding only its entry point.

/// 🌡️ Funds the retained climate-table constructor within the original caller's maintenance grant.
const MOUNTED_POLICY: semio_plugin::MountedOwnerPolicyV1 = {
    let mut policy = semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
    policy.maintenance.maximum_copy_bytes = 256 * 1024;
    policy
};

semio_plugin::plugin_exports!(MOUNTED_POLICY, semio_hub_norm_energy::plugin, semio_hub_norm_energy::NormEnergyApps);
