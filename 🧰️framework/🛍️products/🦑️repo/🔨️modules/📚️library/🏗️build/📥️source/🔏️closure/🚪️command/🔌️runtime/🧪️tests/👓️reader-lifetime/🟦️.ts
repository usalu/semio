import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {mkdtemp,mkdir} from "node:fs/promises";
import {join,relative} from "node:path";
import {fileURLToPath} from "node:url";
import {declareLibraryBuildTestEnvelopeV1} from "../../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import {withBuildTestInvocationV1} from "../../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {createIssuedResourceControlV1,scriptResourceProcessEnvironmentV1} from "../../../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {runOwnedCommand,OwnedCommandFailure} from "../../../../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import {resourceLeasePathV1,resourceQueueTicketV1} from "../../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {captureCompilerBuildSourceClosureV1} from "../../../../🏗️compiler/🟦️.ts";
import {buildSourceClosureTextV1,buildSourceClosureStreamLoaderV1} from "../../../../🟦️.ts";
import nativeSchema from "../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};

declareLibraryBuildTestEnvelopeV1({suite:"build-source-closure",maximumElapsedMilliseconds:300000,scratch:"checkout"});

/** 🧪️ Isolated genuine receivers expose the preparation-owner RED and clean every real charge before assertions. */
test("consumer image and paid charge survive preparation cancellation and obey consumer cancellation",()=>withBuildTestInvocationV1(async original=>{
 const root=process.cwd(),control=createIssuedResourceControlV1(original,"test:native-reader-lifetime",280000),entry=relative(root,fileURLToPath(new URL("./📜️script.ts",import.meta.url))).replaceAll("\\","/"),snapshot=buildSourceClosureTextV1(await captureCompilerBuildSourceClosureV1(root,entry,original,control)),area=await mkdtemp(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"r")),database=new Database(":memory:"),policy=new Ajv({strict:false}).compile({...nativeSchema,$ref:"#/definitions/Policy"});
 const invoke=async(directory:string,phase:string,id:string)=>{
  const chunks:{stdout:Buffer[];stderr:Buffer[]}={stdout:[],stderr:[]};let status:number|null=0,signal:string|null=null;
  try{await runOwnedCommand(process.execPath,["--eval",buildSourceClosureStreamLoaderV1,"--",snapshot.sha256,directory,phase,id],root,"test:reader-lifetime:"+phase,0,{control,signal:control.signal,input:"",env:scriptResourceProcessEnvironmentV1(control,{...process.env,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"}),auxiliaryInput:{version:1,text:snapshot.text},output:{maximumBytes:1048576,maximumLines:8192},outputStyle:"static",outputConsumer:(bytes,stream)=>{chunks[stream].push(Buffer.from(bytes));}});}catch(error){if(!(error instanceof OwnedCommandFailure))throw error;status=error.outcome.status;signal=error.outcome.signal;}
  const stdout=Buffer.concat(chunks.stdout).toString("utf8"),stderr=Buffer.concat(chunks.stderr).toString("utf8"),records=stdout.split("\n").filter(line=>line.startsWith('{"phase":')).map(line=>JSON.parse(line) as Record<string,unknown>);
  return{status,signal,records,diagnostic:JSON.stringify({id,phase,status,signal,stdout:stdout.slice(-16384),stderr:stderr.slice(-32768)})};
 };
 try{
  for(const [index,row] of fixture.cases.entries()){
   const directory=join(area,index.toString(36));resourceLeasePathV1(join(directory,"l"),"native-fixed-image:"+join(directory,"i"),resourceQueueTicketV1({arrivedAtMs:Date.now(),pid:process.pid,owner:Buffer.alloc(16).toString("base64url")}));await mkdir(directory);let exercise:Awaited<ReturnType<typeof invoke>>|undefined,failed=false,failure:unknown;
   try{exercise=await invoke(directory,"exercise",row.id);}catch(error){failed=true;failure=error;}finally{try{const cleanup=await invoke(directory,"cleanup",row.id);expect(cleanup.status,cleanup.diagnostic).toBe(0);expect(cleanup.signal,cleanup.diagnostic).toBeNull();expect(cleanup.records.at(-1),cleanup.diagnostic).toEqual({phase:"cleanup",id:row.id,used:0,retired:true});}catch(error){if(failed)throw new AggregateError([failure,error],"Native reader case and owned cleanup failed");throw error;}}
   if(failed)throw failure;if(!exercise)throw Error("Actual Native reader exercise required");
   const prepared=exercise.records.find(record=>record.phase==="prepared"),result=exercise.records.find(record=>record.phase==="result");expect(prepared,exercise.diagnostic).toBeDefined();expect(policy(prepared!.policy),JSON.stringify(policy.errors)).toBe(true);expect(prepared!.allocation).toMatchObject({owner:"test:native-reader",kind:"compiler-workspace",bytes:fixture.chargeBytes,phase:"committed"});
   if("preparationExpired" in row&&row.preparationExpired){const expired=exercise.records.find(record=>record.phase==="expired");expect(expired,exercise.diagnostic).toMatchObject({phase:"expired",id:row.id,aborted:true,manualAbort:false,message:"Original resource loan deadline exhausted"});expect(Math.ceil(Number(expired?.elapsedMilliseconds))).toBeGreaterThanOrEqual(row.preparationMilliseconds!);}
   const expected=(database.query("select not ? as usable").get(Number(row.consumerCancelled)) as {usable:number}).usable===1;expect(expected).toBe(row.usable);expect(result,exercise.diagnostic).toMatchObject({id:row.id,usable:expected,bytes:fixture.chargeBytes});expect(exercise.signal,exercise.diagnostic).toBeNull();expect(exercise.status,exercise.diagnostic).toBe(row.status);
  }
 }finally{database.close();}
}),290000);
