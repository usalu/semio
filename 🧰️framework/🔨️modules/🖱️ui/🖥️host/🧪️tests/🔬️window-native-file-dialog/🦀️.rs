
use super::*;

#[test]
fn request_schema_normalizes_extensions_deterministically() {
    assert_eq!(NativeFileDialogRequest::open([".JSON", " json ", "png", ""], true), NativeFileDialogRequest::Open { extensions: vec!["json".into(), "png".into()], multiple: true });
}

#[test]
fn save_and_folder_requests_preserve_owned_values() {
    assert_eq!(NativeFileDialogRequest::save("studio.json", [".JSON"]), NativeFileDialogRequest::Save { filename: "studio.json".into(), extensions: vec!["json".into()] });
    assert_eq!(NativeFileDialogRequest::folder(), NativeFileDialogRequest::Folder);
}

#[test]
fn selection_future_can_move_to_the_io_lane() {
    fn assert_send<T: Send>(_: T) {}
    assert_send(select_native_paths(NativeFileDialogRequest::folder()));
}
