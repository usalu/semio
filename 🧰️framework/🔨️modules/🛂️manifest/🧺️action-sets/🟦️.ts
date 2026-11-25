import { type ActionDefinition } from "../🎬️action-types-and-semantics/🟦️.ts";

// #region 🛂️Manifest
//#region 🧺️ActionSets
/** 🧺️ The part of a plugin manifest that carries action tables — see Rust `ActionSet`. */
export type ManifestWithActionSets = {
  readonly apps: readonly { readonly id: string; readonly actions?: readonly ActionDefinition[] }[];
  readonly actionSets?: readonly { readonly id: string; readonly appIds: readonly string[]; readonly actions: readonly ActionDefinition[] }[];
};

/**
 * 🧺️ The app as its windows see it: its own roster extended by the rows of every action set that names it, in table order.
 * Hand-written twin of Rust `PluginManifest::resolved_app`; sets resolve here and nowhere else, so `resolveWindowActions`
 * always runs on a resolved app.
 */
export function resolveManifestApp<A extends { readonly id: string; readonly actions?: readonly ActionDefinition[] }>(manifest: Pick<ManifestWithActionSets, "actionSets">, app: A): A {
  const sets = (manifest.actionSets ?? []).filter((set) => set.appIds.includes(app.id));
  return sets.length === 0 ? app : { ...app, actions: [...sets.flatMap((set) => set.actions), ...(app.actions ?? [])] };
}

/**
 * 🛂️ Why a table of action sets is refused, or `undefined` when it is sound: set ids repeat, a set has no member or names an app the manifest lacks, a set declares a row twice, a member declares a row the set provides, or two sets give one app the same action. Hand-written twin of Rust `PluginManifest::validate_action_sets`.
 */
export function actionSetsFault(manifest: ManifestWithActionSets): string | undefined {
  const sets = manifest.actionSets ?? [];
  const ids = new Set<string>();
  for (const [index, set] of sets.entries()) {
    if (ids.has(set.id)) return `action set ${set.id} is declared twice`;
    ids.add(set.id);
    if (set.appIds.length === 0) return `action set ${set.id} has no member app`;
    const rows = new Set<string>();
    for (const row of set.actions) {
      if (rows.has(row.id)) return `action set ${set.id} declares action ${row.id} twice`;
      rows.add(row.id);
    }
    for (const appId of set.appIds) {
      const app = manifest.apps.find((entry) => entry.id === appId);
      if (!app) return `action set ${set.id} names unknown app ${appId}`;
      const own = (app.actions ?? []).find((action) => rows.has(action.id));
      if (own) return `app ${appId} declares action ${own.id} that action set ${set.id} already provides`;
      const other = sets.slice(0, index).find((earlier) => earlier.appIds.includes(appId) && earlier.actions.some((row) => rows.has(row.id)));
      if (other) return `app ${appId} receives an action from both action sets ${other.id} and ${set.id}`;
    }
  }
  return undefined;
}

/**
 * 🧺️ The manifest with every app resolved and no action set left — what a host that reads app rosters directly works on.
 * Hand-written twin of Rust `PluginManifest::resolved`.
 */
export function resolveManifestActionSets<M extends ManifestWithActionSets>(manifest: M): M {
  if ((manifest.actionSets ?? []).length === 0) return manifest;
  const { actionSets: _tables, ...rest } = manifest;
  return { ...rest, apps: manifest.apps.map((app) => resolveManifestApp(manifest, app)) } as unknown as M;
}
//#endregion 🧺️ActionSets
// #endregion 🛂️Manifest
