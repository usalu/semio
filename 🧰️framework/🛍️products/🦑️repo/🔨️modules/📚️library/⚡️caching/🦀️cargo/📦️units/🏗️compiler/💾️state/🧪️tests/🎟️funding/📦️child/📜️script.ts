/** 📦️ Child: child-process script that receives the issued script-process invocation and runs the case its arguments select. */
import{Database}from"bun:sqlite";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1}from"../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import*as state from"../../../🟦️.ts";
import{compileRustUnitV1}from"../../../../🟦️.ts";
import{planRustUnitsV1}from"../../../../../🟦️.ts";
import fixtures from"../../../../../🧪️tests/🧫️fixtures/🔣️.json";
import corpus from"../🔣️.json";
import type{RepositoryCompilerStoreV1}from"../../../../../../../💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts";
const row=corpus.cases.find(value=>value.name===process.argv[2]);if(!row)throw Error("Closed funded refusal case required");const db=new Database(":memory:");db.exec("create table effects(kind text)");let failure:string|null=null;
try{await receiveScriptProcessInvocation(process.env,async original=>{const control=createIssuedResourceControlV1(original,"funding:copied-owner",0),effect=(kind:string):never=>{db.query("insert into effects values(?)").run(kind);throw Error("Unexpected funding effect: "+kind);},storage=new Proxy({},{get:()=>effect("Native property")})as never,unit={...planRustUnitsV1(fixtures.units,"cache/units")[0]!.unit};Object.defineProperty(unit,"inputs",{get:()=>effect("Source input admission"),enumerable:true});const plan={...planRustUnitsV1(fixtures.units,"cache/units")[0]!,unit},loan=Object.freeze({})as state.RustCompilerStateLoanV1,workspace={directory:"/unused/b/p/units/x",temporary:"/unused/b/p/units/x.tmp",leaseDirectory:"/unused/leases"},policy={version:1 as const,root:"/unused",leaseDirectory:workspace.leaseDirectory,capacityBytes:1073741824,maximumWorkspaceBytes:536870912,maximumArtifactBytes:134217728};
 if(row.name==="state-store")return state.withRustCompilerStateV1({workspace:process.cwd(),invocation:original,store:storage as RepositoryCompilerStoreV1,control,plan},async()=>{effect("State callback");});
 if(row.name==="state-operation")return state.withRustCompilerStateOperationV1(loan,plan,storage,control,workspace,async()=>{effect("State operation");});
 if(row.name==="state-funding")return state.requireRustCompilerStateFundingV1(loan,plan,storage,control,workspace,policy);
 const mode=row.name==="compile-store"?"release":"prototype",funding=row.name==="compile-state"?{kind:"compiler-state" as const,loan}:row.name==="compile-absent"?undefined:{kind:"compiler-store" as const,control};unit.mode=mode;return compileRustUnitV1({root:process.cwd(),plan,control,policy,compiler:{compile:async()=>effect("Compiler dispatch")},fundedWorkspace:{storage,directory:workspace.directory,temporary:workspace.temporary,funding}}as never);
});}catch(error){failure=error instanceof Error?error.message:String(error);}
const effects=(db.query("select count(*) n from effects").get()as{n:number}).n;db.close();console.log(JSON.stringify({failure,effects}));process.exitCode=failure===null?0:1;
