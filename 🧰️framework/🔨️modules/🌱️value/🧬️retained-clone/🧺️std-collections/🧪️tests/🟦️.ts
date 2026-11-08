/** 🎟️ A new copy maximum selects its own UTF8 event without remembering an earlier quote. */
import{expect,test}from"bun:test";
import Ajv from"ajv";
import schema from"../🧬️schema/🔣️.json";
import fixture from"./🧫️fixtures/🔣️.json";
test("language-neutral reselected copy authority copies the exact admitted byte prefix",()=>{expect(new Ajv().compile(schema)(fixture)).toBe(true);for(const row of fixture.copySelection){const source=Buffer.from(row.source,"utf8");const prefix=[...source.subarray(0,Math.min(source.length,row.admittedBytes))];expect(prefix).toEqual(row.expectedPrefixBytes);expect(prefix.length).toBe(row.expectedCopiedBytes);expect(prefix.length).toBeLessThanOrEqual(row.admittedBytes);expect(row.admittedBytes).toBeLessThan(row.requestedBytes);}});

test("nested original completion transitions retain structural authority after the child is ready or terminal",()=>{const validate=new Ajv().compile(schema);expect(validate(fixture)).toBe(true);for(const row of fixture.completionSelection){expect(row.minimumDepth).toBeGreaterThan(0);const denied=structuredClone(fixture);denied.completionSelection=denied.completionSelection.map(x=>({...x,minimumDepth:0}));expect(validate(denied)).toBe(false);expect(row.copiedItems).toBe(1);expect(row.copiedBytes).toBe(0);}});
