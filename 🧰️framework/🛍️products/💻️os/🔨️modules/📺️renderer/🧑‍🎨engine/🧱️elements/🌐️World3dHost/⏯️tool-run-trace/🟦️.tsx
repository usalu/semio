// #region 🧲️Header
/** @emoji ⏯️ `ToolRunTraceLayer` — the React World3d tool run trace: a keyed record store per window fed by
 * the base64url `ToolRunTraceDelta` pages of `World3dScene.toolRunTrace`, one `THREE.InstancedMesh` per
 * `(mesh, verdict)`, verdict paint from design tokens only, age fade to a floor, the highlighted newest
 * `testing` record, and the `provisional` mesh style for document instances a running tool placed.
 * The Rust twin is `♾️infinite/🌍️world/⏯️tool-run-trace/🦀️.rs`; both are pinned by
 * `🧰️framework/🔨️modules/⏯️tool-run/🧫️fixtures/📼️trace-pages.json`.
 * @see ../../../../../../../../🔨️modules/⏯️tool-run/🟦️.ts
 * @see ../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS/📋️tool-run-contract.md */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { createContext, useCallback, useContext, useEffect, useMemo, useRef, useState } from "react";
import type { ViewToolRunTraceCursor } from "@semio-tech/framework";
import { BoxGeometry, BufferGeometry, Color, DoubleSide, DynamicDrawUsage, EdgesGeometry, InstancedBufferAttribute, InstancedMesh, LineBasicMaterial, LineDashedMaterial, LineSegments, MeshStandardMaterial } from "three";
import { useFrame } from "@react-three/fiber";
import { resolveColorHex, semanticVar, STYLING_METRICS, tokenVar } from "@semio-tech/ui-styling";
import { base64UrlDecode } from "../../../../../../../../🔨️modules/🚪️io/🔤️base64/🟦️.ts";
import { decodeToolRunTraceDelta, type ToolRunTraceCursor, type ToolRunTraceDelta, type ToolRunTraceSubject, type ToolRunVerdict, TOOL_RUN_VERDICTS } from "../../../../../../../../🔨️modules/⏯️tool-run/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🗂️RecordStore
/** 🎚️ The `toolRun` styling metrics (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json`). */
export const TOOL_RUN_TRACE_METRICS = STYLING_METRICS.toolRun;

/** 👪️ Subject family of a batch. */
export type ToolRunTraceFamily = ToolRunTraceSubject["kind"];

/** 📦️ One `(family, index, verdict)` batch: parallel columns kept dense by swap-remove, so upsert and
 * retire are O(1) and a renderer uploads only the `dirtyFrom..dirtyTo` range it has not seen. */
export type ToolRunTraceBatch = {
  readonly id: string;
  readonly family: ToolRunTraceFamily;
  readonly index: number;
  readonly verdict: ToolRunVerdict;
  readonly keys: bigint[];
  readonly subjects: ToolRunTraceSubject[];
  stamps: Float32Array;
  matrices: Float32Array;
  count: number;
  dirtyFrom: number;
  dirtyTo: number;
};

/** 📋️ What one lane application changed. */
export type ToolRunTraceStoreApply = { readonly cleared: boolean; readonly pages: number; readonly ops: number };

const NO_CHANGE: ToolRunTraceStoreApply = Object.freeze({ cleared: false, pages: 0, ops: 0 });

type Slot = { batch: ToolRunTraceBatch; index: number };

/** 🧮️ Column-major TRS matrix with a uniform scale — the same layout `Matrix4.elements` uses. */
export function toolRunTraceMatrix(target: Float32Array, offset: number, position: readonly number[], rotation: readonly number[], scale: number): void {
  const [x, y, z, w] = rotation as [number, number, number, number];
  const x2 = x + x, y2 = y + y, z2 = z + z;
  const xx = x * x2, xy = x * y2, xz = x * z2, yy = y * y2, yz = y * z2, zz = z * z2, wx = w * x2, wy = w * y2, wz = w * z2;
  target[offset] = (1 - (yy + zz)) * scale;
  target[offset + 1] = (xy + wz) * scale;
  target[offset + 2] = (xz - wy) * scale;
  target[offset + 3] = 0;
  target[offset + 4] = (xy - wz) * scale;
  target[offset + 5] = (1 - (xx + zz)) * scale;
  target[offset + 6] = (yz + wx) * scale;
  target[offset + 7] = 0;
  target[offset + 8] = (xz + wy) * scale;
  target[offset + 9] = (yz - wx) * scale;
  target[offset + 10] = (1 - (xx + yy)) * scale;
  target[offset + 11] = 0;
  target[offset + 12] = position[0]!;
  target[offset + 13] = position[1]!;
  target[offset + 14] = position[2] ?? 0;
  target[offset + 15] = 1;
}

