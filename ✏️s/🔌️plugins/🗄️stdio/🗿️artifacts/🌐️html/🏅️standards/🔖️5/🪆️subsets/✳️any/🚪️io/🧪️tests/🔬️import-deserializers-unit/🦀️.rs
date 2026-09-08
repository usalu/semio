mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn sniffs_a_real_doctype_case_insensitively() {
        assert!(sniff_real_bytes(b"<!DOCTYPE html>\n<html></html>"));
        assert!(sniff_real_bytes(b"  \n<!doctype HTML>"));
    }

    #[semio_framework_async_macros::async_test]
    async fn rejects_non_html() {
        assert!(!sniff_real_bytes(b"just some text"));
    }
}
