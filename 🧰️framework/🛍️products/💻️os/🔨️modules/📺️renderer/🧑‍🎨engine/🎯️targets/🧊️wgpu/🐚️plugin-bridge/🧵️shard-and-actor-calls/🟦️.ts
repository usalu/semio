import { type ActivationReason, ActivationRegistry, ON_EXTENSION_REQUEST_ACTIVATION_PREFIX } from "@semio-tech/framework";
import { ShardWorkerLike } from "@semio-tech/framework-actor/shard-client/quota";
import { createPooledActorRuntime, DEFAULT_SHARD_BUDGET, type PooledActorRuntime, SHARD_WORKER_URL } from "@semio-tech/framework-actor/shard-runtime";
import { type OriginalBootstrapResponse, RemoteOriginalBootstrapOwner } from "@semio-tech/framework-actor/retained-turn/bootstrap/port";
import { ShardClient } from "@semio-tech/framework-actor/shard-client";
import { rendererResidentLedger } from "@semio-tech/framework-canvas-ts/renderer-resident";
import type { ShardCommandIngressPage, ShardEventEnvelope } from "@semio-tech/framework-actor/shard-client/wire";
import { coerceTurnResult, type WireTurnResult } from "../../../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🔖️turn-result/🟦️.ts";

// #endregion 🔌️Imports

//#region 🧵️MainThreadShardWorkers
/** 🧵️ A shard worker that the UI ISOLATE owns, reached over a `MessagePort`.
 *
 * This frame worker cannot call `new Worker(...)` for a shard itself: a NESTED dedicated worker fails
 * to load outright in some embedded browsers — measured here, where even a one-line
 * `self.postMessage` worker failed with a message-less `error` event while the identical script loaded
 * fine from the page. That presented as all four shards dying at once and
 * `create_app promise failed: shard 0 terminated`, with no filename or line to point at.
 *
 * `ShardClient.createWorker` is synchronous, so this proxy is returned immediately and queues anything
 * sent before the port lands; the queue drains in order the moment it does. `terminate()` is relayed so
 * the UI isolate can drop the real worker, and a relayed `shard-worker-error` is re-raised on `onerror`
 * so `ShardClient`'s existing rebuild/failure ladder behaves exactly as with a direct `Worker`. */
class MainThreadShardWorker implements ShardWorkerLike {
  onmessage: ((event: { readonly data: unknown }) => void) | null = null;
  onerror: ((event: unknown) => void) | null = null;
  private port: MessagePort | null = null;
  private readonly queued: { readonly message: unknown; readonly transfer?: readonly Transferable[] }[] = [];
  private terminated = false;

  constructor(private readonly shardIndex: number) {
    routeShardPorts();
    shardPortWaiters.set(shardIndex, (port) => this.attach(port));
    (self as unknown as { postMessage(message: unknown): void }).postMessage({ kind: "shard-spawn", shardIndex, url: SHARD_WORKER_URL });
  }

  private attach(port: MessagePort): void {
    if (this.terminated) {
      port.close();
      return;
    }
    this.port = port;
    port.onmessage = (event: MessageEvent) => {
      const data = event.data as { readonly kind?: unknown; readonly message?: unknown } | null;
      if (data && typeof data === "object" && data.kind === "shard-worker-error") {
        this.onerror?.(data);
        return;
      }
      this.onmessage?.({ data: event.data });
    };
    port.start();
    for (const entry of this.queued.splice(0)) port.postMessage(entry.message, (entry.transfer ?? []) as Transferable[]);
  }

  postMessage(message: unknown, transfer?: readonly Transferable[]): void {
    if (this.terminated) return;
    if (!this.port) {
      this.queued.push({ message, transfer });
      return;
    }
    this.port.postMessage(message, (transfer ?? []) as Transferable[]);
  }

  terminate(): void {
    if (this.terminated) return;
    this.terminated = true;
    shardPortWaiters.delete(this.shardIndex);
    this.port?.close();
    this.port = null;
    this.queued.length = 0;
    (self as unknown as { postMessage(message: unknown): void }).postMessage({ kind: "shard-terminate", shardIndex: this.shardIndex });
  }
}

const shardPortWaiters = new Map<number, (port: MessagePort) => void>();

let shardPortsRouted = false;

/** 📬️ Arms the `shard-port` handoff route the UI isolate answers a `shard-spawn` with, on the
 * FIRST shard this isolate spawns rather than at module scope. The worker global only exists in the
 * isolate that spawns shards, so a module-scope `self.addEventListener` made merely IMPORTING this
 * bridge throw `ReferenceError: self is not defined` in every other host — which took the package's own
 * node test suite down at import, before a single test ran. */
