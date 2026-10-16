/** 🚚️ The last complete text of each surface scene-lane carrier, keyed by the producer's content hash, with its bounded cache and lane text assembly.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}; importers import this module directly.
 */

import { packedTextLeaf } from "../../🔌️PluginRuntime/🧳️packed-text/🟦️.ts";
import { type SceneLane, type SceneLaneRef } from "@semio-tech/framework";
import { type UiDocumentState } from "../../📃️UiDocumentStore/🟦️.tsx";
import { type UiNodeId, type UiNodeRecord } from "@semio-tech/framework";

//#region 🔖️UiInterpreter
//#region SurfaceBridge
//#region 🚚️SurfaceSceneLanes
/** 🚚️ Last COMPLETE text of one `${nodeId}:${lane}` carrier, keyed by the producer's own content
 * hash. Two jobs, both load-bearing:
 *
 * 1. **Incremental per lane.** A lane whose hash is unchanged is never re-walked or re-concatenated,
 *    so a camera nudge on a 57 KB Nakagin world costs one map lookup per lane instead of 57 KB of
 *    string building — which matters because the spine changes on every frame the camera moves while
 *    `instances` changes only when the document does.
 * 2. **Partial arrival.** A surface tree larger than `SURFACE_RECONCILE_PAGE_BYTES` arrives across
 *    several patches, so a spine can land before the leaves it declares. A lane whose concatenation
 *    does not yet match the declared byte length is omitted when its hash changed (a document swap
 *    must not keep serving the previous fixture) and is served from cache only when the hash still
 *    matches — never as truncated JSON that would parse to nothing.
 *
 * Bounded by {@link SURFACE_SCENE_LANE_CACHE_ENTRIES} with plain insertion-order eviction — an entry
 * is one lane of one live surface node, and a torn-down document simply stops touching its own. */
const SURFACE_SCENE_LANE_CACHE_ENTRIES = 512;
export const surfaceSceneLaneCache = new Map<string, { readonly hash: string; readonly text: string }>();

function rememberSurfaceSceneLane(key: string, hash: string, text: string): void {
  surfaceSceneLaneCache.delete(key);
  surfaceSceneLaneCache.set(key, { hash, text });
  while (surfaceSceneLaneCache.size > SURFACE_SCENE_LANE_CACHE_ENTRIES) {
    const oldest = surfaceSceneLaneCache.keys().next();
    if (oldest.done) break;
    surfaceSceneLaneCache.delete(oldest.value);
  }
}

/** 📏️ UTF-8 length of a JS string without allocating a `Uint8Array` — the producer counts lane bytes
 * in UTF-8 (`String::len` in Rust), so a lane carrying a German label must be compared in the same
 * unit or its completeness check would never agree. */
export function utf8ByteLength(value: string): number {
  let bytes = 0;
  for (let index = 0; index < value.length; index += 1) {
    const code = value.charCodeAt(index);
    if (code < 0x80) bytes += 1;
    else if (code < 0x800) bytes += 2;
    else if (code >= 0xd800 && code <= 0xdbff) {
      bytes += 4;
      index += 1;
    } else bytes += 3;
  }
  return bytes;
}

/** 🧩️ Depth-first concatenation of every packed `text` leaf (`value` then sorted `dataAttributes`) under `root` — the exact inverse of the plugin
 * host's `paged_text_carrier`, and the same walk `sectionValueFromBuiltNode` does for a reserved
 * refresh section. */
export function surfaceSceneLaneText(state: UiDocumentState, root: UiNodeRecord): string {
  let payload = "";
  const stack: UiNodeId[] = [...(root.children ?? [])].reverse();
  while (stack.length > 0) {
    const id = stack.pop()!;
    const record = state.nodes.get(id);
    if (!record) continue;
    if (record.component.type === "text" && typeof record.component.value === "string") {
      payload += packedTextLeaf(record.component.value, record.component.dataAttributes);
    }
    for (let index = (record.children ?? []).length - 1; index >= 0; index -= 1) stack.push(record.children[index]!);
  }
  return payload;
}

/** 🚚️ Collects one paged surface's arrived lane texts, keyed by reserved carrier key, ready for
 * `sceneFromLanes` over the same `lanes` table. A lane the spine does not declare is ignored; a lane
 * that is declared but not yet fully arrived falls back to its last complete text, or is omitted when
 * there is none. */
export function surfaceLaneTexts<S extends object>(record: UiNodeRecord, state: UiDocumentState, declared: readonly SceneLaneRef[], lanes: readonly SceneLane<S>[]): ReadonlyMap<string, string> {
  const texts = new Map<string, string>();
  if (declared.length === 0) return texts;
  const refByLane = new Map(declared.map((ref) => [ref.lane, ref]));
  for (const childId of record.children ?? []) {
    const child = state.nodes.get(childId);
    if (!child) continue;
    const lane = lanes.find((entry) => entry.bodyKey === String(child.key));
    const ref = lane && refByLane.get(lane.lane);
    if (!lane || !ref) continue;
    const cacheKey = `${record.id}:${lane.lane}`;
    const cached = surfaceSceneLaneCache.get(cacheKey);
    if (cached?.hash === ref.hash) {
      texts.set(lane.bodyKey, cached.text);
      continue;
    }
    const text = surfaceSceneLaneText(state, child);
    if (utf8ByteLength(text) === ref.bytes || (text.length > 0 && lane.optional)) {
      rememberSurfaceSceneLane(cacheKey, ref.hash, text);
      texts.set(lane.bodyKey, text);
    }
  }
  return texts;
}
//#endregion 🚚️SurfaceSceneLanes
//#endregion SurfaceBridge
//#endregion 🔖️UiInterpreter
