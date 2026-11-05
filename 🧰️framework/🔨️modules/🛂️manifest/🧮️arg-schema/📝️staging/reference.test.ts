/** 🔐️ Closed lookup coordinates are descriptive; active native source custody supplies authority. */
import {expect,test} from "bun:test";
import {semioSchemaAjvV1} from "@semio-tech/framework/schema/oracles/validator";
import schema from "./value-reference.schema.json" with {type:"json"};
import fixture from "./value-reference-cases.json" with {type:"json"};
type Change={path:readonly string[];value?:unknown;remove?:boolean};
function changed(changes:readonly Change[]):unknown{
 const value=structuredClone(fixture.base) as unknown as Record<string,unknown>;
 for(const change of changes){let target=value;for(const key of change.path.slice(0,-1))target=target[key] as Record<string,unknown>;const key=change.path.at(-1)!;if(change.remove)delete target[key];else target[key]=change.value;}
 return value;
}
test("neutral reference schema refuses malformed exact identities",()=>{
 const admit=semioSchemaAjvV1({strict:true}).compile(schema);
 for(const row of fixture.cases)expect({name:row.name,allowed:admit(changed(row.changes))}).toEqual({name:row.name,allowed:row.allowed});
});
test("source coordinate reader matches the independent schema without invoking accessors",async()=>{
 const {stagedValueReference}=await import("./reference.ts");
 const admit=semioSchemaAjvV1({strict:true}).compile(schema);
 for(const row of fixture.cases){const value=changed(row.changes);expect(stagedValueReference(value)).toBe(admit(value));}
 for(const value of [null,[],Object.create(fixture.base),{...fixture.base,source:Object.create(fixture.base.source)}])expect(stagedValueReference(value)).toBe(false);
 for(const path of [["source"],["source","token"],["source","token","action"],["slot"]]){
  const value=structuredClone(fixture.base) as unknown as Record<string,unknown>;let target=value;for(const key of path.slice(0,-1))target=target[key] as Record<string,unknown>;let reads=0;
  Object.defineProperty(target,path.at(-1)!,{enumerable:true,get(){++reads;throw new Error("source getter");}});
  expect(stagedValueReference(value)).toBe(false);expect(reads).toBe(0);
 }
});
test("all source coordinates and reference purpose participate in exact lookup matching",async()=>{
 const {sameStagedValueReference}=await import("./reference.ts");
 const equals=semioSchemaAjvV1({strict:true}).compile({const:fixture.base});
 for(const row of fixture.matches){const value=changed(row.changes);expect(equals(value)).toBe(row.expected);expect(sameStagedValueReference(fixture.base,value)).toBe(row.expected);}
 expect(sameStagedValueReference(fixture.base,{...fixture.base,extra:true})).toBe(false);
});