/** 🗂️ The per-window resident trace — see this module's header. `version` bumps on every change so a
 * React host can subscribe without diffing records. */
export class ToolRunTraceRecordStore {
  #cursor: ToolRunTraceCursor | null = null;
  #lane: string | null = null;
  #records = new Map<bigint, Slot>();
  #batches = new Map<string, ToolRunTraceBatch>();
  #nextStamp = 0;
  #newestTesting: { key: bigint; stamp: number } | null = null;
  version = 0;

  get cursor(): ToolRunTraceCursor | null {
    return this.#cursor;
  }

  get size(): number {
    return this.#records.size;
  }

  get newestStamp(): number {
    return this.#nextStamp - 1;
  }

  get newestTesting(): bigint | null {
    return this.#newestTesting?.key ?? null;
  }

  /** 🧺️ Every non-empty batch. */
  batches(): IterableIterator<ToolRunTraceBatch> {
    return this.#batches.values();
  }

  has(key: bigint): boolean {
    return this.#records.has(key);
  }

  /** 🔢️ Resident records with `verdict`, optionally restricted to one subject family. */
  count(verdict: ToolRunVerdict, family?: ToolRunTraceFamily): number {
    let total = 0;
    for (const batch of this.#batches.values()) if (batch.verdict === verdict && (family === undefined || batch.family === family)) total += batch.count;
    return total;
  }

  /** 📥️ Applies one lane text. An absent lane keeps the records; an unchanged text is not decoded.
   * Throws `Base64DecodeError`/`ToolRunCodecError` on a malformed lane, leaving the records intact. */
  applyLane(lane: string | null | undefined): ToolRunTraceStoreApply {
    if (lane === null || lane === undefined || lane === this.#lane) return NO_CHANGE;
    const delta = decodeToolRunTraceDelta(base64UrlDecode(lane));
    this.#lane = lane;
    return this.applyDelta(delta);
  }

  /** 📬️ Applies one decoded delta — a clear or a new run/generation drops everything first, and pages
   * below the expected cursor page are skipped, so redelivery is idempotent. */
  applyDelta(delta: ToolRunTraceDelta): ToolRunTraceStoreApply {
    const run = delta.identity.id.run;
    const generation = delta.identity.generation;
    const cleared = delta.clear || this.#cursor === null || this.#cursor.run !== run || this.#cursor.generation !== generation;
    if (cleared) this.#clear();
    const expected = cleared ? 0 : this.#cursor!.page;
    let pages = 0;
    let ops = 0;
    for (const page of delta.pages) {
      if (!cleared && page.page < expected) continue;
      for (const op of page.ops) {
        if (op.op === "clear") this.#clear();
        else if (op.op === "retire") this.#retire(op.key);
        else this.#upsert(op.key, op.verdict, op.subject);
      }
      ops += page.ops.length;
      pages += 1;
    }
    this.#cursor = { run, generation, page: cleared ? delta.next : Math.max(delta.next, expected) };
    if (cleared || ops > 0) this.version += 1;
    return { cleared, pages, ops };
  }

  #clear(): void {
    this.#records.clear();
    this.#batches.clear();
    this.#newestTesting = null;
  }

  #batch(subject: ToolRunTraceSubject, verdict: ToolRunVerdict): ToolRunTraceBatch {
    const index = subject.kind === "instance3d" ? subject.mesh : subject.kind === "placement2d" ? subject.shape : 0;
    const id = `${subject.kind}:${index}:${verdict}`;
    let batch = this.#batches.get(id);
    if (!batch) {
      batch = { id, family: subject.kind, index, verdict, keys: [], subjects: [], stamps: new Float32Array(16), matrices: new Float32Array(16 * 16), count: 0, dirtyFrom: 0, dirtyTo: 0 };
      this.#batches.set(id, batch);
    }
    return batch;
  }

