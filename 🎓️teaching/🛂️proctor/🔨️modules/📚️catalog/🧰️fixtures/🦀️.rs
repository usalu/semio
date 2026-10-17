//! 🧰️ The catalog file the proctor's own tests play and the catalog loaded from it, shared with the tests of the crates above (`testkit`).

use super::*;
use semio_framework_test_fixtures::fixture_path;

/// 📚️ The catalog file the proctor's own tests play.
pub fn fixture_path() -> PathBuf {
    fixture_path!("../../../../🧪️tests/🧫️fixtures/📚️catalog/🔣️.json")
}

/// 📚️ The fixture catalog, loaded.
pub fn fixture() -> LoadedCatalog {
    load_catalog(&fixture_path()).expect("the fixture catalog is valid")
}
