import {expect,spyOn,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import ts from "typescript";
import type {FleetManifestEntryV1} from "../../../📒️journal/🟦️.ts";
import {INDEX_BASE_V1,INDEX_PREFIX_V1,indexRealmV1,type IndexHostV1} from "../../../🌿️git/🟦️.ts";
import {ReplicaQueueV1} from "../../🟦️.ts";
import {CompactTableV1,ReplicaBookV1} from "../🟦️.ts";
import witness from "./🔣️.json" with {type:"json"};
import schema from "./🧬️.json" with {type:"json"};

const source=ts.createSourceFile("replica.ts",readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true),host=source.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="indexHost"),plain=source.statements.find(node=>ts.isVariableStatement(node)&&node.declarationList.declarations.some(value=>value.name.getText(source)==="plain"));
const code=ts.transpileModule(host!.getText(source)+"\n"+plain!.getText(source),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
const buildHost=Function("indexHosts","INDEX_BASE_V1","INDEX_PREFIX_V1","indexRealmV1",code+";return indexHost")(new WeakMap(),INDEX_BASE_V1,INDEX_PREFIX_V1,indexRealmV1) as (input:{book:ReplicaBookV1;manifest:CompactTableV1;files:CompactTableV1;queue:ReplicaQueueV1;settings:{worktree:string;originId:string;groupId:string;now:()=>number}})=>IndexHostV1;
const hlc="1700000000000-000000-g1-server";
function entry(operation:"link"|"delete"|"put"):FleetManifestEntryV1{
 if(operation==="link")return {operation,hlc,target:"100644 "+"a".repeat(40)};
 if(operation==="delete")return {operation,hlc};
 return {operation,hlc,sha256:"b".repeat(64),bytes:1,executable:false};
}

test("stage register projections decode only the exact live sparse registers in ID order",()=>{
 expect(new Ajv({strict:true}).validate(schema,witness)).toBe(true);
 const database=new Database(":memory:");
 try{
  const oracle=database.query(`with typed as (
   select key as id,json_extract(value,'$.path') as path,json_extract(value,'$.manifest') as manifest,json_extract(value,'$.held') as held,json_extract(value,'$.queue') as queue from json_each(?1)
  ), registers as (select * from typed where path=?2 or substr(path,1,length(?3))=?3)
  select 'each' as view,id,path,manifest as operation from registers where path<>?2 and manifest is not null
  union all select 'held',id,path,held from registers where held is not null
  union all select 'pending',id,path,manifest from registers where queue='pending'
  order by view,id`).all(JSON.stringify(witness.rows),INDEX_BASE_V1,INDEX_PREFIX_V1) as {view:string;path:string;operation:string}[];
  const original=new ReplicaBookV1();
  for(const row of witness.rows){const id=original.intern(row.path);expect(original.intern(row.path)).toBe(id);}
  for(let i=0;i<witness.ordinaryRows;i++)original.intern("file-"+i+".ts");
  const columns=original.columns(),checkpoint=new ReplicaBookV1();
  checkpoint.intern(".cache/f/index/replaced");
  checkpoint.adopt({...columns,starts:columns.starts.slice(),lengths:columns.lengths.slice(),hashes:columns.hashes.slice(),folded:columns.folded.slice(),pages:columns.pages.map(page=>Buffer.from(page))},original.count,original.origins);
  for(const book of [original,checkpoint]){
   const later=book.intern(".cache/f/index/later");
   const manifest=new CompactTableV1(book,false),files=new CompactTableV1(book,true),queue=new ReplicaQueueV1(book,manifest);
   for(const row of witness.rows){
    const id=book.find(row.path)!;
    if(row.manifest!==null)manifest.write(id,entry(row.manifest as "link"|"delete"|"put"));
    if(row.held!==null)files.write(id,entry(row.held as "link"|"delete"|"put"));
    if(row.queue==="pending")queue.addId(id);else if(row.queue==="parked")queue.parkId(id);
   }
   for(let i=0;i<witness.ordinaryRows;i++){
    const id=book.find("file-"+i+".ts")!,row=entry("put");
    manifest.write(id,row);files.write(id,row);queue.addId(id);
   }
   manifest.write(later,entry("link"));manifest.clearId(later);
   const views=buildHost({book,manifest,files,queue,settings:{worktree:"/fixture",originId:"g1-server",groupId:"g1",now:()=>0}}),base=views.base(),observed:{view:string;path:string;operation:string}[]=[];
   expect(base?.operation).toBe("link");
   const reads=spyOn(book,"name");
   try{
    views.each((path,row)=>observed.push({view:"each",path,operation:row.operation}));
    views.held((path,row)=>observed.push({view:"held",path,operation:row.operation}));
    for(const [path,row] of views.pending())observed.push({view:"pending",path,operation:row.operation});
    expect(observed).toEqual(oracle.map(({view,path,operation})=>({view,path,operation})));
    expect(reads.mock.calls.length).toBe(oracle.length);
   }finally{reads.mockRestore();}
  }
 }finally{database.close();}
});
