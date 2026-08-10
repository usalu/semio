//! 🧬️ GIF 87a → 89a dialect migration — this session's D4 "Tier 2" (snapshot-type-changing)
//! evolution pilot: a single static 87a image becomes a genuine one-frame 89a animation.
//! Lossless: every 87a pixel value (already RGBA-expanded in `RasterImage.rgba` by the 87a
//! decoder/encoder's shared palette machinery) survives byte-identical into the 89a frame's
//! `rgba`. Standalone leaf — deliberately NOT wired into `ArtifactStore::dispatch`, the hub, or
//! WIT `migrate-artifact` this pass (see `26/08/10`
//! `ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION` ticket's D4 evolution slice
//! scope note); `register()` below wires it into `store`'s new dialect-migration registry purely
//! to prove that registry works end-to-end on a real case.

// 🔀️ S-6: `crate::artifacts::gif::schema`/`GifSnapshot` now shim to 89a (canonical) -- this
// migration explicitly names both standards' own local snapshot types instead.
use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::GifSnapshot as Gif87aSnapshot;
use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::{GifDisposal, GifFrame, GifSnapshot as Gif89aSnapshot, STDIO_GIF89A_DOCUMENT_SCHEMA};

//#region Migrate
/// 🔁️ 87a's `RasterImage` (a single already RGBA-expanded still image, no animation/GCE concept)
/// into 89a's `GifSnapshot` (multi-frame-capable): one frame covering the full logical screen at
/// `(0, 0)`, no loop extension (`loop_count: None` — 87a has no looping concept either, so
/// "absent" rather than "loop forever" is the honest translation), a `0`cs delay and
/// `Unspecified` disposal (GIF89a §23's defaults for a frame with no preceding GCE — 87a never
/// had one), and no transparency (87a's `RasterImage` carries no alpha channel of its own; every
/// pixel is opaque, matching 87a's actual display semantics exactly).
pub fn migrate_87a_to_89a(snapshot_87a: &Gif87aSnapshot) -> Gif89aSnapshot {
    let width = snapshot_87a.image.width;
    let height = snapshot_87a.image.height;
    Gif89aSnapshot {
        schema: STDIO_GIF89A_DOCUMENT_SCHEMA.into(),
        width,
        height,
        loop_count: None,
        frames: vec![GifFrame {
            left: 0,
            top: 0,
            width,
            height,
            rgba: snapshot_87a.image.rgba.clone(),
            delay_cs: 0,
            disposal: GifDisposal::default(),
            transparent: false,
            user_input: false,
        }],
    }
}
//#endregion Migrate

