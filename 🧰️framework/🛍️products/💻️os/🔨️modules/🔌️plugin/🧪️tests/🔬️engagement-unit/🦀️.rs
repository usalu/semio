mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn strip_engagement_prefix_accepts_normalized_and_raw_forms() {
        assert_eq!(strip_engagement_prefix("Fill20", "fill"), Some("20"));
        assert_eq!(strip_engagement_prefix("fill 20", "fill"), Some("20"));
        assert_eq!(strip_engagement_prefix("fill  20", "fill"), Some("20"));
        assert_eq!(strip_engagement_prefix("Fill", "fill"), Some(""));
        assert_eq!(strip_engagement_prefix("FILL20", "fill"), Some("20"));
    }

    #[semio_framework_async_macros::async_test]
    async fn strip_engagement_prefix_preserves_decimal_points() {
        assert_eq!(strip_engagement_prefix("SetHeight3.5", "set height"), Some("3.5"));
        assert_eq!(strip_engagement_prefix("set height 3.5", "set height"), Some("3.5"));
    }

    #[semio_framework_async_macros::async_test]
    async fn strip_engagement_prefix_rejects_non_matching_commands() {
        assert_eq!(strip_engagement_prefix("Brush", "fill"), None);
        assert_eq!(strip_engagement_prefix("Filled", "fill"), Some("ed"));
    }

    #[semio_framework_async_macros::async_test]
    async fn engagement_token_matches_full_token_only() {
        assert!(engagement_token_matches("LineNumbers", "line numbers"));
        assert!(engagement_token_matches("linenumbers", "line numbers"));
        assert!(!engagement_token_matches("LineNumbers2", "line numbers"));
        assert!(!engagement_token_matches("Line", "line numbers"));
    }
}
