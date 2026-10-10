/** 🧮️ Owns the actual OS semantic decoder comparisons for the original neutral wire corpus. */
import {test,expect} from "bun:test";
import equal from "fast-deep-equal";
import fixture from "../../../../🔨️modules/🎒️pack/🌱️value/🧫️fixtures/🧮️wire-materialization/🔣️.json";
import {decodePackValue,packValueToExactJson} from "../../🟦️.ts";

test("OS semantic Pack outputs preserve every original wire materialization comparison",()=>{
 let comparisons=0,probes=0;
 for(const row of fixture.cases){
  if(row.grammar.outcome==="accepted"){
   const actual=packValueToExactJson(decodePackValue(Buffer.from(row.rawHex,"hex")));
   expect(equal(actual,"value" in row.grammar?row.grammar.value:undefined)).toBe(true);
   expect(JSON.parse(JSON.stringify(actual))).toEqual("value" in row.grammar?row.grammar.value:undefined);
   comparisons++;
  }
  if(typeof row.allowanceProbeHex==="string"){expect(packValueToExactJson(decodePackValue(Buffer.from(row.allowanceProbeHex,"hex")))).toBe(null);probes++;}
 }
 expect(fixture.cases.length).toBe(21);
 expect(comparisons).toBe(10);
 expect(probes).toBe(1);
 console.log(`[DEBUG] OS Pack wire: original-cases=${fixture.cases.length} exact-semantic=${comparisons} allowance-probes=${probes}`);
});
