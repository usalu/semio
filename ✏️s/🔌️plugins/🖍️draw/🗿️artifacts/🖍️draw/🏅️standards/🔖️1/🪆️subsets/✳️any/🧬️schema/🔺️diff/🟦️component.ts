/** 🔺️ Mirrors Rust `DrawDiff` (sparse field delta over the draw artifact; sibling `🦀️component.rs`,
 * `#[serde(rename_all = "camelCase", default)]`). Every top-level field is an optional patch slot;
 * a Rust `Option<Option<T>>` field (touched-but-cleared vs untouched) collapses to `T | null`
 * here, since JSON already conflates "absent key" with "not present" once serialized sparsely (same
 * `T | null` collapse as `manifestId`/`rootNodeId` on the trinity/jack artifact's own `JackDiff`).
 * Nested types
 * re-import the artifact's own root schema (`../🟦️component.ts`) rather than re-declaring stubs, so
 * every facet of the draw artifact agrees on the same `DrawLayerNode`/`DrawImageAsset`/
 * `DrawArtboard`/`DrawArtifact`. */
import type { DrawArtifact, DrawLayerNode, DrawImageAsset, DrawArtboard } from "../🟦️component.ts";

export interface DrawDiff {
  /** @state artifact */
  artifact?: DrawArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  id?: string;
  /** @state artifact */
  title?: string | null;
  /** @state artifact */
  layers?: DrawLayersDelta;
  /** @state artifact */
  assets?: DrawAssetsDelta;
  /** @state artifact */
  artboard?: DrawArtboard | null;
  /** @state presence */
  selectedIds?: DrawStringList;
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

/** 🗂️ Mirrors Rust `DrawAssetsDelta` — asset-map wrapper so optional map diffs stay scalar across
 * formats; `null` marks a removed asset. */
export interface DrawAssetsDelta {
  entries: Record<string, DrawImageAsset | null>;
}

/** 📋 Mirrors Rust `DrawStringList` — string-list wrapper so optional list diffs stay scalar
 * across formats. */
export interface DrawStringList {
  values: string[];
}

/** 🧩 Mirrors Rust `DrawLayersDelta` — identified-collection delta for `layers`. */
export interface DrawLayersDelta {
  added: DrawLayerAddition[];
  removed: string[];
  patched: DrawLayerPatchEntry[];
  reordered?: string[];
}

/** ➕️ Mirrors Rust `DrawLayerAddition` — one inserted layer with its real (parent, index)
 * target location. */
export interface DrawLayerAddition {
  parentId?: string;
  index: number;
  layer: DrawLayerNode;
}

/** 🩹 Mirrors Rust `DrawLayerPatchEntry` — one patched layer entry. */
export interface DrawLayerPatchEntry {
  id: string;
  patch: DrawLayerPatch;
}

/** 🩹 Mirrors Rust `DrawLayerPatch` — sparse layer field patch (JSON blobs for complex nested
 * values: transform/fill/stroke/trace params are re-serialized rather than typed directly, matching
 * the Rust struct's own `*_json: Option<String>` fields). */
export interface DrawLayerPatch {
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
