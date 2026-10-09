import assert from "node:assert/strict";
import { applyPatch, type Operation } from "fast-json-patch";
import corpus from "../../🧫️fixtures/🔣️.json" with { type: "json" };
import pageCeiling from "../../🧫️fixtures/📏️owner-page-ceiling.json" with { type: "json" };
import releaseAuthority from "../../../🧬️retained-clone/🧫️fixtures/📏️release-authority/🔣️.json" with { type: "json" };
import { test } from "bun:test";

test("owner payload page ceiling preserves independently patched order and exact UTF-8 input", () => {
  const patched = applyPatch([...pageCeiling.input], pageCeiling.append.map(value => ({op:"add" as const,path:"/-",value})), true).newDocument;
  assert.deepEqual(patched,pageCeiling.expected);
  assert.equal(patched.reduce((sum,value)=>sum+value,0),pageCeiling.sum);
  for(const count of pageCeiling.boundaryCounts){
    const slots=pageCeiling.ownerPayloadPageBytes/pageCeiling.elementBytes;
    assert.equal(Math.ceil(count/slots)*slots >= count,true);
    assert.equal(Math.min(slots,pageCeiling.capacity)*pageCeiling.elementBytes,pageCeiling.ownerPayloadPageBytes);
  }
  assert.deepEqual(Buffer.from(JSON.stringify(patched)),Buffer.from(JSON.stringify(pageCeiling.expected)));
  console.log("[DEBUG] Paged list owner page ceiling=512 ordered third-party rows="+patched.length);
});

export function testPagedListOwnership(): void {
  const operations: Operation[] = Array.from({ length: corpus.ordered.count }, (_, value) => ({ op: "add", path: "/-", value }));
  const values = applyPatch<number[]>([], operations, true).newDocument;
  assert.deepEqual(values, Array.from({ length: corpus.capacity.maximum }, (_, value) => value));
  assert.equal(values.reduce((sum, value) => sum + value, 0), corpus.ordered.sum);
  assert.equal(corpus.capacity.rejected, values.length + 1);
  assert(corpus.oversized.elementBytes > corpus.oversized.grantBytes);
  for (const row of corpus.counter.signedLimits) {
    const limit = BigInt(row.limit);
    assert.equal(limit, (1n << BigInt(row.bits - 1)) - 1n);
    assert(BigInt(row.before) + BigInt(row.requested) <= limit);
    const retained = BigInt(row.before) + BigInt(row.actual);
    assert(retained > limit);
    assert(BigInt(row.actual) <= BigInt(row.grant));
    const result = applyPatch({ allocated: row.before, result: "admitted" }, [
      { op: "replace", path: "/allocated", value: retained.toString() },
      { op: "replace", path: "/result", value: "rejected-owner-retained" },
    ], true).newDocument;
    assert.deepEqual(result, { allocated: row.retained, result: row.result });
    assert.equal(retained - BigInt(row.actual), BigInt(row.before));
  }
}


test("retained release authority keeps physical ownership distinct from copied input", () => {
  const extent = new Uint8Array(releaseAuthority.physicalCursorBytes);
  extent[2] = releaseAuthority.inputByte;
  const copied = Buffer.from(extent.subarray(2, 3));
  assert.equal(copied.byteLength, releaseAuthority.expected.copiedBytes);
  assert.equal(copied[0], releaseAuthority.inputByte);
  assert(copied.byteLength <= releaseAuthority.grant.maximumCopyBytes);
  const retained = applyPatch({owner:"retained",releasedBytes:0}, [], true).newDocument;
  assert(releaseAuthority.oneBelowReleaseBytes < extent.byteLength);
  assert.equal(retained.owner, releaseAuthority.expected.ownerAfterOneBelow);
  const released = applyPatch(retained, [{op:"replace",path:"/owner",value:"empty"},{op:"replace",path:"/releasedBytes",value:extent.byteLength}], true).newDocument;
  assert.equal(released.owner, releaseAuthority.expected.ownerAfterExact);
  assert.equal(released.releasedBytes, releaseAuthority.expected.releasedBytes);
  assert(released.releasedBytes > releaseAuthority.grant.maximumCopyBytes);
  assert(released.releasedBytes <= releaseAuthority.grant.maximumReleaseBytes);
  console.log("[DEBUG] Independent release oracle copy=64 release=4096 physical=128 one-below retains owner");
});

test("retained empty tail permits release while the earlier page stays live", () => {
  const frontier=corpus.ordered.emptyTailRelease;
  const values=applyPatch<number[]>([],Array.from({length:corpus.ordered.count},(_,value)=>({op:"add" as const,path:"/-",value})),true).newDocument;
  const slots=Math.min(corpus.maximumPageBytes/frontier.elementBytes,corpus.capacity.maximum);
  assert.equal(frontier.earlierLiveItems,slots);
  assert.equal(frontier.tailItems,values.length-slots);
  const pages=[values.slice(0,slots),values.slice(slots)];
  const edited=applyPatch(pages,Array.from({length:frontier.tailItems},()=>({op:"remove" as const,path:"/1/0"})),true).newDocument;
  assert.equal(edited[1].length,0);
  assert.equal(edited[0].length,frontier.earlierLiveItems);
  assert.deepEqual(edited[0],Array.from({length:frontier.earlierLiveItems},(_,value)=>value));
  console.log("[DEBUG] original paged list neutral empty tail="+frontier.tailItems+" earlier live="+frontier.earlierLiveItems+" independentJSONPatch=true");
});
