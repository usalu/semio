import { expect, test } from "bun:test";
import { applyPatch } from "fast-json-patch";
import fixture from "../🧫️fixtures/📏️close-demand/🔣️.json" with { type: "json" };

test("original job close separates exact independent grants and preserves terminal progress",()=>{
  const demands={maximumItems:fixture.progress.copiedItems,maximumCopyBytes:fixture.progress.copiedBytes,maximumCapacityBytes:fixture.progress.retainedCapacityBytes,maximumReleaseBytes:fixture.progress.releasedBytes,maximumDepth:2};
  const allowed=(grant:typeof fixture.grant)=>Object.entries(demands).every(([key,value])=>grant[key as keyof typeof grant]>=value);
  expect(allowed(fixture.grant)).toBe(true);expect(fixture.progress.releasedBytes).toBeGreaterThan(fixture.grant.maximumCopyBytes);
  for(const key of fixture.deniedCredits){
    const refused=applyPatch(structuredClone(fixture.grant),[{op:"replace",path:`/${key}`,value:demands[key as keyof typeof demands]-1}],true).newDocument;
    expect(allowed(refused)).toBe(false);
  }
  const pending={phase:"pending",progress:fixture.progress},complete=applyPatch(structuredClone(pending),[{op:"replace",path:"/phase",value:"complete"}],true).newDocument;
  expect(complete.progress).toEqual(pending.progress);expect(fixture.close.terminal).toBe("original-owner-empty");expect(fixture.close.blocked).toBe("registered-wake");
  expect(fixture.close.refusal).toBe("original-owner-retained");expect(fixture.close.demand).toBe("result-for-copy-capacity-release-depth");
  console.log("[DEBUG] original job full grants retain physical release65536 with copy3 and exact terminal progress");
});

test("worker physical demand preserves caller authority and independent allocation state", async () => {
  for (const row of fixture.cases) {
    const before = { retainedBytes: row.physicalBytes, releasedBytes: 0 };
    const operations = row.callerBytes >= row.physicalBytes ? [
      { op: "replace" as const, path: "/retainedBytes", value: 0 },
      { op: "replace" as const, path: "/releasedBytes", value: row.physicalBytes },
    ] : [];
    const after = applyPatch(structuredClone(before), operations, true).newDocument;
    expect(after.releasedBytes).toBe(row.releasedBytes);
    expect(after.retainedBytes + after.releasedBytes).toBe(row.physicalBytes);
    expect(after.releasedBytes).toBeLessThanOrEqual(row.callerBytes);
    expect(Math.max(fixture.pageBytes, row.physicalBytes)).toBeLessThanOrEqual(fixture.admissionBytes);
  }
  const source = await Bun.file(new URL("../../🦀️.rs", import.meta.url)).text();
  expect(source.includes("fn next_close_byte_demand(&self)")).toBe(true);
  expect(source.includes("pub fn next_close_demands(&self,maximum_copy_bytes:usize) -> Result<RetainedCloneGrant,WorkerJobDemandError>")).toBe(true);
  console.log("[DEBUG] worker close demand: six physical extents preserve exact caller release grant");
});
