use crate::viewer::sourcing::modes::view::windows::pool;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::ModeDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::WindowLayout;
#[allow(unused_imports)]
use semio_framework_plugin::WindowLayoutAxisNode;
#[allow(unused_imports)]
use semio_framework_plugin::WindowLayoutChild;
#[allow(unused_imports)]
use semio_framework_plugin::WindowLayoutRoot;
#[allow(unused_imports)]
use semio_framework_plugin::WindowLayoutStackNode;
#[allow(unused_imports)]
use semio_framework_plugin::WindowLayoutWindowNode;

use crate::viewer::sourcing::modes::view::*;

#[semio_framework_async_macros::async_test]
async fn the_layout_references_the_pool_window() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(pool::WINDOW_KIND_ID), "layout must reference window kind {}: {json}", pool::WINDOW_KIND_ID);
}