function routeShardPorts(): void {
  if (shardPortsRouted) return;
  shardPortsRouted = true;
  self.addEventListener("message", (event: MessageEvent) => {
    const data = event.data as { readonly kind?: unknown; readonly shardIndex?: unknown; readonly port?: unknown } | null;
    if (!data || typeof data !== "object" || data.kind !== "shard-port" || typeof data.shardIndex !== "number") return;
    shardPortWaiters.get(data.shardIndex)?.(data.port as MessagePort);
    shardPortWaiters.delete(data.shardIndex);
  });
}

//#endregion 🧵️MainThreadShardWorkers

//#region 🔖️PooledSingletons
let pooledRuntime: PooledActorRuntime | null = null;

let originalWorkOwner: RemoteOriginalBootstrapOwner | undefined;

export function frameOriginalWorkOwner(): RemoteOriginalBootstrapOwner {
  if (originalWorkOwner) return originalWorkOwner;
  const owner = new RemoteOriginalBootstrapOwner(request => self.postMessage(request));
  self.addEventListener("message", (event: MessageEvent<OriginalBootstrapResponse>) => { if (event.data?.kind === "original-stock-result") owner.receiveResponse(event.data); });
  originalWorkOwner = owner;
  return owner;
}

export function getShardClient(): ShardClient {
  pooledRuntime ??= createPooledActorRuntime({
    createWorker: (shardIndex: number) => new MainThreadShardWorker(shardIndex),
    residentLedger: rendererResidentLedger(),
    originalWorkStock: frameOriginalWorkOwner(),
    heartbeatTimeoutMs: 180_000,
    onActorTrap: (actorId, message) => undefined,
    onShardLost: (shardIndex, actorIds) => {
      getActivationRegistry().handleShardLost(shardIndex, actorIds);
    },
  });
  return pooledRuntime.shardClient;
}

let sharedActivationRegistry: ActivationRegistry | null = null;

export function getActivationRegistry(): ActivationRegistry {
  sharedActivationRegistry ??= new ActivationRegistry({ shardClient: getShardClient(), defaultBudget: DEFAULT_SHARD_BUDGET });
  return sharedActivationRegistry;
}

/** 🧲️ The {@link ActivationReason} a request actor activates with: the WIT `on-extension-request`
 * event carrying the capability (the extension point) whose inbound request pulled it up. */
export function extensionRequestActivationReason(capability: string): ActivationReason {
  return `${ON_EXTENSION_REQUEST_ACTIVATION_PREFIX}${capability}`;
}

//#endregion 🔖️PooledSingletons

/** 🚦 Plain per-actor promise chain — never lets a second `turn()` start for the same `actorId` before
 * the previous one settles (the shard worker rejects, not queues, an overlapping turn). See this
 * file's header doc for why this is deliberately simpler than `PluginRuntime`'s lane/coalescing
 * `TurnScheduler`. */
export const actorTurnChains = new Map<string, Promise<unknown>>();

export function submitActorWork<T>(actorId: string, work: () => Promise<T>): Promise<T> {
  getActivationRegistry().touch(actorId);
  const previousSettled = (actorTurnChains.get(actorId) ?? Promise.resolve()).catch(() => undefined);
  const next = previousSettled.then(work);
  actorTurnChains.set(actorId, next);
  return next;
}

export function submitTurn(actorId: string, events: readonly ShardEventEnvelope[], commandPage?: ShardCommandIngressPage): Promise<WireTurnResult> {
  return submitActorWork(actorId, () => getShardClient().turn(actorId, events, DEFAULT_SHARD_BUDGET, commandPage)).then(coerceTurnResult);
}

/** 🧮️ Pending (not yet started) calls one actor may hold before {@link serializeWgpuActorCall} refuses —
 * the twin of `PluginRuntime`'s `SERIALIZE_PER_ACTOR_MAILBOX_CAPACITY` (256): generous, finite, and an
 * honest typed rejection instead of the unbounded promise chain that stood here before. */
export const WGPU_ACTOR_CALL_QUEUE_CAPACITY = 256;

/** 🚫️ The typed refusal {@link serializeWgpuActorCall} rejects with once an actor already holds
 * {@link WGPU_ACTOR_CALL_QUEUE_CAPACITY} pending calls — the wgpu spelling of the runtime's
 * `serializePerActor: actor …'s queue is full`. In-flight work is unaffected; only the new call is refused. */
export class WgpuActorCallQueueFullError extends Error {
  override readonly name = "WgpuActorCallQueueFullError";
  readonly code = "wgpu-actor-call.queue-full" as const;
  constructor(readonly actorId: string, readonly capacity: number) {
    super(`serializeWgpuActorCall: actor ${actorId}'s queue is full (>${capacity} pending calls) — rejected rather than growing unbounded`);
  }
}

