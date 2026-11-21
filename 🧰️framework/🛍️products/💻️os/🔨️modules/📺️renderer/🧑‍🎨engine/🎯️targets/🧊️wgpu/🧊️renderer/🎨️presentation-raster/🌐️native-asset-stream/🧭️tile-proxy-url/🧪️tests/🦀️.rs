//! 🧪️ Native map tile proxy URL conformance.

use super::native_map_tile_proxy_url;

#[test]
fn native_map_tile_paths_use_only_the_launchers_loopback_origin() {
    for row in include_str!("🔣️.tsv").lines().skip(1) {
        let fields: Vec<_> = row.split('\t').collect();
        let [case, base, url, expected] = fields.as_slice() else {
            panic!("invalid native tile URL fixture row: {row}");
        };
        let base = (*base != "-").then_some(*base);
        let expected = (*expected != "-").then_some(*expected);
        assert_eq!(native_map_tile_proxy_url(url, base).as_deref(), expected, "{case}");
    }
}
