/** 🧪️ Graph release failure laws: the failure of g1-m10 (2026-11-18 04:04 UTC: first failure `Original nested cache custody refused`, shown to the user only as `Graph readers and namespace release failed`) now names the operation, the place, every failed step and the first failure in both languages; one failure is never wrapped; the closed schema and an independent re-rendering agree. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {NxGraphReleaseFailedErrorV1,nxGraphReleaseFailureV1} from "../🟦️.ts";
const validate=new Ajv({strict:false}).addSchema(schema).compile({$ref:schema.$id+"#/definitions/Failure"});
const first=Error("Original nested cache custody refused for @semio-tech/norm-din4108:build:prototype on /w/p/.🧾️store/build.json: x"),second=Error("child close: lease released twice"),third=Error("lease: ENOENT");
const where="/w/.cache/b/g/n/0123456789abcdef";
test("the failure of g1-m10 keeps its first failure and names every step in both languages",()=>{
 const error=nxGraphReleaseFailureV1("namespace-release",where,[{step:"readers",error:first},{step:"child",error:second},{step:"lease",error:third}]) as NxGraphReleaseFailedErrorV1;
 expect(error).toBeInstanceOf(NxGraphReleaseFailedErrorV1);expect(error).toBeInstanceOf(AggregateError);expect(error.errors).toEqual([first,second,third]);expect(error.cause).toBe(first);expect(error.first).toBe(first.message);
 expect(error.en).toStartWith("Graph readers and namespace release failed at "+where+": first failure: "+first.message);
 expect(error.de).toStartWith("Freigabe von Graph-Lesern und Namensraum fehlgeschlagen bei "+where+": erster Fehler: "+first.message);
 expect(error.en).toContain("readers ("+first.message+"); funded child ("+second.message+"); namespace lease ("+third.message+")");
 expect(error.de).toContain("Leser ("+first.message+"); finanziertes Kind ("+second.message+"); Namensraum-Lease ("+third.message+")");
 expect(error.message).toBe(error.en+" | "+error.de);expect(validate(error.toJSON())).toBe(true);expect(error.toJSON().steps.map(row=>row.step)).toEqual(["readers","child","lease"]);
});
test("one failure is thrown as it is and none is no error",()=>{
 expect(nxGraphReleaseFailureV1("store-close",where,[{step:"lease",error:first}])).toBe(first);expect(nxGraphReleaseFailureV1("cleanup",where,[])).toBeUndefined();
 expect(nxGraphReleaseFailureV1("cleanup",where,[{step:"lease",error:first},{step:"namespace",error:first}])).toBe(first);
});
test("each operation has its own bilingual title",()=>{
 for(const [operation,en,de] of [["namespace-release","Graph readers and namespace release failed","Freigabe von Graph-Lesern und Namensraum fehlgeschlagen"],["store-close","Graph namespace acquisition and close failed","Erwerb und Schliessen des Graph-Namensraums fehlgeschlagen"],["cleanup","Original native graph cleanup failed","Bereinigung des nativen Graphen fehlgeschlagen"]] as const){
  const error=nxGraphReleaseFailureV1(operation,where,[{step:"lease",error:first},{step:"namespace",error:second}]) as NxGraphReleaseFailedErrorV1;expect(error.operation).toBe(operation);expect(error.en).toStartWith(en);expect(error.de).toStartWith(de);expect(validate(error.toJSON())).toBe(true);
 }
});
test("a long failure message is cut and a non-error value is named",()=>{
 const error=nxGraphReleaseFailureV1("namespace-release",where,[{step:"readers",error:"x".repeat(5000)},{step:"child",error:Error("y")}]) as NxGraphReleaseFailedErrorV1;
 expect(error.first.length).toBeLessThanOrEqual(603);expect(error.first.endsWith("...")).toBe(true);expect(validate(error.toJSON())).toBe(true);
});
test("the rendering agrees with an independent re-rendering over seeded failure lists",()=>{
 const names={readers:["readers","Leser"],child:["funded child","finanziertes Kind"],lease:["namespace lease","Namensraum-Lease"],namespace:["namespace","Namensraum"]} as const,steps=Object.keys(names) as (keyof typeof names)[];let seed=12345;const next=(limit:number)=>{seed=(seed*1103515245+12345)%2147483648;return seed%limit;};
 for(let round=0;round<200;round++){
  const count=2+next(5),rows=Array.from({length:count},(_,at)=>({step:steps[next(4)]!,error:Error("failure "+round+"."+at)})),error=nxGraphReleaseFailureV1("namespace-release",where,rows) as NxGraphReleaseFailedErrorV1;
  expect(error.en).toBe("Graph readers and namespace release failed at "+where+": first failure: failure "+round+".0; failed steps: "+rows.map(row=>names[row.step][0]+" ("+row.error.message+")").join("; "));
  expect(error.de).toBe("Freigabe von Graph-Lesern und Namensraum fehlgeschlagen bei "+where+": erster Fehler: failure "+round+".0; fehlgeschlagene Schritte: "+rows.map(row=>names[row.step][1]+" ("+row.error.message+")").join("; "));
  expect(validate(error.toJSON())).toBe(true);
 }
});
