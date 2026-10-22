//! 🧪️ Unit 1 of the facade test host of `instance-ladders`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_ui_contract::*;
use semio_framework_plugin_decl_app::ArtifactApp;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use std::future::Future;
use semio_framework_plugin_instance_extension::InstanceExtension;
use store::MemberFactory;
use store::Mutation;
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use store::{ReferenceSpaceMember, SpaceMember};

