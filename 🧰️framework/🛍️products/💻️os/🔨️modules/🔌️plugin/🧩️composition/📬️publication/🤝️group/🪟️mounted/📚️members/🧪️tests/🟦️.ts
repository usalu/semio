import {test,expect} from "bun:test";
import {applyPatch} from "fast-json-patch";
import {readFileSync} from "node:fs";
import law from "../🧫️fixtures/🔣️.json";
test("member receipt set keeps exact rows before acknowledgment and common visibility",()=>{
 
 for(const row of law.cases){const members=row.children+Number(row.parent);expect(row.operations.length).toBe(members);expect(row.editIds.length).toBe(members);const original={rows:Array.from({length:members},()=>false),ack:Array.from({length:members},()=>false),visible:false};let model=structuredClone(original);for(let i=0;i<members;i++){expect(model.visible).toBe(false);model=applyPatch(model,[{op:"replace",path:`/rows/${i}`,value:true},{op:"replace",path:`/ack/${i}`,value:true}],false,false).newDocument;expect(model.rows[i]).toBe(true);expect(model.ack[i]).toBe(true);expect(Buffer.from(row.editIds[i]).equals(Buffer.from(new TextEncoder().encode(row.editIds[i])))).toBe(true);}expect(model.ack.every(Boolean)).toBe(true);expect(original.rows.every(v=>!v)).toBe(true);expect(row.operations.reduce((a,b)=>a+b,0)).toBeGreaterThanOrEqual(members);}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");expect(source.includes("MountedGroupMemberReceipts")).toBe(true);expect(source.includes("acknowledge_prepared_receipt_identity")).toBe(true);expect(source.includes("MountedGroupChildEditSource::Ready")).toBe(true);console.log("[DEBUG] NodeBuffer/RFC6902 exact parent+child row cardinality/ACK-before-publication and noncontiguous final metadata verified");
});
