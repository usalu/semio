import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import ts from"typescript";
import{readFile}from"node:fs/promises";
import corpus from"../🔣️.json";
import schema from"../🧬️schema.json";
test("retention negative operations leave accepted parent frames through strict received purpose results",async()=>{
 const source=await readFile(new URL("../../🟦️.ts",import.meta.url),"utf8"),ast=ts.createSourceFile("retention.ts",source,ts.ScriptTarget.Latest,true),direct:string[]=[],db=new Database(":memory:");
 try{expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const visit=(node:import("typescript").Node)=>{if(ts.isAwaitExpression(node)&&/^await expect\((?:owner|storeOwner)\.withRepositoryCacheStore(?:Queued)?MaintenanceV1/u.test(node.getText(ast)))direct.push(node.getText(ast));ts.forEachChild(node,visit);};visit(ast);db.exec("create table cases(purpose text primary key,waited integer)");for(const row of corpus.cases)db.query("insert into cases values(?,?)").run(row.purpose,Number(row.waited));expect(db.query("select count(*) cases,sum(waited) waits from cases").get()).toEqual({cases:8,waits:2});expect(direct).toEqual([]);
 }finally{db.close();}
});

import{existsSync}from"node:fs";
import{join}from"node:path";
import{observeRetentionNegativePurposeV1}from"../🟦️.ts";
import{withBuildTestInvocationV1}from"../../../../../../../../🏗️build/🧪️tests/📥️invocation/🟦️.ts";
import{repositoryCachePartitionRootV1}from"../../../../../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
test("retention negative receiver retains actual strict wire and rejected request status without physical effects",async()=>withBuildTestInvocationV1(async original=>{
 const workspace=join(repositoryCachePartitionRootV1(process.cwd(),"products"),"tr-negative-proof");expect(existsSync(workspace)).toBe(false);for(const purpose of["queued-invalid-0","queued-invalid-1","queued-invalid-2"])await observeRetentionNegativePurposeV1(original,workspace,purpose);expect(existsSync(workspace)).toBe(false);expect(original.control.signal.aborted).toBe(false);
}));

test("retention negative receiver closes partial readers while preserving undefined and frozen setup failures",async()=>withBuildTestInvocationV1(async original=>{
 const text=await readFile(new URL("../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("negative.ts",text,ts.ScriptTarget.Latest,true),node=tree.statements.find(row=>ts.isFunctionDeclaration(row)&&row.name?.text==="runRetentionNegativePurposeV1");if(!node)throw Error("Actual negative receiver function required");const code=ts.transpileModule(node.getText(tree).replace("export ",""),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,routing=await import("../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),db=new Database(":memory:");
 try{for(const kind of["undefined","frozen"]){let opened=0,closed=0,caught=false,actual:unknown;const primary=kind==="undefined"?undefined:Object.freeze(Error("Frozen setup primary")),ports={requirePurpose:()=>"queued-cancel",requireWorkspace:()=>{},checkScriptInvocation:routing.checkScriptInvocation,acknowledgement:()=>({}),ackPrefix:"",failurePrefix:"",console:{log(){}},AbortController,createIssuedResourceControlV1:routing.createIssuedResourceControlV1,repositoryCacheBudgetV1:()=>({partitions:{tools:{directory:"model"}}}),openRepositoryCachePartitionV1:async()=>{opened++;return{close:async()=>{closed++;throw Error("Secondary close failure");}};},leases:Object.defineProperty({},"acquireQueuedResourceLease",{get(){throw primary;}}),physical:{queuedMaintenance:{}},owner:{},delay:()=>{throw Error("No wait entered");},repositoryLeaseDirectoryV1:()=>"model"},run=new Function(...Object.keys(ports),code+";return runRetentionNegativePurposeV1;")(...Object.values(ports));try{await run(original,"explicit-storage-port-model","queued-cancel");}catch(error){caught=true;actual=error;}expect(caught).toBe(true);expect(actual).toBe(primary);expect(db.query("select ?1=1 and ?2=1 closed").get(opened,closed)).toEqual({closed:1});expect(original.control.signal.aborted).toBe(false);}
 }finally{db.close();}
}));
