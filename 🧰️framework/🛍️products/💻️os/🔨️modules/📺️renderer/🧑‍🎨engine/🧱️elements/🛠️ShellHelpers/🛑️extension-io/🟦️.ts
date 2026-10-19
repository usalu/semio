/** 🛑️ Extension request cancellation, latest-wins, directional and coalescing dispatchers and the Puzzle3d brush mesh upload queue.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | ShellHelpers}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { blake3Hex } from "@semio-tech/framework";

//#region ShellHelpers
/** 🎚️ Serializes async updates while retaining only the newest value requested during an in-flight update. */
export function createLatestAsyncDispatcher<T>(dispatchValue: (value: T) => unknown): (value: T) => void {
  let running = false;
  let queued: T | undefined;
  let hasQueued = false;
  const dispatchLatest = (value: T) => {
    if (running) {
      queued = value;
      hasQueued = true;
      return;
    }
    running = true;
    void Promise.resolve(dispatchValue(value)).finally(() => {
      running = false;
      if (!hasQueued) return;
      const next = queued as T;
      queued = undefined;
      hasQueued = false;
      dispatchLatest(next);
    });
  };
  return dispatchLatest;
}

/** ↕️ Serializes numeric slider updates while retaining every direction change and coalescing movement within one direction. */
export function createDirectionalAsyncDispatcher(dispatchValue: (value: number) => unknown): (value: number) => void {
  let running = false;
  let active = 0;
  const queued: number[] = [];
  const dispatchNext = (value: number) => {
    running = true;
    active = value;
    void Promise.resolve(dispatchValue(value)).finally(() => {
      const next = queued.shift();
      if (next === undefined) {
        running = false;
        return;
      }
      dispatchNext(next);
    });
  };
  return (value) => {
    if (!running) {
      dispatchNext(value);
      return;
    }
    const previous = queued.at(-1);
    if (previous === undefined) {
      if (value !== active) queued.push(value);
      return;
    }
    const anchor = queued.at(-2) ?? active;
    const direction = Math.sign(previous - anchor);
    const nextDirection = Math.sign(value - previous);
    if (nextDirection === 0) return;
    if (direction === 0 || nextDirection === direction) queued[queued.length - 1] = value;
    else queued.push(value);
    if (queued.length > 2) queued.splice(0, queued.length - 2);
  };
}

/** 🖱️ Overlay chrome for a world-3d marquee method. `lasso` draws a polygon; `rectangle` and
 * `pick` (click picks, drag sweeps a box — puzzle3d's default) both draw the axis-aligned rectangle. */
export function world3dMarqueeOverlayShape(method: string): "rect" | "polygon" | null {
  if (method === "lasso") return "polygon";
  if (method === "rectangle" || method === "pick") return "rect";
  return null;
}

//#region 🛑️ExtensionRequestCancellation
/**
 * 🛑️ Every extension request currently in flight, keyed by the REQUESTING actor
 * (`<pluginId>:<instanceId>`) — the same key `serializePerActor` already serializes those requests
 * under, because they are the same set of calls.
 *
 * 🚪️ Why the host owns this and not the guest: all of one instance's extension invocations are
 * serialized under that one key, so a cancel the guest emitted could not overtake the request it
 * means to stop — it would queue BEHIND it. Only the host, outside that queue, can abort a call
 * already handed to the door. `driveInboundRequest` reads the signal at turn boundaries, so a parked
 * multi-turn request is retired and a turn already handed to the worker is never half-abandoned.
 */
const inFlightExtensionRequestsByActor = new Map<string, Set<AbortController>>();

/**
 * 🛑️ Action ids a mounted surface has DECLARED as its cancel affordance — the surface reads
 * the id off its own status contract (`World3dScene.statusJson`'s `cancelAction`) and registers it
 * here while that status says `cancellable`. This is what keeps the shell domain-neutral: it never
 * learns a plugin's verb from code, only from the surface that is currently offering it.
 */
const declaredSurfaceCancelActions = new Map<string, number>();

/** 🛑️ Registers one in-flight extension request and hands back its signal plus a retirement. */
export function beginCancellableExtensionRequest(actorKey: string): { readonly signal: AbortSignal; readonly finish: () => void } {
  const controller = new AbortController();
  let live = inFlightExtensionRequestsByActor.get(actorKey);
  if (!live) {
    live = new Set();
    inFlightExtensionRequestsByActor.set(actorKey, live);
  }
  live.add(controller);
  return {
    signal: controller.signal,
    finish: () => {
      const set = inFlightExtensionRequestsByActor.get(actorKey);
      if (!set) return;
      set.delete(controller);
      if (set.size === 0) inFlightExtensionRequestsByActor.delete(actorKey);
    },
  };
}

