/** 🪪️ The stable DOM ids and React reconciliation keys every interpreted node gets from its authored key.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}; importers import this module directly.
 */

import { UiDocumentStore, type UiDocumentState } from "../../📃️UiDocumentStore/🟦️.tsx";
import { type SurfaceId, type UiNodeId, type UiNodeRecord } from "@semio-tech/framework";

//#region 🔖️UiInterpreter
//#region ComponentRenderers
//#region Tree
//#region 🪪️StableDomIds
/** 🪪️ THE stable DOM id of one retained UI node: the surface it belongs to, then the node's own
 * Rust-authored `key`. `UiNodeRecord.id` is a DFS-order integer re-minted on EVERY full-body
 * reconciliation (`builtNodeToSnapshot`, `📃️UiDocumentStore/🟦️.tsx`), so an id built from it names a
 * different row after the next refresh — a scripted or assistive click keyed on `#5` silently targets
 * the wrong node. `key` is authored by the program (`procedural3d-play-generate.add-generation`) and
 * survives every refresh, and the surface prefix is what namespaces it per window, since two windows of
 * one app can render the same authored key. A key is unique only among its siblings (the ui contract), so a node
 * rendered inside a row — a tree row's inline controls, a table row's cells — is namespaced by that row's own DOM id
 * (`scope`) instead: two rows' `edit` inputs or `cell-0` cells never share an id. Falls back to the volatile id only for
 * a keyless node (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export function uiNodeDomId(surface: SurfaceId, key: string, fallbackNodeId: UiNodeId, scope?: string): string {
  return key ? `${scope ?? surface}/${key}` : `node-${fallbackNodeId}`;
}

/** 🪪️ {@link uiNodeDomId} for a record held by a live store — the surface comes off the store's own state, the row scope
 * off the rendering context. */
export function nodeDomId(store: UiDocumentStore, record: UiNodeRecord, scope?: string): string {
  return uiNodeDomId(store.getState().surface, record.key, record.id, scope);
}

/** 🪪️ React reconciliation keys for a sibling run — the SAME authored identity {@link uiNodeDomId}
 * already addresses a node by, applied to reconciliation instead of to the DOM.
 *
 * `builtNodeToSnapshot` mints `UiNodeId`s by pre-order DFS over the WHOLE body, so inserting one node
 * anywhere ahead of a sibling renumbers that sibling and everything after it. Keying children on that
 * number therefore turns any upstream shape change into a React key change — and a key change is an
 * unmount of the entire subtree, however unchanged it is. Measured on `window:procedural-main`: one
 * eval-status refresh grew the outline tree by four port rows (`profile@wire`,
 * `extrusion-axis@vectorOut`, `extrusion-axis@errors`, `extrude@solid`), which moved the node-graph
 * surface from id 30 to 34 and its container from 29 to 33, and React tore down and rebuilt the flow
 * host — a second wasm flow session, a second canvas, a second wasm-side surface, ~5.7 s of attach and
 * a graph that stayed blank for 39 s — for a surface node whose own content had not changed
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
 *
 * A node's authored `key` survives renumbering, so it IS the key wherever it exists and is unique
 * among its siblings; a keyless or ambiguous sibling falls back to the minted id, which stays correct
 * (React only needs sibling-local uniqueness) and is simply not stable. The `k:`/`#` prefixes keep the
 * two namespaces from ever colliding. */
export function uiSiblingReactKeys(siblings: readonly { readonly id: UiNodeId; readonly key: string }[]): readonly string[] {
  const occurrences = new Map<string, number>();
  for (const sibling of siblings) if (sibling.key) occurrences.set(sibling.key, (occurrences.get(sibling.key) ?? 0) + 1);
  return siblings.map((sibling) => (sibling.key && occurrences.get(sibling.key) === 1 ? `k:${sibling.key}` : `#${sibling.id}`));
}

/** 🪪️ {@link uiSiblingReactKeys} for a child-id run resolved against a live document state. A child id
 * with no record keeps the id fallback rather than being dropped — the caller still renders it. */
export function uiChildReactKeys(state: UiDocumentState, children: readonly UiNodeId[]): readonly string[] {
  return uiSiblingReactKeys(children.map((id) => ({ id, key: state.nodes.get(id)?.key ?? "" })));
}
//#endregion 🪪️StableDomIds
//#endregion Tree
//#endregion ComponentRenderers
//#endregion 🔖️UiInterpreter
