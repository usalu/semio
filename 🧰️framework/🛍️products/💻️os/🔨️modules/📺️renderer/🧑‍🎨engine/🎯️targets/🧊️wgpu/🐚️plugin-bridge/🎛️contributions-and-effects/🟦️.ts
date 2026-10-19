import { type Effect, fetchDescriptorManifest, type HostEffectDispatchScope, type PluginManifest, scopeContributionsJson } from "@semio-tech/framework";
import { decodeAppFrame, decodePackWire, encodePackValue, viewContextWireValue } from "@semio-tech/framework-os";
import { coerceWireBytes, shellFrameBytes, wireEffectToFriendly, wireSpawnJob, type WireTurnResult, type WireVariant } from "../../../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts";

/** 🧭️ Posts a nested `boot-phase` when the frame worker installed the helper. Parent `shell-boot` stays declared. */
export function declareWgpuBootSubphase(name: string): { leave: (extra?: string) => void } {
  const started = performance.now();
  const declare = (globalThis as { semioDeclareBootSubphase?: (phase: string, state: "enter" | "leave", elapsedMs: number) => void }).semioDeclareBootSubphase;
  declare?.(name, "enter", 0);
  return {
    leave(extra) {
      const elapsed = performance.now() - started;
      declare?.(name, "leave", elapsed);
    },
  };
}

/** 🎯️ Loaded wgpu handles by plugin id — extension dispatch resolves the callee from this table. */
export const loadedWgpuHandles = new Map<string, object>();

export const contributionManifests = new Map<string, PluginManifest>();

/** 🗂️ Remembers a plugin descriptor so wgpu can send scoped contributions without instantiating every actor. */
export async function primeContributionManifest(pluginId: string, moduleUrl: string, signal?: AbortSignal): Promise<void> {
  const manifest = await fetchDescriptorManifest(pluginId, moduleUrl, signal);
  contributionManifests.set(pluginId, manifest);
}

/** 📦️ One pack-sized contributions payload — the receiver's own contributions plus every topic it consumes. */
export function wgpuBuildScopedContributionsPack(
  receiverPluginId: string,
  loadedManifests: ReadonlyArray<{ readonly pluginId: string; readonly manifest: Pick<PluginManifest, "topicContributions"> }> | undefined,
  consumedTopics: readonly string[],
): { readonly json: string; readonly bytes: Uint8Array; readonly pluginIds: readonly string[]; readonly chars: number; readonly crossings: 1 } | null {
  const loaded = loadedManifests ?? [...contributionManifests.entries()].map(([pluginId, manifest]) => ({ pluginId, manifest }));
  if (!loaded.length) return null;
  const json = scopeContributionsJson(loaded, receiverPluginId, consumedTopics);
  if (!json || json === "[]") return null;
  const bytes = new TextEncoder().encode(json);
  const pluginIds = [...new Set((JSON.parse(json) as { readonly pluginId?: string }[]).map((entry) => entry.pluginId).filter((id): id is string => typeof id === "string"))];
  return { json, bytes, pluginIds, chars: json.length, crossings: 1 };
}

export const WGPU_CONTRIBUTIONS_SLIM_VIEW = Object.freeze({ locale: "en", terminology: "native" });

