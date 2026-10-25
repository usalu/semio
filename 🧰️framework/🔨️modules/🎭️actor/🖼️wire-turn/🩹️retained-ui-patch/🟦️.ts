// #region 🧲️Header
/** 💊️ Retained-mode `UiPatch` ops decoded from the wire and applied onto a kept tree. */
// #endregion 🧲️Header

// #region 🔌️Imports
import { type WireVariant, coerceWireBytes } from "../🔖️turn-result/🟦️.ts";
// #endregion 🔌️Imports

//#region 🔖️RetainedUiPatch
/** 🩹️ `kernel::PatchOp`, TS twin restricted to what `⚛️reactor/🩹️patches/🦀️.rs`'s
 * `PatchTracker` actually emits this wave — full-body only, every dirty surface emits one
 * `PatchOp::Replace` at the root path. `path` is `list<u32>` at the WIT boundary (empty for root). */
export type PatchOp =
  | { readonly kind: "Replace"; readonly path: readonly number[]; readonly node: unknown }
  | { readonly kind: "InsertChild"; readonly path: readonly number[]; readonly index: number; readonly node: unknown }
  | { readonly kind: "RemoveChild"; readonly path: readonly number[]; readonly index: number }
  | { readonly kind: "SetProps"; readonly path: readonly number[]; readonly props: unknown };

export function decodeWirePatchOps(ops: readonly WireVariant[], decodePackValue: (bytes: Uint8Array) => unknown): readonly PatchOp[] {
  const decoded: PatchOp[] = [];
  for (const op of ops) {
    const val = (op.val ?? {}) as Record<string, unknown>;
    const path = Array.isArray(val.path) ? (val.path as number[]) : [];
    switch (op.tag) {
      case "replace":
        decoded.push({ kind: "Replace", path, node: decodePackValue(coerceWireBytes(val.node)) });
        break;
      case "insert-child":
        decoded.push({ kind: "InsertChild", path, index: Number(val.index ?? 0), node: decodePackValue(coerceWireBytes(val.node)) });
        break;
      case "remove-child":
        decoded.push({ kind: "RemoveChild", path, index: Number(val.index ?? 0) });
        break;
      case "set-props":
        decoded.push({ kind: "SetProps", path, props: val.props !== undefined ? decodePackValue(coerceWireBytes(val.props)) : undefined });
        break;
      default:
        break;
    }
  }
  return decoded;
}

export type RetainedSurface = { readonly revision: number; readonly node: unknown };

/**
 * 🖼️ Reconciles one `UiPatch`'s ops onto `previous` (the last body a caller retained for the
 * surface), so the UI thread reads an already-reconciled tree instead of awaiting a plugin turn. Only
 * a root `PatchOp::Replace` (path `[]`) is applied — the only shape any guest emits this wave; anything
 * else, or a `baseRevision` that doesn't match `previous.revision` on a non-full-replace patch, is an
 * honest desync — `previous` is kept rather than an unverified partial walk applied.
 */
export function applyUiPatchToRetained(
  previous: RetainedSurface | null,
  patch: { readonly revision: number; readonly baseRevision: number; readonly ops: readonly PatchOp[] },
): { readonly surface: RetainedSurface | null; readonly desynced: boolean } {
  let node: unknown = previous?.node ?? null;
  let sawFullReplace = false;
  for (const op of patch.ops) {
    if (op.kind === "Replace" && op.path.length === 0) {
      node = op.node;
      sawFullReplace = true;
    } else {
      return { surface: previous, desynced: true };
    }
  }
  if (!sawFullReplace && previous && patch.baseRevision !== previous.revision) return { surface: previous, desynced: true };
  return { surface: node !== null ? { revision: patch.revision, node } : previous, desynced: false };
}
//#endregion 🔖️RetainedUiPatch
