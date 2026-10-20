//! 🚀️ Headless batch engine: the job constructor, the batch run, the batch grant and the owned close driver.

use super::*;

/// ⚡️ Headless BEM simulation engine.
pub struct Engine;

impl Engine {
    /// ⚡️ Create the persistent operation used by interactive hosts and the batch adapter.
    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn job(model: Model, config: SimulationConfig) -> Result<EnergyJob, EnergyAdmissionRejected> {
        EnergyJob::new(Operation::new(allocate_operation_id(), RevisionId(0), Generation(0), 0), model, config)
    }


}

/// 🎟️ The bounded five-currency authority the batch adapter supplies to every admitted step.
pub const ENERGY_BATCH_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 2 << 20, maximum_depth: 128 };
