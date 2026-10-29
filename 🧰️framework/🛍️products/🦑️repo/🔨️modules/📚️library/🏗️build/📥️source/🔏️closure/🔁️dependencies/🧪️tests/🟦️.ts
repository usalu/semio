import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import schema from"../../🧬️schema/🔣️.json";
import fixture from"./🔣️.json";
import{withDependencyReadmissionV1,dependencyReadmissionV1,dependencyReadmissionCodeV1}from"../🟦️.ts";
import{runWithStaleReexecV1}from"../../../../🧷️format-stamps/🔄️reexec/🟨️.mjs";
import{staleImplementationOfV1}from"../../../../🧷️format-stamps/🟨️.mjs";
import{runOwnedCommand}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import{createHash}from"node:crypto";
import{createRequire}from"node:module";
import{mkdtempSync,mkdirSync,writeFileSync,rmSync,readFileSync}from"node:fs";
import{join,dirname}from"node:path";
import{spawn}from"node:child_process";
import ts from"typescript";
import{captureNxAuthorizerSourcesV1,nxAuthorizerCapturedBytesV1,nxAuthorizerCapturedImportsV1}from"../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import{buildSourceClosureStreamLoaderV1}from"../../🟦️.ts";
import{createScriptProcessEnvelope,withScriptProcessEnvelope,createIssuedResourceControlV1,withIssuedResourceOperationV1,requireIssuedScriptInvocationV1,requireIssuedResourceControlV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const terminal=(status:number)=>Object.freeze({name:"OwnedCommandFailure",outcome:{version:1,status,signal:null}});
test("dependency publication crosses only the existing external restart boundary",async()=>{
 const validate=new Ajv({strict:true}).compile(schema.definitions.DependencyReadmissionCase),db=new Database(":memory:");
 try{for(const row of fixture.cases){expect(validate(row),row.name).toBe(true);const oracle=db.query("SELECT CASE WHEN json_extract(?1,'$[0]')=76 THEN 2 ELSE 1 END launches,CASE WHEN json_extract(?1,'$[0]')=0 OR (json_extract(?1,'$[0]')=76 AND json_extract(?1,'$[1]')=0) THEN 1 ELSE 0 END accepted").get(JSON.stringify(row.statuses))as{launches:number;accepted:number};expect(oracle).toEqual({launches:row.launches,accepted:Number(row.accepted)});let calls=0,accepted=false;
  try{const outcome=await runWithStaleReexecV1(()=>withDependencyReadmissionV1(async()=>{const status=row.statuses[calls++]!;if(status)throw terminal(status);}),{outermost:true,environment:{SEMIO_LANGUAGE:"en"},sleep:async()=>{},write:()=>{},launch:async()=>row.statuses[calls++]!});accepted=outcome.status==="completed"||outcome.code===0;}catch(error){expect(error).toEqual(terminal(1));}
  expect(calls,row.name).toBe(row.launches);expect(accepted,row.name).toBe(row.accepted);
 }}finally{db.close();}
});

test("fresh external admission sees a published lazy package while the cold snapshot stays immutable",async()=>{
 const row=fixture.cold;expect(new Ajv({strict:true}).validate(schema.definitions.DependencyColdCaptureCase,row)).toBe(true);
 const generated=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!generated)throw Error("Ticket Generated required");const root=mkdtempSync(join(generated,"cold-")),put=(path:string,text:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);};
 const capture=async()=>{const captured=await captureNxAuthorizerSourcesV1(root,row.entry,new AbortController().signal,{installed:true}),bytes=nxAuthorizerCapturedBytesV1(captured),imports=nxAuthorizerCapturedImportsV1(root,captured),files=[...bytes].sort(([a],[b])=>Buffer.compare(Buffer.from(a),Buffer.from(b))).map(([path,content])=>({path,sha256:createHash("sha256").update(content).digest("hex"),base64:Buffer.from(content).toString("base64"),imports:imports.get(path)!}));return{version:1,root,entry:row.entry,files};};
 const run=async(snapshot:Awaited<ReturnType<typeof capture>>)=>{const text=JSON.stringify(snapshot),digest=createHash("sha256").update(text).digest("hex"),child=spawn(process.execPath,["--eval",buildSourceClosureStreamLoaderV1,"--",digest],{cwd:root,stdio:["ignore","pipe","pipe","pipe"],env:{...process.env,SEMIO_LANGUAGE:"en"}});let out="",err="";child.stdout!.on("data",value=>out+=value);child.stderr!.on("data",value=>err+=value);(child.stdio[3]as NodeJS.WritableStream).end(text);const status=await new Promise<number|null>(accept=>child.on("close",accept));return{out,err,status};};
 try{for(const[path,text]of Object.entries(row.files))put(path,text);const cold=await capture(),before=JSON.stringify(cold);expect(cold.files.find(file=>file.path===row.entry)!.imports[row.specifier]).toBeUndefined();for(const[path,text]of Object.entries(row.installed))put(path,text);expect("RESULT "+createRequire(join(root,row.entry))(row.specifier).answer).toBe(row.output);const old=await run(cold);expect(old.status).not.toBe(0);expect(old.err).toContain(row.specifier);let launches=0;
  const outcome=await runWithStaleReexecV1(()=>withDependencyReadmissionV1(async()=>{launches++;throw terminal(76);}),{outermost:true,environment:{SEMIO_LANGUAGE:"en"},sleep:async()=>{},write:()=>{},launch:async()=>{launches++;const fresh=await capture();expect(fresh.files.find(file=>file.path===row.entry)!.imports[row.specifier]).toBe(row.target);const actual=await run(fresh);expect(actual.status,actual.err).toBe(0);expect(actual.out.trim()).toBe(row.output);return actual.status;}});
  expect(outcome).toEqual({status:"reexecuted",code:0});expect(launches).toBe(2);expect(JSON.stringify(cold)).toBe(before);
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);

test("publication requests require local issuance",()=>{
 const request=dependencyReadmissionV1();expect(dependencyReadmissionCodeV1(request)).toBe(76);expect(dependencyReadmissionCodeV1(new Error(request.message))).toBeUndefined();expect(dependencyReadmissionCodeV1({...request})).toBeUndefined();
});

for(const earlierFailure of[false,true])test("actual Original retires before external dependency restart; prior failure="+earlierFailure,async()=>{
 const source=readFileSync(new URL("../../../../🏛️sealed/📥️execution/🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("execution.ts",source,ts.ScriptTarget.Latest,true),fn=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="executeImplementationV1");if(!fn)throw Error("Authored implementation admission required");
 const envelope=createScriptProcessEnvelope({version:1,owner:"dependency-restart-original",maximumElapsedMilliseconds:5000},{workspaceRoot:process.cwd()},Date.now());
 const body=ts.transpileModule(fn.getText(tree).replace(/^export /u,""),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,args=Object.freeze(["run","@teaching/architecture-course:test-e2e","--configuration=release"]),environment=Object.freeze({SEMIO_LANGUAGE:"de",SEMIO_BUILD_TOOL:"nx",SEMIO_SCRIPT_PROCESS_INVOCATION:JSON.stringify(envelope)}),events:string[]=[],first=Error("real earlier failure");let oldOriginal:any,oldControl:any,calls=0,remaining=0;
 const run=new Function("withDependencyReadmissionV1","executeImplementationOnceV1","executeCapturedBuildSourceV1",body+";return executeImplementationV1;")(withDependencyReadmissionV1,async(workspace:string,entry:string,actualArgs:unknown,original:any,control:any,actualEnvironment:unknown)=>{expect([workspace,entry]).toEqual(["/workspace","entry.ts"]);expect(actualArgs).toBe(args);expect(actualEnvironment).toBe(environment);expect(original).toBe(oldOriginal);expect(control).toBe(oldControl);calls++;return withIssuedResourceOperationV1(control,async()=>{events.push("child");await runOwnedCommand(process.execPath,["-e","process.exit(76)"],process.cwd(),"repository:command",1000,{control,signal:control.signal,env:{...process.env},stdout:"ignore"});});},()=>withIssuedResourceOperationV1(oldControl,async()=>{throw Error("Failed Original cannot be reused");}));
 const runOriginal=()=>withScriptProcessEnvelope(envelope,async original=>{oldOriginal=original;remaining=original.control.remainingMilliseconds()!;oldControl=createIssuedResourceControlV1(original,"dependency:child",0);if(earlierFailure)try{await withIssuedResourceOperationV1(oldControl,async()=>{throw first;});}catch{}try{await run("/workspace","entry.ts",args,original,oldControl,environment);}finally{events.push("retire");}});
 const launch=async(request:any)=>{expect(()=>requireIssuedScriptInvocationV1(oldOriginal)).toThrow();expect(()=>requireIssuedResourceControlV1(oldControl)).toThrow();expect(request.args).toEqual(["door.ts",...args]);expect(request.environment.SEMIO_LANGUAGE).toBe("de");expect(request.environment.SEMIO_BUILD_TOOL).toBe("nx");expect(request.environment.SEMIO_SCRIPT_PROCESS_INVOCATION).toBe(environment.SEMIO_SCRIPT_PROCESS_INVOCATION);events.push("replacement");await withScriptProcessEnvelope(envelope,async fresh=>{expect(fresh).not.toBe(oldOriginal);expect(fresh.control.remainingMilliseconds()!).toBeLessThanOrEqual(remaining);});return 0;};
 if(earlierFailure){await expect(runWithStaleReexecV1(runOriginal,{outermost:true,environment,argv:["bun","door.ts",...args],execArgv:[],launch,sleep:async()=>{},write:()=>{}})).rejects.toBe(first);expect(events).toEqual(["retire"]);expect(calls).toBe(1);}
 else{const outcome=await runWithStaleReexecV1(runOriginal,{outermost:true,environment,argv:["bun","door.ts",...args],execArgv:[],launch,sleep:async()=>{},write:()=>{}});expect(outcome).toEqual({status:"reexecuted",code:0});expect(events).toEqual(["child","retire","replacement"]);expect(calls).toBe(1);}
});