  #upsert(key: bigint, verdict: ToolRunVerdict, subject: ToolRunTraceSubject): void {
    this.#retire(key);
    const stamp = this.#nextStamp++;
    const batch = this.#batch(subject, verdict);
    const at = batch.count++;
    if (batch.count > batch.stamps.length) {
      const stamps = new Float32Array(batch.stamps.length * 2);
      stamps.set(batch.stamps);
      batch.stamps = stamps;
      const matrices = new Float32Array(batch.matrices.length * 2);
      matrices.set(batch.matrices);
      batch.matrices = matrices;
    }
    batch.keys[at] = key;
    batch.subjects[at] = subject;
    batch.stamps[at] = stamp;
    if (subject.kind === "instance3d") toolRunTraceMatrix(batch.matrices, at * 16, subject.position, subject.rotation, subject.scale);
    else if (subject.kind === "placement2d") toolRunTraceMatrix(batch.matrices, at * 16, subject.position, [0, 0, Math.sin(subject.rotation / 2), Math.cos(subject.rotation / 2)], 1);
    this.#dirty(batch, at);
    this.#records.set(key, { batch, index: at });
    if (verdict === "testing") this.#newestTesting = { key, stamp };
  }

  #retire(key: bigint): void {
    const slot = this.#records.get(key);
    if (!slot) return;
    this.#records.delete(key);
    const { batch, index } = slot;
    const last = --batch.count;
    if (index !== last) {
      const moved = batch.keys[last]!;
      batch.keys[index] = moved;
      batch.subjects[index] = batch.subjects[last]!;
      batch.stamps[index] = batch.stamps[last]!;
      batch.matrices.copyWithin(index * 16, last * 16, last * 16 + 16);
      this.#records.get(moved)!.index = index;
      this.#dirty(batch, index);
    }
    batch.keys.length = last;
    batch.subjects.length = last;
    if (last === 0) this.#batches.delete(batch.id);
    if (this.#newestTesting?.key === key) {
      this.#newestTesting = null;
      for (const candidate of this.#batches.values()) {
        if (candidate.verdict !== "testing") continue;
        for (let at = 0; at < candidate.count; at += 1) if (this.#newestTesting === null || candidate.stamps[at]! > this.#newestTesting.stamp) this.#newestTesting = { key: candidate.keys[at]!, stamp: candidate.stamps[at]! };
      }
    }
  }

  #dirty(batch: ToolRunTraceBatch, at: number): void {
    if (batch.dirtyFrom >= batch.dirtyTo) {
      batch.dirtyFrom = at;
      batch.dirtyTo = at + 1;
      return;
    }
    batch.dirtyFrom = Math.min(batch.dirtyFrom, at);
    batch.dirtyTo = Math.max(batch.dirtyTo, at + 1);
  }
}

/** 🌫️ Age fade by sequence distance: 1 for the newest record, the `toolRun.fadeFloorOpacity` token after
 * `toolRun.fadeRecords` newer records. The shader twin evaluates the same formula per instance. */
export function toolRunTraceFade(age: number): number {
  const floor = TOOL_RUN_TRACE_METRICS.fadeFloorOpacity;
  return Math.max(floor, 1 - (age / TOOL_RUN_TRACE_METRICS.fadeRecords) * (1 - floor));
}

/** 🔬️ The `data-tool-run-*` counters a host spreads onto its DOM wrapper for runtime probes. */
export function toolRunTraceDataAttributes(store: ToolRunTraceRecordStore): Record<`data-tool-run-${string}`, string> {
  const cursor = store.cursor;
  const attributes: Record<`data-tool-run-${string}`, string> = {
    "data-tool-run-run": cursor ? String(cursor.run) : "",
    "data-tool-run-generation": cursor ? String(cursor.generation) : "",
    "data-tool-run-page": cursor ? String(cursor.page) : "",
    "data-tool-run-records": String(store.size),
  };
  for (const verdict of TOOL_RUN_VERDICTS) attributes[`data-tool-run-${verdict}`] = String(store.count(verdict));
  return attributes;
}

/** 🪝️ Owns one window's store, applies each lane text as it arrives and reports the echo cursor. A
 * malformed lane is logged once and leaves the previous frame's records visible. */
