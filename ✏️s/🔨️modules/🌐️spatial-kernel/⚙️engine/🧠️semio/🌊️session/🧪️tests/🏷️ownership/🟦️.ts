/** 🌐️ Portable session contracts and exact native law ownership, independently validated by Ajv. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
const owner = fileURLToPath(new URL("../..", import.meta.url));
const read = (path: string) => readFileSync(join(owner,path), "utf8");
export function sessionLaws() {
  const schema = JSON.parse(read("🧬️schema/🔣️.json"));
  const ajv = new Ajv({ strict:true, allErrors:true }).addKeyword("x-semio-formats").addSchema(schema);
  const verbs = ajv.getSchema(`${schema.$id}#/$defs/SemioGeometryVerbsV1`)!;
  assert(verbs(JSON.parse(read("🔣️.json"))), JSON.stringify(verbs.errors));
  const closeReceipt = ajv.getSchema(`${schema.$id}#/$defs/SemioGeometryCloseReceiptV1`)!;
  const retirement = JSON.parse(read("🧫️fixtures/🧹️retirement/🔣️.json"));
  const closeGrant=ajv.getSchema(`${schema.$id}#/$defs/SemioGeometryCloseGrantV1`)!;
  for (const grant of retirement.grants) assert(closeGrant(grant),JSON.stringify(closeGrant.errors));
  for (const receipt of retirement.receipts) assert(closeReceipt(receipt),JSON.stringify(closeReceipt.errors));
  for (const grant of retirement.portBoundary.zeroGrants) assert(closeGrant(grant),JSON.stringify(closeGrant.errors));
  assert(closeGrant(retirement.portBoundary.childGrant),JSON.stringify(closeGrant.errors));
  for (const boundary of [retirement.capture.shellBoundary,retirement.portBoundary]) {
    for (const layout of [1,8,64,257,4096,8192]) {
      const small = Math.floor(layout / boundary.smallGrantDivisor);
      const adequate = layout * boundary.adequateGrantMultiplier;
      assert(small < layout && adequate === layout);
      const admits = ajv.compile({ type:"integer", minimum:layout });
      assert.equal(admits(small) ? "pending" : "blocked",boundary.smallPhase);
      assert.equal(admits(adequate) ? "pending" : "blocked",boundary.adequatePhase);
    }
  }
  const fixture = JSON.parse(read("🧫️fixtures/🏷️ownership/🔣️.json"));
  for (const group of fixture.groups) {
    const source = read(group.source);
    assert.deepEqual([...source.matchAll(/#\[test\]\s*fn (\w+)/g)].map(match => match[1]), group.laws);
  }
  assert(!/static (KERNEL|MESH_CACHE|TESSELLATION_JOBS)|OnceLock/.test(read("🦀️.rs")));
  console.log(`[DEBUG] Semio geometry session: ${fixture.groups.reduce((sum: number, group: any) => sum + group.laws.length, 0)} exact native laws; actual verb descriptor and close receipt contracts`);
  return fixture.groups.map((group: any) => ({ package: fixture.package, target: group.target, laws:group.laws }));
}
