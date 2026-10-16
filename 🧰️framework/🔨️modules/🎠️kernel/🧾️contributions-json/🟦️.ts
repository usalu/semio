/** 🧾️ The contributions JSON a host pushes to the guest: building it from the loaded manifests, scoping it to an app and resolving the layout of a mode.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import type { PluginManifest, ProgramContributionEntry, NamedLayout, WindowLayout } from "../../🛂️manifest/🟦️.ts";

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