/** 🛑️ Aborts every extension request `actorKey` has in flight. Returns how many were aborted. */
export function abortExtensionRequestsForActor(actorKey: string, reason: string): number {
  const live = inFlightExtensionRequestsByActor.get(actorKey);
  if (!live || live.size === 0) return 0;
  const aborted = live.size;
  for (const controller of [...live]) controller.abort(new Error(reason));
  inFlightExtensionRequestsByActor.delete(actorKey);
  return aborted;
}

/** 🔎️ How many extension requests `actorKey` has in flight — readable so a law can state it. */
export function inFlightExtensionRequestCount(actorKey: string): number {
  return inFlightExtensionRequestsByActor.get(actorKey)?.size ?? 0;
}

/** 🛑️ Declares `actionId` as a mounted surface's live cancel affordance; call the returned
 * retirement when the surface stops offering it (it unmounted, or its status stopped being
 * cancellable). Reference-counted, because two preview windows may offer the same verb. */
export function declareSurfaceCancelAction(actionId: string): () => void {
  if (!actionId) return () => undefined;
  declaredSurfaceCancelActions.set(actionId, (declaredSurfaceCancelActions.get(actionId) ?? 0) + 1);
  let retired = false;
  return () => {
    if (retired) return;
    retired = true;
    const count = (declaredSurfaceCancelActions.get(actionId) ?? 1) - 1;
    if (count <= 0) declaredSurfaceCancelActions.delete(actionId);
    else declaredSurfaceCancelActions.set(actionId, count);
  };
}

/** 🔎️ Whether `actionId` is a cancel affordance some mounted surface is currently offering. */
export function isDeclaredSurfaceCancelAction(actionId: string): boolean {
  return declaredSurfaceCancelActions.has(actionId);
}
//#endregion 🛑️ExtensionRequestCancellation

/**
 * 🚦️ Fires `run` at most once at a time — interval ticks that arrive while a previous run is still
 * in flight are dropped (not queued), so a slow program tick cannot unbounded-queue into the serialized
 * WASM handle.
 */
export function createInFlightSkippingInterval<Timer>(run: () => unknown, delayMs: number, setIntervalFn?: (callback: () => void, delayMs: number) => Timer, clearIntervalFn?: (timer: Timer) => void): () => void {
  let cancelled = false;
  let inFlight = false;
  const tick = () => {
    if (cancelled || inFlight) return;
    inFlight = true;
    void Promise.resolve(run()).finally(() => {
      inFlight = false;
    });
  };
  let disposeTimer: () => void;
  if (setIntervalFn && clearIntervalFn) {
    const timer = setIntervalFn(tick, delayMs);
    disposeTimer = () => clearIntervalFn(timer);
  } else {
    if (setIntervalFn || clearIntervalFn) throw new Error("interval scheduling and cancellation ports must be supplied together");
    const timer = setInterval(tick, delayMs);
    disposeTimer = () => clearInterval(timer);
  }
  return () => {
    cancelled = true;
    disposeTimer();
  };
}

/**
 * 🎯️ Coalesces rapid dispatches to the latest value — skips when unchanged and keeps at most one
 * in-flight round trip (used by World3dHost hover so pointermove cannot flood the WASM handle).
 */
export function createCoalescingActionDispatcher<T>(dispatch: (value: T) => unknown, isEqual: (a: T, b: T) => boolean = (a, b) => Object.is(a, b)): (value: T) => void {
  let inFlight = false;
  let pending: T | undefined;
  let lastSent: T | undefined;
  const flush = () => {
    if (inFlight || pending === undefined) return;
    const next = pending;
    pending = undefined;
    if (lastSent !== undefined && isEqual(lastSent, next)) return;
    lastSent = next;
    inFlight = true;
    const release = () => {
      inFlight = false;
      flush();
    };
    void Promise.resolve(dispatch(next)).then(release, release);
  };
  return (value: T) => {
    if (pending === undefined && lastSent !== undefined && isEqual(lastSent, value)) return;
    pending = value;
    flush();
  };
}

//#region 🥽️Puzzle3dBrushMeshUpload
/** 📏️ Raw JSON bytes one retained puzzle command admits — `PUZZLE_COMMAND_RAW_BYTES`
 * (`✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs`). A whole GLB never fitted it: the Nakagin
 * capsule `🧊️placeholder.glb` is 25 344 positions plus 48 384 indices, 64 KB as JSON number arrays,
 * so real collision geometry reaches the plugin only as a page run. */
