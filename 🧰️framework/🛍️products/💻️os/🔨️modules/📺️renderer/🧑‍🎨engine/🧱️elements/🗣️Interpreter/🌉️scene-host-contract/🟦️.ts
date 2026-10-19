/** 🌉️ The scene-host contract every element reads without importing the interpreter or the shell: the wire label mint point, the app catalogue context, the scene JSON and pack field decoder, and the selection-domain and JSON array parsers.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}, which re-exports its public names; `NodeGraph`, `ShellHost` and `World3dHost` read these declarations here so none of them has to import another element's barrel for them.
 */

import { createContext, useContext } from "react";
import type { UiLabel } from "@semio-tech/ui-react";
import type { AppCatalogue, PluginContextMenuSurfaceTarget } from "@semio-tech/framework";
import { decodeScenePackField } from "@semio-tech/framework/packs";

/** 🕳️ Sanctioned wire-boundary mint point (see ui-react's `UiLabel` docstring): brands an
 * already plugin/manifest-resolved string as {@link UiLabel}. */
export function wireLabel(value: string): UiLabel {
  return value as UiLabel;
}

/** 🛍️ The APP-STATIC operator/palette catalogue for the app owning this subtree — fetched ONCE
 * per app instance from the reserved `framework.section.catalogue` retained surface and cached by
 * `ShellHost` for that instance's whole lifetime. Never read off a scene: with the real `brep`/`math`
 * operator sets installed the payload is ~100 KB, more than three times the 32 KiB fixed per-surface
 * admission every scene is encoded against, so carrying it per scene made the node-graph window
 * unrenderable (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1). A scene names an operator by KIND ID;
 * this is where that kind's record, ports and palette entry come from. Declared here, beside the other
 * scene-host contexts, so neither `ShellHost` (the provider) nor `NodeGraph` (the consumer) has to
 * import the other for it. */
export const AppCatalogueContext = createContext<AppCatalogue>(Object.freeze({}));

/** 🛍️ Reads the nearest {@link AppCatalogueContext} — `{}` when no app instance provides one. */
export function useAppCatalogue(): AppCatalogue {
  return useContext(AppCatalogueContext);
}

/** 🪪️ Identifies which window *pane* (not just which window *kind*) a `ComponentSceneHost` is
 * mounted for — every window kind's `UiNode` is shared verbatim across all of its open instances, so a
 * host can't otherwise tell which pane it is; provided per-pane around each `<InterpretedUiNode>` call. */
export const WindowInstanceIdContext = createContext<string | null>(null);

export function parseSceneJsonField<T>(encoded: string): T {
  if (encoded.startsWith("pk:")) return decodeScenePackField(encoded) as T;
  return JSON.parse(encoded) as T;
}

export function parseJsonArray<T>(json: string | undefined): readonly T[] {
  if (!json) return [];
  try {
    const parsed = JSON.parse(json);
    return Array.isArray(parsed) ? (parsed as T[]) : [];
  } catch {
    return [];
  }
}

export function parseSelectionDomainsFromSession(json: string): { readonly nodes: string[]; readonly edges: string[]; readonly handles: string[] } {
  try {
    const parsed = JSON.parse(json) as unknown;
    if (Array.isArray(parsed)) {
      return { nodes: parsed as string[], edges: [], handles: [] };
    }
    if (parsed && typeof parsed === "object") {
      const record = parsed as { nodes?: unknown; edges?: unknown; handles?: unknown; edgeIds?: unknown; handleIds?: unknown };
      return {
        nodes: Array.isArray(record.nodes) ? (record.nodes as string[]) : [],
        edges: Array.isArray(record.edges) ? (record.edges as string[]) : Array.isArray(record.edgeIds) ? (record.edgeIds as string[]) : [],
        handles: Array.isArray(record.handles) ? (record.handles as string[]) : Array.isArray(record.handleIds) ? (record.handleIds as string[]) : [],
      };
    }
  } catch {}
  return { nodes: [], edges: [], handles: [] };
}

export function selectionGroupsFromDomains(domains: { readonly nodes: readonly string[]; readonly edges: readonly string[]; readonly handles: readonly string[] }): NonNullable<PluginContextMenuSurfaceTarget["selection"]> {
  const groups: NonNullable<PluginContextMenuSurfaceTarget["selection"]>[number][] = [];
  if (domains.nodes.length > 0) groups.push({ domain: "node", ids: domains.nodes });
  if (domains.edges.length > 0) groups.push({ domain: "edge", ids: domains.edges });
  if (domains.handles.length > 0) groups.push({ domain: "handle", ids: domains.handles });
  return groups;
}
