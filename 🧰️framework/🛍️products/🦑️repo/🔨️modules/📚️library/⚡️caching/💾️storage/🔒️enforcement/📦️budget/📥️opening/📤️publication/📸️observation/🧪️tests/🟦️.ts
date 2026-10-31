import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import * as ts from "typescript";
import {readFileSync} from "node:fs";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
import * as native from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {nativeStorageBackendsV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🔌️backends/📋️contract/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

function sourceFunction(name:string,ports:Record<string,unknown>):(...args:any[])=>any{
 const source=readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("observation.ts",source,ts.ScriptTarget.Latest,true),node=tree.statements.find((row):row is ts.FunctionDeclaration=>ts.isFunctionDeclaration(row)&&row.name?.text===name);expect(node).toBeDefined();return new Function(...Object.keys(ports),ts.transpileModule(node!.getText(tree)+";return "+name+";",{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText)(...Object.values(ports));
}

test("maintenance parent observation uses actual canonical backing and bounded exact DATA",async()=>{
 const owner=await import("../🟦️.ts");expect(typeof owner.observeRepositoryDependencyMaintenanceParentV1).toBe("function");
 const read=sourceFunction("readObservedRepositoryDependencyParentV1",{observationSchema:schema,validateJsonSchemaSubset,readNativeProtectedMaintenanceRequestV1:native.readNativeProtectedMaintenanceRequestV1}),validate=new Ajv({strict:true}).compile(schema),db=new Database(":memory:");
 try{for(const row of fixture.cases){
  const value:any=structuredClone(fixture.facts);
  if(row.mode==="equal-ceiling")value.maximumPhysicalBytes=value.backing.size;
  if(row.mode==="over-ceiling")value.maximumPhysicalBytes=value.backing.size-1;
  if(row.mode==="directory-link")value.directory.kind="link";
  if(row.mode==="backing-link")value.backing.kind="link";
  if(row.mode==="root-link")value.root.kind="link";
  if(row.mode==="hardlink")value.backing.nlink=2;
  if(row.mode==="empty-backing")value.backing.size=0;
  if(row.mode==="unsafe-inode")value.directory.ino=Number.MAX_SAFE_INTEGER+1;
  if(row.mode==="bad-receipt")value.receiptSha256="opaque";
  if(row.mode==="extra")value.callback=true;
  let accepted=true;try{expect(read(value)).toEqual(fixture.expected);}catch{accepted=false;}
  const relation=db.query("select json_extract(?1,?2)<=json_extract(?1,?3) and json_extract(?1,?4)<=json_extract(?1,?3) accepted").get(JSON.stringify(value),"$.backing.size","$.maximumPhysicalBytes","$.backing.allocatedBytes")as{accepted:number};
  expect(accepted,row.mode).toBe(row.accepted);expect(accepted).toBe(Boolean(validate(value)&&relation.accepted));expect(validateJsonSchemaSubset(schema as never,value).length===0).toBe(Boolean(validate(value)));
 }}finally{db.close();}
});

test("backing selection refuses missing ambiguous and foreign entries without a host default",()=>{
 const select=sourceFunction("selectObservedRepositoryDependencyBackingV1",{nativeStorageBackendsV1}),db=new Database(":memory:");try{for(const row of fixture.backings){let result:string|null=null;try{result=select(row.names).backingFilename;}catch{}expect(result).toBe(row.expected);const decision=db.query("select json_array_length(?1)=1 and (select value from json_each(?1) limit 1) in (?2,?3) accepted").get(JSON.stringify(row.names),"fixed.dmg","fixed.dir")as{accepted:number};expect(result!==null).toBe(Boolean(decision.accepted));}}finally{db.close();}
});
