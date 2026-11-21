import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {join,dirname,relative,resolve,sep} from "node:path";
import ts from "typescript";
import {cargoRecipeInputsV1} from "../../../../../🦀️cargo-workspace/🔒️lock/📐️closure/🟦️.ts";
import {ConcurrentEditErrorV1,concurrentEditV1} from "../../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import corpus from "./🔣️.json" with {type:"json"};

test("actual Cargo capture checks the retained recipe after capacity and lock reconciliation",async()=>{
 const tree=ts.createSourceFile("native.ts",readFileSync(new URL("../../../🟦️.ts",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true);
 const definition=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="captureCargoArtifacts")!;
 const code=ts.transpileModule(definition.getText(tree),{compilerOptions:{target:ts.ScriptTarget.ESNext}}).outputText;
 const root=resolve("fixture"),lock=join(root,"Cargo.lock"),manifest=join(root,"Cargo.toml");
 const db=new Database(":memory:");
 try{
  db.exec("create table dependency(parent text, child text)");
  for(const [parent,child] of corpus.dependencies)db.query("insert into dependency values(?,?)").run(parent!,child!);
  const oracle=db.query<{outcome:string},[string,string|null]>("with recursive selected(package) as (select 'descriptor' union select child from dependency join selected on parent=package) select case when ?1='cancel' then 'cancel' when ?2 in selected then 'retry' else 'spawn' end as outcome");
  for(const row of corpus.cases){
   expect(oracle.get(row.mutation,row.package)?.outcome,row.id).toBe(row.outcome);
   const events:string[]=[],abort=new AbortController(),control={signal:abort.signal},capture=join(root,"capture"),spawned=Error("fake spawn"),cancelled=Error("stop");
   let changed:string|null=null;
   const witness=await cargoRecipeInputsV1([lock],["descriptor"],{async digest(){throw Error("expected projection");},async recheck(){}},undefined,{async read(){
    const entry=(name:string)=>'[[package]]\nname = "'+name+'"\nversion = "'+(name===changed?"2":"1")+'"\n';
    const bytes=new TextEncoder().encode('version = 4\n\n'+entry("descriptor")+'dependencies = [\n "used",\n]\n\n'+entry("used")+'\n'+entry("unrelated"));
    return {bytes,before:"stable",after:"stable"};
   }});
   const ports={resolve,join,dirname,relative,sep,ConcurrentEditErrorV1,
    requireNativeStoragePathV1(){},mkdirSync(){},mkdtempSync:()=>capture,
    rmSync(){events.push("cleanup");},phaseLedgerV1:()=>({begin:()=>()=>{}}),
    measureV1:async(_phase:string,body:()=>Promise<unknown>)=>body(),
    compileUnitsForHostV1:async()=>({request:{}}),compilerAdmissionBudgetV1(){},
    async withCompilerCapacityScopeV1(_input:unknown,body:(facts:{parallelism:number},issued:typeof control)=>Promise<unknown>){
     if(row.mutation==="capacity")changed=row.package;
     events.push("charged");
     try{return await body({parallelism:1},control);}catch(error){events.push("failed inside capacity");throw error;}finally{events.push("released");}
    },
    async ensureCargoLockCurrentV1(){
     events.push("lock");
     if(row.mutation==="lock")changed=row.package;
     if(row.mutation==="cancel")abort.abort(cancelled);
    },
    checkResourceLeaseControl:()=>control.signal.throwIfAborted(),
    cargoTimingsArgumentsV1:()=>[],COMPILER_RESOURCE_DIRECTORY_V1:"resource",
    spawn(){events.push("spawn");throw spawned;},
   };
   const run=new Function(...Object.keys(ports),code+";return captureCargoArtifacts;")(...Object.values(ports));
   const recheckRecipe=async()=>{events.push("recipe");await witness.recheck();};
   const expected=row.outcome==="retry"?concurrentEditV1("input",lock,undefined,"build"):row.outcome==="cancel"?cancelled:spawned;
   let failure:unknown;
   try{
    await run(manifest,[],{cwd:root,captureDirectory:capture,buildDirectory:root},{control,storage:{backing:{root}},output:"out",environment:{}},root,async(body:()=>Promise<unknown>)=>body(),0,()=>[],async()=>{throw Error("extra full plan recheck");},async()=>{},recheckRecipe);
   }catch(error){failure=error;}
   if(expected instanceof ConcurrentEditErrorV1)expect(failure,row.id).toMatchObject({reason:expected.reason,path:expected.path,rank:expected.rank});
   else expect(failure,row.id).toBe(expected);
   expect(events,row.id).toEqual(row.outcome==="retry"?["charged","lock","recipe","released","cleanup"]:
    row.outcome==="cancel"?["charged","lock","failed inside capacity","released","cleanup"]:
    ["charged","lock","recipe","spawn","failed inside capacity","released","cleanup"]);
  }
 }finally{db.close();}
});
