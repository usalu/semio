import { test, expect } from "bun:test";
import Ajv2020 from "ajv/dist/2020.js";
import { applyPatch } from "fast-json-patch";
import { readFileSync } from "node:fs";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };

test("original fixture caller spends one source stock and CPU balance through all actions", () => {
  const validate = new Ajv2020({ strict: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  expect(validate({ ...fixture, fuel: 14 })).toBe(false);
  let balance = { ...fixture.stock, fuel: fixture.fuel - 3 };
  for (const action of fixture.actions) {
    balance = applyPatch(balance, [
      { op: "replace", path: "/items", value: balance.items - 1 },
      { op: "replace", path: "/copy", value: balance.copy - action.copy },
      { op: "replace", path: "/fuel", value: balance.fuel - action.fuel },
    ], true, false).newDocument;
  }
  expect(balance.copy).toBe(3);
  expect(balance.fuel).toBe(5);
  expect(balance.capacity).toBe(fixture.stock.capacity);
  expect(balance.release).toBe(fixture.stock.release);
  expect(balance.depth).toBe(fixture.stock.depth);
  let last = 0;
  const admitted = fixture.clockRegression.map(actual => {
    const permit = actual >= last;
    if (permit) last = actual;
    return permit;
  });
  expect(admitted).toEqual([true, false, true]);
  const root = fixture.rootRetirement;
  expect(root.allocationHeaderBytes + root.aliasCounterBytes + root.rootsBytes).toBe(root.releasedBytes);
  expect(root.clockFree).toBe(true);
  const source = readFileSync(new URL("../🦀️.rs", import.meta.url), "utf8");
  expect(source).toContain("stock:RetainedCloneGrant");
  expect(source).toContain("spent:RetainedCloneProgress");
  expect(source).toContain("with_retained_loan(grant");
  expect(source).toContain("self.fuel=cx.fuel_remaining()");
  expect(source).toContain("pub fn retire_step");
  expect(source).not.toContain("let mut receipt=RetainedCloneProgress::default();let mut cx=");
});

test("original work loan narrows CPU and absolute deadline while preserving parent balances",()=>{
  const validate=new Ajv2020({strict:true}).compile(schema);
  expect(validate(fixture)).toBe(true);
  let fuel=fixture.fuel-3;
  for(const loan of fixture.workLoans){
    const child={fuel:Math.min(fuel,loan.fuel),deadline:Math.min(fixture.deadline,loan.deadline)};
    const permitted=child.fuel>0&&fixture.clock<child.deadline;
    expect(loan.spent<=child.fuel).toBe(true);
    if(!permitted)expect(loan.spent).toBe(0);
    fuel=applyPatch({fuel},[{op:"replace",path:"/fuel",value:fuel-loan.spent}],true,false).newDocument.fuel;
    expect(fuel).toBe(loan.remaining);
    expect(child.deadline<=fixture.deadline).toBe(true);
  }
  const source=readFileSync(new URL("../../../../🦀️.rs",import.meta.url),"utf8");
  expect(source.includes("pub fn with_original_work_loan")).toBe(true);
});
