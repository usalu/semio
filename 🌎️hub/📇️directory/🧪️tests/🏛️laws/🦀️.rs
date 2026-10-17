//! 🏛️ Directory laws: the writer and decider unit laws and the per-backend artifact-creation laws, run over the real backends.

#[cfg(test)]
use semio_hub_directory::{AdminOperationDirectory, ArtifactCasDirectory, ArtifactCreationDirectory, AuthSessionDirectory, DocumentDirectory, EventLog, InviteDirectory, ProjectionRebuild, ReceiptDirectory, SpaceDirectory, UserDirectory};

#[cfg(test)]

#[path = "../🔬️unit/🦀️.rs"]
mod unit;


#[cfg(test)]
#[path = "../../🐘️postgres/🌱️creation-v1/🧪️tests/🔬️standalone/🦀️.rs"]
mod postgres_creation_tests;

#[cfg(test)]
#[path = "../../🌐️neo4j/🌱️creation-v1/🧪️tests/🔬️standalone/🦀️.rs"]
mod neo4j_creation_tests;

