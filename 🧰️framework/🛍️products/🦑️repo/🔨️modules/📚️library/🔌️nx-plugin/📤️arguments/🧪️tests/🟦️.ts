/** 📤️ Pins raw Nx owner argv to real Node process arguments across shell metacharacters. */
import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import {createRequire} from "node:module";
import {dirname,resolve} from "node:path";
const fixture=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
const schema=JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json",import.meta.url),"utf8"));
test("owner arguments are admitted then transported literally and consumed exactly once",async()=>{
 const admit=new Ajv({strict:true}).compile(schema);
 for(const row of fixture.cases)expect(admit({version:1,arguments:row.arguments})).toBe(true);
 const api=await import(new URL("../🟨️.mjs",import.meta.url).href);
 for(const row of fixture.cases){
  const encoded=api.encodeOwnerArgumentsV1(row.arguments),env={KEEP:"original",SEMIO_OWNER_ARGUMENTS:encoded};
  const consumed=api.consumeOwnerArgumentsV1(env);
  expect(consumed.arguments).toEqual(row.arguments);expect(consumed.environment).toEqual({KEEP:"original"});expect(env.SEMIO_OWNER_ARGUMENTS).toBe(encoded);
  expect(api.consumeOwnerArgumentsV1(consumed.environment).arguments).toEqual([]);
  const oracle=Bun.spawnSync(["node","-e","process.stdout.write(JSON.stringify(process.argv.slice(1)))","--",...consumed.arguments]);
  expect(oracle.exitCode).toBe(0);expect(JSON.parse(new TextDecoder().decode(oracle.stdout))).toEqual(row.arguments);
  console.log(`[DEBUG] Nx owner argv ${row.id} actualNode=true arguments=${row.arguments.length} consumedOnce=true`);
 }
 for(const row of fixture.invalid){expect(admit(row)).toBe(false);expect(()=>api.consumeOwnerArgumentsV1({SEMIO_OWNER_ARGUMENTS:JSON.stringify(row)})).toThrow();}
});
test("Nx delegates the unchanged static owner command while literal argv stays outside shell interpolation",async()=>{
 const api=await import(new URL("../🟨️.mjs",import.meta.url).href);
 for(const row of fixture.cases){
  let observed:any;
  const native=fixture.nativeOriginal,root=process.cwd(),require=createRequire(import.meta.url),nxRoot=dirname(require.resolve("nx/package.json"));
  const {createTaskGraph}=require(resolve(nxRoot,"dist/src/tasks-runner/create-task-graph.js"));
  const options={command:'bun "./📜️script.ts" native owner-command --manifest "owner/Cargo.toml" --cwd "owner" -- bun "./📜️script.ts" test',cwd:".",forwardAllArgs:true,nativeOwnerCommand:structuredClone(native.command),env:{KEEP:"original"} as Record<string,string>,__unparsed__:row.arguments,E:"parsed-duplicate"};
  const projects={"literal-owner":{name:"literal-owner",root:".",targets:{probe:{executor:"@semio-tech/repo-lib:owner-command",options:{command:options.command,cwd:options.cwd,forwardAllArgs:true,nativeOwnerCommand:structuredClone(native.command)}}}}};
  const projectGraph={nodes:{"literal-owner":{name:"literal-owner",type:"lib",data:projects["literal-owner"]}},dependencies:{"literal-owner":[]}};
  const taskGraph=createTaskGraph(projectGraph,{},["literal-owner"],["probe"],undefined,{__overrides_unparsed__:row.arguments,E:"parsed-duplicate"},false);
  const capabilities={workspaceRoot:root,arguments:["nx","run",native.selected,"--",...row.arguments],selection:["nx","run",native.selected,"--",...row.arguments],native:structuredClone(native.resources)};
  const original={version:1,policy:{version:1,owner:native.owner,maximumElapsedMilliseconds:native.maximumElapsedMilliseconds},deadlineEpochMilliseconds:Date.now()+native.maximumElapsedMilliseconds,capabilities};
  const callerSchema=JSON.parse(readFileSync(new URL("../🧬️schema/📥️native-caller.json",import.meta.url),"utf8"));
  const nativeSchema=JSON.parse(readFileSync(new URL("../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🧬️schema/🔣️.json",import.meta.url),"utf8"));
  expect(new Ajv({strict:true}).compile(callerSchema)(capabilities)).toBe(true);
  options.env={...options.env,SEMIO_SCRIPT_PROCESS_INVOCATION:JSON.stringify(original),SEMIO_SCRIPT_CAPABILITIES:JSON.stringify(capabilities),CARGO_TARGET_DIR:native.resources.artifactDirectory,CARGO_NET_OFFLINE:String(native.resources.network.offline)};
  const result=await api.executeOwnerArgumentsV1(options,{root,projectName:"literal-owner",targetName:"probe",taskGraph,projectsConfigurations:{projects}},async(plan:any)=>{observed=plan;return {success:true};});
  const issued=JSON.parse(observed.env.SEMIO_SCRIPT_PROCESS_INVOCATION);
  expect(new Ajv({strict:true}).compile(nativeSchema)(issued.capabilities)).toBe(true);
  expect(issued.policy).toEqual(original.policy);expect(issued.deadlineEpochMilliseconds).toBe(original.deadlineEpochMilliseconds);
  expect(issued.capabilities.command).toEqual({...native.command,arguments:[...native.command.arguments,...row.arguments]});
  expect(result.success).toBe(true);expect(observed.command).toBe(options.command);expect(observed.forwardAllArgs).toBe(false);expect(observed.__unparsed__).toEqual([]);expect(observed.args).toBeUndefined();
  expect(api.consumeOwnerArgumentsV1(observed.env).arguments).toEqual(row.arguments);
 }
 expect(()=>api.encodeOwnerArgumentsV1(["\u0000"])).toThrow();
 await expect(api.executeOwnerArgumentsV1({command:"bun ./📜️script.ts test"},{},async()=>({success:true}))).rejects.toThrow("original forwarded arguments");
 console.log("[DEBUG] Nx owner executor staticCommand=preserved shellArgumentInterpolation=false preparationAndChildRoute=preserved");
});

