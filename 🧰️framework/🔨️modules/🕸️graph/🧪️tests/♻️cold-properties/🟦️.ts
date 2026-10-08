import {expect,test} from "bun:test";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/♻️cold-properties/🔣️.json";
import schema from "../../🛂️manifest/🌱️value/🧬️schema/🔣️.json";
import {parsePropertyValueJson,propertyValueToJson} from "../../🛂️manifest/🌱️value/🟦️.ts";

test("cold Graph ownership preserves schema-native trees against independent JSON traversal",()=>{
 const validate=new Ajv({strict:true}).compile(schema);let actual=0,independent=0;
 for(const row of fixture.values){
  expect(validate(row)).toBe(true);const owned=propertyValueToJson(parsePropertyValueJson(row));expect(owned).toEqual(row);
  JSON.stringify(row,(_key,value)=>{if(value&&typeof value==="object"&&typeof value.kind==="string")independent++;return value;});
  const pending=[owned as typeof row];while(pending.length){const node=pending.pop()!;actual++;if(node.kind==="array")pending.push(...node.values as typeof pending);else if(node.kind==="object")pending.push(...Object.values(node.values!) as typeof pending);}
 }
 expect(actual).toBe(independent);expect(actual).toBe(fixture.expectedNodes);
 for(const profile of fixture.deepProfiles){let row:unknown={kind:"string",value:"original\u0000文字"};for(let i=0;i<profile.depth;i++)row=profile.alternating&&i%2?{kind:"object",values:{original:row}}:{kind:"array",values:[row]};const owned=parsePropertyValueJson(row);let node=owned,count=0;while(node.kind==="array"||node.kind==="object"){node=node.kind==="array"?node.values[0]!:node.values.original!;count++;}expect(count).toBe(profile.depth);expect(node).toEqual({kind:"string",value:"original\u0000文字"});console.log(`[DEBUG] cold-graph-property schema-nodes=${actual} depth=${count} alternating=${profile.alternating}`);}
});
