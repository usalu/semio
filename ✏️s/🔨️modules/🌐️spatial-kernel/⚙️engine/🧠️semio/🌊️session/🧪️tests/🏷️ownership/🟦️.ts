/** 🌐️ Portable session contracts and exact native law ownership, independently validated by Ajv. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
const owner = fileURLToPath(new URL("../../", import.meta.url));
const read = (path: string) => readFileSync(join(owner,path), "utf8");
export function sessionLaws() {
  const schema = JSON.parse(read("🧬️schema/🔣️.json"));
  const ajv = new Ajv({ strict:true, allErrors:true }).addKeyword("x-semio-formats").addSchema(schema);
  for (const [path, definition] of [["🔣️.json","SemioGeometryVerbsV1"],["🧫️fixtures/🏷️session-lifetime/🔣️.json","SemioGeometrySessionLifetimeV1"],["🧫️fixtures/🏷️ownership/🔣️.json","SemioGeometrySessionOwnershipV1"],["🧫️fixtures/🧹️retirement/🔣️.json","SemioGeometrySessionRetirementV1"]]) {
    const validate = ajv.getSchema(`${schema.$id}#/$defs/${definition}`)!;
    assert(validate(JSON.parse(read(path!))), JSON.stringify(validate.errors));
  }
  const retirement = JSON.parse(read("🧫️fixtures/🧹️retirement/🔣️.json"));
  let remaining = retirement.payloadBytes;
  const receipts = retirement.grants.map(([items, bytes]: number[]) => {
    if (remaining === 0) return { phase:"complete", items:0, bytes:0 };
    if (items === 0 || bytes === 0) return { phase:"blocked", items:0, bytes:0 };
    const credit = Math.min(bytes,remaining); remaining -= credit;
    return { phase:"pending", items:1, bytes:credit };
  });
  assert.deepEqual(receipts,retirement.receipts);
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
    const source = read(`🧪️tests/${group.directory}/🦀️.rs`);
    assert.deepEqual([...source.matchAll(/#\[test\]\s*fn (\w+)/g)].map(match => match[1]), group.laws);
  }
  assert(!/static (KERNEL|MESH_CACHE|TESSELLATION_JOBS)|OnceLock/.test(read("🦀️.rs")));
  console.log(`Semio geometry session: ${fixture.groups.reduce((sum: number, group: any) => sum + group.laws.length, 0)} exact native laws;4 portable contracts`);
  return fixture.groups.map((group: any) => ({ package: fixture.package, target: { kind:"test" as const, name:group.target }, laws:group.laws }));
}
