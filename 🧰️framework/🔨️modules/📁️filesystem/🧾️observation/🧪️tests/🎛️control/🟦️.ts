import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import{sha256}from"@noble/hashes/sha2.js";
import{mkdtemp,mkdir,writeFile,rm}from"node:fs/promises";
import{join}from"node:path";
import{observePhysicalFileV1,readPhysicalFileV1,FileObservationErrorV1,type FileObservationProgressV1}from"../../🟦️.ts";
import{runFileObservationChecksV1}from"../🟦️.ts";
import corpus from"../🧫️fixtures/🔣️.json";
import schema from"../../🧬️schema/🔣️.json";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1,scriptInvocationEnvironment,withIssuedResourceOperationV1}from"@semio-tech/process/routing/invocation/process";
import{captureOwnedCommandResultV1}from"@semio-tech/process/owned-execution";

const selected=process.env.SEMIO_FILE_OBSERVATION_CASE;
if(selected===undefined){
 test("neutral physical file existing portable corpus",async()=>{expect(await runFileObservationChecksV1()).toBe(corpus.cases.length+9);});
 test("neutral physical file preserves caller primary and captured control",async()=>{
  const ajv=new Ajv({strict:true});ajv.addSchema(schema);expect(ajv.getSchema(schema.$id+"#/definitions/controlCases")!(corpus.controlCases)).toBe(true);
  const db=new Database(":memory:");db.exec("create table cases(id text primary key, fault text, mutation text)");for(const row of corpus.controlCases)db.query("insert into cases values(?,?,?)").run(row.id,row.fault,row.mutation);
  const observations:{id:string;operation:string;actual:{kind:string;preserved:boolean};oracle:string;expected:string;status:number|null;output:string}[]=[];
  try{await receiveScriptProcessInvocation(process.env,async original=>{const control=createIssuedResourceControlV1(original,"test:file-observation-controls",0);for(const row of corpus.controlCases)for(const operation of["read","observe"]){
   const result=await captureOwnedCommandResultV1(process.execPath,["test",import.meta.path,"--test-name-pattern=^file observation isolated"],process.cwd(),{control,env:scriptInvocationEnvironment(original,{...process.env,SEMIO_FILE_OBSERVATION_CASE:row.id,SEMIO_FILE_OBSERVATION_OPERATION:operation,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0",NODE_DISABLE_COMPILE_CACHE:"1"}),output:{maximumBytes:32768,maximumLines:256}});
   const lines=result.stdout.split(/\r?\n/u).filter(line=>line.startsWith("file-observation-result:"));expect(lines,result.stderr+result.stdout).toHaveLength(1);const actual=JSON.parse(lines[0]!.slice(24))as{kind:string;preserved:boolean};const oracle=db.query("select case when mutation='raise-bytes' then 'byte-capacity' when mutation='raise-work' then 'work-capacity' when fault is null then 'value' else fault end as kind from cases where id=?").get(row.id)as{kind:string};observations.push({id:row.id,operation,actual,oracle:oracle.kind,expected:row.expected,status:result.status,output:result.stderr+result.stdout});
  }});for(const row of observations){expect(row.oracle).toBe(row.expected);expect(row.actual.kind,row.id+":"+row.operation).toBe(row.oracle);expect(row.actual.preserved,row.id+":"+row.operation).toBe(true);expect(row.status,row.output).toBe(0);}}finally{db.close();}
 },120000);
}else test("file observation isolated received caller",async()=>{
 const row=corpus.controlCases.find(row=>row.id===selected),operation=process.env.SEMIO_FILE_OBSERVATION_OPERATION;if(!row||(operation!=="read"&&operation!=="observe"))throw Error("Closed file observation case required");
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Ticket DATA output directory required");await mkdir(output,{recursive:true});const directory=await mkdtemp(join(output,"file-control-")),path=join(directory,"input.bin"),bytes=new TextEncoder().encode("abc"),marker=Error("file observation caller primary"),poison=Error("mutated callback must not run");await writeFile(path,bytes);
 let stage:FileObservationProgressV1["phase"]|null=null,failed=false,failure:unknown,kind="value",preserved=false,claim:{sha256:string;byteLength:number}|undefined;
 try{try{await receiveScriptProcessInvocation(process.env,async original=>{const actual=createIssuedResourceControlV1(original,"test:file-observation-isolated",0),fault=(callback:string)=>{if(row.callback===callback&&row.phase===stage){if(row.fault==="error")throw marker;if(row.fault==="undefined")throw undefined;}};
  const control={maxBytes:row.mutation==="raise-bytes"?2:3,maxWork:row.mutation==="raise-work"?7:8,chunkBytes:1,cancelled(){expect(this).toBe(control);fault("cancelled");return actual.signal.aborted||(row.callback==="cancelled"&&row.fault==="cancelled"&&row.phase===stage);},remainingMs(){expect(this).toBe(control);fault("remainingMs");const remaining=actual.remainingMilliseconds();if(remaining===null)throw Error("Finite received deadline required");return row.callback==="remainingMs"&&row.fault==="deadline"&&row.phase===stage?0:remaining;},onProgress(progress:FileObservationProgressV1){expect(this).toBe(control);stage=progress.phase;fault("onProgress");}};
  await withIssuedResourceOperationV1(actual,async()=>{const pending=operation==="read"?readPhysicalFileV1(path,control):observePhysicalFileV1(path,control);if(row.mutation==="raise-bytes"||row.mutation==="raise-work")Object.assign(control,{maxBytes:3,maxWork:8});if(row.mutation==="limits")Object.assign(control,{maxBytes:0,maxWork:1,chunkBytes:1048576});if(row.mutation==="methods")Object.assign(control,{cancelled:()=>{throw poison;},remainingMs:()=>{throw poison;},onProgress:()=>{throw poison;}});const result=await pending;claim="claim"in result?result.claim:result;if("bytes"in result){expect(result.bytes).toEqual(bytes);result.bytes.fill(0);}});
 });}catch(error){failed=true;failure=error;}
 if(failed){kind=failure===marker?"error":failure===undefined?"undefined":failure instanceof FileObservationErrorV1?failure.code:"other";preserved=kind===row.expected&&(row.expected==="error"?failure===marker:row.expected==="undefined"?failure===undefined:true);}else{const noble=Buffer.from(sha256(bytes)).toString("hex"),web=Buffer.from(await crypto.subtle.digest("SHA-256",bytes)).toString("hex");expect(noble).toBe(web);preserved=claim?.sha256===noble&&claim.byteLength===bytes.length&&stage==="complete";}
 console.log("file-observation-result:"+JSON.stringify({kind,preserved}));expect(kind).toBe(row.expected);expect(preserved).toBe(true);
 }finally{await rm(directory,{recursive:true,force:true});}
});
