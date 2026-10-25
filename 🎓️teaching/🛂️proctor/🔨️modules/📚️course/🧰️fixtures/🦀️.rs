//! 🧰️ The course file the proctor's own tests play and the course loaded from it, shared with the tests of the crates above (`testkit`).

use super::*;
use semio_framework_test_fixtures::fixture_path;

/// 📚️ The course file the proctor's own tests play.
pub fn fixture_path() -> PathBuf {
    fixture_path!("../../../../🧪️tests/🧫️fixtures/📚️catalog/🔣️.json")
}

/// 🧫️ The fixture course, loaded.
pub fn fixture() -> LoadedCourse {
    load_course(&fixture_path()).expect("the fixture course is valid")
}
