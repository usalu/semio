/** 🔌️ Command stall lines, hub catalog refresh and plugin install tuning, runtime diagnostics and the invoke-extension effect dispatch.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | ShellHost}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { SemioFaultError, type Effect, type Fault, type InvocationResponse } from "@semio-tech/framework";
import { encodePackValue } from "@semio-tech/framework-os";
import { type ActiveSession, type LoadedProgramState, type SpawnedAppEntry } from "../../🐚️Shell/🟦️.tsx";
import { resolveManifestLabel, beginCancellableExtensionRequest } from "../../🛠️ShellHelpers/🟦️.tsx";
import { type PluginExtensionCompletion, serializePerActor } from "../../🔌️PluginRuntime/🟦️.tsx";
import { isShardLostError } from "../../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";
import { hopTrace, type HopTraceDetail } from "../../../../../../../../🔨️modules/⏱️trace/🟦️.ts";
import { commandStallBandTextV1, commandStallHeldSecondsV1, type CommandStallV1 } from "../../🔌️PluginRuntime/⏱️command-stall/🟦️.ts";

//#region FrameworkOsShell
//#region 🐚️ShellMount
//#region 🧵️ConcurrencyHelpers
/** 🧮️ terra-web-shellhost (finding 2) — MIRRORS `PluginRuntime/🟦️.tsx`'s private
 * `poolConcurrency()` formula exactly (`min(hardwareConcurrency-1, 4)`, same `5` SSR/test fallback so
 * the clamp still lands on `4`): the boot-time install fan-out below must not run more concurrent
 * plugin activations than there are shard workers to service them (`PluginRuntime`'s own
 * `getShardClient` sizes its pool with this same formula) — requesting more would only add
 * `ActivationRegistry.evictForMemoryPressure` LRU thrashing for zero extra real parallelism, exactly
 * the reasoning `poolConcurrency()`'s own doc comment gives. This is a duplicate of that private
 * function, not a second invented bound — see this packet's `📓️terra-web-shellhost-report.md`
 * `## lease-requests` for the ask to export it from `PluginRuntime` instead, at which point this
 * function should be deleted in favor of importing it directly. */
/** ⏱️ One stall band line: the held program as the person knows it (its spawned window's breadcrumb, else its first app's)
 * and the held command by its own manifest label, in the person's language, with the seconds it has held its lane. */
export function commandStallLineV1(stall: CommandStallV1, programs: readonly LoadedProgramState[], spawnedApps: readonly SpawnedAppEntry[], terminology: string, locale: string, nowMs: number): string {
  const instanceId = Number(stall.actorId.slice(stall.actorId.lastIndexOf("#") + 1));
  const spawned = spawnedApps.find((entry) => entry.pluginId === stall.programId && entry.instanceId === instanceId);
  const program = programs.find((entry) => entry.handle.pluginId === stall.programId);
  const app = program?.manifest.apps.find((candidate) => candidate.id === spawned?.appId) ?? program?.manifest.apps[0];
  const programLabel = spawned ? spawned.breadcrumb.join(" · ") : app ? app.breadcrumb.join(" · ") : stall.programId;
  const actions = (program?.manifest.apps ?? []).flatMap((candidate) => [...(candidate.actions ?? []), ...candidate.windowKinds.flatMap((kind) => kind.actions ?? [])]);
  const action = stall.commandId === null ? undefined : actions.find((candidate) => candidate.id === stall.commandId);
  const commandLabel = stall.commandId === null ? null : action ? resolveManifestLabel(action.label, terminology, locale) : stall.commandId;
  return commandStallBandTextV1(programLabel, commandLabel, commandStallHeldSecondsV1(stall, nowMs), locale);
}

/** 🌎️ How often the shell re-reads the serving hub's catalog of plugin modules for its registry's hub half. */
export const HUB_CATALOG_REFRESH_MS = 60_000;

export function pluginInstallConcurrency(): number {
  const hardwareConcurrency = typeof navigator !== "undefined" && typeof navigator.hardwareConcurrency === "number" ? navigator.hardwareConcurrency : 5;
  return Math.max(1, Math.min(hardwareConcurrency - 1, 4));
}
//#endregion 🧵️ConcurrencyHelpers

