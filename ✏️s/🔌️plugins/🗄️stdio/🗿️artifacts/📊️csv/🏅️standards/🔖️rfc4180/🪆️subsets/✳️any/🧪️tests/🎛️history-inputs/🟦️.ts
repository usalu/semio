import {expect,test} from "bun:test";
import Ajv from "ajv";
import {applyPatch,type Operation} from "fast-json-patch";
import {existsSync,readFileSync} from "node:fs";
import {resolve} from "node:path";

const root=resolve(import.meta.dir,"../..");
const read=(path:string)=>JSON.parse(readFileSync(resolve(root,path),"utf8"));
const before={schema:"stdio.csv",hasHeader:false,records:[{fields:[{value:"One",quoted:false},{value:"Two",quoted:true}]},{fields:[{value:"Three",quoted:false},{value:"Four",quoted:true}]}]};
const rows=[
 {id:"header",leaf:"🧾set-has-header",mutation:{mutation:"setHasHeader",hasHeader:true},patch:[{op:"replace",path:"/hasHeader",value:true}]},
 {id:"field",leaf:"✏️set-field",mutation:{mutation:"setField",recordIndex:0,fieldIndex:0,value:"Changed",quoted:true},patch:[{op:"replace",path:"/records/0/fields/0/value",value:"Changed"},{op:"replace",path:"/records/0/fields/0/quoted",value:true}]}
];
test("CSV committed header and ordinal field intents match independent RFC6902",()=>{
 const ajv=new Ajv({strict:false});const validate=ajv.compile(read("🧬️schema/📸️snapshot/🔣️.json"));expect(validate(before)).toBe(true);
 for(const row of rows){
  const after=applyPatch(structuredClone(before),row.patch as Operation[],true,true).newDocument;expect(validate(after)).toBe(true);expect(after).not.toEqual(before);
  expect(ajv.compile(read(`🧬️schema/🧬️mutations/${row.leaf}/🧬️schema/🔣️.json`))(row.mutation)).toBe(true);
  expect(after.records[1]).toEqual(before.records[1]);
  const path=resolve(root,`🧫️fixtures/🧬️history-edits/${row.id}/🦠️mutation/🔣️.json`);expect(existsSync(path)).toBe(true);expect(JSON.parse(readFileSync(path,"utf8"))).toEqual({mutation:row.mutation,before,after});
  console.log("[DEBUG] CSV committed native intent agrees with RFC6902",row.id);
 }
});
