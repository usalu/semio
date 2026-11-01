//! 📚️ IFC-domain frame kinds of the retained machine: header normalization, domain checks, population rule lanes and procedural root reads, plus the local-rule candidate adapter.
#[cfg(test)]
#[global_allocator]
static IFC_ORIGINAL_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

pub use semio_s_artifact_stdio_ifc4x3_machine::machine::{self, *};
pub use semio_s_artifact_stdio_ifc4x3_machine::{admission, ast, ifc4x3_express_program, metadata, overlay, program_index, receive, retained, IFC4X3_NATIVE_EXPRESS_EXECUTION, IFC4X3_NATIVE_EXPRESS_IMPLEMENTATION, IFC4X3_RETAINED_EXPRESS_IMPLEMENTATION, IFC4X3_RETAINED_EXPRESS_RECEIVING_IMPLEMENTATION};
use ast::*;
use overlay::*;
use receive::PopulationReceiver;
use retained::*;
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{
    RetirementDemand, ValueError,
    retained_clone::{RetainedCloneClose, RetainedCloneGrant, RetainedCloneStep},
};
use std::{cmp::Ordering, mem::size_of};
#[cfg(test)]
use semio_s_artifact_stdio_ifc4x3_step as io;

include!("../machine-domain.rs");
include!("../machine-rules.rs");
include!("../machine-root.rs");
#[path = "../local.rs"]
pub mod local;

/// 🧵 Executes the frame kinds the machine core delegates to the IFC domain.
pub fn execute_domain(machine: &mut RetainedMachine, frame: Frame, env: &MachineEnvironment<'_>) -> Result<(), ValueError> {
    match frame.kind {
        Kind::RootRead => root_read(machine, frame, env),
        Kind::Dependency => dependency(machine, frame),
        Kind::PreparedQuery => prepared_query(machine, frame, env),
        Kind::Header => header(machine, frame, env),
        Kind::Normalize => normalize(machine, frame, env),
        Kind::Domain => domain(machine, frame, env),
        Kind::Select => select_domain(machine, frame, env),
        Kind::Population | Kind::TypeRules | Kind::Bound | Kind::InverseRule | Kind::UniqueRule | Kind::Global | Kind::Record | Kind::AggregateUnique => population_rules(machine, frame, env),
        _ => Err(refusal("express.foreign-kind")),
    }
}

/// 🏗️ Constructs the retained machine whose IFC-domain frame kinds this crate executes.
pub trait NewMachine {
    fn new(program: &'static ExpressProgram) -> Self;
}
impl NewMachine for RetainedMachine {
    fn new(program: &'static ExpressProgram) -> Self {
        RetainedMachine::with_foreign(program, execute_domain)
    }
}

#[cfg(test)]
#[path = "../retained-tests.rs"]
mod retained_tests;
#[cfg(test)]
#[path = "../receive-tests.rs"]
mod receive_tests;
#[cfg(test)]
#[path = "../machine-tests.rs"]
mod machine_tests;
#[cfg(test)]
#[path = "../machine-model-tests.rs"]
mod machine_model_tests;
#[cfg(test)]
#[path = "../machine-local-tests.rs"]
mod local_tests;
#[cfg(test)]
#[path = "../machine-custody-tests.rs"]
mod custody_tests;
#[cfg(test)]
#[path = "../machine-aggregate-tests.rs"]
mod aggregate_tests;
#[cfg(test)]
#[path = "../machine-typeof-tests.rs"]
mod typeof_tests;
#[cfg(test)]
#[path = "../machine-root-tests.rs"]
mod root_tests;