//#region 🩺️RuntimeDiagnostics
/** 🩺️ The ONE key that arms this shell's per-action runtime traces — the refresh/completion
 * chatter a boot emits once per dispatched action. Off by default: a served boot of the
 * hexagonal-mushroom-column printed these on every one of ~1440 typed-operation completions
 * (`📓️runtime-verification-2026-09-09.md` boot #7), which is signal a perf run wants and an
 * interactive boot must never pay for.
 *
 * 🪞️ Mirrors the guest-side switch `RUNTIME_DIAGNOSTICS_ENV`
 * (`🧰️framework/🔨️modules/⏱️trace/🦀️.rs`) key for key, so one name arms both sides of the wire; the
 * two cannot share a declaration across the language boundary, so
 * `🧪️tests/🔬️engine-contract/🟦️.ts` pins the string instead
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export const RUNTIME_DIAGNOSTICS_KEY = "SEMIO_RUNTIME_DIAGNOSTICS";

/** 🩺️ Armed by `1`/`true`/`on`/`yes`; anything else, including absent, leaves it off. */
function runtimeDiagnosticsArmed(value: unknown): boolean {
  return typeof value === "string" && ["1", "true", "on", "yes"].includes(value.trim().toLowerCase());
}

/** 🖼️ Publishes the `commit` stage of one hop: from the moment a refresh's React state is
 * dispatched to the frame the compositor next runs, which is the only interval that contains React's
 * own reconcile/commit plus layout and paint. A host with no frame clock (test/SSR) publishes
 * nothing rather than inventing a number. */
export function traceCommitToNextFrame(detail: HopTraceDetail): void {
  if (typeof requestAnimationFrame !== "function") return;
  const close = hopTrace.open("commit", detail);
  requestAnimationFrame(() => close());
}

let runtimeDiagnosticsOverride: boolean | undefined;
let runtimeDiagnosticsResolved: boolean | undefined;

/** 🩺️ Arms or disarms the traces for this page, outranking build env and stored preference —
 * the browser's counterpart to the guest's `set_runtime_diagnostics`. */
export function setRuntimeDiagnostics(enabled: boolean | undefined): void {
  runtimeDiagnosticsOverride = enabled;
  runtimeDiagnosticsResolved = undefined;
}

/** 🩺️ Whether the shell's per-action traces may print. Resolved once per page: an explicit
 * override wins, then the build's `VITE_SEMIO_RUNTIME_DIAGNOSTICS`, then a `localStorage` key of the
 * same name so a live tab can be armed without a rebuild. Every reader is wrapped, because a
 * sandboxed tab throws on `localStorage` and a non-Vite host has no `import.meta.env`. */
export function runtimeDiagnosticsEnabled(): boolean {
  if (runtimeDiagnosticsOverride !== undefined) return runtimeDiagnosticsOverride;
  if (runtimeDiagnosticsResolved !== undefined) return runtimeDiagnosticsResolved;
  let armed = false;
  try {
    armed = runtimeDiagnosticsArmed((import.meta as { readonly env?: Record<string, unknown> }).env?.[`VITE_${RUNTIME_DIAGNOSTICS_KEY}`]);
  } catch {
    armed = false;
  }
  if (!armed) {
    try {
      armed = runtimeDiagnosticsArmed(globalThis.localStorage?.getItem(RUNTIME_DIAGNOSTICS_KEY));
    } catch {
      armed = false;
    }
  }
  runtimeDiagnosticsResolved = armed;
  return armed;
}
//#endregion 🩺️RuntimeDiagnostics

//#region 🔁️InvokeExtensionDispatch
function captureExtensionCompletion(requestingPlugin: LoadedProgramState, instanceId: number, req: bigint): PluginExtensionCompletion {
  const capture = requestingPlugin.handle.captureExtensionCompletion;
  if (typeof capture !== "function") throw new Error("extension.completion-unavailable");
  const completion = capture.call(requestingPlugin.handle, instanceId, req);
  if (!completion || typeof completion.complete !== "function" || typeof completion.assertActive !== "function") throw new Error("extension.completion-unavailable");
  if (completion.instanceId !== instanceId || completion.req !== req) throw new Error("extension.completion-owner-mismatch");
  completion.assertActive();
  return completion;
}

