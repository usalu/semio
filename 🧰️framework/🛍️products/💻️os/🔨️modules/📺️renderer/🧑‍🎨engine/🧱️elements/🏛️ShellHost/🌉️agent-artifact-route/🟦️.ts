import { parseResolvedPluginViewState, type ResolvedPluginViewState } from "@semio-tech/framework";
/** 🌉️ Runs live agent artifact commands through the shell's owned action lane. */
import { shellAppFault, type ShellAppFrameV1, type ShellAppCommandV1 } from "@semio-tech/framework/mcp/channel";
import { spawnedIdOfWindowInstanceV1, spawnedWindowInstanceIdV1 } from "../🪟️spawned-program/🟦️.ts";
import { SEGMENTED_DOWNLOAD_CONTRACT } from "../../📤️SegmentedDownload/🟦️.ts";
import type { AppDocumentIdentity, MediaExportHandle, MediaExportStatus } from "@semio-tech/framework/wire-contracts";
import type { InputOutcomeV1, ShellInputActionV1 } from "../🎯️input-ledger/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

const exportOperationPolicy = schema.$defs.ExportOperationPolicy.const;
export type AgentArtifactTimingV1 = Readonly<{ now: () => number; yieldTurn: (delayMs: number) => Promise<void> }>;
const defaultTiming: AgentArtifactTimingV1 = { now: () => globalThis.performance.now(), yieldTurn: delayMs => new Promise(resolve => globalThis.setTimeout(resolve, delayMs)) };

type RefV1<T> = Readonly<{ current: T }>;
type VerbV1 = Readonly<{ id: string }>;
type AppV1 = Readonly<{ id: string; controllerId: string; actions?: readonly VerbV1[]; commands?: readonly VerbV1[]; modes?: readonly Readonly<{ id: string; commands?: readonly VerbV1[] }>[]; windowKinds: readonly Readonly<{ id: string; actions?: readonly VerbV1[] }>[] }>;
type ActionLifecycleV1 = Readonly<{ started: (control: Readonly<{ operationId: string; generation: string; cancel: () => void | Promise<unknown> }>) => void }>;
type AgentPluginPortsV1 = Readonly<{
  pluginId: string;
  readHistory?: (instanceId: number) => Promise<Readonly<{ cursor?: number; upserts?: readonly Readonly<{ seq: number; actionId?: string }>[] }> | null>;
  readAppDocumentPack?: (instanceId: number) => Promise<Readonly<{ pack: Uint8Array; spr: Uint8Array }> | null>;
  readAppDocumentIdentity?: (instanceId: number) => Promise<AppDocumentIdentity>;
  submitMediaExport?: (instanceId: number, port: string, documentId: string, revision: bigint) => Promise<MediaExportHandle>;
  pollMediaExport?: (instanceId: number, handle: MediaExportHandle) => Promise<MediaExportStatus>;
  cancelMediaExport?: (instanceId: number, handle: MediaExportHandle) => Promise<void>;
  takeMediaExportChunk?: (instanceId: number, handle: MediaExportHandle) => Promise<Readonly<{ handle: MediaExportHandle; data: Uint8Array; terminal: boolean }>>;
}>;
export type AgentArtifactPortsV1 = Readonly<{
  agentDeferredPlansRef: RefV1<Map<string, string>>;
  sessionRef: RefV1<Readonly<{ pluginId: string; instanceId: number; app: AppV1 }> | null | undefined>;
  spawnedAppsRef: RefV1<readonly Readonly<{ id: string; pluginId: string; appId: string; instanceId: number }>[]>;
  loadedPluginsRef: RefV1<readonly Readonly<{ handle: AgentPluginPortsV1; manifest: Readonly<{ apps: readonly AppV1[]; commands?: readonly VerbV1[] }> }>[]>;
  onActionRef: RefV1<(action: ShellInputActionV1, origin?: undefined, propagate?: false, lifecycle?: ActionLifecycleV1) => Promise<InputOutcomeV1>>;
  activeWindowIdRef: RefV1<string | null | undefined>;
}>;

