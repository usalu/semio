/** 🧾️ The contributions JSON a host pushes to the guest: building it from the loaded manifests, scoping it to an app and resolving the layout of a mode.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import type { PluginManifest, ProgramContributionEntry, NamedLayout, WindowLayout } from "@semio-tech/framework-manifest";

// #region 🎠️Kernel
export function buildContributionsJson(loaded: ReadonlyArray<{ readonly pluginId: string; readonly manifest: PluginManifest }>): string {
  const entries: ProgramContributionEntry[] = [];
  for (const entry of loaded) {
    for (const topicContribution of entry.manifest.topicContributions ?? []) {
      entries.push({ pluginId: entry.pluginId, topicContribution });
    }
  }
  return JSON.stringify(entries);
}

/**
 * ✂️ Host→guest contributions cut to exactly the topics the receiver consumes, plus its own.
 *
 * ⚖️ `consumedTopics` is the receiver's `consumes` row in the plugin registry — the one authority for
 * what a plugin acts on — and an empty set forwards NO foreign contribution: passing every pack put
 * `gis`'s 196 400-byte `stdio.artifact-catalog.v1` (a topic no plugin consumes) into all four
 * demonstrator apps and blew their wire admission (measured 2026-09-16: 226 310-byte pack).
 *
 * 🧩️ A consumed topic crosses WHOLE. A palette can only offer an extension the push carried, so the
 * operator-reachability cut this replaces offered a document only the extensions it already used
 * (measured 2026-09-27: a fresh flow document could never gain a brep node, its catalogue's extension
 * group stayed empty), and made the pack depend on a guest document read the push had to await.
 */
export function scopeContributionsJson(
  loaded: ReadonlyArray<{ readonly pluginId: string; readonly manifest: Pick<PluginManifest, "topicContributions"> }>,
  receiverPluginId: string,
  consumedTopics: readonly string[],
): string {
  const consumed = new Set(consumedTopics);
  const entries: ProgramContributionEntry[] = [];
  for (const entry of loaded) {
    const own = entry.pluginId === receiverPluginId;
    for (const topicContribution of entry.manifest.topicContributions ?? []) {
      if (own || consumed.has(topicContribution.topic)) entries.push({ pluginId: entry.pluginId, topicContribution });
    }
  }
  return JSON.stringify(entries);
}

/**
 * 🎯️ The topics one app's `setContributions` pack is cut to: the `consumes` row of the plugin that OWNS the app, never the composition's union.
 *
 * 🧱️ A composition re-hosts apps exactly as their owners declared them, and each owner's `consumes` row is the authority for what that app acts on (and prices its wire against). Handing every app the composition's union put `flow.extension`'s 555 312-byte pack into the cad, process3d and sourcing apps, whose retained `setContributions` admits `CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES` (49 152) (measured 2026-11-07, ticket 26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS). The owner is the loaded plugin other than the receiver whose manifest lists the app id; the result never exceeds the receiver's own row, and an app nobody else owns keeps the receiver's row whole.
 */
export function consumedTopicsForApp(
  registry: ReadonlyArray<{ readonly pluginId: string; readonly consumes?: readonly string[] }>,
  loaded: ReadonlyArray<{ readonly pluginId: string; readonly manifest: { readonly apps?: ReadonlyArray<{ readonly id: string }> } }>,
  receiverPluginId: string,
  appId: string | undefined,
): readonly string[] {
  const receiverRow = registry.find((row) => row.pluginId === receiverPluginId)?.consumes ?? [];
  if (appId === undefined) return receiverRow;
  const owner = loaded.find((entry) => entry.pluginId !== receiverPluginId && (entry.manifest.apps ?? []).some((app) => app.id === appId));
  if (owner === undefined) return receiverRow;
  const allowed = new Set(receiverRow);
  return (registry.find((row) => row.pluginId === owner.pluginId)?.consumes ?? []).filter((topic) => allowed.has(topic));
}

export function resolveLayoutForMode(
  app: { readonly defaultLayout?: WindowLayout; readonly namedLayouts?: readonly NamedLayout[]; readonly modes: readonly { readonly id: string; readonly layoutId?: string }[] },
  modeId: string,
): WindowLayout | undefined {
  const mode = app.modes.find((entry) => entry.id === modeId);
  if (mode?.layoutId) {
    const named = app.namedLayouts?.find((entry) => entry.id === mode.layoutId);
    if (named) return named.layout;
  }
  return app.defaultLayout;
}
// #endregion 🎠️Kernel
