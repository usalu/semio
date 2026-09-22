import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import { ACTIVATION_RECEIPT_FILE, developmentRuntimeRoot, parseActivationReceipt, pluginModulesRootIn, publishActivationReceipt, readActivationReceipt, type ActivationReceipt } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";
import type { ActivationComponentSpec } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts";
import { runtimeComponentClosure } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs";
import { EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts";
import { MODULE_SHARD_DIRECTORY, moduleDirectoryName } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { artifactFiles } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/📋️inventory/🟦️.ts";
import { PREVIEW2_VENDOR_RELATIVE } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🕸️imports/🟦️.ts";
import { FONT_ASSET } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🔤️fonts/🟦️.ts";
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

//#region 🔏️PlayInstalledArtifact
/** @emoji 🔏️ The file-identity rule an activation receipt row carries, restated SYNCHRONOUSLY: every file
 * of a staged directory in name order, each preceded by its `[name, size]` header, exactly as
 * `♻️activation/📥️installation/🟦️.ts`'s `activationFilesDigest` streams it. Restated rather than imported
 * because that module pulls the whole materialization tool-chain in and answers asynchronously, while play
 * publishes its union INSIDE the Vite config factory — and pinned byte-for-byte to the framework's own
 * digest by a unit law (`🧪️tests/🧪️playactivation`), so the two can never drift apart silently. */
function activationFilesDigestSync(files: ReadonlyMap<string, string>): string {
  const hash = createHash("sha256");
  for (const [name, path] of [...files].sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0)) {
    hash.update(JSON.stringify([name.replaceAll("\\", "/"), statSync(path).size]) + "\n");
    hash.update(readFileSync(path));
  }
  return hash.digest("hex");
}

/** @emoji 💿️ What the ONE `🔌️plugin-modules` root ACTUALLY holds for a component right now, as the
 * `artifactSha256` an activation of it would record: `sha256(<support digest> + <component digest>)`, the
 * identity `♻️activation/🏃️execution/🟦️.ts` publishes. Every lane stages into that one root, so exactly one
 * artifact per component exists on disk and this answer decides WHICH disagreeing lane is the served one.
 * Lazy and memoised (the support digest covers the vendor shim, the shard worker and the packed fonts —
 * tens of megabytes), so a fleet whose lanes all agree never reads a byte. `undefined` when the component
 * or the support tree is not staged at all, which is a fact for the caller, never a thrown error. */