type InvocationKindV1 = "action" | "appCommand" | "modeCommand" | "pluginCommand";
type AgentDeferredPlanV1 = Readonly<{ pluginId: string; appId: string; instanceId: string; windowKindId: string; deferredAction: string; invocationKind: InvocationKindV1; modeId: string; args: Record<string, unknown>; context?: ResolvedPluginViewState }>;
const actionWindowKind = (app: AppV1, action: string): string | null => app.windowKinds.find(kind => kind.actions?.some(declared => declared.id === action))?.id ?? (app.actions?.some(declared => declared.id === action) ? "*" : null);

/** 🧫️ Validates the language-neutral deferred plan before the shell acknowledges preparation. */
export function decodeAgentDeferredPlanV1(text: string): AgentDeferredPlanV1 | null {
  try {
    const value: unknown = JSON.parse(text);
    if (value === null || typeof value !== "object" || Array.isArray(value)) return null;
    const plan = value as Record<string, unknown>;
    if (![8, 9].includes(Object.keys(plan).length) || Object.keys(plan).some(key => !["pluginId","appId","instanceId","windowKindId","deferredAction","invocationKind","modeId","args","context"].includes(key)) || !["pluginId", "appId", "instanceId", "windowKindId", "deferredAction"].every(key => typeof plan[key] === "string" && plan[key].length > 0)) return null;
    if (typeof plan.invocationKind !== "string" || !["action", "appCommand", "modeCommand", "pluginCommand"].includes(plan.invocationKind) || typeof plan.modeId !== "string") return null;
    if (plan.args === null || typeof plan.args !== "object" || Array.isArray(plan.args)) return null;
    if (plan.context !== undefined) plan.context = parseResolvedPluginViewState(plan.context);
    return plan as AgentDeferredPlanV1;
  } catch {
    return null;
  }
}

