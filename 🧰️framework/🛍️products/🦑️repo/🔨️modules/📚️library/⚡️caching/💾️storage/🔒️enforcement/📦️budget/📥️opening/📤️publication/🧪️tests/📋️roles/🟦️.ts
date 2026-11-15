import{expect,test}from"bun:test";
import Ajv from"ajv";
import{mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import corpus from"./🔣️.json";
import{javascriptInstallSchemaV1 as schema}from"../../../../../../../🚀️bootstrap/📦️dependencies/💾️storage/🧱️schemas/🟦️.ts";

const protocol=schema.$defs.SelectedPlanProtocol.const,declared=[...protocol.directoryRoles,...protocol.fileRoles,protocol.service,...protocol.partitionRoles.map((role:{name:string})=>role.name)] as string[];

test("every role of the dependency partition has exactly one declaration with a bilingual writer and fits the plan bound",()=>{
 const valid=new Ajv({strict:true}).compile(schema.$defs.PartitionRole);
 for(const role of protocol.partitionRoles){expect(valid(role),role.name).toBe(true);}
 expect(new Set(declared).size).toBe(declared.length);
 expect(declared.length).toBeLessThanOrEqual(protocol.maximumRoles);
 for(const role of protocol.partitionRoles)expect(role.writer.en.length>0&&role.writer.de.length>0).toBe(true);
});

test("the plan accepts the real partition listings and refuses foreign, mistyped, incomplete and oversized ones, as an independent validator does",async()=>{
 const{requireRepositoryDependencyImageRolesV1}=await import("../../🟦️.ts"),names=new Ajv({strict:true}).compile({type:"object",maxProperties:protocol.maximumRoles,propertyNames:{enum:declared},properties:{j:{},"dependencies.json":{},"storage-events.jsonl":{}},required:["j","dependencies.json","storage-events.jsonl"]});
 for(const row of corpus.cases){
  const root=mkdtempSync(join(tmpdir(),"roles-"));
  try{
   for(const entry of row.listing){if(entry.kind==="directory")mkdirSync(join(root,entry.name));else writeFileSync(join(root,entry.name),"");}
   if(row.accepted){expect(requireRepositoryDependencyImageRolesV1(root,()=>{}).names,row.id).toEqual(row.listing.map(entry=>entry.name));}
   else expect(()=>requireRepositoryDependencyImageRolesV1(root,()=>{}),row.id).toThrow(row.refusal!);
   if(row.refusal!=="Physical current dependency image role required")expect(names(Object.fromEntries(row.listing.map(entry=>[entry.name,true]))),row.id).toBe(row.accepted);
  }finally{rmSync(root,{recursive:true,force:true});}
 }
});