/** 🚑️ The typed fault an extension request carries when its callee's WORKER was taken down (the host
 * watchdog, or a crash) rather than the guest refusing. Retryable by construction: the shard is
 * rebuilt before this is even raised, so re-issuing the identical request is the correct response. */
export const EXTENSION_WORKER_LOST_FAULT = "extension.worker-lost";

async function runCapturedExtensionEffect(
  completion: PluginExtensionCompletion,
  extensionEntry: LoadedProgramState | undefined,
  extensionId: string,
  capability: string,
  requestJson: string,
  requesterActorKey: string,
): Promise<InvocationResponse> {
  completion.assertActive();
  const { instanceId, req } = completion;
  const unavailable = (code: string) => new SemioFaultError({
    origin: "os", code, severity: "error", message: code,
    scope: { pluginId: extensionId, instanceId: String(instanceId) }, retryable: false,
  });
  let outcome: { readonly ok: Uint8Array } | { readonly fault: Uint8Array };
  try {
    if (!extensionEntry) throw unavailable("extension.missing");
    // 📥️ `PluginWasmHandle.invoke` IS the ABI's inbound-request door (`🔌️PluginRuntime`'s own
    // `invoke` doc): it submits `Event::Request` on the extension's request actor and returns the
    // `respond` answer. The typed refusal below now only ever names a handle that genuinely has no
    // door — a test double, or a target that never adapted one — never a loaded extension.
    const { invoke } = extensionEntry.handle;
    if (typeof invoke !== "function") throw unavailable("extension.invoke-unavailable");
    // 🛑️ The door honours an `AbortSignal` at turn boundaries and `invoke` forwards one, but nothing
    // ever handed it one, so `cancelPreviewEval` reached the requesting guest's own bookkeeping and
    // never the request already parked on the extension actor. Registering the controller under the
    // REQUESTER's actor key is what lets a surface's declared cancel affordance retire it from
    // outside the per-actor queue those requests are serialized in — a guest-emitted cancel would
    // queue behind the very call it means to stop (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    const cancellation = beginCancellableExtensionRequest(requesterActorKey);
    let raw: Uint8Array;
    try {
      raw = await invoke.call(extensionEntry.handle, capability, requestJson, { originInstanceId: instanceId, signal: cancellation.signal });
    } finally {
      cancellation.finish();
    }
    completion.assertActive();
    // 📦️ The guest answers a capability in the capability's OWN encoding (`evaluate_invoke_json`
    // produces JSON text); the completion that carries it back is a `pack`, so the one re-encoding
    // happens here and nowhere else (`📓️extension-result-realloc-2026-09-10.md` §4.5).
    outcome = { ok: encodePackValue(JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(raw))) };
  } catch (error) {
    completion.assertActive();
    // 🚑️ A worker the host's own watchdog took down is NOT an anonymous invocation failure: it is a
    // recoverable, retryable loss of the callee, and the requester's surface must be able to say so
    // (`flow.extension-evaluate-failed` carries `faultCode` straight into the preview status). Naming
    // it `extension.invoke-failed` alongside a genuine guest refusal left the surface unable to
    // distinguish "the geometry kernel refused this" from "the geometry kernel's worker died"
    // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️extension-evaluate-budget-2026-09-12.md`).
    const fault: Fault = error instanceof SemioFaultError ? error.fault : isShardLostError(error) ? {
      origin: "os", code: EXTENSION_WORKER_LOST_FAULT, severity: "error", message: error instanceof Error ? error.message : String(error),
      scope: { pluginId: extensionId, instanceId: String(instanceId) }, retryable: true,
    } : {
      origin: "os", code: "extension.invoke-failed", severity: "error", message: error instanceof Error ? error.message : String(error),
      scope: { pluginId: extensionId, instanceId: String(instanceId) }, retryable: false,
    };
    outcome = { fault: encodePackValue(fault) };
    // 🩺 The DECODED fault, not just "something faulted": the completion this builds reaches the
    // requesting guest as opaque bytes, so this line is the only place a boot log can say WHICH
    // door refused (ticket 26/09/09/PROCEDURAL-3D-END-TO-END — 154 fault bytes that turned out to
    // be `extension.invoke-unavailable` on every evaluate, for eight boots running).
  }
  completion.assertActive();
  const response = await completion.complete(outcome);
  completion.assertActive();
  return response;
}

