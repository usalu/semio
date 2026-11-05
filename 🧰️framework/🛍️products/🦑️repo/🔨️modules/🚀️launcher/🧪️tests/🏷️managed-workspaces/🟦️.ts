import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import {readFileSync} from "node:fs";

const module=new URL("../../",import.meta.url),repository=new URL("../../../../../",module),read=(path:string,base=module)=>JSON.parse(readFileSync(new URL(path,base),"utf8")),schema=read("📦️workspaces/🧬️schema/🔣️.json"),corpus=read("📦️workspaces/🧫️fixtures/🏷️managed.json"),source=read(schema.$defs.Protocol.const.managed.sourceSchema,repository),protocol=source.$defs.SourceNamespaceClassificationProtocol.const;

test("managed namespace classification matches the existing ready schema without granting authority",()=>{
 const ajv=new Ajv({strict:true}),shape=ajv.compile(source.$defs.SourceOwnerReady),rowShape=ajv.compile(schema.$defs.ManagedCase),classification=ajv.compile(schema.$defs.NamespaceClassification),generation="a".repeat(64),sourceDirectory="/workspace/node_modules",targetDirectory="/cache/j/"+generation.slice(0,protocol.generationPrefixUnits)+"/node_modules";
 for(const row of corpus){expect(rowShape(row),row.id).toBe(true);const binding={sourceDirectory,targetDirectory},claim:any={version:source.$defs.SourceOwnerReady.properties.version.const,purpose:source.$defs.SourceOwnerReady.properties.purpose.const,scope:protocol.rootScope,generation,ownerRelationSha256:"b".repeat(64),groupSha256:"c".repeat(64),recipeSha256:"d".repeat(64),members:[],bindings:[binding],ownerReferences:[],singletons:[]};let target=targetDirectory;
  switch(row.mutation){case"generation":claim.generation="e".repeat(64);break;case"binding-source":binding.sourceDirectory="/wrong";break;case"binding-target":binding.targetDirectory="/wrong";break;case"purpose":claim.purpose="forged";break;case"version":claim.version=0;break;case"unknown-field":claim.authority=true;break;case"missing-members":delete claim.members;break;case"path":target="/foreign/"+generation.slice(0,protocol.generationPrefixUnits)+"/node_modules";break;}
  const accepted=Boolean(shape(claim)&&target===targetDirectory&&claim.generation.slice(0,protocol.generationPrefixUnits)===generation.slice(0,protocol.generationPrefixUnits)&&claim.bindings.some((binding:any)=>binding.sourceDirectory===sourceDirectory&&binding.targetDirectory===target));expect(accepted,row.id).toBe(row.accepted);
 }
 expect(classification({kind:"managed",authority:false,readiness:false,contentObserved:false})).toBe(true);
 expect(classification({kind:"managed",authority:true,readiness:false,contentObserved:false})).toBe(false);
 expect([protocol.authority,protocol.readiness,protocol.contentObserved]).toEqual([false,false,false]);
});
