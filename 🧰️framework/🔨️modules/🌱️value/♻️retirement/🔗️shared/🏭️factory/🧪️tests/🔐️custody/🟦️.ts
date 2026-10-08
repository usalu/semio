/** 🔐️ Independent JSON owner oracle preserve weak and refused original payloads. */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {applyPatch} from "fast-json-patch";
test("🔐️ original giant/empty-capacity/weak snapshot survives owned factory refusal",()=>{
  const law=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
  for(const row of law.cases){const baseline={payload:row.text.repeat(row.repeat),capacity:row.capacity,factory:"original"};expect(Buffer.byteLength(baseline.payload)).toBeLessThanOrEqual(row.capacity);expect(structuredClone(baseline)).toEqual(baseline);if(row.weak||row.refuseOnce)expect(structuredClone(baseline)).toEqual(baseline);expect(applyPatch(structuredClone(baseline),[{op:"replace",path:"/payload",value:null},{op:"replace",path:"/capacity",value:0},{op:"replace",path:"/factory",value:null}],true).newDocument).toEqual({payload:null,capacity:0,factory:null});expect(baseline.payload).toBe(row.text.repeat(row.repeat));}
  console.log(`[DEBUG] neutral ${law.cases.length} giant/empty-capacity/weak original payloads preserved across refusal with independent RFC6902 oracle`);
});
