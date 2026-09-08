
use super::*;

#[semio_framework_async_macros::async_test]
async fn home_command_op_text_round_trips_every_variant() {
    use crate::editor::home::HomeCommand;
    store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::NavigateVirtualFileSystemNode(NavigateVirtualFileSystemNode { node_id: "studio:s1".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::DeleteVirtualFileSystemNode(crate::editor::home::commands::delete_virtual_file_system_node::DeleteVirtualFileSystemNode { node_id: "studio:s1".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::GoHome(crate::editor::home::commands::go_home::GoHome {}));
}
