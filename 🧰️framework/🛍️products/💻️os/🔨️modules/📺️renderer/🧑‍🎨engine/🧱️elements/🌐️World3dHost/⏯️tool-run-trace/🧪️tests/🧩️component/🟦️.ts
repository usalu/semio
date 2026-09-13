/** @emoji 🧪️ Laws of the React World3d tool run trace store and its instancing: the record store reproduces
 * the ledger's resident set from the language-neutral `📼️trace-pages.json` through real base64url lanes,
 * and a `THREE.InstancedMesh` per `(mesh, verdict)` carries exactly the resident records (three.js is the
 * oracle for counts and for matrix composition). */
import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import viewContextSchema from "../../../../../../../../../../🔨️modules/🛂️manifest/🪟️view-context/🧬️schema/🔣️.json";
import { parseResolvedPluginViewState } from "../../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts";
import { Matrix4, Quaternion, Vector3, BoxGeometry, type InstancedMesh } from "three";
import fixture from "../../../../../../../../../../🔨️modules/⏯️tool-run/🧫️fixtures/📼️trace-pages.json";
import canvas2dLanes from "../../../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/🚚️canvas2d-scene-lanes/🔣️.json";
import base64UrlVectors from "../../../../../../../../../../🔨️modules/🚪️io/🔤️base64/🧫️fixtures/🔣️rfc4648-base64url-vectors.json";
import { base64UrlDecode, base64UrlEncode } from "../../../../../../../../../../🔨️modules/🚪️io/🔤️base64/🟦️.ts";
import {
  encodeToolRunTraceDelta,
  toolRunHexToBytes,
  toolRunIdentityFromJson,
  toolRunTraceDeltaFromJson,
  toolRunTraceOpFromJson,
  ToolRunTraceStore,
  TOOL_RUN_VERDICTS,
  type ToolRunTraceOp,
  type ToolRunTraceSubject,
  type ToolRunVerdict,
} from "../../../../../../../../../../🔨️modules/⏯️tool-run/🟦️.ts";
import { resolveMeshStyle } from "../../../🟦️.tsx";
import { createToolRunTraceInstancedMesh, publishToolRunTraceCursor, syncToolRunTraceInstancedMesh, toolRunTraceCapacity, toolRunTraceCursorViewState, toolRunTraceDataAttributes, toolRunTraceFade, ToolRunTraceRecordStore, TOOL_RUN_TRACE_METRICS } from "../../🟦️.tsx";

type Resident = Map<bigint, { readonly verdict: ToolRunVerdict; readonly subject: ToolRunTraceSubject }>;

function residentOfStore(store: ToolRunTraceRecordStore): Resident {
  const resident: Resident = new Map();
  for (const batch of store.batches()) for (let at = 0; at < batch.count; at += 1) resident.set(batch.keys[at]!, { verdict: batch.verdict, subject: batch.subjects[at]! });
  return resident;
}

function residentOfLedger(ledger: ToolRunTraceStore): Resident {
  const resident: Resident = new Map();
  for (const [key, record] of ledger.records()) resident.set(key, { verdict: record.verdict, subject: record.subject });
  return resident;
}

function sorted(resident: Resident): [bigint, { readonly verdict: ToolRunVerdict; readonly subject: ToolRunTraceSubject }][] {
  return [...resident].sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0));
}

/** 🚚️ Delivers every pending ledger page through the real lane text, redelivering each delta twice. */
function deliver(ledger: ToolRunTraceStore, store: ToolRunTraceRecordStore, budget: number): void {
  for (let round = 0; round < 10_000; round += 1) {
    const delta = ledger.deltaAfter(store.cursor, budget);
    store.applyLane(base64UrlEncode(encodeToolRunTraceDelta(delta)));
    store.applyDelta(delta);
    if (store.cursor?.page === ledger.nextPage) return;
  }
  throw new Error("delivery never caught up");
}

function lcg(seed: number): (bound: number) => number {
  let state = BigInt(seed);
  return (bound) => {
    state = (state * 6364136223846793005n + 1442695040888963407n) & 0xffff_ffff_ffff_ffffn;
    return Number((state >> 33n) % BigInt(bound));
  };
}

