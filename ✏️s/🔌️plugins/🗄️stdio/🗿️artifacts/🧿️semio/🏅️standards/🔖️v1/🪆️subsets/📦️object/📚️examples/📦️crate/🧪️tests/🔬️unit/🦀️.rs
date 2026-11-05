#[allow(unused_imports)]
use semio_plugin::ExampleSource;
#[allow(unused_imports)]
use semio_ui_locale::LocalizedLabel;
use crate::examples::crate_::*;
#[semio_framework_async_macros::async_test]
async fn crate_source_constructs() {
    let _ = source();
}
