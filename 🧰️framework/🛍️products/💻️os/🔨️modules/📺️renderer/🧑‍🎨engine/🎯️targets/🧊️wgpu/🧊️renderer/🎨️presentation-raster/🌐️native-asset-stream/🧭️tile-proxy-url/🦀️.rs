//! 🧭️ Resolves native map tile paths against the launcher-owned asset proxy.

/// 🔗️ Rebases a root-relative map path to the exact IPv4 origin issued to the native child.
pub(super) fn native_map_tile_proxy_url(url: &str, asset_base: Option<&str>) -> Option<String> {
    let base = asset_base?;
    let port = base.strip_prefix("http://127.0.0.1:")?;
    if port.starts_with('0') || !port.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    port.parse::<u16>().ok()?;
    if !url.starts_with('/') || url.starts_with("//") || url.bytes().any(|byte| byte.is_ascii_control() || matches!(byte, b' ' | b'\\' | b'#')) {
        return None;
    }
    Some(format!("{base}{url}"))
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