export function useToolRunTraceStore(lane: string | null | undefined, onCursor?: (cursor: ToolRunTraceCursor) => void): { readonly store: ToolRunTraceRecordStore; readonly version: number } {
  const store = useMemo(() => new ToolRunTraceRecordStore(), []);
  const version = useMemo(() => {
    try {
      store.applyLane(lane);
    } catch (error) {
      console.warn("toolRunTrace lane rejected", error);
    }
    return store.version;
  }, [store, lane]);
  const cursor = store.cursor;
  useEffect(() => {
    if (cursor) onCursor?.(cursor);
  }, [cursor?.run, cursor?.generation, cursor?.page, onCursor]);
  return { store, version };
}
//#endregion 🗂️RecordStore

//#region 🧭️CursorEcho
const toolRunTraceCursorByWindowId = new Map<string, ToolRunTraceCursor>();

/** 🧭️ Records (or, with `null`, forgets) the cursor window `windowId`'s trace store reached — the echo the
 * shell carries as `toolRunTraceCursorByWindowId` in the next refresh's view state (contract §3.2). */
export function publishToolRunTraceCursor(windowId: string, cursor: ToolRunTraceCursor | null): void {
  if (cursor) toolRunTraceCursorByWindowId.set(windowId, cursor);
  else toolRunTraceCursorByWindowId.delete(windowId);
}

/** 📮️ The view-state form of every echoed cursor whose window is still live; a run id beyond exact JavaScript
 * integers is not echoed, so the guest resends that layer from page 0 instead of receiving a rounded run. */
export function toolRunTraceCursorViewState(liveWindowIds: readonly string[]): Record<string, ViewToolRunTraceCursor> {
  const echoed: Record<string, ViewToolRunTraceCursor> = {};
  for (const windowId of liveWindowIds) {
    const cursor = toolRunTraceCursorByWindowId.get(windowId);
    if (cursor && cursor.run <= BigInt(Number.MAX_SAFE_INTEGER)) echoed[windowId] = { run: Number(cursor.run), generation: cursor.generation, page: cursor.page };
  }
  return echoed;
}

/** 🪝️ A stable `onCursor` for {@link useToolRunTraceStore} that publishes window `windowId`'s echo and forgets
 * it when the host unmounts or moves to another window. */
export function useToolRunTraceCursorEcho(windowId: string | null | undefined): (cursor: ToolRunTraceCursor) => void {
  useEffect(() => (windowId ? () => publishToolRunTraceCursor(windowId, null) : undefined), [windowId]);
  return useCallback((cursor: ToolRunTraceCursor) => {
    if (windowId) publishToolRunTraceCursor(windowId, cursor);
  }, [windowId]);
}
//#endregion 🧭️CursorEcho

//#region 🎨️Paint
/** 🎨️ Verdict paint as design-token CSS expressions: `testing` is the neutral progress accent at the
 * `toolRun.testingOpacity` token, the rest are the theme's success, warning and error tones. */
export const TOOL_RUN_TRACE_VERDICT_PAINT: Readonly<Record<ToolRunVerdict, { readonly fill: string; readonly opacity: number }>> = {
  testing: { fill: semanticVar("accent"), opacity: TOOL_RUN_TRACE_METRICS.testingOpacity },
  success: { fill: tokenVar("success"), opacity: 1 },
  warning: { fill: tokenVar("warning"), opacity: 1 },
  danger: { fill: tokenVar("danger"), opacity: 1 },
};

/** 🔦️ The newest `testing` record's outline — the same `highlighted` token the World3d mesh styles use. */
export const TOOL_RUN_TRACE_HIGHLIGHT_PAINT = tokenVar("secondary");

/** 🟩️ The `provisional` style token: success hue at `toolRun.provisionalOpacity`, with a dashed outline
 * that breathes over `toolRun.provisionalDashPeriodMs` unless `prefers-reduced-motion` asks for a static one. */
export const TOOL_RUN_PROVISIONAL_PAINT = {
  fill: tokenVar("success"),
  line: tokenVar("success"),
  opacity: TOOL_RUN_TRACE_METRICS.provisionalOpacity,
  dash: TOOL_RUN_TRACE_METRICS.provisionalDash,
  dashPeriodMs: TOOL_RUN_TRACE_METRICS.provisionalDashPeriodMs,
} as const;

