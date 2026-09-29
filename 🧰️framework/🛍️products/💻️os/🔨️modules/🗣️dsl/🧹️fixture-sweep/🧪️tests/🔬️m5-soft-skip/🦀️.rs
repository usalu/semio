/// ⏭️ Returns true when the pilot constant/spec text is missing or still a stub.
pub async fn soft_skip_missing(label: &str, text: &str) -> bool {
    let trimmed = text.trim();
    if trimmed.is_empty() || (trimmed.contains("TODO") && trimmed.lines().count() < 4) {
        return true;
    }
    false
}

/// ⏭️ Soft-skip when binary example payload is empty after unwrap.
pub async fn soft_skip_empty_bytes(label: &str, bytes: &[u8]) -> bool {
    if bytes.is_empty() {
        return true;
    }
    false
}