/** 🧹 Drops the host-owned panel payload from the live view so the scoped pack is the only bulk
 * thing this crossing carries, and resolves the preferences the guest's `ViewModel` declares
 * non-optional. Contributions are no longer a view-state field at all — they are installed by this
 * very `setContributions` run — so there is nothing left here to strip on their account
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export function wgpuSlimContributionsView(viewState: unknown): Record<string, unknown> {
  const raw = viewState && typeof viewState === "object" && !Array.isArray(viewState) ? { ...(viewState as Record<string, unknown>) } : {};
  delete raw.panelJson;
  if (typeof raw.locale !== "string" || raw.locale.length === 0) raw.locale = WGPU_CONTRIBUTIONS_SLIM_VIEW.locale;
  if (raw.terminology == null || raw.terminology === "") raw.terminology = WGPU_CONTRIBUTIONS_SLIM_VIEW.terminology;
  return raw;
}

function unsignedVarintByteLength(value: number): number {
  if (!Number.isSafeInteger(value) || value < 0) throw new RangeError("unsigned varint length requires a non-negative safe integer");
  let bytes = 1;
  while (value >= 128) {
    value = Math.floor(value / 128);
    bytes += 1;
  }
  return bytes;
}

/** 📏️ Exact encoded length of AppCommand::Command for its sequence and two packed vectors. */
export function wgpuCommandIngressByteLength(sequence: number, commandBytes: Uint8Array, viewBytes: Uint8Array): number {
  return 1 + unsignedVarintByteLength(sequence) + unsignedVarintByteLength(commandBytes.byteLength) + commandBytes.byteLength + unsignedVarintByteLength(viewBytes.byteLength) + viewBytes.byteLength;
}

/** 📕️ Command-ingress size of one setContributions pack crossing — slim view, never the live document. */
export function wgpuContributionsIngressSize(command: unknown, viewState: unknown, sequence: number): { readonly commandBytes: number; readonly viewBytes: number; readonly ingressBytes: number; readonly ingressPages: number } {
  const commandBytes = encodePackValue(command);
  const viewBytes = encodePackValue(viewContextWireValue(viewState));
  const ingressBytes = wgpuCommandIngressByteLength(sequence, commandBytes, viewBytes);
  const ingressPages = Math.ceil(ingressBytes / 4096);
  return { commandBytes: commandBytes.byteLength, viewBytes: viewBytes.byteLength, ingressBytes, ingressPages };
}

export function wgpuSetContributionsCommand(pluginId: string, appId: string, json: string): { readonly address: { readonly owner: { readonly app: { readonly pluginId: string; readonly appId: string } }; readonly commandId: "setContributions" }; readonly arguments: { readonly json: string; readonly page: 0; readonly pageCount: 1 } } {
  return { address: { owner: { app: { pluginId, appId } }, commandId: "setContributions" }, arguments: { json, page: 0, pageCount: 1 } };
}

/** 🔁️ This target's half of {@link hostEffectInvocationV1}: the `(plugin, app, mode, window kind,
 * window instance)` a re-armed host effect is addressed in, read off the slim view state the
 * contributions crossing already carries. */
export function wgpuEffectDispatchScope(pluginId: string, appId: string, viewState: Record<string, unknown>): HostEffectDispatchScope {
  const text = (key: string, fallback: string): string => (typeof viewState[key] === "string" && (viewState[key] as string).length > 0 ? (viewState[key] as string) : fallback);
  const windowKindId = text("activeWindowKindId", text("active_window_kind_id", ""));
  return {
    pluginId,
    appId,
    modeId: text("activeModeId", text("active_mode_id", "")),
    windowKindId,
    windowInstanceId: text("windowId", text("window_id", text("focusedWindowId", text("focused_window_id", windowKindId)))),
  };
}

/** 📄️ JSON-safe leftover effects — `req` is a bigint on the wire and JSON.stringify refuses it. */
export function jsonEffects(effects: readonly Effect[]): unknown[] {
  return JSON.parse(JSON.stringify(effects, (_key, value) => (typeof value === "bigint" ? Number(value) : value))) as unknown[];
}

/** 📡️ A guest's `send-message` to its bound document backbone, projected onto the host `Effect` the
 * wgpu shell routes — `SendMessage { target: Backbone { uri }, payload }`, byte for byte. React's
 * document port consumes these at this layer; the wgpu shell owns its document actor itself
 * (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` `route_document_backbone_effects`), so the bridge hands them over
 * instead of dropping them. Any other effect answers `null`. */
