/** ⏱️ Progress and cancellation of the expensive operations; invalid documents are refused. */
import { describe, expect, it } from "vitest";
import { CancelledError, ModelInvalidError, Pace, featureEdges, tessellate, tessellateSolid, validateModel } from "../../🟦️.ts";
import { triangulatePolygon } from "../../🔺️tessellation/🔺️polygon/🟦️.ts";
import type { ModelDocument, Operation } from "../../🧬️schema/🟦️.ts";
import { fixture } from "../🧰️support/🟦️.ts";
import { caseOf, denseDocument, denseResolved, expectLaw, polygonDocument, polygonOf, probe, vector } from "../🧰️support/⏱️operation/🟦️.ts";

const quality = { chordM: 0.002, minSegments: 12, maxSegments: 64 };
const many = (): ModelDocument => {
  const base = fixture("📦️box").cases[0].document as ModelDocument;
  return { ...base, solids: Array.from({ length: 40 }, (_, i) => ({ ...base.solids[0]!, id: `box-${i}`, transform: { translation: [i, 0, 0] } })) } as ModelDocument;
};

describe("operation", () => {
  it("reports monotone progress ending at one", async () => {
    const seen: number[] = [];
    await tessellate(many(), quality, { report: (f) => seen.push(f), cancelled: () => false });
    expect(seen.length).toBeGreaterThan(2);
    expect(seen.every((f, i) => f >= 0 && f <= 1 && (i === 0 || f >= seen[i - 1]!))).toBe(true);
    expect(seen[seen.length - 1]).toBe(1);
  });
  it("rejects with CancelledError once cancelled", async () => {
    let calls = 0;
    const operation: Operation = { report: () => { calls++; }, cancelled: () => calls > 3 };
    await expect(tessellate(many(), quality, operation)).rejects.toBeInstanceOf(CancelledError);
    await expect(featureEdges(await tessellate(many(), quality), 0.5, {}, { report: () => {}, cancelled: () => true })).rejects.toBeInstanceOf(CancelledError);
  });
  it("refuses invalid documents with the issues", async () => {
    const doc = { ...(many()), version: "x" } as unknown as ModelDocument;
    await expect(tessellate(doc, quality)).rejects.toBeInstanceOf(ModelInvalidError);
  });

  it("Pace polls at its interval, reports with the poll and throws at the first positive answer", () => {
    const { operation, state } = probe(2);
    const pace = new Pace(operation, 10);
    for (let i = 0; i < 19; i++) pace.tick(1, 0.5);
    expect(state.polls).toBe(1);
    expect(state.reports).toEqual([0.5]);
    expect(() => pace.tick(30)).toThrow(CancelledError);
    expect(state.polls).toBe(2);
  });
});

describe("the cancellation law (fixture ⏱️operation, fast-check picks the poll)", () => {
  const quality = vector().quality;
  it("validateModel cancels inside the outline pair loops", () => expectLaw((operation) => validateModel(polygonDocument(), operation), caseOf("validateModel")));
  it("tessellateSolid cancels inside one dense solid and its result is unchanged by polling", async () => {
    const document = denseDocument();
    const reference = tessellateSolid(document.solids[0]!, quality);
    const result = await expectLaw((operation) => tessellateSolid(document.solids[0]!, quality, undefined, operation), caseOf("tessellateSolid"));
    expect(result).toEqual(reference);
  });
  it("tessellate cancels inside the only solid, rejects without a document and leaves the input alone", async () => {
    const document = denseDocument();
    const before = JSON.stringify(document);
    const result = await expectLaw((operation) => tessellate(document, quality, operation), caseOf("tessellate"));
    expect(result.solids).toEqual([]);
    expect(JSON.stringify(document)).toBe(before);
  });
  it("triangulatePolygon cancels inside the ear search", async () => {
    const outer = polygonOf();
    const reference = triangulatePolygon(outer, []);
    const result = await expectLaw((operation) => triangulatePolygon(outer, [], new Pace(operation)), caseOf("triangulatePolygon"));
    expect(result).toEqual(reference);
  });
  it("featureEdges cancels inside one dense primitive", async () => {
    const resolved = await denseResolved();
    const reference = await featureEdges(resolved, 0.5);
    const result = await expectLaw((operation) => featureEdges(resolved, 0.5, {}, operation), caseOf("featureEdges"));
    expect(result).toEqual(reference);
  });
});
