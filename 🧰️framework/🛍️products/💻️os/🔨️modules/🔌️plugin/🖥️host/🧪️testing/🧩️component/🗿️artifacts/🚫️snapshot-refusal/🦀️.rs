//! 🚫️ Fixture-only refusal artifact authority.
pub const KIND:&str="fixture.neutral-host-fixture.snapshot-refusal";
#[path="🧬️schema/📸️snapshot/🦀️.rs"]
mod snapshot;
pub use snapshot::Snapshot;
#[path="🧬️schema/🔺️diff/🦀️.rs"]
mod diff;
pub use diff::Diff;
#[path="🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::Mutation;

#[path = "🚪️io/🦀️.rs"]
pub mod io;

fn facet(role:&str,json_schema:&'static str)->semio_framework_schema_registry::FacetLeaves{
 let(rust,typescript,graphql,proto)=match role{
  "diff"=>(include_str!("🧬️schema/🔺️diff/🦀️.rs"),include_str!("🧬️schema/🔺️diff/🟦️.ts"),"",""),
  "mutations"=>(include_str!("🧬️schema/🧬️mutations/🦀️.rs"),include_str!("🧬️schema/🧬️mutations/🟦️.ts"),"",""),
  _=>(include_str!("🧬️schema/📸️snapshot/🦀️.rs"),include_str!("🧬️schema/📸️snapshot/🟦️.ts"),include_str!("🧬️schema/📸️snapshot/🔗️.graphql"),include_str!("🧬️schema/📸️snapshot/🛰️.proto"))
 };semio_framework_schema_registry::FacetLeaves{rust,typescript,graphql,proto,json_schema}
}

/// 🧬️ The schema descriptor of the refusal artifact: the four facets with the sources this crate embeds.
pub fn schema_descriptor()->semio_framework_schema_registry::ArtifactSchemaDescriptor{
 semio_framework_schema_registry::ArtifactSchemaDescriptor{id:KIND,artifact:facet("artifact",include_str!("🧬️schema/🔣️.json")),snapshot:facet("snapshot",include_str!("🧬️schema/📸️snapshot/🔣️.json")),diff:facet("diff",include_str!("🧬️schema/🔺️diff/🔣️.json")),mutations:facet("mutations",include_str!("🧬️schema/🧬️mutations/🔣️.json"))}
}