export function wgpuBackboneMessageEffect(effect: WireVariant): Effect | null {
  const value = effect.val as { readonly target?: WireVariant; readonly payload?: unknown } | undefined;
  if (effect.tag !== "send-message" || value?.target?.tag !== "backbone" || typeof value.target.val !== "string") return null;
  return { sendMessage: { target: { backbone: { uri: value.target.val } }, payload: Array.from(coerceWireBytes(value.payload)) } };
}

/** 🔀️ The ONE wire→host effect projection of this bridge: document-backbone messages cross as
 * {@link wgpuBackboneMessageEffect}, everything else through the shared `wireEffectToFriendly`. */
export function wgpuHostEffect(effect: WireVariant): Effect | null {
  return wgpuBackboneMessageEffect(effect) ?? wireEffectToFriendly(effect, decodePackWire);
}

export function leftoverFriendlyEffects(instanceId: number, turns: readonly WireTurnResult[]): Effect[] {
  const leftover: WireVariant[] = [];
  for (const turn of turns) {
    for (const effect of turn.effects) {
      if (admitSpawnedJob(instanceId, effect)) continue;
      if (!shellFrameBytes(effect, instanceId)) leftover.push(effect);
    }
  }
  return leftover.map((effect) => wgpuHostEffect(effect)).filter((effect): effect is Effect => effect !== null);
}

/** 🧩️ Appends every non-`SendMessage{Shell}` effect of `turns` to this instance's leftover ledger.
 *
 * ⚖️ `pendingTurnEffects` is the ONE place a turn's leftover effects wait for the host, and
 * `performInvocation` is the only drain. Turns that ran outside a command — the instance-open
 * lifecycle and the acknowledgement turns its patch intake produced — used to be settled for their UI
 * patches alone, so a `flowEvalTick` the guest armed while the instance opened was dropped by the host
 * while the guest's own latch stayed armed, and nothing could ever re-arm it
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-blank-paint-2026-09-12.md` §6). Appends rather
 * than replaces: one instance can accumulate leftovers across several settles before the next drain. */
export function stashLeftoverEffects(instanceId: number, turns: readonly WireTurnResult[]): number {
  return stashLeftoverHostEffects(instanceId, turns.flatMap((turn) => turn.effects));
}

/** 🧩️ {@link stashLeftoverEffects} over an already-scanned effect list — the form the typed-operation
 * drive uses, since its result pages are transport that was consumed before the host ever sees them. */
export function stashLeftoverHostEffects(instanceId: number, effects: readonly WireVariant[]): number {
  const leftover = pendingTurnEffects.get(instanceId) ?? [];
  const before = leftover.length;
  for (const effect of effects) {
    if (admitSpawnedJob(instanceId, effect)) continue;
    const frame = shellFrameBytes(effect, instanceId);
    if (!frame || !shellFrameAnswersACaller(frame)) leftover.push(effect);
  }
  if (leftover.length === 0) return 0;
  pendingTurnEffects.set(instanceId, leftover);
  return leftover.length - before;
}

/** 🕹️ Whether one `Shell{instance}` `AppFrame` can still reach the caller waiting for it.
 *
 * ⚖️ `AppChannelClient` correlates every reply by `AppFrame::*.in_reply_to`, and sequences start at
 * 1 — so `in_reply_to: 0` names NO caller. A framework-reserved tool job's settled
 * `InvocationResult` is published exactly that way (`plugin_complete_reserved_spawned_job`,
 * `💻️os/🔨️modules/🔌️plugin/🦀️.rs`), because it answers a JOB rather than a command sequence, and
 * that result IS the whole answer of `interactionSelect`/`interactionHover`/`clearSelection`:
 * `output.interactionView` plus the app-declared refresh scope. Put on the frame lane it matched no
 * waiter, fell into the channel's `onOperationProgress` lane — which this target subscribes to
 * nowhere — and was dropped, so every pick answered `uiScope: undefined` (`UiDirtyScope::None` at
 * the shell), refreshed nothing, and left the guest's `World3dScene.selectionJson` empty however the
 * pick went (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️wgpu-selection-roundtrip-2026-09-15.md`).
 *
 * An uncorrelated frame therefore belongs in the LEFTOVER lane, which `wgpuInvocationFromFrames`
 * folds through `leftoverShellInvocationFrames` — the same law `PluginRuntime` reads for its own
 * job-completion leftovers. Undecodable bytes answer `true`: only a frame this host can positively
 * read as uncorrelated is rerouted. */