export function playInstalledArtifactSha256(workspace: string): (pluginId: string) => string | undefined {
  const moduleRoot = pluginModulesRootIn(workspace, "dev"), digests = new Map<string, string | undefined>();
  let support: string | undefined;
  return (pluginId) => {
    if (digests.has(pluginId)) return digests.get(pluginId);
    let answer: string | undefined;
    try {
      support ??= activationFilesDigestSync(new Map<string, string>([
        ...[PREVIEW2_VENDOR_RELATIVE, MODULE_SHARD_DIRECTORY].flatMap(directory => [...artifactFiles(join(moduleRoot, directory))].map(([name, path]): readonly [string, string] => [join(directory, name), path])),
        [FONT_ASSET, join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/dist/fonts", FONT_ASSET)],
      ]));
      answer = createHash("sha256").update(support + activationFilesDigestSync(artifactFiles(join(moduleRoot, moduleDirectoryName(pluginId))))).digest("hex");
    } catch { answer = undefined; }
    digests.set(pluginId, answer);
    return answer;
  };
}
//#endregion 🔏️PlayInstalledArtifact

//#region 🔖️PlayUnionReceipt
export type PlayActivationLaneReceipt = { readonly lane: string; readonly receipt: ActivationReceipt; readonly receiptMtimeMs?: number };

/** @emoji ⚖️ How a union merge resolves a lane disagreement and where it says so. Both are injected, so
 * the whole rule stays pure and a fixture can drive every outcome without a staged workspace. */
export type PlayActivationMergeOptions = {
  /** @emoji 💿️ The sha the staged artifact on disk carries, or `undefined` when it cannot be read. */
  readonly installedArtifactSha256?: (pluginId: string) => string | undefined;
  readonly warn?: (line: string) => void;
};

type PlayActivationCandidate = { readonly lane: string; readonly row: ActivationReceipt["plugins"][number]; readonly receiptMtimeMs: number };

/** @emoji ⚖️ Picks the row a component is SERVED from when its lanes disagree. All 28 lanes stage into one
 * `🔌️plugin-modules` root, so a disagreement never means two artifacts — it means one lane re-activated
 * (a peer running `activate-<lane>-react-dev` after the coordinator's full activation) and its siblings
 * still carry the previous sha. Refusing the whole merge there killed :6033 at three serve starts in two
 * days for a runtime that was perfectly consistent on disk, so the rule is: the row whose sha equals the
 * INSTALLED artifact wins (newest receipt by mtime among equals), the other lanes are named in a one-line
 * warning, and only a disagreement where NO lane activated what is installed is a refusal — that one is
 * real, because then the served bytes have no receipt at all. `rebuiltAt` still keeps the EARLIEST value
 * among the agreeing rows, so a lane re-activating unchanged bytes never fakes a hot-swap. */
function playServedActivationRow(pluginId: string, candidates: readonly PlayActivationCandidate[], options: PlayActivationMergeOptions): ActivationReceipt["plugins"][number] {
  const earliest = (rows: readonly PlayActivationCandidate[]): number => Math.min(...rows.map(candidate => candidate.row.rebuiltAt));
  if (new Set(candidates.map(candidate => candidate.row.artifactSha256)).size === 1) return { ...candidates[0]!.row, rebuiltAt: earliest(candidates) };
  const installed = options.installedArtifactSha256?.(pluginId);
  const served = candidates.filter(candidate => candidate.row.artifactSha256 === installed);
  if (served.length === 0) throw new Error(`Stale play activation lane: ${pluginId} is ${candidates.map(candidate => `${candidate.row.artifactSha256} in ${candidate.lane}`).join(" but ")}, and the installed artifact (${installed ?? "unreadable"}) matches none of them (run bun nx run @semio-tech/framework-os-dev:activate-${candidates[0]!.lane}-react-dev)`);
  const newest = served.reduce((best, candidate) => candidate.receiptMtimeMs > best.receiptMtimeMs ? candidate : best);
  options.warn?.(`[play] lane drift: ${pluginId} served from ${newest.lane} (${installed!.slice(0, 8)}), stale in ${candidates.filter(candidate => candidate.row.artifactSha256 !== installed).map(candidate => candidate.lane).join(", ")}`);
  return { ...newest.row, rebuiltAt: earliest(served) };
}

/** @emoji 🧾️ Merges every lane's completion receipt into the ONE receipt describing play's exact runtime
 * union — pure, so the rule is testable without a staged workspace. Disagreements are resolved against the
 * installed artifact by {@link playServedActivationRow}. */
export function mergePlayActivationReceipts(lanes: readonly PlayActivationLaneReceipt[], expectedPluginIds: readonly string[], variant: string, options: PlayActivationMergeOptions = {}): ActivationReceipt {
  const candidates = new Map<string, PlayActivationCandidate[]>();
  for (const { lane, receipt, receiptMtimeMs } of lanes) {
    parseActivationReceipt(receipt);
    if (receipt.profile !== "dev") throw new Error(`Play activation lane ${lane} is not a dev activation: ${receipt.profile}`);
    if (receipt.variant !== lane) throw new Error(`Play activation lane ${lane} carries a ${receipt.variant} receipt`);
    for (const row of receipt.plugins) {
      const candidate = { lane, row, receiptMtimeMs: receiptMtimeMs ?? 0 }, listed = candidates.get(row.pluginId);
      if (listed) listed.push(candidate); else candidates.set(row.pluginId, [candidate]);
    }
  }
  const expected = [...expectedPluginIds].sort(), present = [...candidates.keys()].sort();
  const missing = expected.filter(id => !candidates.has(id)), extra = present.filter(id => !expected.includes(id));
  if (missing.length > 0 || extra.length > 0) throw new Error(`Play activation does not contain its exact runtime union — missing: ${missing.join(", ") || "(none)"}; extra: ${extra.join(", ") || "(none)"}`);
  return parseActivationReceipt({ schema: "semio.dev.activation/v1", variant, profile: "dev", plugins: present.map(id => playServedActivationRow(id, candidates.get(id)!, options)) });
}

/** @emoji 📖️ Reads one lane's receipt, naming the exact target that produces it when it is absent. Its
 * mtime rides along: when two lanes activated the same installed bytes, the newer receipt is the one that
 * describes the current staging pass. */
function readPlayActivationLane(workspace: string, lane: string): PlayActivationLaneReceipt {
  const directory = join(playLaneRuntimeRoot(workspace, lane), "activation");
  try { return { lane, receipt: readActivationReceipt(directory), receiptMtimeMs: statSync(join(directory, ACTIVATION_RECEIPT_FILE)).mtimeMs }; }
  catch (error) {
    if ((error as NodeJS.ErrnoException | undefined)?.code !== "ENOENT") throw error;
    throw new Error(`Missing play activation lane receipt: ${lane} (run bun nx run @semio-tech/framework-os-dev:activate-${lane}-react-dev)`);
  }
}

/** @emoji 📬️ Publishes the merged union receipt into play's own `dist` and returns it. Lane drift is
 * WARNED about on the serve's own console, next to the `[stale]` freshness lines that name the same lane. */
export function publishPlayUnionReceipt(workspace: string, options: PlayActivationMergeOptions = { installedArtifactSha256: playInstalledArtifactSha256(workspace), warn: line => console.warn(line) }): { readonly receiptDirectory: string; readonly receipt: ActivationReceipt } {
  const lanes = playActivationLanes();
  const receipt = mergePlayActivationReceipts(lanes.map(lane => readPlayActivationLane(workspace, lane)), playRuntimeComponentIds(), lanes[0]!, options);
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
  await registerTests1(import.meta.vitest, { playActivationLanes, playPaneClosureRoot, playRuntimeComponentIds, mergePlayActivationReceipts, playExtensionDirectory, activationFilesDigestSync, playInstalledArtifactSha256 }, join(import.meta.dirname, "../../../../.."));
}
