//! 👁️ The exact published `workingGraph` member, read through the document's captured child view (design §20.15): the member
//! store is the working graph's single truth.
use semio_framework_plugin::{ChildContentView, Fault, FaultCode, FaultOrigin};
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::{SemioGraphSnapshot, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA};

pub(crate) const WORKING_CHILD_SLOT: &str = "workingGraph";

fn refused(message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("trinity.rewriting.child-refused"), message)
}

pub(crate) fn read<'a>(parent: &crate::RewritingSnapshot, children: &'a ChildContentView) -> Result<store::SnapshotReadRef<'a, SemioGraphSnapshot>, Fault> {
    let handle = &parent.working_graph.content;
    let dialect = &handle.target.dialect;
    if dialect.artifact_kind != "s.stdio.semio" || dialect.standard != "v1" || dialect.subset != "graph" {
        return Err(refused("Rewriting requires the exact Semio v1 graph child dialect"));
    }
    let owner = children.typed_read::<SemioGraphSnapshot>(WORKING_CHILD_SLOT, &handle.child_id)?;
    if owner.schema != STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA {
        return Err(refused("Rewriting retained child has a different graph document schema"));
    }
    Ok(owner)
}
