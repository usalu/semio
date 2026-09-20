import { existsSync, mkdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { developmentRuntimeRoot, parseActivationReceipt, publishActivationReceipt, readActivationReceipt, type ActivationReceipt } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";
import type { ActivationComponentSpec } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts";
import { runtimeComponentClosure } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs";
import { EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts";
import { moduleDirectoryName } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { PLAY_RUNTIME_TARGETS, playPaneClosureRoot, playRuntimeComponentIds } from "../🟦️.ts";

//#region 🛣️PlayActivationLanes
/** @emoji 📦️ Repository-relative root of the `@semio-tech/framework-os-dev` package every lane's
 * `activate-<lane>-react-dev` target writes its receipt below. */
const OS_DEV_PACKAGE_ROOT = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript";

/** @emoji 🧾️ The play-owned directory holding the merged receipt its Vite server consumes — no single
 * framework lane can produce play's cross-app union, because each one publishes one variant's closure. */
export const PLAY_UNION_RECEIPT_DIRECTORY = "🏢️semio-tech/🎡️play/dist/♻️activation/dev";

/** @emoji 🛣️ The fewest pane variants whose activation closures together cover every pane component —
 * greedy by newly covered components, ties broken by grid order, so the result is deterministic. Pane
 * variants are the only candidates: the `s` host lane is the launcher play itself replaces, and every
 * component play needs — the stdio one included, since its own `stdio` pane activates it — is reachable
 * from some pane. */
export function playActivationLanes(): readonly string[] {
  const components = [...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS];
  const closures = PLAY_RUNTIME_TARGETS.map(target => ({ variant: target.variant, ids: runtimeComponentClosure(components, [playPaneClosureRoot(target)]) as readonly string[] }));
  const union = new Set(playRuntimeComponentIds()), covered = new Set<string>(), lanes: string[] = [];
  while ([...union].some(id => !covered.has(id))) {
    let best: { readonly variant: string; readonly ids: readonly string[] } | undefined, bestGain = 0;
    for (const closure of closures) {
      const gain = closure.ids.filter(id => union.has(id) && !covered.has(id)).length;
      if (gain > bestGain) { best = closure; bestGain = gain; }
    }
    if (!best) throw new Error(`Play components no pane lane reaches: ${[...union].filter(id => !covered.has(id)).join(", ")}`);
    lanes.push(best.variant);
    for (const id of best.ids) covered.add(id);
  }
  return lanes;
}

/** @emoji 🗂️ One lane's framework-owned development runtime root. */
function playLaneRuntimeRoot(workspace: string, lane: string): string {
  return developmentRuntimeRoot(join(workspace, OS_DEV_PACKAGE_ROOT), lane, "dev", "react");
}

/** @emoji 🗂️ Every lane's receipt directory, in lane order — what the union watcher subscribes to. */
export function playActivationLaneReceiptDirectories(workspace: string): readonly string[] {
  return playActivationLanes().map(lane => join(playLaneRuntimeRoot(workspace, lane), "activation"));
}

/** @emoji 🧩️ Maps every installed extension directory name to the first lane that staged it. */
export function playExtensionDirectories(workspace: string): ReadonlyMap<string, string> {
  const byId = new Map(EXTENSION_TARGETS.map(row => [row.pluginId, row]));
  const directories = new Map<string, string>(), lanes = playActivationLanes();
  for (const id of playRuntimeComponentIds().filter(id => byId.has(id))) {
    const name = moduleDirectoryName(id);
    const lane = lanes.find(candidate => existsSync(join(playLaneRuntimeRoot(workspace, candidate), "extensions", name)));
    if (!lane) throw new Error(`Play extension ${id} is installed by no activation lane`);
    directories.set(name, join(playLaneRuntimeRoot(workspace, lane), "extensions", name));
  }
  return directories;
}
//#endregion 🛣️PlayActivationLanes

//#region 🔖️PlayUnionReceipt
export type PlayActivationLaneReceipt = { readonly lane: string; readonly receipt: ActivationReceipt };

/** @emoji 🧾️ Merges every lane's completion receipt into the ONE receipt describing play's exact runtime
 * union — pure, so the rule is testable without a staged workspace. All lanes stage into the same
 * `🔌️plugin-modules` root, so two lanes disagreeing about one component's `artifactSha256` means one lane
 * is stale. Overlapping rows keep the EARLIEST `rebuiltAt`, so a lane re-activating unchanged bytes never
 * fakes a hot-swap. */
export function mergePlayActivationReceipts(lanes: readonly PlayActivationLaneReceipt[], expectedPluginIds: readonly string[], variant: string): ActivationReceipt {
  const rows = new Map<string, { readonly lane: string; readonly row: ActivationReceipt["plugins"][number] }>();
  for (const { lane, receipt } of lanes) {
    parseActivationReceipt(receipt);
    if (receipt.profile !== "dev") throw new Error(`Play activation lane ${lane} is not a dev activation: ${receipt.profile}`);
    if (receipt.variant !== lane) throw new Error(`Play activation lane ${lane} carries a ${receipt.variant} receipt`);
    for (const row of receipt.plugins) {
      const previous = rows.get(row.pluginId);
      if (!previous) { rows.set(row.pluginId, { lane, row }); continue; }
      if (previous.row.artifactSha256 !== row.artifactSha256) throw new Error(`Stale play activation lane: ${row.pluginId} is ${previous.row.artifactSha256} in ${previous.lane} but ${row.artifactSha256} in ${lane} (run bun nx run @semio-tech/framework-os-dev:activate-${lane}-react-dev)`);
      if (row.rebuiltAt < previous.row.rebuiltAt) rows.set(row.pluginId, { lane, row });
    }
  }
  const expected = [...expectedPluginIds].sort(), present = [...rows.keys()].sort();
  const missing = expected.filter(id => !rows.has(id)), extra = present.filter(id => !expected.includes(id));
  if (missing.length > 0 || extra.length > 0) throw new Error(`Play activation does not contain its exact runtime union — missing: ${missing.join(", ") || "(none)"}; extra: ${extra.join(", ") || "(none)"}`);
  return parseActivationReceipt({ schema: "semio.dev.activation/v1", variant, profile: "dev", plugins: present.map(id => ({ ...rows.get(id)!.row })) });
}

/** @emoji 📖️ Reads one lane's receipt, naming the exact target that produces it when it is absent. */
function readPlayActivationLane(workspace: string, lane: string): PlayActivationLaneReceipt {
  try { return { lane, receipt: readActivationReceipt(join(playLaneRuntimeRoot(workspace, lane), "activation")) }; }
  catch (error) {
    if ((error as NodeJS.ErrnoException | undefined)?.code !== "ENOENT") throw error;
    throw new Error(`Missing play activation lane receipt: ${lane} (run bun nx run @semio-tech/framework-os-dev:activate-${lane}-react-dev)`);
  }
}

/** @emoji 📬️ Publishes the merged union receipt into play's own `dist` and returns it. */
export function publishPlayUnionReceipt(workspace: string): { readonly receiptDirectory: string; readonly receipt: ActivationReceipt } {
  const lanes = playActivationLanes();
  const receipt = mergePlayActivationReceipts(lanes.map(lane => readPlayActivationLane(workspace, lane)), playRuntimeComponentIds(), lanes[0]!);
  const receiptDirectory = join(workspace, PLAY_UNION_RECEIPT_DIRECTORY);
  mkdirSync(receiptDirectory, { recursive: true });
  publishActivationReceipt(receiptDirectory, receipt);
  return { receiptDirectory, receipt };
}

/** @emoji 🧾️ Requires the exact completed union before starting the development host. */
export function readPlayActivation(workspace: string): { readonly receiptDirectory: string; readonly extensionsDirectory: string; readonly extensionDirectories: ReadonlyMap<string, string>; readonly receipt: ActivationReceipt } {
  const { receiptDirectory, receipt } = publishPlayUnionReceipt(workspace);
  return { receiptDirectory, extensionsDirectory: join(playLaneRuntimeRoot(workspace, playActivationLanes()[0]!), "extensions"), extensionDirectories: playExtensionDirectories(workspace), receipt };
}
//#endregion 🔖️PlayUnionReceipt

/** @emoji 🗂️ Where one installed extension's module directory is served from. A dev server reads it from the
 * lane that staged it ({@link playExtensionDirectories}); a production build has no activation lane at all
 * and every component sits under the release `🔌️plugin-modules` root — the same fallback the demonstrator's
 * `installedExtensionsDir` makes, expressed once so the build mode is a tested rule, not a `??` in a config. */
export function playExtensionDirectory(name: string, pluginModulesDirectory: string, laneDirectories?: ReadonlyMap<string, string>): string {
  return laneDirectories?.get(name) ?? join(pluginModulesDirectory, name);
}

/** @emoji 🔎️ Describes every union component for the activation-receipt freshness watcher. */
export function playActivationComponents(workspace: string, extensionDirectories: ReadonlyMap<string, string>): readonly ActivationComponentSpec[] {
  const byId = new Map([...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS].map(row => [row.pluginId, row]));
  return playRuntimeComponentIds().map(id => {
    const row = byId.get(id)!, directoryName = moduleDirectoryName(id), installDirectory = extensionDirectories.get(directoryName);
    return { pluginId: id, directoryName, role: row.role === "extension" ? "extension" : "plugin", sourceRoot: resolve(workspace, row.cratePath, "..", ".."), ...(installDirectory ? { installDirectory } : {}) };
  });
}

if (import.meta.vitest) {
  const { registerTests1 } = await import("../../../🧪️tests/🧪️playactivation/🟦️.ts");
  await registerTests1(import.meta.vitest, { playActivationLanes, playPaneClosureRoot, playRuntimeComponentIds, mergePlayActivationReceipts, playExtensionDirectory }, join(import.meta.dirname, "../../../../.."));
}
