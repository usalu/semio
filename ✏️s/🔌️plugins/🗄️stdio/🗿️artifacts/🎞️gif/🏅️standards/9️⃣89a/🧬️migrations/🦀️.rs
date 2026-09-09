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

// 🔀️ S-6: `crate::schema`/`GifSnapshot` now shim to 89a (canonical) -- this
// migration explicitly names both standards' own local snapshot types instead.
use crate::standards::v87a::subsets::any::schema::snapshot::{GifColorTable as Gif87aColorTable, GifImage, GifSnapshot as Gif87aSnapshot};
use crate::standards::v89a::subsets::any::schema::snapshot::{GifColorTable as Gif89aColorTable, GifDisposal, GifFrame, GifRgb as Gif89aRgb, GifSnapshot as Gif89aSnapshot, STDIO_GIF89A_DOCUMENT_SCHEMA};

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
    store::register_dialect_migration(store::DialectMigration {
        from: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.gif".into(), standard: "87a".into(), subset: "*".into() },
        to: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.gif".into(), standard: "89a".into(), subset: "*".into() },
        lossless: true,
        migrate_pack: migrate_87a_to_89a_pack,
    })
    .expect("static Stdio registration must be available and conflict-free");
}
//#endregion Registration

//#region Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion Tests
