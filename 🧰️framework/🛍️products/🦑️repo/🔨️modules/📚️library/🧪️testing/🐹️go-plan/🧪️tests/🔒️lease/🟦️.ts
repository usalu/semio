import{test,expect}from"bun:test";
import ts from"typescript";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
import corpus from"./🔣️.json";
import schema from"../../🧬️schema/🔣️.json";

test("Go compiler and artifact publication retain the physical lease partition through settlement",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);
 const source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("go.ts",source,ts.ScriptTarget.Latest,true),node=tree.statements.find(row=>ts.isFunctionDeclaration(row)&&row.name?.text==="runCanonicalGoBuild");if(!node)throw Error("Actual Go build entry required");
 const compiled=ts.transpileModule(node.getText(tree).replace("export ","").replace(/const \{ buildGoArtifactsV1 \} = await import\([^;]+;/u,"const {buildGoArtifactsV1}=goBuilder;"),{compilerOptions:{target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.None}}).outputText,db=new Database(":memory:");
 try{for(const row of corpus.cases){
  const events:string[]=[],fail=(name:string)=>{events.push(name);if(row.fault===name||row.fault==="build-close"&&(name==="build"||name==="close"))throw Error(name);},control={},directory="/workspace/leases/m/data",image={root:"/workspace/leases/m",recheck:async()=>fail("recheck"),close:async()=>fail("close")},bindings={requestedCompilerAdmissionControlV1:()=>control,buildBudgetMs:()=>100,readBuildMode:()=>"prototype",getWorkspaceRoot:()=>"/workspace",pinnedGoEnvironmentV1:async(environment:{GOCACHE:string;GOMODCACHE:string},root:string)=>{expect(root).toBe("/workspace/tools/m");expect(environment.GOCACHE).toBe(root+"/go/cache");expect(environment.GOMODCACHE).toBe(root+"/go/mod");return {};},repoToolCacheEnv:()=>({}),repositoryCachePartitionRootV1:()=>"/workspace/tools/m",systemNativeToolPortV1:()=>({}),canonicalGoPlan:()=>({replacements:{}}),resolve:(...parts:string[])=>parts.join("/"),mkdirSync:()=>{},mkdtempSync:()=>"/scratch/go",join:(...parts:string[])=>parts.join("/"),writeFileSync:()=>{},rmSync:()=>{},process,withRepositoryCommandCustodyV1:async(_root:unknown,_image:unknown,body:()=>Promise<unknown>)=>body(),withRepositoryCacheStoreV1:async(_root:unknown,id:string,_invocation:unknown,actual:unknown,body:(store:unknown,control:unknown)=>Promise<unknown>)=>{expect(id).toBe("tools");expect(actual).toBe(control);fail("tools");return body({root:"/workspace/tools/m",environment:{},recheck:async()=>{}},actual);},requireIssuedResourceControlInvocationV1:()=>{},goBuilder:{buildGoArtifactsV1:async(_args:unknown,policy:{leaseDirectory:string})=>{expect(policy.leaseDirectory).toBe(directory);fail("build");}},openRepositoryCachePartitionV1:async(workspace:string,id:string,actual:unknown)=>{expect(workspace).toBe("/workspace");expect(id).toBe("leases");expect(actual).toBe(control);fail("open");return image;},repositoryLeaseDirectoryV1:()=>directory,requireNativeStoragePathV1:(root:string,path:string)=>{expect(root).toBe(image.root);expect(path).toBe(directory);fail("path");return path;},checkResourceLeaseControl:()=>{}},run=new Function(...Object.keys(bindings),compiled+";return runCanonicalGoBuild;")(...Object.values(bindings));
  let failure:unknown;try{await run("/module",["."],{control,invocation:{},env:{}});}catch(error){failure=error;}
  const errors=failure instanceof AggregateError?failure.errors.map((error:Error)=>error.message):failure instanceof Error?[failure.message]:[];
  expect(events).toEqual(row.events);expect(errors).toEqual(row.errors);
  const oracle=db.query("SELECT value FROM json_each(?1) ORDER BY key").all(JSON.stringify(events))as{value:string}[];expect(oracle.map(value=>value.value)).toEqual(row.events);
 }}finally{db.close();}
});
