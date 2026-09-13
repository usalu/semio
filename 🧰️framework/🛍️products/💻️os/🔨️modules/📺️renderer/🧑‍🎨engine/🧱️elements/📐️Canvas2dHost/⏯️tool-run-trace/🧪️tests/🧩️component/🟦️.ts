/** @emoji 🧪️ Laws of the canvas-2d tool run trace: one fill per resident placement record, batched per
 * `(shape, verdict)`, at the camera transform the host's `worldToScreenLogical` defines, faded by age and
 * with exactly one highlighted newest `testing` record. The ledger `ToolRunTraceStore` is the oracle. */
import { describe, expect, it } from "vitest";
import { encodeToolRunTraceDelta, ToolRunTraceStore, TOOL_RUN_VERDICTS, type ToolRunTraceOp } from "../../../../../../../../../../🔨️modules/⏯️tool-run/🟦️.ts";
import { base64UrlEncode } from "../../../../../../../../../../🔨️modules/🚪️io/🔤️base64/🟦️.ts";
import { ToolRunTraceRecordStore, TOOL_RUN_TRACE_METRICS } from "../../../../🌐️World3dHost/⏯️tool-run-trace/🟦️.tsx";
import { worldToScreenLogical } from "../../../🟦️.tsx";
import { paintToolRunTrace2d, type ToolRunTrace2dContext } from "../../🟦️.tsx";

type Call = { readonly kind: "fill" | "stroke"; readonly transform: readonly number[]; readonly alpha: number; readonly style: string };

function recordingContext(): { readonly ctx: ToolRunTrace2dContext; readonly calls: Call[] } {
  const calls: Call[] = [];
  let transform: readonly number[] = [1, 0, 0, 1, 0, 0];
  const state = { fillStyle: "", strokeStyle: "", globalAlpha: 1, lineWidth: 1 };
  const ctx = {
    ...state,
    save() {},
    restore() {},
    setTransform(a: number, b: number, c: number, d: number, e: number, f: number) {
      transform = [a, b, c, d, e, f];
    },
    fill() {
      calls.push({ kind: "fill", transform, alpha: ctx.globalAlpha, style: String(ctx.fillStyle) });
    },
    stroke() {
      calls.push({ kind: "stroke", transform, alpha: ctx.globalAlpha, style: String(ctx.strokeStyle) });
    },
  } as unknown as ToolRunTrace2dContext;
  return { ctx, calls };
}

describe("tool run trace 2d paint", () => {
  it("fills every resident placement once per record, batched per shape and verdict, at the host camera transform", () => {
    const ledger = new ToolRunTraceStore({ id: { appInstanceId: 5, run: 3n }, generation: 1, baseRevision: new Uint8Array(32) }, 40, 8);
    const store = new ToolRunTraceRecordStore();
    let seed = 7n;
    const next = (bound: number) => {
      seed = (seed * 6364136223846793005n + 1442695040888963407n) & 0xffff_ffff_ffff_ffffn;
      return Number((seed >> 33n) % BigInt(bound));
    };
    const palette = { fill: { testing: "#111111", success: "#222222", warning: "#333333", danger: "#444444" }, highlight: "#555555" };
    const camera = { x: 3, y: -2, zoom: 1.5 };
    const viewport = { width: 400, height: 300, pixelRatio: 2 };
    const path = {} as Path2D;
    for (let step = 0; step < 40; step += 1) {
      const ops: ToolRunTraceOp[] = Array.from({ length: next(8) + 1 }, () => {
        const roll = next(20);
        if (roll === 0) return { op: "clear" };
        if (roll < 4) return { op: "retire", key: BigInt(next(64)) };
        return { op: "upsert", key: BigInt(next(64)), verdict: TOOL_RUN_VERDICTS[next(4)]!, reason: 0, subject: roll === 4 ? { kind: "entity", entity: 1n } : { kind: "placement2d", shape: next(2), position: [next(50) - 25, next(40) - 20], rotation: next(4) * 0.5 } };
      });
      ledger.applyOps(ops);
      store.applyLane(base64UrlEncode(encodeToolRunTraceDelta(ledger.deltaAfter(store.cursor, Number.MAX_SAFE_INTEGER))));
      const { ctx, calls } = recordingContext();
      const report = paintToolRunTrace2d(ctx, store, camera, viewport, () => path, palette);
      const fills = calls.filter((call) => call.kind === "fill");
      const placements = [...ledger.records()].filter(([, record]) => record.subject.kind === "placement2d");
      expect(fills.length).toBe(placements.length);
      for (const verdict of TOOL_RUN_VERDICTS) {
        expect([...report.fills].filter(([id]) => id.endsWith(`:${verdict}`)).reduce((sum, [, count]) => sum + count, 0)).toBe(placements.filter(([, record]) => record.verdict === verdict).length);
        expect(fills.filter((call) => call.style === palette.fill[verdict]).length).toBe(placements.filter(([, record]) => record.verdict === verdict).length);
      }
      const expectedOrigins = placements.map(([, record]) => {
        const position = (record.subject as { position: readonly [number, number] }).position;
        const screen = worldToScreenLogical(position[0], position[1], camera, viewport.width, viewport.height);
        return `${(screen.x * viewport.pixelRatio).toFixed(3)},${(screen.y * viewport.pixelRatio).toFixed(3)}`;
      });
      expect(fills.map((call) => `${call.transform[4]!.toFixed(3)},${call.transform[5]!.toFixed(3)}`).sort()).toEqual(expectedOrigins.sort());
      for (const call of fills) expect(call.alpha).toBeGreaterThanOrEqual(TOOL_RUN_TRACE_METRICS.fadeFloorOpacity * TOOL_RUN_TRACE_METRICS.testingOpacity - 1e-9);
      const newest = store.newestTesting;
      const newestIsPlacement = newest !== null && ledger.record(newest)?.subject.kind === "placement2d";
      expect(report.highlighted).toBe(newestIsPlacement);
      expect(calls.filter((call) => call.kind === "stroke").length).toBe(newestIsPlacement ? 1 : 0);
    }
    const { ctx, calls } = recordingContext();
    paintToolRunTrace2d(ctx, store, camera, viewport, () => path, palette, { testing: false, accepted: true, rejected: false });
    expect(calls.every((call) => call.kind === "fill" && call.style === palette.fill.success)).toBe(true);
  });
});
