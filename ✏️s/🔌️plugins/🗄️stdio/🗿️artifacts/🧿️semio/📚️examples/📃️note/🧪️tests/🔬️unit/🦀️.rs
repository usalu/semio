#[allow(unused_imports)]
use semio_framework_plugin::ExampleSource;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;

use crate::examples_note::*;
#[semio_framework_async_macros::async_test]
async fn note_source_nonempty() {
    assert!(!PRIMARY_TEXT.is_empty());
    let _ = source();
}
