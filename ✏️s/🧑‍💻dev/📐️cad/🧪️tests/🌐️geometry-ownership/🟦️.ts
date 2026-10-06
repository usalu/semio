/** 🌐️ Physical mathematical ownership and dependency laws with independent JSON and TOML oracles. */
import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import * as TOML from "@iarna/toml";
import { getWorkspaceRoot } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
export function geometryOwnershipLaws(): void {
  const root = getWorkspaceRoot();
  const owner = join(root,"✏️s/🧑‍💻dev/📐️cad");
  const read = (path:string) => readFileSync(join(root,path),"utf8");
  const fixture = JSON.parse(readFileSync(join(owner,"🧫️fixtures/🌐️geometry-ownership/🔣️.json"),"utf8"));
  for (const row of fixture.moves) { assert(!existsSync(join(root,row.from)),`specific owner still holds generic mathematics: ${row.from}`); assert(existsSync(join(root,row.to,"🦀️.rs")),`missing general implementation: ${row.to}`); }
  const manifest = TOML.parse(read(fixture.sessionManifest)) as { dependencies:Record<string,unknown> };
  assert(manifest.dependencies[fixture.kernelPackage]);
  assert(!Object.keys(manifest.dependencies).some(name => name.includes("artifact")),"general session links a specific artifact");
  const engine = read(`${fixture.kernelRoot}/⚙️engine/🦀️.rs`);
  for (const verb of fixture.excludedKernelVerbs) assert(!engine.includes(`fn ${verb}`),`specific format verb in general kernel: ${verb}`);
  assert(existsSync(join(root,fixture.formatOwner,"🦀️.rs")),"STEP geometry owner missing");
  const genericSchema=JSON.parse(read(fixture.sessionManifest.replace("📦️packages/🦀️rust/Cargo.toml","🧬️schema/🔣️.json")));
  const operations=JSON.parse(read(`${fixture.formatOwner}/🌊️session/🧫️fixtures/🏃️operations/🔣️.json`));
  const admitsVerb = new Ajv({strict:true}).addKeyword("x-semio-formats").addSchema(genericSchema).getSchema(`${genericSchema.$id}#/$defs/SemioGeometryVerb`)!;
  for (const operation of operations.operations) assert(admitsVerb(operation), JSON.stringify(admitsVerb.errors));
  assert.deepEqual(operations.operations.map((row:{kernelOperation:string})=>row.kernelOperation),fixture.excludedKernelVerbs);
  const sessionFixture=JSON.parse(readFileSync(join(owner,"🧫️fixtures/📐️step-session/🔣️.json"),"utf8"));
  assert.equal(sessionFixture.expectedVolume,sessionFixture.box.width*sessionFixture.box.depth*sessionFixture.box.height);
  const laws=readFileSync(join(owner,"🧪️tests/📐️step-session/🦀️.rs"),"utf8");
  assert.deepEqual([...laws.matchAll(/#\[test\]\s*fn (\w+)/g)].map(match=>match[1]),sessionFixture.nativeLaws);
  console.log(`[DEBUG] geometry ownership: ${fixture.moves.length} physical mathematical owners; closed format boundary`);
}
