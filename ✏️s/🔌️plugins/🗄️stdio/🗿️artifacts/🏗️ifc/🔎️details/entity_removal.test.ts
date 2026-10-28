import {test,expect} from "bun:test";
import Ajv from "ajv";
import {applyPatch} from "fast-json-patch";
import contract from "./🧪️tests/entity-removal.json";
import {applyIfcMutation,inverseIfcMutation} from "../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts";
import type {IfcSnapshot} from "../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
const exact=(value:unknown)=>JSON.stringify(value,(_key,item)=>typeof item==="bigint"?item.toString():item);
const unique=new Ajv().compile({type:"array",uniqueItems:true});
for(const row of contract.cases)test(`original entity removal ${row.id}`,()=>{
 const source:IfcSnapshot={schema:"stdio.ifc",header:{fileDescription:[],fileName:[],fileSchema:[]},entities:contract.ids.map((id,index)=>({id:BigInt(id),name:"IFCMATERIAL",args:[{kind:"string",value:contract.names[index]!},{kind:"unset"},{kind:"unset"}]}))};
 if("duplicate"in row&&row.duplicate)source.entities.push(structuredClone(source.entities[row.position]!));
 const selected=source.entities[row.position],admitted=Boolean(selected)&&Boolean(unique(source.entities.filter(entity=>entity.id===selected!.id).map(entity=>entity.id.toString())));
 expect(admitted).toBe(row.admitted);if(!admitted)return;
 const mutation={mutation:"removeEntity" as const,id:selected!.id},next=applyIfcMutation(source,mutation);
 const oracle=applyPatch(JSON.parse(exact(source)),[{op:"remove",path:`/entities/${row.position}`}],true,true).newDocument;
 expect(exact(next)).toBe(exact(oracle));expect(next.entities.map(entity=>entity.id.toString())).toEqual(row.remaining??[]);
 expect(exact(inverseIfcMutation(source,mutation).reduce(applyIfcMutation,next))).toBe(exact(source));
});