describe("tool run trace record store", () => {
  it("encodes and decodes base64url exactly like the RFC 4648 vectors and the Node Buffer oracle", () => {
    for (const row of base64UrlVectors.cases) {
      const bytes = toolRunHexToBytes(row.input_hex);
      expect(base64UrlEncode(bytes)).toBe(row.encoded);
      expect(Buffer.from(bytes).toString("base64url")).toBe(row.encoded);
      expect([...base64UrlDecode(row.encoded)]).toEqual([...bytes]);
    }
    for (const row of base64UrlVectors.rejected) expect(() => base64UrlDecode(row.encoded)).toThrow();
    const next = lcg(0x5eed);
    for (let length = 0; length < 96; length += 1) {
      const bytes = Uint8Array.from({ length }, () => next(256));
      expect(base64UrlEncode(bytes)).toBe(Buffer.from(bytes).toString("base64url"));
    }
  });

  it("decodes the Rust-verified fixture delta hex through the same lane text the Rust scene fixture carries", () => {
    const row = fixture.deltas[0]!;
    const lane = base64UrlEncode(toolRunHexToBytes(row.hex));
    expect(lane).toBe(canvas2dLanes.roundTrip.laneTexts["framework.scene.canvas2d.toolRunTrace"]);
    const store = new ToolRunTraceRecordStore();
    const applied = store.applyLane(lane);
    const delta = toolRunTraceDeltaFromJson(row.delta as never);
    expect(applied.cleared).toBe(true);
    expect(store.cursor).toEqual({ run: delta.identity.id.run, generation: delta.identity.generation, page: delta.next });
    expect(store.applyLane(lane)).toEqual({ cleared: false, pages: 0, ops: 0 });
    const version = store.version;
    expect(() => store.applyLane("!!")).toThrow();
    expect(store.version).toBe(version);
  });

  it("reproduces every fixture residency case (eviction, retire, clear, compaction) through lanes", () => {
    for (const row of fixture.residency) {
      const ledger = new ToolRunTraceStore(toolRunIdentityFromJson(row.identity as never), row.capacity, row.compactFloor);
      const store = new ToolRunTraceRecordStore();
      for (const page of row.pages) {
        ledger.applyOps(page.map((op) => toolRunTraceOpFromJson(op as never)));
        deliver(ledger, store, 1);
      }
      const verdicts = new Map([...residentOfStore(store)].map(([key, record]) => [key, record.verdict]));
      expect(verdicts).toEqual(new Map(row.resident.map((record) => [BigInt(record.key), record.verdict as ToolRunVerdict])));
      expect(sorted(residentOfStore(store))).toEqual(sorted(residentOfLedger(ledger)));
      const cold = new ToolRunTraceRecordStore();
      cold.applyLane(base64UrlEncode(encodeToolRunTraceDelta(ledger.deltaAfter(null, Number.MAX_SAFE_INTEGER))));
      expect(sorted(residentOfStore(cold))).toEqual(sorted(residentOfLedger(ledger)));
    }
  });

  it("keeps InstancedMesh counts per verdict equal to the ledger's resident records across clear, retire and eviction", () => {
    const identity = toolRunIdentityFromJson(fixture.residency[0]!.identity as never);
    const next = lcg(0xd2a3);
    const geometry = new BoxGeometry(1, 1, 1);
    for (let round = 0; round < 12; round += 1) {
      const ledger = new ToolRunTraceStore(identity, 64, 8);
      const store = new ToolRunTraceRecordStore();
      const meshes = new Map<string, InstancedMesh>();
      for (let step = 0; step < 50; step += 1) {
        const ops: ToolRunTraceOp[] = Array.from({ length: next(14) + 1 }, () => {
          const roll = next(24);
          if (roll === 0) return { op: "clear" };
          if (roll < 5) return { op: "retire", key: BigInt(next(128)) };
          return {
            op: "upsert",
            key: BigInt(next(128)),
            verdict: TOOL_RUN_VERDICTS[next(4)]!,
            reason: 0,
            subject: roll === 5 ? { kind: "entity", entity: BigInt(next(9)) } : { kind: "instance3d", mesh: next(3), position: [next(20) - 10, next(5), 0.5], rotation: [0, Math.SQRT1_2, 0, Math.SQRT1_2], scale: 0.5 + next(3) },
          };
        });
        ledger.applyOps(ops);
        deliver(ledger, store, 48 + round * 29);
        const live = new Set<string>();
        for (const batch of store.batches()) {
          if (batch.family !== "instance3d") continue;
          live.add(batch.id);
          let mesh = meshes.get(batch.id);
          if (!mesh || !syncToolRunTraceInstancedMesh(mesh, batch, store.newestStamp)) {
            mesh = createToolRunTraceInstancedMesh(geometry, batch.verdict, toolRunTraceCapacity(batch.count));
            batch.dirtyFrom = 0;
            batch.dirtyTo = batch.count;
            expect(syncToolRunTraceInstancedMesh(mesh, batch, store.newestStamp)).toBe(true);
            meshes.set(batch.id, mesh);
          }
        }
        for (const id of [...meshes.keys()]) if (!live.has(id)) meshes.delete(id);
        for (const verdict of TOOL_RUN_VERDICTS) {
          const instanced = [...meshes.entries()].filter(([id]) => id.endsWith(`:${verdict}`)).reduce((sum, [, mesh]) => sum + mesh.count, 0);
          const resident = [...ledger.records()].filter(([, record]) => record.verdict === verdict && record.subject.kind === "instance3d").length;
          expect(instanced, `round ${round} step ${step} ${verdict}`).toBe(resident);
        }
        for (const batch of store.batches()) {
          if (batch.family !== "instance3d") continue;
          const mesh = meshes.get(batch.id)!;
          for (let at = 0; at < batch.count; at += 1) {
            const record = ledger.record(batch.keys[at]!)!;
            expect(record.verdict).toBe(batch.verdict);
            const subject = record.subject as Extract<ToolRunTraceSubject, { kind: "instance3d" }>;
            const actual = new Matrix4();
            mesh.getMatrixAt(at, actual);
            const oracle = new Matrix4().compose(new Vector3(...subject.position), new Quaternion(...subject.rotation), new Vector3(subject.scale, subject.scale, subject.scale));
            actual.elements.forEach((value, index) => expect(value).toBeCloseTo(oracle.elements[index]!, 5));
          }
        }
      }
    }
  });

  it("publishes data-tool-run-* counters and the fade token formula", () => {
    const row = fixture.residency[0]!;
    const ledger = new ToolRunTraceStore(toolRunIdentityFromJson(row.identity as never), row.capacity, row.compactFloor);
    const store = new ToolRunTraceRecordStore();
    for (const page of row.pages) ledger.applyOps(page.map((op) => toolRunTraceOpFromJson(op as never)));
    deliver(ledger, store, Number.MAX_SAFE_INTEGER);
    const attributes = toolRunTraceDataAttributes(store);
    expect(attributes["data-tool-run-run"]).toBe(String(ledger.identity.id.run));
    expect(attributes["data-tool-run-generation"]).toBe(String(ledger.identity.generation));
    expect(attributes["data-tool-run-page"]).toBe(String(ledger.nextPage));
    expect(attributes["data-tool-run-records"]).toBe(String(ledger.size));
    for (const verdict of TOOL_RUN_VERDICTS) expect(attributes[`data-tool-run-${verdict}`]).toBe(String([...ledger.records()].filter(([, record]) => record.verdict === verdict).length));
    expect(toolRunTraceFade(0)).toBe(1);
    expect(toolRunTraceFade(TOOL_RUN_TRACE_METRICS.fadeRecords)).toBeCloseTo(TOOL_RUN_TRACE_METRICS.fadeFloorOpacity, 10);
    expect(toolRunTraceFade(1e9)).toBe(TOOL_RUN_TRACE_METRICS.fadeFloorOpacity);
  });

  it("echoes each live window's cursor in the schema-valid view-state form the guest resumes from", () => {
    const row = fixture.residency[0]!;
    const ledger = new ToolRunTraceStore(toolRunIdentityFromJson(row.identity as never), row.capacity, row.compactFloor);
    const store = new ToolRunTraceRecordStore();
    for (const page of row.pages) ledger.applyOps(page.map((op) => toolRunTraceOpFromJson(op as never)));
    deliver(ledger, store, 1);
    publishToolRunTraceCursor("world-left", store.cursor);
    publishToolRunTraceCursor("closed-window", store.cursor);
    publishToolRunTraceCursor("world-right", { run: BigInt(Number.MAX_SAFE_INTEGER) + 1n, generation: 0, page: 1 });
    const echoed = toolRunTraceCursorViewState(["world-left", "world-right", "main"]);
    expect(echoed).toEqual({ "world-left": { run: Number(ledger.identity.id.run), generation: ledger.identity.generation, page: ledger.nextPage } });
    const view = { locale: "en", terminology: "native", windowInstances: [{ id: "world-left", windowKindId: "world" }], toolRunTraceCursorByWindowId: echoed };
    const validate = new Ajv({ strict: true, allErrors: true }).compile(viewContextSchema);
    expect(validate(view), JSON.stringify(validate.errors)).toBe(true);
    expect(parseResolvedPluginViewState(view).toolRunTraceCursorByWindowId).toEqual(echoed);
    const caughtUp = ledger.deltaAfter({ run: BigInt(echoed["world-left"]!.run), generation: echoed["world-left"]!.generation, page: echoed["world-left"]!.page }, Number.MAX_SAFE_INTEGER);
    expect([caughtUp.clear, caughtUp.pages.length]).toEqual([false, 0]);
    publishToolRunTraceCursor("world-left", null);
    expect(toolRunTraceCursorViewState(["world-left"])).toEqual({});
  });

  it("ranks the provisional mesh style below refusal and lock but above every interaction state", () => {
    expect(resolveMeshStyle({ provisional: true, selected: true, hovered: true, highlighted: true, celebrating: true })).toBe("provisional");
    expect(resolveMeshStyle({ provisional: true, danger: true })).toBe("danger");
    expect(resolveMeshStyle({ provisional: true, disabled: true })).toBe("disabled");
    expect(resolveMeshStyle({ provisional: false, selected: true })).toBe("selected");
  });
});
