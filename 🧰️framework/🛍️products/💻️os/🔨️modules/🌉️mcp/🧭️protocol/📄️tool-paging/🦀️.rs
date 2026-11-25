//! 📄️ Paging window of the cursor-paged list tools, sharing the opaque cursor `*/list` already mints.

use crate::notify::{decode_cursor, encode_cursor};
use semio_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};

/// 🪟️ The slice of an ordered list one call serves: `offset` rows skipped, at most `size` returned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageWindow {
    pub offset: usize,
    pub size: usize,
}

/// 📄️ Reads `limit` (an integer in `1..=max_size`, else `default_size`) and `cursor` (only one this
/// server minted, else a typed `INPUT_INVALID`) from a list tool's arguments.
pub fn page_window(arguments: &serde_json::Value, default_size: usize, max_size: usize) -> Result<PageWindow, GatewayError> {
    let size = match arguments.get("limit") {
        None | Some(serde_json::Value::Null) => default_size,
        Some(limit) => limit
            .as_u64()
            .filter(|limit| (1..=max_size as u64).contains(limit))
            .map(|limit| limit as usize)
            .ok_or_else(|| GatewayError::new(GatewayErrorCode::InputInvalid, format!("limit must be an integer between 1 and {max_size}")).with_details(serde_json::json!({ "field": "limit", "minimum": 1, "maximum": max_size })))?,
    };
    let offset = match arguments.get("cursor") {
        None | Some(serde_json::Value::Null) => 0,
        Some(cursor) => cursor
            .as_str()
            .and_then(decode_cursor)
            .ok_or_else(|| GatewayError::new(GatewayErrorCode::InputInvalid, "cursor was not minted by this server — pass the `nextCursor` of a previous page unchanged").with_details(serde_json::json!({ "field": "cursor" })))?,
    };
    Ok(PageWindow { offset, size })
}

impl PageWindow {
    /// ➡️ The cursor continuing this window over `total` rows; `None` on the last page.
    pub fn next_cursor(&self, total: usize) -> Option<String> {
        let end = self.offset.saturating_add(self.size);
        (end < total).then(|| encode_cursor(end))
    }
}
