import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import ts from"typescript";
import{readFileSync}from"node:fs";
import{join,dirname,relative,isAbsolute}from"node:path";
import{buildInputSourceRoleV1,buildNativeSourceObservationV1}from"../../../../🚪️admission/🟦️.ts";
import admissionSchema from"../../../../../🧬️schema/🚪️admission.json";
import schema from"./🧬️schema/🔣️.json";
import fixture from"./🔣️.json";
test("actual owner discovery constructs local native targets once per observation without changing precedence",async()=>{
 const source=ts.createSourceFile("owner.mjs",readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true,ts.ScriptKind.JS),declaration=source.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="discoverBuildOwnerAdmissionFactsV1");if(!declaration)throw Error("Actual owner discovery required");expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);const db=new Database(":memory:");
 try{for(const row of fixture.cases){let calls=0,physicalReads=0;const owner={name:"fixture",root:"owner",manifest:"owner/"+(row.cargo?"Cargo.toml":"📋️project.json"),packages:[]},declaration={name:owner.name,package:{name:owner.name},targets:row.authored?{build:{marker:"authored"}}:{}},context={workspaceRoot:"/workspace"},registry={owners:[owner],catalogProviders:[]},ports={libraryBootstrap:Promise.resolve(),join,dirname,relative,isAbsolute,buildPath:(value:string)=>value.replaceAll("\\","/"),readPhysicalSource:()=>{physicalReads++;return Buffer.from(JSON.stringify(declaration));},readToml:()=>declaration,createScriptInputCache:()=>({}),cargoTargets:async()=>{calls++;return row.native?{build:{marker:"native"}}:{};},componentBuildTargets:()=>row.component?{build:{marker:"component"}}:{},existsSync:()=>false,readTaxonomyProjectionV1:()=>({generatorContracts:{}}),yieldInferenceV1:async()=>{},buildInputSourceRoleV1,buildNativeSourceObservationV1,ownerAdmissionSchema:admissionSchema,readBuildGeneratedCompileOutputV1:()=>undefined,buildRelativeImportProjectV1:()=>undefined,createRequire:()=>()=>({sys:{}}),observedTypeScriptHost:()=>({})};
  const execute=new Function(...Object.keys(ports),"return ("+declarationNodeText(source)+")")(...Object.values(ports)),actual=await execute(owner,[],registry,context);const expected=db.query("select case when ?1 then 'component' when ?2 then 'authored' else 'native' end marker, 1 calls").get(Number(row.component),Number(row.authored))as{marker:string;calls:number};expect(calls,row.id).toBe(expected.calls);expect(actual.project.targets.build.marker,row.id).toBe(expected.marker);expect(expected.marker,row.id).toBe(row.expected);expect(actual.files,row.id).toEqual([owner.manifest]);expect(actual.dependencies,row.id).toEqual([]);expect(actual.dependencyInputs,row.id).toEqual({});expect(physicalReads,row.id).toBe(1);
 }}finally{db.close();}
 function declarationNodeText(file:ts.SourceFile){const node=file.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="discoverBuildOwnerAdmissionFactsV1")!;return node.getText(file).replace(/^export\s+/,"").replace("createRequire(import.meta.url)","createRequire('fixture-module')");}
});