/** 🗿️ Creates an instance-addressed route without mounting the shell chrome. */
export function createAgentArtifactRouteV1({ agentDeferredPlansRef, sessionRef, spawnedAppsRef, loadedPluginsRef, onActionRef, activeWindowIdRef }: AgentArtifactPortsV1, timing: AgentArtifactTimingV1 = defaultTiming): (request: Readonly<{ seq: bigint; instanceId: string; command: ShellAppCommandV1 }>) => Promise<readonly ShellAppFrameV1[]> {
    const deferred = agentDeferredPlansRef.current;
    const pending = new Map<string, { cancelled: boolean; cancel?: () => void | Promise<unknown> }>();
    const commandKey = (instanceId: string, seq: bigint | number | string): string => JSON.stringify([instanceId, String(seq)]);
    return async (request): Promise<readonly ShellAppFrameV1[]> => {
      if (request.command.kind === "cancel") {
        const operation = pending.get(commandKey(request.instanceId, request.command.cancelSeq));
        if (!operation) return [shellAppFault("app.command.rejected", `command ${request.command.cancelSeq} has no cancellable shell action for this instance`)];
        if (!operation.cancelled) {
          operation.cancelled = true;
          await operation.cancel?.();
        }
        return [shellAppFault("budget.exceeded", `the gateway cancelled command ${request.command.cancelSeq}`)];
      }
      const session = sessionRef.current;
      if (!session) return [shellAppFault("plugin.unavailable", "no plugin session is established in this shell — open an artifact here before driving it from an agent")];
      const spawnedTarget = spawnedAppsRef.current.find((entry) => String(entry.instanceId) === request.instanceId);
      const spawnedTargetApp = spawnedTarget ? loadedPluginsRef.current.find((entry) => entry.handle.pluginId === spawnedTarget.pluginId)?.manifest.apps.find((candidate) => candidate.id === spawnedTarget.appId) : undefined;
      const live =
        request.instanceId === String(session.instanceId)
          ? { pluginId: session.pluginId, instanceId: session.instanceId, app: session.app }
          : spawnedTarget && spawnedTargetApp
            ? { pluginId: spawnedTarget.pluginId, instanceId: spawnedTarget.instanceId, app: spawnedTargetApp }
            : null;
      if (!live) {
        const open = [`${session.pluginId}:${session.app.id}:${session.instanceId}`, ...spawnedAppsRef.current.map((entry) => `${entry.pluginId}:${entry.appId}:${entry.instanceId}`)].join(", ");
        return [shellAppFault("plugin.unavailable", `this shell has no open instance \`${request.instanceId}\` — the agent addressed an instance this shell has closed (open: ${open})`)];
      }
      const ownsPlan = (plan: AgentDeferredPlanV1): boolean => plan.pluginId === live.pluginId && plan.appId === live.app.id && plan.instanceId === String(live.instanceId);
      const declaredPlan = (plan: AgentDeferredPlanV1): boolean => {
        if (plan.invocationKind === "action") return plan.modeId === "" && actionWindowKind(live.app, plan.deferredAction) === plan.windowKindId;
        if (plan.windowKindId !== "*") return false;
        if (plan.invocationKind === "modeCommand") return (live.app.modes ?? []).some(mode => mode.id === plan.modeId && mode.commands?.some(command => command.id === plan.deferredAction));
        if (plan.modeId !== "") return false;
        const commands = plan.invocationKind === "appCommand" ? live.app.commands : loadedPluginsRef.current.find(entry => entry.handle.pluginId === live.pluginId)?.manifest.commands;
        return commands?.some(command => command.id === plan.deferredAction) ?? false;
      };
      const transactionKey = (txnId: string): string => JSON.stringify([live.pluginId, live.app.id, String(live.instanceId), txnId]);
      const readHistory = async (): Promise<ShellAppFrameV1> => {
        const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === live.pluginId)?.handle;
        if (!plugin?.readHistory) return shellAppFault("plugin.unavailable", `plugin \`${live.pluginId}\` exposes no history port in this shell`);
        const patch = await plugin.readHistory(live.instanceId);
        const head = [...(patch?.upserts ?? [])].sort((left, right) => right.seq - left.seq)[0];
        return { kind: "historySnapshot", artifactId: `${live.pluginId}:${live.app.id}:${live.instanceId}`, headEditId: head?.actionId ?? "", cursor: String(patch?.cursor ?? 0) };
      };
      let settledOutput: unknown;
      const dispatchVerb = async (action: string, args: Record<string, unknown>, windowKindId = "*", plan?: AgentDeferredPlanV1): Promise<ShellAppFrameV1 | null> => {
        const active = activeWindowIdRef.current;
        const spawnedIds = spawnedAppsRef.current.map(entry => entry.id);
        const activeOwner = active ? spawnedIdOfWindowInstanceV1(active, spawnedIds) : null;
        const ownsActive = spawnedTarget ? activeOwner === spawnedTarget.id : activeOwner === null;
        const useActive = windowKindId === "*" && active && ownsActive;
        const windowId = windowKindId === "*" ? live.app.windowKinds[0]?.id : windowKindId;
        const targetWindow = plan?.context?.windowId ?? (useActive ? active : windowId && spawnedTarget ? spawnedWindowInstanceIdV1(spawnedTarget.id, windowId) : windowId);
        if (!targetWindow) return shellAppFault("channel.not-wired", `app \`${live.app.id}\` declares no window through which this shell can dispatch an agent action`);
        const key = commandKey(request.instanceId, request.seq);
        if (pending.has(key)) return shellAppFault("transaction.instance-busy", `command ${request.seq} is already executing for this instance`);
        const operation: { cancelled: boolean; cancel?: () => void | Promise<unknown> } = { cancelled: false };
        pending.set(key, operation);
        try {
          const commandAddress: ShellInputActionV1["commandAddress"] = !plan || plan.invocationKind === "action" ? undefined : { commandId: action, owner: plan.invocationKind === "pluginCommand" ? { plugin: { pluginId: live.pluginId } } : plan.invocationKind === "modeCommand" ? { mode: { pluginId: live.pluginId, appId: live.app.id, modeId: plan.modeId } } : { app: { pluginId: live.pluginId, appId: live.app.id } } };
          const outcome = await onActionRef.current({ controllerId: live.app.controllerId, action: commandAddress ? `command:${action}` : action, ...(commandAddress ? { commandAddress } : {}), args, ...(plan?.context ? { callerViewState: plan.context } : {}), provenance: { origin: "agent", windowId: targetWindow, causedBy: null } }, undefined, false, { started: control => {
            operation.cancel = control.cancel;
            if (operation.cancelled) void Promise.resolve(control.cancel()).catch(() => undefined);
          } });
          if (operation.cancelled) return shellAppFault("budget.exceeded", `the gateway cancelled command ${request.seq}`);
          if (outcome.kind === "refused") return shellAppFault(outcome.reason === "viewer-read-only" ? "viewer.read-only" : outcome.reason === "mutation-rejected" ? "app.command.rejected" : "channel.not-wired", `the shell refused \`${action}\`: ${outcome.reason}${outcome.detail ? ` (${outcome.detail})` : ""}`);
          if (outcome.kind !== "applied") return shellAppFault("app.command.rejected", `the shell superseded \`${action}\` before it was applied`);
          settledOutput = outcome.output;
          return null;
        } finally {
          pending.delete(key);
        }
      };
      switch (request.command.kind) {
        case "readHistory":
          return [await readHistory()];
        case "pureCommand": {
          const prefix = `${live.pluginId}.${live.app.id}.`;
          let verb = request.command.capabilityId.startsWith(prefix) ? request.command.capabilityId.slice(prefix.length) : "";
          let invocationKind: InvocationKindV1 = "action", modeId = "", windowKindId = actionWindowKind(live.app, verb);
          if (windowKindId === null) {
            const appCommand = live.app.commands?.find(command => request.command.kind === "pureCommand" && request.command.capabilityId === `${prefix}cmd.${command.id}`);
            const mode = live.app.modes?.find(mode => mode.commands?.some(command => request.command.kind === "pureCommand" && request.command.capabilityId === `${prefix}mode.${mode.id}.${command.id}`));
            const modeCommand = mode?.commands?.find(command => request.command.kind === "pureCommand" && request.command.capabilityId === `${prefix}mode.${mode.id}.${command.id}`);
            const pluginCommand = loadedPluginsRef.current.find(entry => entry.handle.pluginId === live.pluginId)?.manifest.commands?.find(command => request.command.kind === "pureCommand" && request.command.capabilityId === `${live.pluginId}.cmd.${command.id}`);
            if (appCommand) { invocationKind = "appCommand"; verb = appCommand.id; windowKindId = "*"; }
            else if (mode && modeCommand) { invocationKind = "modeCommand"; modeId = mode.id; verb = modeCommand.id; windowKindId = "*"; }
            else if (pluginCommand) { invocationKind = "pluginCommand"; verb = pluginCommand.id; windowKindId = "*"; }
          }
          if (windowKindId === null) {
            return [shellAppFault("capability.not-found", `app \`${live.app.id}\` declares no action \`${verb}\` (from capability \`${request.command.capabilityId}\`)`)];
          }
          const args = request.command.input ?? {};
          if (typeof args !== "object" || Array.isArray(args)) return [shellAppFault("app.command.rejected", "the shell action arguments must be an object")];
          let context: ResolvedPluginViewState | undefined;
          try { context = request.command.context == null ? undefined : parseResolvedPluginViewState(request.command.context); } catch { return [shellAppFault("app.command.rejected", "invalid explicit caller context")]; }
          if (context?.activeModeId && !(live.app.modes ?? []).some(mode => mode.id === context.activeModeId)) return [shellAppFault("app.command.rejected", "caller mode belongs to no declared mode of this app")];
          if (invocationKind === "modeCommand" && context && context.activeModeId !== modeId) return [shellAppFault("app.command.rejected", "caller mode does not own this command")];
          if (context?.windowId) {
            const owner = spawnedIdOfWindowInstanceV1(context.windowId, spawnedAppsRef.current.map(entry => entry.id));
            if (spawnedTarget ? owner !== spawnedTarget.id : owner !== null) return [shellAppFault("app.command.rejected", "caller window belongs to another live instance")];
          }
          const plan = new TextEncoder().encode(JSON.stringify({ pluginId: live.pluginId, appId: live.app.id, instanceId: String(live.instanceId), windowKindId, invocationKind, modeId, deferredAction: verb, args, ...(context ? { context } : {}) }));
          return [{ kind: "emit", ops: { document: [plan], config: [], draft: [] }, warnings: ["This live shell prepares a deferred action; domain argument validation and mutation occur at commit."] }];
        }
        case "transactionPrepare": {
          const ops = request.command.ops;
          if (ops.document.length !== 1 || ops.config.length !== 0 || ops.draft.length !== 0) return [shellAppFault("app.command.rejected", "the shell route requires exactly one deferred document plan and no config or draft operations")];
          const text = new TextDecoder().decode(ops.document[0]);
          const plan = decodeAgentDeferredPlanV1(text);
          if (!plan || !ownsPlan(plan) || !declaredPlan(plan)) return [shellAppFault("app.command.rejected", "the deferred plan does not name a declared invocation of this live instance")];
          const key = transactionKey(request.command.txnId);
          if (deferred.has(key)) return [shellAppFault("transaction.generation-mismatch", "this live instance already holds a prepared plan for this transaction")];
          deferred.set(key, text);
          return [{ kind: "transactionPrepared", txnId: request.command.txnId }];
        }
        case "transactionRollback":
          deferred.delete(transactionKey(request.command.txnId));
          return [{ kind: "transactionRolledBack", txnId: request.command.txnId }];
        case "transactionCommit": {
          const key = transactionKey(request.command.txnId);
          const plan = deferred.get(key);
          deferred.delete(key);
          if (plan === undefined) return [shellAppFault("transaction.generation-mismatch", `this shell holds no prepared plan for transaction \`${request.command.txnId}\``)];
          const parsed = decodeAgentDeferredPlanV1(plan);
          if (!parsed || !ownsPlan(parsed) || !declaredPlan(parsed)) return [shellAppFault("app.command.rejected", "the prepared plan no longer names a declared invocation of this live instance")];
          const refusal = await dispatchVerb(parsed.deferredAction, parsed.args, parsed.windowKindId, parsed);
          if (refusal) return [refusal];
          const after = await readHistory();
          if (after.kind !== "historySnapshot") return [after];
          return [{ kind: "transactionCommitted", txnId: request.command.txnId, editId: after.headEditId, ...(settledOutput === undefined ? {} : { output: settledOutput }) }];
        }
        case "transactionUndo":
        case "transactionRedo": {
          const verb = request.command.kind === "transactionUndo" ? "undo" : "redo";
          const refusal = await dispatchVerb(verb, {});
          if (refusal) return [refusal];
          return request.command.kind === "transactionUndo" ? [{ kind: "transactionUndone", groupId: request.command.groupId }] : [{ kind: "transactionRedone", groupId: request.command.groupId }];
        }
        case "readArtifact": {
          const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === live.pluginId)?.handle;
          if (!plugin?.readAppDocumentPack) return [shellAppFault("plugin.unavailable", `plugin \`${live.pluginId}\` exposes no document port in this shell — an agent needing a genesis read must resolve a headless context (\`--folder\`/\`--hub\`)`)];
          const document = await plugin.readAppDocumentPack(live.instanceId);
          if (!document) return [shellAppFault("plugin.unavailable", `plugin \`${live.pluginId}\` instance ${live.instanceId} holds no document to read`)];
          return [{ kind: "artifact", pack: document.pack, spr: document.spr }];
        }
        case "exportMedia": {
          const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === live.pluginId)?.handle;
          if (!plugin?.submitMediaExport || !plugin.readAppDocumentIdentity || !plugin.pollMediaExport || !plugin.cancelMediaExport || !plugin.takeMediaExportChunk) return [shellAppFault("plugin.unavailable", `plugin \`${live.pluginId}\` exposes no bounded media export port in this shell`)];
          const cancelExport = plugin.cancelMediaExport;
          const key = commandKey(request.instanceId, request.seq);
          if (pending.has(key)) return [shellAppFault("transaction.instance-busy", `command ${request.seq} is already executing for this instance`)];
          const operation: { cancelled: boolean; cancel?: () => void | Promise<unknown> } = { cancelled: false };
          pending.set(key, operation);
          let handle: MediaExportHandle | undefined;
          const cancelled = (): readonly ShellAppFrameV1[] => [shellAppFault("budget.exceeded", `the gateway cancelled export command ${request.seq}`)];
          const sameHandle = (candidate: MediaExportHandle): boolean => handle !== undefined && ["app_instance_id", "parent_document_id", "operation_id", "base_revision", "generation"].every(field => candidate[field as keyof MediaExportHandle] === handle![field as keyof MediaExportHandle]);
          try {
            const started = timing.now();
            let latest = started;
            const checkpoint = (): void => {
              const now = timing.now();
              if (!Number.isFinite(started) || started < 0 || !Number.isFinite(now) || now < latest) throw new Error("media-export.clock");
              latest = now;
              if (now - started >= exportOperationPolicy.maximumDurationMs) throw new Error("media-export.deadline");
            };
            checkpoint();
            const identity = await plugin.readAppDocumentIdentity(live.instanceId);
            checkpoint();
            if (identity.app_instance_id !== live.instanceId || !identity.parent_document_id) throw new Error("media-export.document-owner");
            if (operation.cancelled) return cancelled();
            handle = await plugin.submitMediaExport(live.instanceId, request.command.port, identity.parent_document_id, identity.base_revision);
            if (handle.app_instance_id !== live.instanceId || handle.parent_document_id !== identity.parent_document_id || handle.base_revision !== identity.base_revision) throw new Error("media-export.authority-mismatch");
            let cancelSent = false;
            operation.cancel = async () => { if (cancelSent) return; cancelSent = true; await cancelExport(live.instanceId, handle!); };
            if (operation.cancelled) await operation.cancel();
            let complete: Awaited<ReturnType<typeof plugin.pollMediaExport>> | undefined;
            for (;;) {
              checkpoint();
              const status = await plugin.pollMediaExport(live.instanceId, handle);
              checkpoint();
              if (!sameHandle(status.handle)) throw new Error("media-export.status-owner");
              if (status.state === "cancelled") return cancelled();
              if (status.state === "failed") throw new Error(status.detail || "media-export.failed");
              if (status.state === "complete") { complete = status; break; }
              if (operation.cancelled) await operation.cancel();
              await timing.yieldTurn(exportOperationPolicy.pollIntervalMs);
            }
            if (complete.total_bytes < 0n || complete.total_bytes > BigInt(SEGMENTED_DOWNLOAD_CONTRACT.maximumTotalBytes) || complete.descriptor.length === 0 || complete.descriptor.length > exportOperationPolicy.maximumDescriptorBytes) throw new Error("media-export.capacity-or-descriptor");
            const chunks: Uint8Array[] = [];
            let bytes = 0;
            for (let page = 0; page < SEGMENTED_DOWNLOAD_CONTRACT.maximumOutstandingChunks; page++) {
              checkpoint();
              const chunk = await plugin.takeMediaExportChunk(live.instanceId, handle);
              checkpoint();
              if (!sameHandle(chunk.handle)) throw new Error("media-export.chunk-owner");
              bytes += chunk.data.byteLength;
              if (chunk.data.byteLength > SEGMENTED_DOWNLOAD_CONTRACT.chunkBytes || bytes > SEGMENTED_DOWNLOAD_CONTRACT.maximumTotalBytes || BigInt(bytes) > complete.total_bytes) throw new Error("media-export.capacity");
              if (!operation.cancelled && chunk.data.byteLength > 0) chunks.push(chunk.data);
              if (chunk.terminal) {
                if (BigInt(bytes) !== complete.total_bytes) throw new Error("media-export.length");
                if (operation.cancelled) return cancelled();
                const data = new Uint8Array(bytes);
                let offset = 0;
                for (const chunk of chunks) { data.set(chunk, offset); offset += chunk.byteLength; }
                return [{ kind: "exported", port: request.command.port, descriptor: Uint8Array.from(complete.descriptor), data }];
              }
              if (chunk.data.byteLength === 0) throw new Error("media-export.empty-page");
              await timing.yieldTurn(0);
            }
            throw new Error("media-export.page-capacity");
          } catch (error) {
            let detail = error instanceof Error ? error.message : "bounded media export failed";
            const code = operation.cancelled || detail === "media-export.deadline" ? "budget.exceeded" : "app.command.rejected";
            try { await operation.cancel?.(); } catch (cleanup) { detail += `; cancellation failed: ${cleanup instanceof Error ? cleanup.message : String(cleanup)}`; }
            return [shellAppFault(code, detail)];
          } finally {
            pending.delete(key);
          }
        }
      }
    };

}