export function shellFrameAnswersACaller(payload: Uint8Array): boolean {
  try {
    const carrier = Object.values(decodeAppFrame(payload))[0] as { readonly in_reply_to?: unknown } | undefined;
    return carrier?.in_reply_to !== 0;
  } catch {
    return true;
  }
}

//#region 🧵️SpawnedJobs
/** 🧵️ Jobs this instance's guest asked the host to run, in arrival order, one entry per `job` id.
 *
 * ⚖️ A `spawn-job` is neither a shell frame nor a leftover the shell acts on: it is WORK THIS HOST
 * OWES THE GUEST. Before this lane the wgpu bridge had no case for it at all, so every framework
 * reserved tool verb — `interactionSelect`, `interactionHover`, `clearSelection` — reached
 * `wireEffectToFriendly`'s default arm and became a console warning: 18 per run on 6118, exactly two
 * per gesture. The gesture published a perfect wire message and was applied never
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-world3d-interaction-2026-09-13.md` §7). */
const pendingSpawnedJobs = new Map<number, Map<string, Extract<Effect, { readonly spawnJob: unknown }>["spawnJob"]>>();

/** 🧵️ How many job→completion→job rounds one drain admits before it refuses to keep going. A reserved
 * tool job completes in one round; the bound exists so a guest that re-spawns forever fails loudly
 * instead of wedging the actor's serialization chain. */
export const WGPU_SPAWNED_JOB_ROUNDS = 16;

/** 🧵️ What one spawned-job drain hands back: the shell frames its completions published, and the LAST
 * turn it actually drove. The status of that turn — not the caller's own last turn, which ended before
 * the job started — is what decides whether the guest still needs polling. */
export type WgpuSpawnedJobDrain = { readonly frames: readonly Uint8Array[]; readonly last: WireTurnResult | undefined };

/** 🧵️ Takes one `spawn-job` out of the effect stream, deduplicated by job id — the SAME effect is
 * observed twice per gesture (once as the command turn's host effect, once as the leftover drain's),
 * and starting a job twice is a guest-side identity collision, not a retry. */
export function admitSpawnedJob(instanceId: number, effect: WireVariant): boolean {
  if (effect.tag !== "spawn-job") return false;
  const { spawnJob } = wireSpawnJob(effect);
  const queue = pendingSpawnedJobs.get(instanceId) ?? new Map();
  const key = String(spawnJob.job);
  if (!queue.has(key)) {
    queue.set(key, spawnJob);
    pendingSpawnedJobs.set(instanceId, queue);
  }
  return true;
}

export function takeSpawnedJobs(instanceId: number): readonly Extract<Effect, { readonly spawnJob: unknown }>["spawnJob"][] {
  const queue = pendingSpawnedJobs.get(instanceId);
  if (!queue || queue.size === 0) return [];
  const jobs = [...queue.values()];
  pendingSpawnedJobs.delete(instanceId);
  return jobs;
}

export function forgetSpawnedJobs(instanceId: number): void {
  pendingSpawnedJobs.delete(instanceId);
}

//#endregion 🧵️SpawnedJobs

function effectTags(effects: readonly Effect[]): string[] {
  return effects.map((effect) => (typeof effect === "string" ? effect : Object.keys(effect)[0] ?? "unknown"));
}

/** 🎯️ Per-instance "leftover" `TurnResult.effects` — everything a turn produced that was NOT a
 * `SendMessage{Shell}` reply frame. Filled by `exchange` on every turn, drained by `performInvocation`
 * right after its own `client.command()` call resolves. */
export const pendingTurnEffects = new Map<number, WireVariant[]>();
