//! 🧪️ Native Energy snapshot laws — state rows, entity lookup, original projection, Pack, genesis and the explicit reference — kept in a dedicated test crate so the snapshot crate stays inside its source boundary.
#![allow(clippy::too_many_arguments)]
#![allow(unused_imports)]
extern crate protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
extern crate semio_s_plugin_energy_engine_air_exchange as air_exchange;
extern crate semio_s_plugin_energy_engine_calendar as calendar;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_schedule as schedule;

pub use diff::original as native_diff;
pub use snapshot::reference::{energy_model_from_structure, energy_structure_from_model};

#[global_allocator]
static HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[path = "."]
pub mod state {
    pub use crate::snapshot::state::*;
    use crate::model::*;
    use crate::schedule::ScheduleId;
    use semio_framework_value::shared_issuance::SealedShared;
    use semio_framework_value_persistent::list::persistent::PersistentList;
    #[path = "../🧳️state/🧪️tests/🦀️.rs"]
    pub(crate) mod tests;
    #[path = "."]
    pub mod fields {
        pub use crate::snapshot::state::fields::*;
        use protocol::{RetainedFieldPreparation, retained_mutation_demand_fits};
        use std::mem::{ManuallyDrop, size_of};
        #[path = "../🧳️state/🪡️fields/🧪️tests/🦀️.rs"]
        mod tests;
        #[path = "."]
        pub mod key {
            pub use crate::snapshot::state::fields::*;
            use semio_framework_value_persistent::list::persistent::{PersistentList, PersistentListRead, PersistentListReadStep};
            use protocol::retained_mutation_demand_fits;
            use std::mem::size_of;
            #[path = "../🧳️state/🪡️fields/🔎️key/🧪️tests/🦀️.rs"]
            mod tests;
        }
    }
    #[path = "."]
    pub mod read {
        pub use crate::snapshot::state::read::*;
        use super::*;
        #[path = "../🧳️state/👁️read/🧪️tests/🦀️.rs"]
        mod tests;
    }
}

#[path = "."]
pub mod original {
    pub use crate::snapshot::original::*;
    #[path = "../🧬️original/🧪️tests/🦀️.rs"]
    mod tests;
    #[path = "."]
    pub mod pack {
        pub use crate::snapshot::original::pack::*;
        use super::EnergyModelSnapshot;
        use semio_framework_dsl_record::{BorrowedDslRecord, BorrowedFieldSpec, BorrowedRecordSpec, BorrowedShape, RecordLayout, native_encoding::{FieldProjectionSource, FieldProjectionView as View}};
        use semio_framework_pack_json::{ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonText as Text, ArtifactCanonicalJsonTree as Tree};
        use semio_framework_value::{Number, ValueError, ValueRefusalKind};
        #[path = "../🧬️original/📦️pack/🧪️tests/🦀️.rs"]
        mod tests;
    }
    #[path = "."]
    pub mod genesis {
        pub use crate::snapshot::original::genesis::*;
        use super::EnergyModelSnapshot;
        use crate::state::*;
        use semio_framework_value_persistent::list::persistent::PersistentList;
        use std::mem::{ManuallyDrop, size_of};
        #[path = "../🧬️original/🌱️genesis/🧪️tests/🦀️.rs"]
        mod tests;
    }
}

#[path = "."]
pub mod reference {
    pub use crate::snapshot::reference::*;
    use crate::model::{EntityId, Model};
    use crate::snapshot::reference::EnergyModelSnapshot;
    use semio_framework_value::{DslValue, FromValue, ToValue, ValueError};
    #[path = "🔬️round-trip/🦀️.rs"]
    mod round_trip_tests;
    #[path = "🔬️composition/🦀️.rs"]
    mod composition_tests;
}