/** 👁️ The legend toggles a window config exposes (persisted local-only by the host). */
export type ToolRunTraceVisibility = { readonly testing: boolean; readonly accepted: boolean; readonly rejected: boolean };

export const TOOL_RUN_TRACE_VISIBLE_ALL: ToolRunTraceVisibility = Object.freeze({ testing: true, accepted: true, rejected: true });

export function toolRunTraceShows(visibility: ToolRunTraceVisibility, verdict: ToolRunVerdict): boolean {
  return verdict === "testing" ? visibility.testing : verdict === "success" ? visibility.accepted : visibility.rejected;
}

/** 🐢️ `prefers-reduced-motion: reduce`, live. */
export function usePrefersReducedMotion(): boolean {
  const query = typeof window !== "undefined" ? window.matchMedia?.("(prefers-reduced-motion: reduce)") : undefined;
  const [reduced, setReduced] = useState(query?.matches === true);
  useEffect(() => {
    if (!query) return;
    const update = () => setReduced(query.matches);
    query.addEventListener?.("change", update);
    return () => query.removeEventListener?.("change", update);
  }, [query]);
  return reduced;
}
//#endregion 🎨️Paint

//#region 🧊️Instancing
/** 🧊️ Creates the instanced mesh one batch draws through. The material fades each instance by its
 * stamp against `uNewestStamp` in the shader, so an age fade costs one uniform per frame. */
export function createToolRunTraceInstancedMesh(geometry: BufferGeometry, verdict: ToolRunVerdict, capacity: number): InstancedMesh {
  const paint = TOOL_RUN_TRACE_VERDICT_PAINT[verdict];
  const material = new MeshStandardMaterial({ color: new Color(resolveColorHex(paint.fill)), transparent: true, opacity: paint.opacity, depthWrite: false, side: DoubleSide });
  const uniforms = { uNewestStamp: { value: 0 }, uFadeRecords: { value: TOOL_RUN_TRACE_METRICS.fadeRecords }, uFadeFloor: { value: TOOL_RUN_TRACE_METRICS.fadeFloorOpacity } };
  material.onBeforeCompile = (shader) => {
    Object.assign(shader.uniforms, uniforms);
    shader.vertexShader = `attribute float instanceStamp;\nvarying float vToolRunFade;\nuniform float uNewestStamp;\nuniform float uFadeRecords;\nuniform float uFadeFloor;\n${shader.vertexShader}`.replace(
      "#include <begin_vertex>",
      "#include <begin_vertex>\n  vToolRunFade = max(uFadeFloor, 1.0 - (uNewestStamp - instanceStamp) / uFadeRecords * (1.0 - uFadeFloor));",
    );
    shader.fragmentShader = `varying float vToolRunFade;\n${shader.fragmentShader}`.replace("#include <dithering_fragment>", "#include <dithering_fragment>\n  gl_FragColor.a *= vToolRunFade;");
  };
  const mesh = new InstancedMesh(geometry, material, capacity);
  mesh.instanceMatrix.setUsage(DynamicDrawUsage);
  mesh.geometry = geometry.clone();
  mesh.geometry.setAttribute("instanceStamp", new InstancedBufferAttribute(new Float32Array(capacity), 1).setUsage(DynamicDrawUsage));
  mesh.userData.toolRunTraceUniforms = uniforms;
  mesh.count = 0;
  mesh.frustumCulled = false;
  return mesh;
}

/** 🔁️ Uploads a batch's dirty range into its instanced mesh and sets the live count and fade uniform.
 * Returns `false` when the batch outgrew the mesh capacity and the caller must recreate it. */
export function syncToolRunTraceInstancedMesh(mesh: InstancedMesh, batch: ToolRunTraceBatch, newestStamp: number): boolean {
  const capacity = mesh.instanceMatrix.count;
  if (batch.count > capacity) return false;
  const stamps = mesh.geometry.getAttribute("instanceStamp") as InstancedBufferAttribute;
  if (batch.dirtyTo > batch.dirtyFrom) {
    const from = batch.dirtyFrom;
    const to = Math.min(batch.dirtyTo, batch.count);
    if (to > from) {
      (mesh.instanceMatrix.array as Float32Array).set(batch.matrices.subarray(from * 16, to * 16), from * 16);
      (stamps.array as Float32Array).set(batch.stamps.subarray(from, to), from);
      mesh.instanceMatrix.addUpdateRange(from * 16, (to - from) * 16);
      stamps.addUpdateRange(from, to - from);
      mesh.instanceMatrix.needsUpdate = true;
      stamps.needsUpdate = true;
    }
    batch.dirtyFrom = 0;
    batch.dirtyTo = 0;
  }
  mesh.count = batch.count;
  (mesh.userData.toolRunTraceUniforms as { uNewestStamp: { value: number } }).uNewestStamp.value = newestStamp;
  return true;
}

