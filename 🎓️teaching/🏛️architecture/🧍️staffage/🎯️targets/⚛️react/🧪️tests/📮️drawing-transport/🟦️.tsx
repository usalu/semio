/** 📮️ A catalog asset reaches a worker as the compressed payload it was shipped as: posting the decoded asset would read every deferred geometry of every scale on the page (a structured clone visits each property) and copy all of it.
 * @see ./🥒️.feature — the scenarios
 * @see ../../🔨️modules/📚️catalog/🗜️payload/🟦️.ts — `unpackAsset`, `packedAsset`
 * @see ../../🔨️modules/🧮️pipeline/🏁️drawing/queue.ts — the drawing worker's mailbox */
import { inflateSync } from "node:zlib";
import { describe, expect, it } from "vitest";
import type { Asset } from "@teaching/staffage";
import type { StaffageServedPayload } from "../../🌐️vite/🟦️.ts";
import { packedAsset, unpackAsset } from "../../🔨️modules/📚️catalog/🗜️payload/🟦️.ts";
import { flowWireIn, flowWireOut } from "../../🔨️modules/🌊️flow/🧵️server/wire.ts";
import { DrawingQueue } from "../../🔨️modules/🧮️pipeline/🏁️drawing/queue.ts";
import { drawFigure, drawingBench, type Drawing, type DrawingRequest } from "../../🔨️modules/🧮️pipeline/🏁️drawing/🟦️.ts";
import { depictionOptions } from "../../🔨️modules/🧮️pipeline/🟦️.ts";
import { walker } from "../🧰️support/🧰️.ts";

const served = import.meta.glob<StaffageServedPayload>("../../../../🗂️catalog/*/toaster/🔣️.json", { query: "?staffage-asset", import: "default" });
const shipped = async (): Promise<StaffageServedPayload> => Object.values(served)[0]!();
const inflate = (packed: string): unknown => JSON.parse(inflateSync(Buffer.from(packed, "base64")).toString("utf8"));
const options = depictionOptions({ view: "dark-line", viewpoint: "elevation", scale: { denominator: 100 }, language: "en" });

function deferred(asset: Asset): PropertyDescriptor[] {
  const volumes = (asset.volumes ?? {}) as object;
  return [...Object.values(asset.representations).map((representation) => Object.getOwnPropertyDescriptor(representation, "geometry")), ...Object.keys(volumes).map((scale) => Object.getOwnPropertyDescriptor(volumes, scale))].filter((descriptor) => descriptor !== undefined);
}

function mailbox(): { readonly posted: DrawingRequest[]; readonly queue: DrawingQueue } {
  const posted: DrawingRequest[] = [];
  return { posted, queue: new DrawingQueue({ postMessage: (request) => void posted.push(structuredClone(request)), terminate: () => undefined, onmessage: null, onerror: null }, () => undefined) };
}

describe("📮️ drawing transport", () => {
  it("posts the compressed payload of a catalog asset and decodes no geometry on the page", async () => {
    const payload = await shipped();
    const asset = unpackAsset(payload);
    expect(deferred(asset).length).toBeGreaterThan(4);
    expect(deferred(asset).every((descriptor) => typeof descriptor.get === "function")).toBe(true);
    const { posted, queue } = mailbox();
    queue.retain(asset, options, false, () => undefined, () => undefined);
    expect(posted).toEqual([{ kind: "packed", id: 1, packed: { header: payload.header, scales: payload.scales, drawings: payload.drawings } }]);
    expect(deferred(asset).every((descriptor) => typeof descriptor.get === "function")).toBe(true);
  });

  it("draws in the worker what the authored asset draws", async () => {
    const payload = await shipped();
    const authored = inflate(payload.header) as Asset;
    for (const [scale, packed] of Object.entries(payload.drawings ?? {})) (authored.representations[scale as keyof Asset["representations"]] as { geometry?: unknown }).geometry = inflate(packed);
    Object.assign(authored, { volumes: Object.fromEntries(Object.entries(payload.scales ?? {}).map(([scale, packed]) => [scale, inflate(packed)])) });
    const query = { kind: "pose", pose: authored.presentation.defaultPose } as const;
    const { posted, queue } = mailbox();
    const drawings: Drawing[] = [];
    const lease = queue.retain(unpackAsset(payload), options, false, (drawing) => void drawings.push(drawing), (error) => { throw error; });
    lease.request(query);
    const answer = drawingBench();
    const replies = posted.map((request) => answer(request)).filter((reply) => reply !== null);
    expect(replies).toHaveLength(1);
    expect(replies[0]).toEqual({ lease: 2, drawing: drawFigure(authored, options, query) });
    expect((replies[0] as { drawing: Drawing }).drawing.markup).toContain("<svg");
  });

  it("posts an asset that was not shipped compressed as it is", () => {
    expect(packedAsset(walker)).toBeUndefined();
    const { posted, queue } = mailbox();
    queue.retain(walker, options, false, () => undefined, () => undefined);
    expect(posted).toEqual([{ kind: "asset", id: 1, asset: walker }]);
  });

  it("carries the compressed payload to the air flow worker", async () => {
    const payload = await shipped();
    const asset = unpackAsset(payload);
    const wire = structuredClone(flowWireOut({ kind: "asset", id: 7, asset }));
    expect(wire).toEqual({ kind: "packed", id: 7, packed: { header: payload.header, scales: payload.scales, drawings: payload.drawings } });
    expect(deferred(asset).every((descriptor) => typeof descriptor.get === "function")).toBe(true);
    const received = flowWireIn(wire);
    expect(received.kind === "asset" && received.id === 7 && received.asset.id === asset.id && deferred(received.asset).every((descriptor) => typeof descriptor.get === "function")).toBe(true);
    expect(flowWireIn(flowWireOut({ kind: "release", id: 7 }))).toEqual({ kind: "release", id: 7 });
    expect(flowWireOut({ kind: "asset", id: 8, asset: walker })).toEqual({ kind: "asset", id: 8, asset: walker });
  });
});