export const PUZZLE3D_MESH_COMMAND_RAW_BYTES = 8_192;

/** 📏️ Values — positions and indices counted together — one page carries at most. Mirrors
 * `PUZZLE3D_MESH_PAGE_VALUES` (`✏️editor/⏳️precompute/🦀️.rs`); a mesh id long enough to crowd the
 * envelope shrinks its own pages below this instead of overrunning the wire. */
export const PUZZLE3D_MESH_PAGE_VALUES = 1_024;

/** 🔤️ Base64 padding characters reserved per page — one group per payload string. */
const PUZZLE3D_MESH_PAGE_PADDING_CHARS = 8;

/** 📦️ Partial uploads the plugin stages at once — `PUZZLE3D_MESH_UPLOAD_SLOTS`
 * (`✏️editor/⏳️precompute/🦀️.rs`). The host drains its queue one page at a time, so it only ever holds
 * one run open; the count bounds how many runs may wait behind it. */
export const PUZZLE3D_MESH_UPLOAD_SLOTS = 4;

/** 🧮️ Longest page run one mesh identity may claim — `PUZZLE3D_MESH_UPLOAD_MAX_PAGES`
 * (`✏️editor/⏳️precompute/🦀️.rs`). A mesh needing more pages than this is beyond what the collision
 * engine admits at all, so it is never paged onto the wire. */
export const PUZZLE3D_MESH_UPLOAD_MAX_PAGES = 384;

/** 📦️ Pages the host may hold undispatched: every staging slot the plugin owns, each at its own run
 * ceiling. A run that would overrun this is refused here rather than queued into unbounded memory. */
export const PUZZLE3D_MESH_UPLOAD_QUEUE_PAGES = PUZZLE3D_MESH_UPLOAD_SLOTS * PUZZLE3D_MESH_UPLOAD_MAX_PAGES;

/** 🚚️ How many times one mesh identity's guest-side re-upload request may be claimed under a single
 * guest instantiation — see {@link Puzzle3dBrushMeshRegistry.claimReupload}. Two: the page run the
 * request asks for, plus exactly one retry for a run that failed part-way (a `Gap`, a refused digest, a
 * world host unmounted mid-drain). A third claim can only ever repeat an outcome the first two already
 * produced, and each one costs a full `PUZZLE3D_MESH_UPLOAD_MAX_PAGES`-bounded run on the serialized
 * per-actor command lane, which is what every user interaction then queues behind. */
export const PUZZLE3D_MESH_REUPLOAD_CLAIMS = 2;

/** 🥽️ One page of a `registerBrushMesh` upload run. Positions fill a page first; the indices stream
 * continues in whatever of the page's value budget is left, so the run is dense and its last page is
 * the only partial one. */
export type Puzzle3dBrushMeshPage = {
  readonly url: string;
  readonly digest: string;
  readonly page: number;
  readonly pageCount: number;
  readonly positionsB64?: string;
  readonly indicesB64?: string;
};

/** 🥽️ What this page believes ONE guest instantiation holds: mesh identities it paged, by id, with the
 * digest that was uploaded — so a later window adopts the same geometry by `{url, digest}` alone
 * instead of paging it again, and a mesh whose bytes changed uploads afresh.
 *
 * The claim is bounded by the guest's lifetime, never by the tab's. A plain `Map` outlived every fact
 * it recorded: the plugin's own mesh store is a `static OnceLock` in the wasm instantiation
 * (`✏️editor/⏳️precompute/🦀️.rs`), so a restored actor holds nothing, and the seven id-only
 * re-announcements that every later window activation then sent were each refused and each dropped on
 * the floor — the brush utility silently kept no collision geometry until a full page reload. Two
 * guest-published facts close that hole, both on `interactionJson`:
 *
 * - `meshResidency` — the guest's monotone install counter. It only ever climbs inside one
 *   instantiation and starts at zero in a fresh one, so a value below the high-water mark this page
 *   saw is proof of a restart and voids every entry ({@link observeResidency}).
 * - `meshReuploadUrls` — identities a refused id-only announcement is waiting on bytes for
 *   ({@link claimReupload}), claimed at most {@link PUZZLE3D_MESH_REUPLOAD_CLAIMS} times per guest
 *   instantiation so a republished stale scene cannot re-drive an upload that already ran. */
