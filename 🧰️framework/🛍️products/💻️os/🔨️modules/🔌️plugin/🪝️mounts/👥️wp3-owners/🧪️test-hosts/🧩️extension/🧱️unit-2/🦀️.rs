//! 🧪️ Unit 2 of the facade test host of `extension`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

use semio_framework::IoEntryDescriptor;
use semio_framework::IoEntryDirection;
use semio_framework::PackageDescriptor;
use semio_framework::PluginManifest;
use semio_framework_os_kernel::io;

#[doc(hidden)]
#[cfg(test)]
#[path = "../../../../../🛂️describe/🧪️tests/🔬️unit/🦀️.rs"]
pub mod tests;
#[cfg(test)]
#[path = "../../../../../🛂️describe/🧪️tests/🔬️example-assets/🦀️.rs"]
mod example_asset_tests;