/** 🔁️ Captures one requester before evaluation and delivers only to that same activation. */
export async function runInvokeExtensionEffect(requestingPlugin: LoadedProgramState, extensionEntry: LoadedProgramState | undefined, instanceId: number, extensionId: string, capability: string, requestJson: string, req: bigint): Promise<InvocationResponse> {
  return runCapturedExtensionEffect(captureExtensionCompletion(requestingPlugin, instanceId, req), extensionEntry, extensionId, capability, requestJson, `${requestingPlugin.handle.pluginId}:${instanceId}`);
}

/** 🧩️ The program an extension request of `requester` reaches: the loaded program of that plugin id for the device's own
 * programs, the same-generation peer (`catalogModule.peers`) for a hub document's catalog-resolved program. */
export function extensionProgramForV1(plugins: readonly LoadedProgramState[], requester: LoadedProgramState, extensionId: string): LoadedProgramState | undefined {
  const programId = requester.catalogModule === undefined ? extensionId : requester.catalogModule.peers[extensionId];
  return programId === undefined ? undefined : plugins.find((entry) => entry.handle.pluginId === programId);
}

/** 📨️ Resolves an extension address and serializes requests belonging to one originating instance.
 *
 * 🪪️ `extensionId` IS a loaded program's `pluginId` — the one address this shell can resolve, and the
 * one the framework's own fixture pins (`🧫️fixtures/🔣️extension-invocation.json`). A producer whose
 * domain names extensions differently (flow calls the brep kernel `brep`, its plugin is
 * `flow-extension-brep`) translates on ITS side, where the owning plugin id is known — never here:
 * this shell stays domain-neutral and knows no topic vocabulary. The former fallback scanned
 * `manifest.contributions` for an `extensionId` field, which no manifest has ever carried (that
 * lane is `playbookBlockKind` rows; contributed extension payloads live under
 * `manifest.topicContributions[].payload`), so it matched nothing and every flow evaluation faulted
 * `extension.missing` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
 *
 * 🧩️ A hub document's catalog-resolved program addresses its extensions by plugin id too; they resolve to the programs
 * of the SAME catalog generation (`catalogModule.peers`), never to the device's own program of that plugin. */
export async function dispatchInvokeExtensionEffect(
  plugins: readonly LoadedProgramState[],
  requester: Pick<ActiveSession, "pluginId" | "instanceId">,
  invocation: Extract<Effect, { readonly invokeExtension: unknown }>["invokeExtension"],
  publish: (requestingPlugin: LoadedProgramState, response: InvocationResponse) => Promise<void>,
): Promise<void> {
  const { extensionId, capability, requestJson, req } = invocation;
  const requestingPlugin = plugins.find((entry) => entry.handle.pluginId === requester.pluginId);
  if (!requestingPlugin) throw new Error("extension.requester-unavailable");
  const extensionEntry = extensionProgramForV1(plugins, requestingPlugin, extensionId);
  const completion = captureExtensionCompletion(requestingPlugin, requester.instanceId, req);
  const requesterActorKey = `${requester.pluginId}:${requester.instanceId}`;
  const response = await serializePerActor(requesterActorKey, () => runCapturedExtensionEffect(completion, extensionEntry, extensionId, capability, requestJson, requesterActorKey));
  completion.assertActive();
  await publish(requestingPlugin, response);
}
//#endregion 🔁️InvokeExtensionDispatch
//#endregion 🐚️ShellMount
//#endregion FrameworkOsShell
