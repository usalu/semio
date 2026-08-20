//! 🧬️ GIF 87a → 89a dialect migration — this session's D4 "Tier 2" (snapshot-type-changing)
//! evolution pilot: 87a's `images: Vec<GifImage>` becomes genuine 89a `frames: Vec<GifFrame>`.
//! Lossless: since this ticket's F3 mop-up rewrite made BOTH standards store palette indices (not
//! decoded RGBA), the migration is now a near-direct field carry-over per image/frame — no pixel
//! re-quantization happens here at all, unlike the prior (pre-rewrite) migration which round-
//! tripped through already-RGBA-expanded `RasterImage` bytes. Standalone leaf — deliberately NOT
//! wired into `ArtifactStore::dispatch`, the hub, or WIT `migrate-artifact` this pass (see
//! `26/08/10` `ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION` ticket's D4 evolution
//! slice scope note); `register()` below wires it into `store`'s dialect-migration registry purely
//! to prove that registry works end-to-end on a real case.

// 🔀️ S-6: `crate::artifacts::gif::schema`/`GifSnapshot` now shim to 89a (canonical) -- this
// migration explicitly names both standards' own local snapshot types instead.
use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::{GifColorTable as Gif87aColorTable, GifImage, GifSnapshot as Gif87aSnapshot};
use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::{GifColorTable as Gif89aColorTable, GifDisposal, GifFrame, GifRgb as Gif89aRgb, GifSnapshot as Gif89aSnapshot, STDIO_GIF89A_DOCUMENT_SCHEMA};

//#region ColorTableConv
/// 🔀️ 87a and 89a deliberately declare distinct `GifColorTable` types (per the recipe's "no
/// copy-pasted shared types" rule) — this migration is the one legitimate cross-standard bridge
/// point, converting field-for-field (identical shape: `sorted: bool`, `colors: Vec<GifRgb>`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn migrate_color_table(table: &Gif87aColorTable) -> Gif89aColorTable {
    Gif89aColorTable { sorted: table.sorted, colors: table.colors.iter().map(|c| Gif89aRgb { r: c.r, g: c.g, b: c.b }).collect() }
}
//#endregion ColorTableConv

//#region Migrate
/// 🔁️ Each 87a `GifImage` becomes one 89a `GifFrame`, in order: `left`/`top`/`width`/`height`/
/// `interlace`/`lct`/`indices` carry straight over (both standards use the identical shape for
/// these fields now), with GIF89a-only fields defaulted honestly — `delay_cs: 0`,
/// `disposal: Unspecified`, `transparent_index: None` (87a's image model has no transparency
/// concept at all), `user_input: false`, `plain_text: None`. Screen-level `width`/`height`/`gct`/
/// `background_color_index`/`pixel_aspect_ratio` carry straight over too; `loop_count: None` (87a
/// has no looping concept, so "absent" — not "loop forever" — is the honest translation);
/// `comments`/`app_extensions` are empty (87a has neither extension block kind, GIF89a-only
/// features).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn migrate_87a_to_89a(snapshot_87a: &Gif87aSnapshot) -> Gif89aSnapshot {
    Gif89aSnapshot {
        schema: STDIO_GIF89A_DOCUMENT_SCHEMA.into(),
        width: snapshot_87a.width,
        height: snapshot_87a.height,
        gct: snapshot_87a.gct.as_ref().map(migrate_color_table),
        background_color_index: snapshot_87a.background_color_index,
        pixel_aspect_ratio: snapshot_87a.pixel_aspect_ratio,
        loop_count: None,
        frames: snapshot_87a.images.iter().map(migrate_image_to_frame).collect(),
        comments: Vec::new(),
        app_extensions: Vec::new(),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn migrate_image_to_frame(image: &GifImage) -> GifFrame {
    GifFrame {
        left: image.left,
        top: image.top,
        width: image.width,
        height: image.height,
        interlace: image.interlace,
        lct: image.lct.as_ref().map(migrate_color_table),
        indices: image.indices.clone(),
        delay_cs: 0,
        disposal: GifDisposal::default(),
        transparent_index: None,
        user_input: false,
        plain_text: None,
    }
}
//#endregion Migrate

//#region Registration
/// 🧳️ Pack-bytes bridge for `store::DialectMigration.migrate_pack`'s `fn(&[u8]) ->
/// Result<Vec<u8>, String>` shape: decodes 87a pack bytes to a snapshot, migrates, re-encodes as
/// 89a pack bytes. A bare non-capturing `fn`, coercible to the registry's `fn` pointer field.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn migrate_87a_to_89a_pack(pack_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let snapshot_87a = <Gif87aSnapshot as store::ArtifactPack>::decode_pack(pack_bytes).map_err(|error| error.to_string())?;
    let snapshot_89a = migrate_87a_to_89a(&snapshot_87a);
    Ok(<Gif89aSnapshot as store::ArtifactPack>::encode_pack(&snapshot_89a))
}