export class Puzzle3dBrushMeshRegistry {
  #residency = -1;
  readonly #entries = new Map<string, { readonly digest: string; readonly paged: boolean }>();
  readonly #claims = new Map<string, number>();
  readonly #refusedAlias = new Set<string>();

  /** 🔄️ Folds one published `meshResidency` in. Answers `true` exactly when the count went backwards —
   * the guest was re-instantiated and holds nothing this page uploaded — having dropped every claim. */
  observeResidency(installs: number): boolean {
    if (!Number.isFinite(installs) || installs < 0) return false;
    const restarted = installs < this.#residency;
    this.#residency = installs;
    if (restarted) {
      this.#entries.clear();
      this.#claims.clear();
      this.#refusedAlias.clear();
    }
    return restarted;
  }

  /** 🔢️ The last residency this registry observed, `-1` before the guest published one. */
  get residency(): number {
    return this.#residency;
  }

  holds(url: string, digest: string): boolean {
    return this.#entries.get(url)?.digest === digest;
  }

  /** 🪢️ True when this guest holds that GEOMETRY, PAGED, under some id. Mesh ids are distinct identities
   * to the collision engine, but the bytes behind them are content-addressed — every `dist/mesh/*.glb` in
   * this repo is the same capsule — so a second id whose digest is already resident is announced by
   * `{url, digest}` and aliased guest-side (`adopt_brush_mesh_by_digest`, `✏️editor/⏳️precompute/🦀️.rs`)
   * instead of being paged again. Only a PAGED entry answers: an entry that is itself an alias proves
   * nothing about what the guest holds, so a chain of aliases can never stand in for the bytes. */
  holdsDigest(digest: string): boolean {
    if (digest.length === 0) return false;
    for (const held of this.#entries.values()) {
      if (held.paged && held.digest === digest) return true;
    }
    return false;
  }

  /** ✅️ Records an upload as this guest's, at the residency it was dispatched under. Called when the
   * run's LAST page goes out, never when it is queued: a queued run confirms nothing. */
  confirm(url: string, digest: string): void {
    this.#entries.set(url, { digest, paged: true });
    this.#refusedAlias.delete(url);
  }

  /** 🪢️ Records an identity carried by {@link holdsDigest} alone — the bytes crossed under a sibling id.
   * Never proof for a third id ({@link holdsDigest} ignores it), and withdrawn the moment the guest says
   * it cannot serve this identity ({@link claimReupload}), which is what stops a guest that refuses an
   * alias from being handed the same alias forever instead of the bytes. */
  alias(url: string, digest: string): void {
    this.#entries.set(url, { digest, paged: false });
  }

  /** 🪢️ Whether this identity may still be announced by digest alone. False once the guest refused an
   * alias for it — the next announcement pages the bytes. */
  mayAlias(url: string): boolean {
    return !this.#refusedAlias.has(url);
  }

  forget(url: string): void {
    this.#entries.delete(url);
  }

