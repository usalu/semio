//! 📬️ Energy edits use the original retained mutation, central diff and inverse publication pipeline.
use super::*;
/// 🔋️ Every authored Energy leaf drives the same native bounded preparation factory.
pub(super)type EnergyModelStorePreparationFactory=store::PagedOneItemPreparationFactory<EnergyModelSnapshot,EnergyModelMutation,store::MutationApplyEdit<EnergyModelSnapshot,EnergyModelMutation>>;
