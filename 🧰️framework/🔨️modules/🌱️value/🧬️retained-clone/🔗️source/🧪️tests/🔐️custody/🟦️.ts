/** 🔐️ Canonical original clone-source custody uses schema validation and independent patch replay. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { validateJsonSchemaSubset } from "../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { applyPatch } from "fast-json-patch";
import schema from "../../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔐️custody.json";

test("original clone-source corpus keeps issuer and aliases before terminal handback", async () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  for (const row of fixture.cases) {
    const original = { owner: row.owner, metadata: row.metadata, alias: row.owner, terminal: false };
    for (const _axis of row.denied) {
      expect(applyPatch(structuredClone(original), []).newDocument).toEqual(original);
    }
    const returned = applyPatch(structuredClone(original), [{ op: "replace", path: "/terminal", value: true }]).newDocument;
    expect(returned).toEqual({ ...original, terminal: row.expected.terminal });
    expect(returned.owner).toBe(row.owner);
    expect(returned.metadata).toBe(row.metadata);
  }
  const source = await Bun.file(new URL("../../🦀️.rs", import.meta.url)).text();
  expect(source).toContain("pub fn admit_borrowed");
  expect(source).toContain("SealedShared::<RetainedCloneLeaseOwner>");
  expect(source).not.toContain("pub fn take_owner(&mut self)->Option<Arc<T>>");
  const binding = await Bun.file(new URL("../../📦️packages/🦀️rust/🦀️.rs", import.meta.url)).text();
  expect(binding).toContain("binding_copy_bytes");
  expect(binding).toContain("try_duplicate(grant)");
});
            
test("zero metadata copy admits an exact unsigned budget and rejects unrepresentable negative authority with the independent AJV oracle",()=>{
 const budget={type:"integer",minimum:fixture.currency.minimumCopyBudget};const independent=new Ajv().compile(budget);
 for(const value of [fixture.currency.metadataCopyBytes,Number(BigInt(fixture.currency.minimumCopyBudget)-1n)])expect(validateJsonSchemaSubset(budget,value).length===0).toBe(independent(value));
 expect(fixture.currency.metadataCopyBytes).toBe(0);expect(fixture.currency.negativeCopyBudgetRepresentable).toBe(false);expect(fixture.currency.bindingDenied).toEqual(["items","depth"]);expect(fixture.currency.transferDenied).toEqual(["items","release","depth"]);
 const validate=new Ajv().compile(schema);expect(validate({...fixture,currency:{...fixture.currency,metadataCopyBytes:1}})).toBe(false);expect(validate({...fixture,currency:{...fixture.currency,minimumCopyBudget:-1}})).toBe(false);
});