type WgpuActorCallEntry = Readonly<{ order: number | undefined; start: () => void }>;

type WgpuActorCallQueue = { running: boolean; readonly pending: WgpuActorCallEntry[] };

const actorCallQueues = new Map<string, WgpuActorCallQueue>();

/** 🔗️ The mailbox's `## causal order` insertion rule (`🎭️actor/📬️mailbox/🟦️.ts`), applied to the
 * PENDING list only: an ordered entry goes immediately before the earliest pending entry with a
 * strictly larger `order` (equal `order` and unordered neighbours are not passed), else at the tail;
 * an unordered entry always goes at the tail. */
function insertWgpuActorCall(pending: WgpuActorCallEntry[], entry: WgpuActorCallEntry): void {
  if (entry.order !== undefined) {
    for (let index = 0; index < pending.length; index += 1) {
      const queued = pending[index]!.order;
      if (queued !== undefined && queued > entry.order) {
        pending.splice(index, 0, entry);
        return;
      }
    }
  }
  pending.push(entry);
}

function pumpWgpuActorCalls(actorId: string, queue: WgpuActorCallQueue): void {
  if (queue.running) return;
  const next = queue.pending.shift();
  if (next === undefined) {
    if (actorCallQueues.get(actorId) === queue) actorCallQueues.delete(actorId);
    return;
  }
  queue.running = true;
  next.start();
}

/** 🚦 Per-actor serialization at CALL granularity — the wgpu twin of `PluginRuntime`'s
 * `serializeCommandIngressForActor`.
 *
 * ⚖️ {@link submitActorWork} only keeps two TURNS from overlapping, which is all the shard worker
 * demands. It is not enough once a standing typed-operation drain shares the actor with host calls: a
 * drain poll slipping between two of a render's turns collects that render's own effects, so the
 * `dispatchAction` the guest armed for the host is reported as `effects=0` and the chain stops
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). A whole host call is therefore one unit here, and the
 * drain waits behind it. Callers must never nest: the command path reaches the actor through
 * `enqueue`, which is fire-and-forget.
 *
 * ## `order` (INPUT-CAUSALITY-LEDGER §2 B/F, law L2 — transport parity with the React target)
 * `order` is the input ledger's causal key for ONE dispatched input (`causalOrderKeyV1(provenance)` =
 * `causedBy ?? inputSeq`, `🏛️ShellHost/🎯️input-ledger/🟦️.ts`; threaded from
 * {@link WgpuPluginHandle.handleAction}/`handleCommand` via `PluginDispatchHintV1.order`). This queue
 * keeps one small ordered PENDING list per actor and applies exactly the mailbox's `## causal order`
 * rule at enqueue time ({@link insertWgpuActorCall}): a call WITH `order` is inserted immediately before
 * the earliest pending call of the same actor carrying a strictly larger `order` (else appended), so a
 * guest follow-up of input N runs before the already-queued input N+1; equal `order` keeps arrival
 * order. Calls WITHOUT `order` (render, refresh, drains, job/extension completions — maintenance)
 * are appended and never reorder among themselves, and an ordered call never overtakes an unordered
 * call queued ahead of every larger-ordered one. IN-FLIGHT WORK IS NEVER REORDERED: the running call
 * has already left the pending list, and nothing preempts it. Absent everywhere ⇒ plain FIFO, the
 * exact behaviour of the promise chain this replaced. Bounded: past
 * {@link WGPU_ACTOR_CALL_QUEUE_CAPACITY} pending calls the returned promise rejects with
 * {@link WgpuActorCallQueueFullError} and nothing is queued. */
export function serializeWgpuActorCall<T>(actorId: string, work: () => Promise<T>, order?: number): Promise<T> {
  return new Promise<T>((resolve, reject) => {
    let queue = actorCallQueues.get(actorId);
    if (queue === undefined) {
      queue = { running: false, pending: [] };
      actorCallQueues.set(actorId, queue);
    }
    if (queue.pending.length >= WGPU_ACTOR_CALL_QUEUE_CAPACITY) {
      reject(new WgpuActorCallQueueFullError(actorId, WGPU_ACTOR_CALL_QUEUE_CAPACITY));
      return;
    }
    const owner = queue;
    const start = (): void => {
      void Promise.resolve().then(work).then(resolve, reject).finally(() => {
        owner.running = false;
        pumpWgpuActorCalls(actorId, owner);
      });
    };
    insertWgpuActorCall(queue.pending, { order, start });
    pumpWgpuActorCalls(actorId, queue);
  });
}