test("the current Nx process executor returns literal envelope values without executing their shell syntax",async()=>{
 const api=await import(new URL("../🟨️.mjs",import.meta.url).href);
 const context={root:process.cwd(),projectName:"literal-owner",targetName:"probe",projectsConfigurations:{projects:{"literal-owner":{root:"."}}}};
 for(const row of fixture.cases){
  const result=await api.default({command:'node -e "console.log(JSON.stringify(JSON.parse(process.env.SEMIO_OWNER_ARGUMENTS).arguments))"',cwd:".",usePty:false,streamOutput:false,__unparsed__:row.arguments},context);
  expect(result.success,row.id).toBe(true);const lines=result.terminalOutput.trim().split(/\r?\n/);expect(JSON.parse(lines.at(-1)),row.id).toEqual(row.arguments);
 }
 console.log("[DEBUG] Nx delegated lifecycle actualNode=true filtersAndQuotes=literal platformIndependentEnvelope=true");
});

test("the actual Nx schema admits every original CLI argument into the owner boundary",async()=>{
 const {createRequire}=await import("node:module"),{dirname,resolve}=await import("node:path"),require=createRequire(import.meta.url),root=dirname(require.resolve("nx/package.json"));
 const {combineOptionsForExecutor}=require(resolve(root,"dist/src/utils/params.js"));
 const executorSchema=JSON.parse(readFileSync(new URL("../🧬️schema/🔣️executor.json",import.meta.url),"utf8"));
 for(const row of fixture.cases){
  const admitted=combineOptionsForExecutor({__overrides_unparsed__:row.arguments},undefined,{options:{command:"bun ./📜️script.ts test",forwardAllArgs:true}},executorSchema,"fixture",".",false);
  expect(admitted.__unparsed__,row.id).toEqual(row.arguments);
 }
 console.log("[DEBUG] actual Nx schema CLI admission originals=preserved filterAndPositionalScope=exact");
});

test("the canonical Nx bootstrap preserves the complete literal filter and positional scope",async()=>{
 const arguments_=fixture.cases.flatMap((row:any)=>row.arguments);
 const bootstrap=fileURLToPath(new URL("../../../⚡️caching/🚀️bootstrap/📜️script.ts",import.meta.url));
 const child=Bun.spawn([process.execPath,bootstrap,"nx","run","@semio-tech/repo-lib:probe-nx-owner-arguments","--excludeTaskDependencies","--skip-nx-cache","--",...arguments_],{cwd:process.cwd(),stdout:"pipe",stderr:"pipe"});
 const [stdout,stderr,status]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
 expect(status,stderr+stdout).toBe(0);
 const marker="[DEBUG] ownerArgumentReceipt=",line=stdout.split(/\r?\n/).find(line=>line.startsWith(marker));
 expect(line).toBeDefined();const receipt=JSON.parse(line!.slice(marker.length));expect(receipt.arguments).toEqual(arguments_);expect(receipt.carrierPresent).toBe(false);
 console.log(`[DEBUG] actual Nx CLI -> declared owner -> child arguments=${arguments_.length} exact=true envelopeConsumed=true nativePreparation=unchanged`);
},180000);

test("bun nx retains both original Nextest filter and positional selections",async()=>{
 const arguments_=fixture.cases.filter((row:any)=>row.id.startsWith("nextest-")).flatMap((row:any)=>row.arguments);
 const child=Bun.spawn([process.execPath,"nx","run","@semio-tech/repo-lib:probe-nx-owner-arguments","--excludeTaskDependencies","--skip-nx-cache","--",...arguments_],{cwd:process.cwd(),stdout:"pipe",stderr:"pipe"});
 const [stdout,stderr,status]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
 expect(status,stderr+stdout).toBe(0);
 const marker="[DEBUG] ownerArgumentReceipt=",line=stdout.split(/\r?\n/).find(line=>line.startsWith(marker));
 expect(line).toBeDefined();const receipt=JSON.parse(line!.slice(marker.length));expect(receipt.arguments).toEqual(arguments_);expect(receipt.carrierPresent).toBe(false);
 console.log(`[DEBUG] bun nx -> declared owner -> child Nextest arguments=${arguments_.length} exact=true selections=unchanged envelopeConsumed=true`);
},180000);