//#region Registration
/// 🧳️ Pack-bytes bridge for `store::DialectMigration.migrate_pack`'s `fn(&[u8]) ->
/// Result<Vec<u8>, String>` shape: decodes 87a pack bytes to a snapshot, migrates, re-encodes as
/// 89a pack bytes. A bare non-capturing `fn`, coercible to the registry's `fn` pointer field.
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
pub fn register() {
    store::register_dialect_migration(store::DialectMigration {
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
    use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::RasterImage;

    /// 🎨️ A real 2x2, 4-distinct-opaque-color RGBA image — small enough to hand-inspect,
    /// varied enough (four different colors, one per pixel) to catch a migration that scrambles
    /// pixel order or drops a channel.
    fn sample_rgba_2x2() -> Vec<u8> {
        vec![
            255, 0, 0, 255, // top-left: opaque red
            0, 255, 0, 255, // top-right: opaque green
            0, 0, 255, 255, // bottom-left: opaque blue
            255, 255, 0, 255, // bottom-right: opaque yellow
        ]
    }

    /// 🏗️ Builds a REAL 87a snapshot by round-tripping through the actual 87a encoder/decoder
    /// (`standards::v87a::engine`), not a hand-built struct literal — so this test exercises the
    /// genuine on-disk GIF87a byte shape, matching the ticket's "via the gif artifact's own real
    /// analyzer/decoder" instruction.
    fn real_87a_snapshot() -> Gif87aSnapshot {
        let source = Gif87aSnapshot { schema: crate::artifacts::gif::STDIO_GIF_DOCUMENT_SCHEMA.into(), image: RasterImage { width: 2, height: 2, rgba: sample_rgba_2x2() } };
        let encoded = crate::artifacts::gif::standards::v87a::engine::encode_gif(&source).expect("real 87a encode of a small opaque image must succeed");
        assert_eq!(&encoded[0..6], b"GIF87a", "sanity: this really is a GIF87a byte stream");
        crate::artifacts::gif::standards::v87a::engine::decode_gif(&encoded).expect("real 87a decode of its own encoded bytes must succeed")
    }

    #[test]
    fn migrate_87a_to_89a_preserves_pixels_byte_identically_in_a_single_frame() {
        let snapshot_87a = real_87a_snapshot();
        let snapshot_89a = migrate_87a_to_89a(&snapshot_87a);

        assert_eq!(snapshot_89a.width, snapshot_87a.image.width);
        assert_eq!(snapshot_89a.height, snapshot_87a.image.height);
        assert_eq!(snapshot_89a.loop_count, None, "87a has no loop concept — must translate to absent, not loop-forever");
        assert_eq!(snapshot_89a.frames.len(), 1, "a static image becomes exactly one frame");

        let frame = &snapshot_89a.frames[0];
        assert_eq!(frame.left, 0);
        assert_eq!(frame.top, 0);
        assert_eq!(frame.width, snapshot_87a.image.width);
        assert_eq!(frame.height, snapshot_87a.image.height);
        assert_eq!(frame.rgba, snapshot_87a.image.rgba, "every 87a pixel byte must survive migration byte-identically — this IS the lossless claim");
        assert_eq!(frame.delay_cs, 0);
        assert_eq!(frame.disposal, GifDisposal::Unspecified);
        assert!(!frame.transparent);
        assert!(!frame.user_input);
    }

    #[test]
    fn migrate_87a_to_89a_round_trips_through_real_89a_codec() {
        // 🔁️ Not just the in-memory struct — the migrated snapshot must also be a real, valid 89a
        // document: encode it with the real 89a encoder and decode it back.
        let snapshot_87a = real_87a_snapshot();
        let snapshot_89a = migrate_87a_to_89a(&snapshot_87a);

        let encoded_89a = crate::artifacts::gif::standards::v89a::engine::encode_gif(&snapshot_89a).expect("real 89a encode of the migrated snapshot must succeed");
        assert_eq!(&encoded_89a[0..6], b"GIF89a", "sanity: this really is a GIF89a byte stream");
        let redecoded_89a = crate::artifacts::gif::standards::v89a::engine::decode_gif(&encoded_89a).expect("real 89a decode of its own encoded bytes must succeed");
        assert_eq!(redecoded_89a.frames.len(), 1);
        assert_eq!(redecoded_89a.frames[0].rgba, snapshot_87a.image.rgba, "pixels must still be byte-identical after a real 89a encode/decode round trip");
    }

    #[test]
    fn registered_migration_runs_end_to_end_through_the_store_registry() {
        register();
        let from = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.gif".into(), standard: "87a".into(), subset: "*".into() };
        let to = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.gif".into(), standard: "89a".into(), subset: "*".into() };

        let snapshot_87a = real_87a_snapshot();
        let pack_87a = <Gif87aSnapshot as store::ArtifactPack>::encode_pack(&snapshot_87a);

        let pack_89a = store::migrate_document(&from, &to, &pack_87a).expect("a registered (87a -> 89a) migration must be found and must succeed");
        let snapshot_89a = <Gif89aSnapshot as store::ArtifactPack>::decode_pack(&pack_89a).expect("migrated pack bytes must decode as a real 89a snapshot");

        assert_eq!(snapshot_89a.frames.len(), 1);
        assert_eq!(snapshot_89a.frames[0].rgba, snapshot_87a.image.rgba, "pixels must be byte-identical end-to-end through the registry");

        let unregistered_to = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.gif".into(), standard: "99z".into(), subset: "*".into() };
        assert!(store::migrate_document(&from, &unregistered_to, &pack_87a).is_err(), "an unregistered (from, to) pair must return a clear Err, not panic or silently succeed");
    }
}
//#endregion Tests
