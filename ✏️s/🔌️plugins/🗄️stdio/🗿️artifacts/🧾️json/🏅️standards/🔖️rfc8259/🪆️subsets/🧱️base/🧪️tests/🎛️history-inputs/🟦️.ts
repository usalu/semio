import {expect,test} from "bun:test";
import Ajv from "ajv";
import {applyPatch,type Operation} from "fast-json-patch";
import {existsSync,readFileSync} from "node:fs";
import {resolve} from "node:path";
const root=resolve(import.meta.dir,"../../..");const read=(path:string)=>JSON.parse(readFileSync(resolve(root,path),"utf8"));
const before={schema:"stdio.json",value:{kind:"object",members:[{key:"title",value:{kind:"string",value:"Old"}},{key:"enabled",value:{kind:"bool",value:false}}]}};
const rows=[
 {subset:"🧱️base",id:"string",leaf:"🔢️set-scalar",mutation:{mutation:"setScalar",payload:{phase:"apply",value:{path:[{kind:"key",value:"title"}],value:{kind:"string",value:"Hallo 😀"}}}},patch:[{op:"replace",path:"/value/members/0/value/value",value:"Hallo 😀"}]},
 {subset:"🧱️base",id:"boolean",leaf:"🔢️set-scalar",mutation:{mutation:"setScalar",payload:{phase:"apply",value:{path:[{kind:"key",value:"enabled"}],value:{kind:"bool",value:true}}}},patch:[{op:"replace",path:"/value/members/1/value/value",value:true}]},
 {subset:"🛜️i-json",id:"string",leaf:"🔤set-string",mutation:{mutation:"setString",path:[{kind:"key",value:"title"}],value:"Hallo 😀"},patch:[{op:"replace",path:"/value/members/0/value/value",value:"Hallo 😀"}]},
 {subset:"🛜️i-json",id:"rename",leaf:"🏷️rename-member",mutation:{mutation:"renameMember",path:[],from:"title",to:"heading"},patch:[{op:"replace",path:"/value/members/0/key",value:"heading"}]},
 {subset:"🧱️base",id:"member-key",leaf:"🩹️patch-snapshot",mutation:{mutation:"patchSnapshot",payload:{patch:{operation:"set",path:"/value/members/0/key",value:"heading"}}},patch:[{op:"replace",path:"/value/members/0/key",value:"heading"}]}
];
test("JSON and I-JSON committed scalar and member intents match independent RFC6902",()=>{
 const ajv=new Ajv({strict:false});const schema=read("🧱️base/🧬️schema/📸️snapshot/🔣️.json");ajv.addSchema(schema);ajv.addSchema(JSON.parse(readFileSync(resolve(root,"../../../../../📇️registry/🧬️schema/🔣️.json"),"utf8")));const validate=ajv.compile(schema);expect(validate(before)).toBe(true);
 for(const row of rows){
  const after=applyPatch(structuredClone(before),row.patch as Operation[],true,true).newDocument;expect(validate(after)).toBe(true);expect(after).not.toEqual(before);
  const payload=row.subset==="🧱️base"?(row.leaf==="🔢️set-scalar"?row.mutation.payload!.value:row.mutation.payload):row.mutation;const leafSchema=read(`${row.subset}/🧬️schema/🧬️mutations/${row.leaf}/🧬️schema/🔣️.json`);expect((ajv.getSchema(leafSchema.$id)??ajv.compile(leafSchema))(payload)).toBe(true);
  const document=Object.fromEntries(after.value.members.map(member=>[member.key,member.value.value]));expect(Object.keys(JSON.parse(JSON.stringify(document)))).toEqual(after.value.members.map(member=>member.key));
  const path=resolve(root,`${row.subset}/🧫️fixtures/🧬️history-edits/${row.id}/🦠️mutation/🔣️.json`);expect(existsSync(path)).toBe(true);expect(JSON.parse(readFileSync(path,"utf8"))).toEqual({mutation:row.mutation,before,after});
  console.log("[DEBUG] JSON committed native intent agrees with RFC6902 and ordered JSON",row.subset,row.id);
 }
});
