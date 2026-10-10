import {expect, test} from "bun:test";
import {createRequire} from "node:module";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {type ScriptInvocation} from "../../../../../../🏃️process/🧭️routing/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";

test("JSON source owner requires its original finite child capability before effects", async () => {
  const require=createRequire(import.meta.url);
  const validate=new(require("ajv"))({strict:true}).compile({...schema,properties:{...schema.properties,sourceRoot:{const:fixture.sourceRoot}}});
  for(const row of fixture.cases) expect(validate(row.capabilities)).toBe(row.capabilityAccepted);
  const owner=await import(resolve(import.meta.dir,"../🟦️.ts"));
  expect(typeof owner.admitJsonReadSourceOwner).toBe("function");
  let effects=0;
  for(const row of fixture.cases) {
    const capabilities=structuredClone(row.capabilities),policy={version:1 as const,owner:"semio.pack.json.source",maximumElapsedMilliseconds:row.elapsed};
    const control={signal:new AbortController().signal,remainingMilliseconds:()=>row.remaining,publish:()=>{effects++;},yieldContinuation:async()=>{effects++;}};
    const invocation:ScriptInvocation={policy,control,capabilities},before=effects;
    const admit=()=>owner.admitJsonReadSourceOwner(invocation,fixture.sourceRoot);
    if(row.accepted) {
      const admitted=admit(),original=admitted.invocation;
      expect(original).toBe(invocation);
      expect(original.policy).toBe(policy);
      expect(original.control).toBe(control);
      expect(original.capabilities).toBe(capabilities);
      expect(original.control.remainingMilliseconds()).toBe(row.remaining);
      const budget=admitted.childBudgetMilliseconds();
      expect(budget).toBe(Math.floor(Math.min(row.remaining as number,Number(capabilities.maximumChildElapsedMilliseconds))));
      if(typeof row.budget!=="number") throw Error("Authored accepted child budget required");
      expect(budget).toBe(row.budget);
      capabilities.maximumChildElapsedMilliseconds=fixture.widenedAfterAdmissionChildMilliseconds;
      expect(admitted.childBudgetMilliseconds()).toBe(budget);
    } else expect(admit).toThrow();
    expect(effects).toBe(before);
  }
  console.log("[DEBUG] JSON source finite child admission preserves original identities and refuses before effects; independent Ajv corpus passed");
});

test("JSON source command consumes its mandatory admitted child ceiling",()=>{
  const source=readFileSync(resolve(import.meta.dir,"../../../../📦️packages/🦀️rust/📜️script.ts"),"utf8");
  expect(source.includes("admitJsonReadSourceOwner(this.invocation")).toBe(true);
  expect(source.includes("owner.childBudgetMilliseconds()")).toBe(true);
});