  /** 🚚️ Claims one guest-side re-upload request, dropping the stale claim. `false` once this identity
   * has been claimed {@link PUZZLE3D_MESH_REUPLOAD_CLAIMS} times under the live guest instantiation.
   *
   * 🐛️ The gate used to be `residency <= #repaged[url]` — "at most one claim per published residency
   * value" — and `meshResidency` is the guest's own install counter, which EVERY accepted announcement
   * increments (`derive_brush_mesh`/`adopt_brush_mesh_by_digest`, `✏️editor/⏳️precompute/🦀️.rs`). So the
   * brake was moved by the very traffic it existed to suppress: one standing request that outlived its
   * identity's install re-opened the gate on every unit of progress anywhere in the tab, each claim
   * deleted the paged entry and permanently set {@link mayAlias} false, and the next announcement was
   * therefore a full 72-command page run instead of a one-command adopt. Measured at wasm #58 on the
   * 180-object Nakagin document: `registerBrushMesh` still arriving 8 minutes after the example switch
   * (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B46 §5, wave B48 §1.2).
   *
   * A count is a fact the guest's accepted work cannot move, and only a RESTART
   * ({@link observeResidency}) or a deliberate {@link clear} resets it — so the announce is bounded per
   * document per guest, with exactly one retry left for a page run that failed mid-way. */
  claimReupload(url: string): boolean {
    const claimed = this.#claims.get(url) ?? 0;
    if (claimed >= PUZZLE3D_MESH_REUPLOAD_CLAIMS) return false;
    this.#claims.set(url, claimed + 1);
    if (this.#entries.get(url)?.paged === false) this.#refusedAlias.add(url);
    this.#entries.delete(url);
    return true;
  }

  clear(): void {
    this.#entries.clear();
    this.#claims.clear();
    this.#refusedAlias.clear();
    this.#residency = -1;
  }

  get size(): number {
    return this.#entries.size;
  }
}

/** 🥽️ The page's single {@link Puzzle3dBrushMeshRegistry} — one guest per tab, so one registry. */
export const puzzle3dBrushMeshRegistry = new Puzzle3dBrushMeshRegistry();

/** 📏️ Upper bound on the bytes a JSON string costs on the retained wire: ASCII exactly, every other
 * UTF-16 code unit charged as a six-character `\uXXXX` escape, which no JSON encoder exceeds. */
function puzzle3dWireBytes(text: string): number {
  let bytes = 0;
  for (let index = 0; index < text.length; index += 1) bytes += text.charCodeAt(index) < 0x80 ? 1 : 6;
  return bytes;
}

function bytesToBase64(bytes: Uint8Array): string {
  let binary = "";
  for (let index = 0; index < bytes.length; index += 1) binary += String.fromCharCode(bytes[index]!);
  return btoa(binary);
}

/** 🔗️ The mesh's bytes on the wire and in the digest: little-endian `f32` positions followed by
 * little-endian `u32` indices — the exact layout `decode_brush_mesh_page_values` reassembles. */
function puzzle3dBrushMeshBytes(positions: readonly number[], indices: readonly number[]): Uint8Array {
  const bytes = new Uint8Array((positions.length + indices.length) * 4);
  new Float32Array(bytes.buffer, 0, positions.length).set(positions);
  new Uint32Array(bytes.buffer, positions.length * 4, indices.length).set(indices);
  return bytes;
}

/** #️⃣ The identity one uploaded mesh is keyed by: unkeyed BLAKE3 over {@link puzzle3dBrushMeshBytes},
 * byte-identical to the plugin's own `brush_mesh_digest` (`✏️editor/⏳️precompute/🦀️.rs`), which is what
 * lets the plugin refuse a run whose pages do not reassemble into the mesh the client announced. */
export function puzzle3dBrushMeshDigest(positions: readonly number[], indices: readonly number[]): string {
  return blake3Hex(puzzle3dBrushMeshBytes(positions, indices));
}

/** 🚪️ The mesh identities a world-3d surface may announce to its guest. `registerBrushMesh` is an
 * APP-declared action, never a framework-reserved one, so a guest that does not own that lane answers
 * every announcement with an `undeclared-action` drop and an upload run that can never settle
 * (measured on `🎥️shooting` 2026-09-19: 12 dropped dispatches at boot). A guest that owns the lane
 * publishes its monotone install counter as `interactionJson.meshResidency` — the same fact
 * {@link Puzzle3dBrushMeshRegistry.observeResidency} folds — so the counter's PRESENCE is the opt-in,
 * and a surface without one announces nothing however many meshes its scene names. */
export function puzzle3dAnnounceableBrushMeshUrls(meshResidency: number | undefined, meshes: readonly { readonly url?: string }[]): readonly string[] {
  if (meshResidency === undefined) return [];
  return [...new Set(meshes.map((mesh) => mesh.url).filter((url): url is string => Boolean(url)))];
}

/** 📏️ Values this mesh id may put in one page without pushing the JSON envelope past
 * {@link PUZZLE3D_MESH_COMMAND_RAW_BYTES}. Zero means the id alone already exhausts the wire, and the
 * mesh cannot be uploaded at all. */
function puzzle3dBrushMeshPageCapacity(url: string, surfaceId: string, digest: string): number {
  const probe = { surfaceId, url, digest, page: 999_999, pageCount: 999_999, positionsB64: "", indicesB64: "" };
  const budget = PUZZLE3D_MESH_COMMAND_RAW_BYTES - puzzle3dWireBytes(JSON.stringify(["registerBrushMesh", probe])) - PUZZLE3D_MESH_PAGE_PADDING_CHARS;
  return Math.max(0, Math.min(PUZZLE3D_MESH_PAGE_VALUES, Math.floor((budget * 3) / 16)));
}

/** 🥽️ Splits one loaded GLB's collision geometry into the `registerBrushMesh` page run that carries it
 * inside the retained command contract. An empty result means the mesh cannot be uploaded at all — the
 * id leaves no room for a payload on the wire, or the run would be longer than
 * {@link PUZZLE3D_MESH_UPLOAD_MAX_PAGES} — and the caller uploads nothing rather than emitting a command
 * the framework will refuse. */
export function puzzle3dBrushMeshPages(url: string, surfaceId: string, positions: readonly number[], indices: readonly number[]): readonly Puzzle3dBrushMeshPage[] {
  const digest = puzzle3dBrushMeshDigest(positions, indices);
  const capacity = puzzle3dBrushMeshPageCapacity(url, surfaceId, digest);
  const total = positions.length + indices.length;
  if (capacity === 0 || total === 0) return [];
  const pageCount = Math.ceil(total / capacity);
  if (pageCount > PUZZLE3D_MESH_UPLOAD_MAX_PAGES) return [];
  const pages: Puzzle3dBrushMeshPage[] = [];
  for (let page = 0; page < pageCount; page += 1) {
    const start = page * capacity;
    const end = Math.min(total, start + capacity);
    const positionSlice = positions.slice(Math.min(start, positions.length), Math.min(end, positions.length));
    const indexSlice = indices.slice(Math.max(0, start - positions.length), Math.max(0, end - positions.length));
    pages.push({
      url,
      digest,
      page,
      pageCount,
      ...(positionSlice.length > 0 ? { positionsB64: bytesToBase64(new Uint8Array(Float32Array.from(positionSlice).buffer)) } : {}),
      ...(indexSlice.length > 0 ? { indicesB64: bytesToBase64(new Uint8Array(Uint32Array.from(indexSlice).buffer)) } : {}),
    });
  }
  return pages;
}
/** 🚚️ What the drain does with the page it just took off the queue. `adopt` means the run collapsed:
 * a sibling identity already put this exact geometry into the guest, so the remaining pages of THIS
 * identity were dropped and the announcement alone carries it. */
export type Puzzle3dBrushMeshQueueStep =
  | { readonly kind: "page"; readonly page: Puzzle3dBrushMeshPage }
  | { readonly kind: "adopt"; readonly url: string; readonly digest: string }
  | { readonly kind: "idle" };

/** 🚚️ Takes the next unit of work off a brush-mesh page queue, in place. A page whose digest the guest
 * already holds under ANY id collapses its whole remaining run into one `adopt` announcement — this is
 * what keeps a scene of several ids over byte-identical geometry from paging the same 294 912 bytes
 * once per id. Pure: the caller owns the dispatch and the registry. */
export function puzzle3dBrushMeshQueueStep(queue: Puzzle3dBrushMeshPage[], adoptable: (page: Puzzle3dBrushMeshPage) => boolean): Puzzle3dBrushMeshQueueStep {
  const page = queue.shift();
  if (!page) return { kind: "idle" };
  if (!adoptable(page)) return { kind: "page", page };
  for (let index = queue.length - 1; index >= 0; index -= 1) {
    if (queue[index]!.url === page.url) queue.splice(index, 1);
  }
  return { kind: "adopt", url: page.url, digest: page.digest };
}

/** ⏳️ Drains a brush-mesh page queue with BACK PRESSURE: exactly one command is ever outstanding, so a
 * user action that arrives mid-run queues behind one page instead of behind the whole run. `dispatch`
 * must settle on the dispatched command's own guest completion (`ComponentSceneHostProps.onAction`
 * does); `live` false retires the drain without dispatching anything further. A run's LAST page — and a
 * collapsed run's announcement — confirms the identity through `confirm`, never its enqueue. */
export async function drainPuzzle3dBrushMeshQueue(
  queue: Puzzle3dBrushMeshPage[],
  registry: {
    readonly holdsDigest: (digest: string) => boolean;
    readonly mayAlias: (url: string) => boolean;
    readonly alias: (url: string, digest: string) => void;
    readonly confirm: (url: string, digest: string) => void;
  },
  dispatch: (args: Record<string, unknown>) => Promise<unknown>,
  live: () => boolean,
): Promise<void> {
  while (live()) {
    const step = puzzle3dBrushMeshQueueStep(queue, (page) => registry.mayAlias(page.url) && registry.holdsDigest(page.digest));
    if (step.kind === "idle") return;
    if (step.kind === "adopt") {
      registry.alias(step.url, step.digest);
      await dispatch({ url: step.url, digest: step.digest });
      continue;
    }
    await dispatch(step.page);
    if (step.page.page === step.page.pageCount - 1) registry.confirm(step.page.url, step.page.digest);
  }
}
//#endregion 🥽️Puzzle3dBrushMeshUpload
//#endregion ShellHelpers
