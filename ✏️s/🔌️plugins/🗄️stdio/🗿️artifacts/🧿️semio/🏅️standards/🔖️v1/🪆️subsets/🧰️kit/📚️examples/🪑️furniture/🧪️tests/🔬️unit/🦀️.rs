#[allow(unused_imports)]
use semio_plugin::ExampleSource;
#[allow(unused_imports)]
use semio_ui_locale::LocalizedLabel;
use crate::examples::furniture::*;
#[semio_framework_async_macros::async_test]
async fn furniture_source_constructs() {
    let _ = source();
}
