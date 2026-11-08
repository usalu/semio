//! ♻️ Command-owned original retirement; the generic command and supersede input derive theirs.
use crate::{HistoryLane, OpenToolTransaction};
use semio_framework_value::artifact_retire_struct;
artifact_retire_struct!(OpenToolTransaction { transaction, edit_id });
semio_framework_value::artifact_retire_leaf!(HistoryLane);
