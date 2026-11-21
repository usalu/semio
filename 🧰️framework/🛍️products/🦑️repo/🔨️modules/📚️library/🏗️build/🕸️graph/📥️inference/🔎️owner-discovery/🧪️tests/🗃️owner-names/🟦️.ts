import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import {evaluateExtractedFunctionV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧩️extracted-function/🟦️.ts";
import {buildInputSourceRoleV1,buildNativeSourceObservationV1,registryOwnerIndexesV1,type BuildOwnerRegistryV1} from "../../../../🚪️admission/🟦️.ts";
import ownerAdmissionSchema from "../../../../../🧬️schema/🚪️admission.json";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

test("actual named input discovery preserves exact owners without scanning the registry per reference",async()=>{
 const source=ts.createSourceFile("owner.mjs",readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true,ts.ScriptKind.JS);
 const declaration=source.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="discoverBuildOwnerAdmissionFactsV1");
 if(!declaration)throw Error("Actual owner discovery required");
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const ownerName=(index:number)=>"p"+String(index).padStart(5,"0");
 let visited=0;
 const rows=Array.from({length:fixture.owners},(_,index)=>({name:ownerName(index),root:"p/"+index,manifest:"p/"+index+"/📋️project.json",packages:[]}));
 const owners=new Proxy(rows,{get(target,key,receiver){if(key==="some")return (predicate:(row:typeof rows[number],index:number)=>boolean)=>target.some((row,index)=>{visited++;return predicate(row,index);});return Reflect.get(target,key,receiver);}});
 const registry:BuildOwnerRegistryV1={version:1,owners,catalogProviders:[]},owner=rows[0]!;
 using oracle=new Database(":memory:");
 const query=oracle.query(`WITH RECURSIVE owners(n) AS (SELECT 0 UNION ALL SELECT n+1 FROM owners WHERE n+1 < ?1),
 refs AS (SELECT DISTINCT value AS n FROM json_each(?2))
 SELECT printf('p%05d',refs.n) AS name, owners.n IS NOT NULL AND length(?3)>0 AS valid
 FROM refs LEFT JOIN owners USING(n) ORDER BY refs.n`);
 for(const row of fixture.cases){
  const expected=query.all(fixture.owners,JSON.stringify(row.projects),row.input) as {name:string;valid:number}[];
  expect(expected.every(ref=>ref.valid===1),row.name).toBe(row.accepted);
  const execute=evaluateExtractedFunctionV1(declaration.getText(source).replace(/^export\s+/,""),"discoverBuildOwnerAdmissionFactsV1",{
   libraryBootstrap:Promise.resolve(),join,readPhysicalSource:()=>Buffer.from(JSON.stringify({name:owner.name})),createScriptInputCache:()=>({}),cargoTargets:async()=>({}),componentBuildTargets:()=>({}),existsSync:()=>false,yieldInferenceV1:async()=>{},buildInputSourceRoleV1,buildNativeSourceObservationV1,ownerAdmissionSchema,registryOwnerIndexesV1,parserTypescriptV1:()=>({sys:{}}),observedTypeScriptHost:()=>({}),
   projectWithDefaults:async()=>({namedInputs:{selected:[{input:row.input,projects:row.projects.map(ownerName)}]}}),WORKSPACE_POLICY:{},readBuildSourceInputMatchesV1:async()=>({matched:[],compileByTarget:new Map()}),
  });
  visited=0;
  const result=execute(owner,[],registry,{workspaceRoot:process.cwd()},["selected"]);
  if(row.accepted)expect((await result).dependencyInputs,row.name).toEqual(Object.fromEntries(expected.map(ref=>[ref.name,[row.input]])));
  else await expect(result,row.name).rejects.toThrow("Canonical named input owner required");
  expect(visited,row.name).toBe(0);
 }
});
