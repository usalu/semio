/** 🔺️ Mirrors Rust `DrawingDiff` (sparse field delta over the drawing artifact; sibling `🦀️.rs`,
 * `#[serde(rename_all = "camelCase", default)]`). Every top-level field is an optional patch slot;
 * a Rust `Option<Option<T>>` field (touched-but-cleared vs untouched) collapses to `T | null`
 * here, since JSON already conflates "absent key" with "not present" once serialized sparsely (same
 * `T | null` collapse as `manifestId`/`rootNodeId` on the trinity/jack artifact's own `JackDiff`).
 * Nested types
 * re-import the artifact's own root schema (`../🟦️.ts`) rather than re-declaring stubs, so
 * every facet of the drawing artifact agrees on the same `DrawingLayerNode`/`DrawingImageAsset`/
 * `DrawingArtboard`/`DrawingArtifact`. */
import type { DrawingArtifact, DrawingLayerNode, DrawingImageAsset, DrawingArtboard } from "../🟦️.ts";

export interface DrawingDiff {
  /** @state artifact */
  artifact?: DrawingArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  id?: string;
  /** @state artifact */
  title?: string | null;
  /** @state artifact */
  layers?: DrawingLayersDelta;
  /** @state artifact */
  assets?: DrawingAssetsDelta;
  /** @state artifact */
  artboard?: DrawingArtboard | null;
  /** @state presence */
  selectedIds?: DrawingStringList;
  /** @state presence */
  activeUtilityId?: string;
  /** @state config */
  engagementInput?: string;
  /** @state config */
  cameraX?: number;
  /** @state config */
  cameraY?: number;
  /** @state config */
  cameraZoom?: number;
  /** @state config */
  locale?: string;
  /** @state artifact */
  hoveredId?: string | null;
}

/** 🗂️ Mirrors Rust `DrawingAssetsDelta` — asset-map wrapper so optional map diffs stay scalar across
 * formats; `null` marks a removed asset. */
export interface DrawingAssetsDelta {
  entries: Record<string, DrawingImageAsset | null>;
}

/** 📋 Mirrors Rust `DrawingStringList` — string-list wrapper so optional list diffs stay scalar
 * across formats. */
export interface DrawingStringList {
  values: string[];
}

/** 🧩 Mirrors Rust `DrawingLayersDelta` — identified-collection delta for `layers`. */
export interface DrawingLayersDelta {
  added: DrawingLayerAddition[];
  removed: string[];
  patched: DrawingLayerPatchEntry[];
  reordered?: string[];
}

/** ➕️ Mirrors Rust `DrawingLayerAddition` — one inserted layer with its real (parent, index)
 * target location. */
export interface DrawingLayerAddition {
  parentId?: string;
  index: number;
  layer: DrawingLayerNode;
}

/** 🩹 Mirrors Rust `DrawingLayerPatchEntry` — one patched layer entry. */
export interface DrawingLayerPatchEntry {
  id: string;
  patch: DrawingLayerPatch;
}

/** 🩹 Mirrors Rust `DrawingLayerPatch` — sparse layer field patch (JSON blobs for complex nested
 * values: transform/fill/stroke/trace params are re-serialized rather than typed directly, matching
 * the Rust struct's own `*_json: Option<String>` fields). */
export interface DrawingLayerPatch {
  visible?: boolean;
  locked?: boolean;
  name?: string;
  opacity?: number;
  blendMode?: string;
  transformJson?: string;
  fillJson?: string;
  strokeJson?: string;
  booleanOperation?: string;
  traceParamsJson?: string;
  layerJson?: string;
}