/// 📝️ Wires `migrate_87a_to_89a_pack` into `store`'s dialect-migration registry (see
/// `store::register_dialect_migration`, `26/08/10` D4 evolution slice) — call once at
/// program-init time, mirroring every other `register_*` call in this codebase's init path. Not
/// yet called from any real init path (no dispatch/hub/WIT wiring exists this pass — see this
/// module's own doc comment); exercised directly by this module's own test below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {
    let _ = store::register_dialect_migration(store::DialectMigration {
        from: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.gif".into(), standard: "87a".into(), subset: "*".into() },
        to: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.gif".into(), standard: "89a".into(), subset: "*".into() },
        lossless: true,
        migrate_pack: migrate_87a_to_89a_pack,
    });
}
//#endregion Registration

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🏗️ Builds a REAL 87a snapshot by round-tripping through the actual 87a encoder/decoder
    /// (`standards::v87a::engine`), not a hand-built struct literal — so this test exercises the
    /// genuine on-disk GIF87a byte shape.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn real_87a_snapshot() -> Gif87aSnapshot {
        let (palette, indices, _) = crate::artifacts::gif::standards::v87a::engine::quantize_rgba(&sample_rgba_2x2()).expect("quantize");
        let source = Gif87aSnapshot {
            schema: crate::artifacts::gif::STDIO_GIF_DOCUMENT_SCHEMA.into(),
            width: 2,
            height: 2,
            gct: None,
            background_color_index: 0,
            pixel_aspect_ratio: 0,
            images: vec![GifImage { left: 0, top: 0, width: 2, height: 2, interlace: false, lct: Some(crate::artifacts::gif::standards::v87a::engine::color_table_from_bytes(palette, false)), indices }],
        };
        let encoded = crate::artifacts::gif::standards::v87a::engine::encode_gif(&source).expect("real 87a encode of a small opaque image must succeed");
        assert_eq!(&encoded[0..6], b"GIF87a", "sanity: this really is a GIF87a byte stream");
        crate::artifacts::gif::standards::v87a::engine::decode_gif(&encoded).expect("real 87a decode of its own encoded bytes must succeed")
    }

    /// 🎨️ A real 2x2, 4-distinct-opaque-color RGBA image — small enough to hand-inspect, varied
    /// enough to catch a migration that scrambles pixel order.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_rgba_2x2() -> Vec<u8> {
        vec![
            255, 0, 0, 255, // top-left: opaque red
            0, 255, 0, 255, // top-right: opaque green
            0, 0, 255, 255, // bottom-left: opaque blue
            255, 255, 0, 255, // bottom-right: opaque yellow
        ]
    }

    #[semio_framework_async_macros::async_test]
    async fn migrate_87a_to_89a_preserves_indices_in_a_single_frame() {
        let snapshot_87a = real_87a_snapshot();
        let snapshot_89a = migrate_87a_to_89a(&snapshot_87a);

        assert_eq!(snapshot_89a.width, snapshot_87a.width);
        assert_eq!(snapshot_89a.height, snapshot_87a.height);
        assert_eq!(snapshot_89a.loop_count, None, "87a has no loop concept — must translate to absent, not loop-forever");
        assert_eq!(snapshot_89a.frames.len(), 1, "one image becomes exactly one frame");

        let frame = &snapshot_89a.frames[0];
        let image = &snapshot_87a.images[0];
        assert_eq!(frame.left, image.left);
        assert_eq!(frame.top, image.top);
        assert_eq!(frame.width, image.width);
        assert_eq!(frame.height, image.height);
        assert_eq!(frame.lct, image.lct.as_ref().map(migrate_color_table));
        assert_eq!(frame.indices, image.indices, "every palette index must survive migration byte-identically — this IS the lossless claim");
        assert_eq!(frame.delay_cs, 0);
        assert_eq!(frame.disposal, GifDisposal::Unspecified);
        assert_eq!(frame.transparent_index, None);
        assert!(!frame.user_input);
        assert_eq!(frame.rgba(snapshot_89a.gct.as_ref()), image.rgba(snapshot_87a.gct.as_ref()), "derived RGBA must match too");
    }

    #[semio_framework_async_macros::async_test]
    async fn migrate_87a_to_89a_round_trips_through_real_89a_codec() {
        // 🔁️ Not just the in-memory struct — the migrated snapshot must also be a real, valid 89a
        // document: encode it with the real 89a encoder and decode it back.
        let snapshot_87a = real_87a_snapshot();
        let snapshot_89a = migrate_87a_to_89a(&snapshot_87a);

        let encoded_89a = crate::artifacts::gif::standards::v89a::engine::encode_gif(&snapshot_89a).expect("real 89a encode of the migrated snapshot must succeed");
        assert_eq!(&encoded_89a[0..6], b"GIF89a", "sanity: this really is a GIF89a byte stream");
        let redecoded_89a = crate::artifacts::gif::standards::v89a::engine::decode_gif(&encoded_89a).expect("real 89a decode of its own encoded bytes must succeed");
        assert_eq!(redecoded_89a.frames.len(), 1);
        assert_eq!(redecoded_89a.frames[0].indices, snapshot_87a.images[0].indices, "indices must still be identical after a real 89a encode/decode round trip");
    }

    #[semio_framework_async_macros::async_test]
    async fn registered_migration_runs_end_to_end_through_the_store_registry() {
        register();
        let from = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.gif".into(), standard: "87a".into(), subset: "*".into() };
        let to = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.gif".into(), standard: "89a".into(), subset: "*".into() };

        let snapshot_87a = real_87a_snapshot();
        let pack_87a = <Gif87aSnapshot as store::ArtifactPack>::encode_pack(&snapshot_87a);

        let pack_89a = store::migrate_document(&from, &to, &pack_87a).expect("a registered (87a -> 89a) migration must be found and must succeed");
        let snapshot_89a = <Gif89aSnapshot as store::ArtifactPack>::decode_pack(&pack_89a).expect("migrated pack bytes must decode as a real 89a snapshot");

        assert_eq!(snapshot_89a.frames.len(), 1);
        assert_eq!(snapshot_89a.frames[0].indices, snapshot_87a.images[0].indices, "indices must be byte-identical end-to-end through the registry");

        let unregistered_to = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.gif".into(), standard: "99z".into(), subset: "*".into() };
        assert!(store::migrate_document(&from, &unregistered_to, &pack_87a).is_err(), "an unregistered (from, to) pair must return a clear Err, not panic or silently succeed");
    }
}
//#endregion Tests