/** 📐️ Next power-of-two instance capacity for `count` records. */
export function toolRunTraceCapacity(count: number): number {
  return 2 ** Math.max(6, Math.ceil(Math.log2(Math.max(1, count))));
}

const UNIT_BOX = new BoxGeometry(1, 1, 1);

function ToolRunTraceBatchMesh({ batch, store, geometry }: { readonly batch: ToolRunTraceBatch; readonly store: ToolRunTraceRecordStore; readonly geometry: BufferGeometry }) {
  const [capacity, setCapacity] = useState(() => toolRunTraceCapacity(batch.count));
  const mesh = useMemo(() => {
    batch.dirtyFrom = 0;
    batch.dirtyTo = batch.count;
    return createToolRunTraceInstancedMesh(geometry, batch.verdict, capacity);
  }, [geometry, batch, capacity]);
  useEffect(() => () => {
    mesh.geometry.dispose();
    (mesh.material as MeshStandardMaterial).dispose();
  }, [mesh]);
  useFrame(() => {
    if (!syncToolRunTraceInstancedMesh(mesh, batch, store.newestStamp)) setCapacity(toolRunTraceCapacity(batch.count));
  });
  return <primitive object={mesh} raycast={() => null} />;
}

function ToolRunTraceNewestOutline({ store, geometryForMesh, reducedMotion }: { readonly store: ToolRunTraceRecordStore; readonly geometryForMesh: (mesh: number) => BufferGeometry; readonly reducedMotion: boolean }) {
  const key = store.newestTesting;
  const slot = key === null ? null : [...store.batches()].find((batch) => batch.verdict === "testing" && batch.keys.includes(key));
  const at = slot && key !== null ? slot.keys.indexOf(key) : -1;
  const subject = slot && at >= 0 ? slot.subjects[at] : undefined;
  const edges = useMemo(() => (subject?.kind === "instance3d" ? new EdgesGeometry(geometryForMesh(subject.mesh)) : null), [subject?.kind === "instance3d" ? subject.mesh : -1, geometryForMesh]);
  const line = useMemo(() => (edges ? new LineSegments(edges, new LineBasicMaterial({ color: new Color(resolveColorHex(TOOL_RUN_TRACE_HIGHLIGHT_PAINT)), transparent: true, depthTest: false, linewidth: TOOL_RUN_TRACE_METRICS.testingOutlineWidth })) : null), [edges]);
  useEffect(() => () => line?.geometry.dispose(), [line]);
  useFrame(({ clock }) => {
    if (!line || !slot || at < 0) return;
    line.matrixAutoUpdate = false;
    line.matrix.fromArray(slot.matrices, at * 16);
    (line.material as LineBasicMaterial).opacity = reducedMotion ? 1 : 0.6 + 0.4 * Math.abs(Math.sin((clock.elapsedTime * 1000 * Math.PI) / TOOL_RUN_TRACE_METRICS.testingPulseMs));
  });
  return line && subject?.kind === "instance3d" ? <primitive object={line} renderOrder={3} /> : null;
}

export type ToolRunTraceLayerProps = {
  /** 🚚️ `World3dScene.toolRunTrace`. */
  readonly lane: string | null | undefined;
  /** 🔺️ Geometry of the scene's `meshesJson[mesh]`; a unit box when absent. */
  readonly geometryForMesh?: (mesh: number) => BufferGeometry | null;
  /** 👁️ Legend toggles from the window config. */
  readonly visibility?: ToolRunTraceVisibility;
  /** 🐢️ Overrides the live `prefers-reduced-motion` query. */
  readonly reducedMotion?: boolean;
  /** 🧭️ Receives the cursor the window echoes as `toolRunTraceCursor` in its instance view state. */
  readonly onCursor?: (cursor: ToolRunTraceCursor) => void;
  /** 🗂️ A host-owned store (so the host can also spread {@link toolRunTraceDataAttributes}). */
  readonly store?: { readonly store: ToolRunTraceRecordStore; readonly version: number };
};

/** ⏯️ Mounts inside the World3d R3F scene: one instanced mesh per visible `(mesh, verdict)` batch plus the
 * highlighted newest `testing` outline. Placement and entity subjects are counted but not drawn in 3d. */
export function ToolRunTraceLayer({ lane, geometryForMesh, visibility = TOOL_RUN_TRACE_VISIBLE_ALL, reducedMotion, onCursor, store: hosted }: ToolRunTraceLayerProps) {
  const own = useToolRunTraceStore(hosted ? undefined : lane, onCursor);
  const { store, version } = hosted ?? own;
  const prefersReduced = usePrefersReducedMotion();
  const geometries = useRef(new Map<number, BufferGeometry>());
  const resolve = useMemo(() => {
    geometries.current.clear();
    return (mesh: number): BufferGeometry => {
      let geometry = geometries.current.get(mesh);
      if (!geometry) {
        geometry = geometryForMesh?.(mesh) ?? UNIT_BOX;
        geometries.current.set(mesh, geometry);
      }
      return geometry;
    };
  }, [geometryForMesh]);
  const batches = useMemo(() => [...store.batches()].filter((batch) => batch.family === "instance3d" && toolRunTraceShows(visibility, batch.verdict)), [store, version, visibility]);
  return (
    <group name="tool-run-trace">
      {batches.map((batch) => (
        <ToolRunTraceBatchMesh key={batch.id} batch={batch} store={store} geometry={resolve(batch.index)} />
      ))}
      {visibility.testing ? <ToolRunTraceNewestOutline key={String(store.newestTesting)} store={store} geometryForMesh={resolve} reducedMotion={reducedMotion ?? prefersReduced} /> : null}
    </group>
  );
}
//#endregion 🧊️Instancing

//#region 🟩️Provisional
/** 🟩️ Instance ids (or interaction ids) a running tool placed provisionally. The host provides the set the
 * framework derives from `ArtifactView::tool_run().provisionalEntities`; the World3d instance path paints
 * members with the `provisional` style and excludes them from picking. */
export const ToolRunProvisionalIdsContext = createContext<ReadonlySet<string>>(new Set());

/** 🟩️ Whether `instance` belongs to the provisional set in context. */
export function useToolRunProvisional(instance: { readonly id: string; readonly interactionId?: string }): boolean {
  const ids = useContext(ToolRunProvisionalIdsContext);
  return ids.size > 0 && (ids.has(instance.id) || (instance.interactionId !== undefined && ids.has(instance.interactionId)));
}

/** 🟩️ The dashed `provisional` outline around one instance's border geometry; it breathes with the
 * `toolRun.provisionalDashPeriodMs` token and stands still under reduced motion. */
export function ToolRunProvisionalOutline({ geometry, reducedMotion }: { readonly geometry: EdgesGeometry; readonly reducedMotion?: boolean }) {
  const prefersReduced = usePrefersReducedMotion();
  const still = reducedMotion ?? prefersReduced;
  const [dashSize, gapSize] = TOOL_RUN_PROVISIONAL_PAINT.dash as readonly [number, number];
  const line = useMemo(() => {
    const segments = new LineSegments(geometry, new LineDashedMaterial({ color: new Color(resolveColorHex(TOOL_RUN_PROVISIONAL_PAINT.line)), dashSize: dashSize / 100, gapSize: gapSize / 100, depthTest: false, transparent: true, linewidth: TOOL_RUN_TRACE_METRICS.provisionalOutlineWidth }));
    segments.computeLineDistances();
    return segments;
  }, [geometry, dashSize, gapSize]);
  useEffect(() => () => (line.material as LineDashedMaterial).dispose(), [line]);
  useFrame(({ clock }) => {
    (line.material as LineDashedMaterial).opacity = still ? 1 : 0.55 + 0.45 * Math.abs(Math.cos((clock.elapsedTime * 1000 * Math.PI) / TOOL_RUN_PROVISIONAL_PAINT.dashPeriodMs));
  });
  return <primitive object={line} scale={1.002} renderOrder={3} raycast={() => null} />;
}
//#endregion 🟩️Provisional
