import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020";
import {readFileSync,mkdirSync,mkdtempSync,writeFileSync,symlinkSync} from "node:fs";
import {join,resolve,dirname} from "node:path";
import {withBuildTestInvocationV1} from "../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {withScriptInvocationSignalV1,acknowledgeScriptInvocationSignalStopV1,type ScriptInvocation} from "@semio-tech/process/routing";
import {createDeveloperProcessV1,createNativeDebuggerSetupHostV1,probeNativeDebuggerV1,type DeveloperProcessIOV1} from "../🏃️process/🟦️.ts";
import {planDeveloperRequest,parseDeveloperArguments,executeDeveloperRequest,readDeveloperRequest,developerDebuggers} from "../🟦️.ts";
const owner=resolve(import.meta.dir,".."),fixture=JSON.parse(readFileSync(join(owner,"🧫️fixtures/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(owner,"🧬️schema/🔣️.json"),"utf8"));
const oracle=new Ajv({strict:true}).compile(schema),generated=process.env.SEMIO_TEST_ARTIFACT_DIR!;
if(!generated)throw Error("Ticket-owned test artifact directory required");
livePath(join(generated,"dt-XXXXXX/app/debug-native-lldb.c"));livePath(join(generated,"dg-XXXXXX/foreign.ts"));
const root=mkdtempSync(join(generated,"dt-")),file=livePath(join(root,"app/space 🧪 file.ts"));mkdirSync(livePath(join(root,"app")));writeFileSync(file,"export const value=1;\n");

test("live protocol obligations agree with closed schema and independent SQLite",async()=>{
 const {Database}=await import("bun:sqlite"),policy=schema.$defs.LiveProcessPolicy.const,validate=new Ajv({strict:true}).compile({$schema:schema.$schema,$defs:schema.$defs,$ref:"#/$defs/LiveProtocolFixture"}),db=new Database(":memory:");
 try{expect(validate(fixture.liveProtocols)).toBe(true);for(const row of fixture.liveProtocols.cases){const oracle=db.query("select case when ?3='gdb' then case when ?1 and ?2 and ?4='native' then 'passed' else 'qualified' end when ?1 and ?2 and ?4=json_extract(?5,'$.'||?3) then 'passed' else 'unmet' end disposition").get(Number(row.available),Number(row.executed),row.profile,row.evidence,JSON.stringify(policy.requiredEvidence))as{disposition:string};expect(oracle.disposition,row.name).toBe(row.disposition);expect(policy.platforms).toContain(row.platform);expect(policy.requiredProfiles.includes(row.profile)).toBe(row.profile!==policy.qualifiedProfile);}for(const row of policy.shellInputs){expect(Buffer.byteLength(row.input)).toBeLessThanOrEqual(policy.maximumInputBytes);const oracle=db.query("select instr(?1,?2)>0 token").get(row.input,policy.shellToken)as{token:number};expect(oracle.token).toBe(1);}for(const row of fixture.liveProtocols.inputCases){const oracle=db.query("select length(cast(?1 as blob)) bytes").get(row.text)as{bytes:number};expect(oracle.bytes<=policy.maximumInputBytes,row.name).toBe(row.accepted);await withBuildTestInvocationV1(async original=>{const create=()=>createDeveloperProcessV1(root,original,readDeveloperRequest({command:"terminal",shell:"system"}),{input:row.text});if(row.accepted)expect(create).not.toThrow();else expect(create).toThrow("developer.process.io-invalid");});}expect(validate({...fixture.liveProtocols,extra:true})).toBe(false);}finally{db.close();}
});

async function liveInvocation(milliseconds:number,body:(original:ScriptInvocation)=>Promise<void>):Promise<void>{
 await withBuildTestInvocationV1(async parent=>{const controller=new AbortController(),timer=setTimeout(()=>controller.abort(Error("Developer live protocol deadline exhausted")),milliseconds);try{await withScriptInvocationSignalV1(parent,controller.signal,async original=>{try{await body(original);}finally{if(controller.signal.aborted)acknowledgeScriptInvocationSignalStopV1(original);}});}finally{clearTimeout(timer);}});
}

async function liveRequest(root:string,request:unknown,original:ScriptInvocation,io:DeveloperProcessIOV1={}):Promise<void>{
 const captured=readDeveloperRequest(request);await executeDeveloperRequest(root,captured,createDeveloperProcessV1(root,original,captured,io),original.control.signal,process.platform,process.env);
}

function livePath(value:string):string{if(value.length>256)throw Error("Developer fixture path exceeds 256 UTF-16 units");return value;}

for(const row of fixture.gitCases)test(`read-only ${JSON.stringify(row.request)}`,async()=>{
 expect(oracle(row.request)).toBe(true);const plan=await planDeveloperRequest(root,row.request,process.platform,{});
 expect(plan.command).toBe("git");expect(plan.args.slice(-row.tail.length)).toEqual(row.tail);expect(plan.args).toContain("--no-optional-locks");expect(plan.args).toContain("core.hooksPath=");expect(plan.args).toContain("diff.external=");
});
for(const request of fixture.refusals)test(`refuses ${JSON.stringify(request)}`,async()=>{await expect(planDeveloperRequest(root,request,process.platform,{})).rejects.toBeDefined();});
test("original cancellation and recorded process port are retained",async()=>{
 const controller=new AbortController(),records:unknown[]=[];const port={run:async(plan:unknown,signal:AbortSignal)=>{expect(signal).toBe(controller.signal);records.push(plan);}};
 await executeDeveloperRequest(root,{command:"git-status"},port,controller.signal,process.platform,{});expect(records.length).toBe(1);controller.abort();
 await expect(executeDeveloperRequest(root,{command:"git-status"},port,controller.signal,process.platform,{})).rejects.toBeDefined();expect(records.length).toBe(1);
});
test("typed CLI refuses duplicates, trailing arbitrary words and unsupported flags",()=>{
 expect(parseDeveloperArguments(["git-diff","--path","space 🧪 file.ts","--revision","HEAD~1","--area","staged"])).toEqual({command:"git-diff",path:"space 🧪 file.ts",revision:"HEAD~1",area:"staged"});
 expect(parseDeveloperArguments(["git-blame","--file","app/space 🧪 file.ts","--line","8","--count","3"])).toEqual({command:"git-blame",file:"app/space 🧪 file.ts",line:8,count:3});
 expect(parseDeveloperArguments(["git-show","--revision","HEAD","--view","patch"])).toEqual({command:"git-show",revision:"HEAD",view:"patch"});
 for(const args of [["git-status","--path","a","--path","b"],["git-status","reset"],["git-status","--hard"],["git-history","--count","0"]])expect(()=>parseDeveloperArguments(args)).toThrow();
});

for(const row of fixture.admissionCases)test(`schema admission ${row.name}`,()=>{
 let accepted=true;try{readDeveloperRequest(row.request);}catch{accepted=false;}
 expect(accepted).toBe(row.accepted);expect(oracle(row.request)).toBe(row.accepted);
});

for(const row of fixture.runtimeCases)test("native Git "+row.name,async()=>{
 const workspace=resolve(owner,"../../../../../.."),output:string[]=[];await liveInvocation(30000,original=>liveRequest(workspace,row.request,original,{output:bytes=>{output.push(Buffer.from(bytes).toString());}}));for(const text of row.contains??[])expect(output.join("")).toContain(text);
});

for(const row of fixture.terminalCases)test(`terminal ${row.platform} ${row.request.shell}`,async()=>{
 expect(oracle(row.request)).toBe(true);const plan=await planDeveloperRequest(root,row.request,row.platform,{});expect(plan.command).toBe(row.command);expect(plan.args).toEqual(row.args);expect(plan.cwd).toBe(resolve(root,row.request.directory??"."));expect(plan.detached).toBe(false);
});
test("system shell uses the production owned process with bounded input output and exit",async()=>{
 const policy=schema.$defs.LiveProcessPolicy.const,row=policy.shellInputs.find((entry:any)=>entry.platform===process.platform);expect(row,"Declared supported shell host required").toBeDefined();const output:string[]=[];await liveInvocation(15000,async original=>{const request=readDeveloperRequest({command:"terminal",shell:"system"}),input=new TextEncoder().encode(row.input),port=createDeveloperProcessV1(root,original,request,{input,output:bytes=>{output.push(Buffer.from(bytes).toString());}});input.fill(120);await executeDeveloperRequest(root,request,port,original.control.signal,process.platform,process.env);});expect(output.join("").split(/\r?\n/u)).toContain(policy.shellToken);
});

test("localized owner declarations validate against the independent registry schema",()=>{
 const registrySchema=JSON.parse(readFileSync(resolve(owner,"../../🎛️dashboard/🧬️schema/🎮️registry/🔣️.json"),"utf8")),project=JSON.parse(readFileSync(join(owner,"📋️project.json"),"utf8"));
 const validate=new Ajv({strict:false}).compile({...registrySchema,$ref:"#/$defs/ProjectManifest"});expect(validate(project),JSON.stringify(validate.errors)).toBe(true);
 for(const branch of schema.oneOf){const command=branch.properties.command.const;expect(project.targets[command]?.options.command).toBe(`repo ./📜️script.ts ${command}`);}
 for(const target of Object.values(project.targets) as any[]){const dashboard=target.metadata.semio.dashboard;expect(Object.keys(dashboard.title).sort()).toEqual(["de","en"]);expect(Object.keys(dashboard.description).sort()).toEqual(["de","en"]);for(const parameter of dashboard.parameters)if(parameter.kind==="choice")for(const choice of parameter.values)expect(choice.args).toEqual([`--${parameter.id}`,choice.id]);}
});


for(const row of fixture.debugCases)test(`debug plan ${row.platform} ${row.request.profile}`,async()=>{
 expect(oracle(row.request)).toBe(true);if(row.rejected){await expect(planDeveloperRequest(root,row.request,row.platform,{})).rejects.toThrow(row.rejected);return;}const plan=await planDeveloperRequest(root,row.request,row.platform,{});
 expect(plan.command).toBe(row.command);expect(plan.args).toEqual(row.arguments.map((word:string)=>word.replace("{program}","./space 🧪 file.ts")));expect(plan.cwd).toBe(join(root,"app"));expect(plan.detached).toBe(false);
 const {default:parse}=await import("string-argv");const args=[plan.command,...plan.args];expect(parse(args.map(word=>'"'+word+'"').join(" "))).toEqual(args);
});
for(const request of fixture.debugRefusals)test(`debug refuses ${JSON.stringify(request)}`,async()=>{await expect(planDeveloperRequest(root,request,process.platform,{})).rejects.toBeDefined();});
test("debug argument form admits a literal bounded list without shell parsing",()=>{
 expect(parseDeveloperArguments(["debug","--profile","node-inspect","--program","app/space 🧪 file.ts","--arguments",'["a b","; $(literal)"]'])).toEqual({command:"debug",profile:"node-inspect",program:"app/space 🧪 file.ts",arguments:["a b","; $(literal)"]});
 for(const value of["plain shell text","{}",'["a",1]'])expect(()=>parseDeveloperArguments(["debug","--profile","node-inspect","--program","app/space 🧪 file.ts","--arguments",value])).toThrow();
});


test("debug availability inventory agrees with schema and independent SQLite discovery",async()=>{
 const {Database}=await import("bun:sqlite"),validate=new Ajv({strict:true}).compile({$schema:schema.$schema,...schema.$defs.DebuggerAvailability}),db=new Database(":memory:");
 try{db.exec("create table installed(program text primary key,path text)");for(const row of fixture.debugAvailabilityCases){db.exec("delete from installed");for(const[program,path]of Object.entries(row.installed))db.query("insert into installed values(?,?)").run(program,path as string);const inventory=developerDebuggers(row.platform,program=>row.installed[program]??null);expect(validate(inventory)).toBe(true);expect(inventory.map(({title,...entry})=>entry)).toEqual(row.expected);for(const entry of inventory){const match=db.query("select path from installed where program=?").get(entry.program)as{path:string}|null;expect(entry.installed).toBe(match!==null);expect(entry.path??null).toBe(match?.path??null);expect(Object.keys(entry.title).sort()).toEqual(["de","en"]);}}}finally{db.close();}
 const installed=developerDebuggers(process.platform,program=>Bun.which(program));console.log("[DEBUG] Debugger availability "+JSON.stringify(installed.map(({title,...entry})=>entry)));
});

test("debug custody preserves immutable arguments, cancellation and physical workspace bounds",async()=>{
 const input={command:"debug",profile:"node-inspect",program:"app/space 🧪 file.ts",arguments:["original"]},captured=readDeveloperRequest(input);input.arguments[0]="changed";expect(captured.command==="debug"&&captured.arguments).toEqual(["original"]);
 const foreign=mkdtempSync(join(generated,"dg-"));writeFileSync(join(foreign,"foreign.ts"),"");symlinkSync(foreign,join(root,"debug-linked"),process.platform==="win32"?"junction":"dir");await expect(planDeveloperRequest(root,{command:"debug",profile:"node-inspect",program:"debug-linked/foreign.ts"},process.platform,{})).rejects.toBeDefined();
 await expect(planDeveloperRequest(root,{command:"debug",profile:"node-inspect",program:"app/space 🧪 file.ts",arguments:Array(5).fill("x".repeat(2048))},process.platform,{})).rejects.toBeDefined();
 const controller=new AbortController(),records:unknown[]=[];await executeDeveloperRequest(root,captured,{run:async(plan,signal)=>{expect(signal).toBe(controller.signal);records.push(plan);}},controller.signal,process.platform,{});expect(records).toHaveLength(1);controller.abort();await expect(executeDeveloperRequest(root,captured,{run:async()=>{throw Error("Cancelled debugger executed");}},controller.signal,process.platform,{})).rejects.toBeDefined();
});


for(const profile of["node-inspect","python-pdb"] as const)test("debug runtime "+profile,async()=>{
 const program=profile==="node-inspect"?"app/debug-target.js":"app/debug-target.py";writeFileSync(livePath(join(root,program)),profile==="node-inspect"?fixture.debugPrograms.node:fixture.debugPrograms.python);
 const arguments_=["value with spaces",'quote"mark',"trailing\\","; $(literal)","🧪"],output:string[]=[];await liveInvocation(15000,async original=>{
  const installed=developerDebuggers(process.platform,program=>Bun.which(program),process.env).find(entry=>entry.profile===profile);expect(installed?.installed,"Required debugger prerequisite unmet: "+profile).toBe(true);
  const request={command:"debug",profile,program,arguments:arguments_};
  if(profile==="node-inspect"){const controller=new AbortController();await withScriptInvocationSignalV1(original,controller.signal,async cancelled=>{const run=liveRequest(root,request,cancelled,{input:"",output:bytes=>{output.push(Buffer.from(bytes).toString());if(output.join("").includes("Break on start"))controller.abort(Error("Observed real Node inspector pause"));}});try{await expect(run).rejects.toThrow("cancelled");expect(output.join("")).toContain("Debugger attached");expect(output.join("")).toContain("Break on start");}finally{if(controller.signal.aborted)acknowledgeScriptInvocationSignalStopV1(cancelled);}});console.log("[DEBUG] Actual production Node inspector attached, paused and cancelled with its child");}
  else{await liveRequest(root,request,original,{input:"p 6*7\nc\nq\n",output:bytes=>{output.push(Buffer.from(bytes).toString());}});expect(output.join("")).toMatch(/(?:^|\n)(?:\(Pdb\) )?42\r?(?:\n|$)/u);expect(output.join("")).not.toContain("UnicodeEncodeError");const encoded=/DEBUGGER_ARGV:([^\r\n]+)/u.exec(output.join(""));expect(encoded).not.toBeNull();expect(JSON.parse(encoded![1]!)).toEqual(arguments_);console.log("[DEBUG] Actual production Python Pdb evaluated42 and preserved Unicode arguments");}
 });
},20000);

const gdbAvailable=Boolean(Bun.which("gdb"))&&(process.platform==="win32"||Boolean(Bun.which("gcc")));
if(!gdbAvailable)console.log("[DEBUG] Qualified optional GDB runtime unmet: "+JSON.stringify({gdb:Boolean(Bun.which("gdb")),compiler:process.platform==="win32"?"frontend-only":Boolean(Bun.which("gcc"))}));
test.skipIf(!gdbAvailable)(process.platform==="win32"?"debug runtime gdb installed frontend":"debug runtime gdb native breakpoint and arguments",async()=>{
 const output:string[]=[];await liveInvocation(40000,async original=>{
  if(process.platform==="win32"){const text=await createNativeDebuggerSetupHostV1(root,original).capture({command:"gdb",args:["--nx","--quiet","-batch","-ex","print 6*7"],cwd:root,detached:false},original.control.signal);output.push(text);expect(output.join("")).toMatch(/\$1 = 42/u);console.log("[DEBUG] Windows GDB frontend42 only; native Unicode breakpoint remains unsupported");return;}
  const program="app/debug-native";writeFileSync(livePath(join(root,"app/debug-native.c")),fixture.debugPrograms.native);livePath(join(root,program));await createNativeDebuggerSetupHostV1(root,original).run({command:"gcc",args:["-g","-O0","app/debug-native.c","-o",program],cwd:root,detached:false},original.control.signal);
  await liveRequest(root,{command:"debug",profile:"gdb",program,arguments:["value with spaces"]},original,{input:"break main\nrun\nprint argc\nprint 6*7\ncontinue\nquit\n",output:bytes=>{output.push(Buffer.from(bytes).toString());}});expect(output.join("")).toMatch(/Breakpoint 1, main/u);expect(output.join("")).toMatch(/\$1 = 2/u);expect(output.join("")).toMatch(/\$2 = 42/u);expect(output.join("")).toContain("DEBUGGER_FIXTURE_OK");console.log("[DEBUG] Actual production GDB hit main, preserved argv, evaluated42 and continued");
 });
},45000);

test("debug runtime lldb native breakpoint and arguments",async()=>{
 const row=fixture.lldbRuntime,output:string[]=[];expect(oracle(row.request)).toBe(true);expect(readDeveloperRequest(row.request)).toEqual(row.request);await liveInvocation(50000,async original=>{
  const host=createNativeDebuggerSetupHostV1(root,original),executable=await probeNativeDebuggerV1(root,original,host);expect(executable,"Required LLDB Python startup42 prerequisite unmet").toBeDefined();const sibling=join(dirname(executable!),process.platform==="win32"?"clang.exe":"clang"),compiler=host.find(sibling)??host.find("clang");expect(compiler,"Required native LLDB fixture compiler prerequisite unmet").toBeTruthy();
  const program=row.request.program+(process.platform==="win32"?".exe":"");writeFileSync(livePath(join(root,"app/debug-native-lldb.c")),fixture.debugPrograms.native);livePath(join(root,program));await host.run({command:compiler!,args:["-g","-O0","app/debug-native-lldb.c","-o",program],cwd:root,detached:false,environment:host.environment},original.control.signal);
  await liveRequest(root,{...row.request,program},original,{input:row.input,output:bytes=>{output.push(Buffer.from(bytes).toString());}});for(const pattern of row.patterns)expect(output.join("")).toMatch(new RegExp(pattern,"u"));console.log("[DEBUG] Actual production LLDB native breakpoint/argv42/continue/exit passed: "+executable);
 });
},55000);

test.skipIf(/[^\u0020-\u007e]/u.test(process.cwd()))("debug Windows GDB keeps an ASCII workspace target and root cwd without relocation",async()=>{
 const workspace=process.cwd(),plan=await planDeveloperRequest(workspace,{command:"debug",profile:"gdb",program:"package.json",directory:"."},"win32",{});expect(plan.args).toEqual(["--nx","--quiet","--args","./package.json"]);expect(plan.cwd).toBe(workspace);expect(plan.detached).toBe(false);
});


for(const row of fixture.nativeSetupCases)test(`native debugger setup plan ${row.name}`,async()=>{
 const {planNativeDebuggerProvisionV1}=await import("../🏃️process/🟦️.ts"),{Database}=await import("bun:sqlite"),steps=planNativeDebuggerProvisionV1(row,root),db=new Database(":memory:");
 try{const policy=schema.$defs.NativeDebuggerSetupPolicy.const,selected=policy.installers.find((entry:any)=>entry.platform===row.platform&&entry.manager===row.manager);expect(selected).toBeDefined();db.exec("create table steps(id text,command text,args text)");for(const step of steps)db.query("insert into steps values(?,?,?)").run(step.id,step.command,JSON.stringify(step.args));expect(steps.map(({cwd,detached,...step}:any)=>step)).toEqual(row.expected);const supplied=db.query("select count(*) as n from steps where command=? or command='sudo'").get(row.manager)as{n:number};expect(supplied.n).toBe(steps.length);for(const step of steps)expect(step.cwd).toBe(root);const actual=db.query("select value from steps,json_each(steps.args) where value in ('lldb','python3-lldb','Python.Python.3.11','LLVM.LLVM') order by value").all();const expected=db.query("select arg.value from json_each(?,'$.steps') as step,json_each(step.value,'$.args') as arg where arg.value in ('lldb','python3-lldb','Python.Python.3.11','LLVM.LLVM') order by arg.value").all(JSON.stringify(selected));expect(actual).toEqual(expected);for(let index=0;index<selected.steps.length;index++){const step=selected.steps[index],offset=row.platform==="linux"&&row.privilege==="sudo"?1:0,windows=policy.windows,python=(row.environment.LOCALAPPDATA??"")+"\\"+windows.pythonDirectory.replaceAll("/","\\"),llvm=(row.environment.ProgramFiles??"")+"\\"+windows.llvmDirectory.replaceAll("/","\\"),args=db.query("select replace(replace(value,'{pythonRoot}',?),'{llvmRoot}',?) as word from json_each(?) order by cast(key as integer)").all(python,llvm,JSON.stringify(step.args))as{word:string}[];const variables=db.query("select key || '=' || value as word from json_each(?) order by key").all(JSON.stringify(step.environment??{}))as{word:string}[];expect(steps[index+offset].args).toEqual([...(offset?["-n",...(variables.length?[policy.privilegeEnvironmentCommand,...variables.map(row=>row.word)]:[]),step.command]:[]),...args.map(row=>row.word)]);}}finally{db.close();}
});
for(const row of fixture.nativeSetupRefusals)test(`native debugger setup refuses ${JSON.stringify(row)}`,async()=>{const {planNativeDebuggerProvisionV1}=await import("../🏃️process/🟦️.ts");expect(()=>planNativeDebuggerProvisionV1(row,root)).toThrow();});

test("native setup actual dispatch, lifecycle, cancellation and production no-install frontend",async()=>{
 const {SetupDebuggerScript,setupNativeDebugger,createNativeDebuggerSetupHostV1}=await import("../🏃️process/🟦️.ts"),{receiveScriptProcessInvocation,withScriptInvocationSignalV1,acknowledgeScriptInvocationSignalStopV1}=await import("@semio-tech/process/routing");
 await receiveScriptProcessInvocation(process.env,async original=>{
  const records:any[]=[],host:any={platform:"linux",manager:"apt-get",privilege:"sudo",environment:{},find:(program:string)=>program==="lldb"||program==="/usr/bin/lldb"?"/usr/bin/lldb":null,run:async(plan:any,signal:AbortSignal)=>{expect(signal).toBe(original.control.signal);records.push(plan);},capture:async(plan:any,signal:AbortSignal)=>{expect(signal).toBe(original.control.signal);records.push(plan);return "(lldb) script print(6*7)\n42\n";}};
  await new SetupDebuggerScript(owner,root,original,host).run([]);expect(records).toHaveLength(1);expect(records[0].args).toEqual(schema.$defs.NativeDebuggerSetupPolicy.const.probeArguments);
  records.length=0;host.capture=async()=>{throw Error("frontend unavailable");};host.run=async(plan:any)=>{records.push(plan);throw Error("sudo requires OS authentication");};await expect(new SetupDebuggerScript(owner,root,original,host).run([])).rejects.toThrow();expect(records).toHaveLength(1);expect(records[0].command).toBe("sudo");expect(records[0].args).toEqual(["-n","true"]);
  records.length=0;let ready=false;host.capture=async()=>ready?"42\n":Promise.reject(Error("not installed"));host.run=async(plan:any,signal:AbortSignal)=>{expect(signal).toBe(original.control.signal);records.push(plan);if(plan.command==="sudo"&&plan.args.includes("install"))ready=true;};await new SetupDebuggerScript(owner,root,original,host).run([]);expect(records.map(row=>row.args)).toEqual(fixture.nativeSetupCases.find((row:any)=>row.name==="linux-sudo").expected.map((row:any)=>row.args));
  for(const row of fixture.nativeSetupLifecycleCases){
   const selection=fixture.nativeSetupCases.find((entry:any)=>entry.name===row.host);let ready=false;const installs:string[]=[],host={...selection,find:(program:string)=>/lldb(?:\.exe)?$/u.test(program)?program:null,capture:async(_plan:any,signal:AbortSignal)=>{expect(signal).toBe(original.control.signal);if(!ready)throw Error("scripting runtime unavailable");return "42\n";},run:async(plan:any,signal:AbortSignal)=>{expect(signal).toBe(original.control.signal);const id=plan.args.includes("Python.Python.3.11")?"python":"lldb";installs.push(id);if(id===row.readyAfter)ready=true;}};const result=await setupNativeDebugger(root,original,host);expect(result.installed,row.name).toBe(true);expect(installs,row.name).toEqual(row.expectedInstallIds);
  }
  const controller=new AbortController();await withScriptInvocationSignalV1(original,controller.signal,async cancelled=>{let effects=0;const host:any={platform:"linux",manager:"apt-get",privilege:"root",environment:{},find:(program:string)=>program==="lldb"||program==="/usr/bin/lldb"?"/usr/bin/lldb":null,capture:async()=>{controller.abort(Error("fixture native setup stop"));throw Error("probe interrupted");},run:async()=>{effects++;throw Error("Cancelled setup must never install");}};await expect(setupNativeDebugger(root,cancelled,host)).rejects.toThrow("fixture native setup stop");expect(effects).toBe(0);acknowledgeScriptInvocationSignalStopV1(cancelled);});
  const actualHost=createNativeDebuggerSetupHostV1(root,original),actual=await probeNativeDebuggerV1(root,original,actualHost);expect(actual,"Required real no-install LLDB Python startup42 prerequisite unmet").toBeTruthy();console.log("[DEBUG] Production owned LLDB Python startup42 verified without package installation: "+actual);
 });
},60_000);

test("managed Windows LLDB preserves the default Python and supplies its private scripting runtime to children",async()=>{
 const {nativeDebuggerLocations}=await import("../🟦️.ts"),row=fixture.nativeSetupCases.find((row:any)=>row.platform==="win32"),environment={...row.environment,Path:"C:\\Python314"},before=JSON.stringify(environment),locations=nativeDebuggerLocations("win32",environment);expect(locations.candidates).toEqual(["C:\\Program Files\\semio\\LLVM\\22.1.8\\bin\\lldb.exe"]);expect(locations.environment.PATH).toStartWith("C:\\Program Files\\semio\\LLVM\\22.1.8\\bin;C:\\Users\\Dev\\AppData\\Local\\semio\\Python\\3.11.9;");expect(JSON.stringify(environment)).toBe(before);expect(Object.keys(locations.environment).filter(key=>key.toLowerCase()==="path")).toEqual(["PATH"]);
});



/** 🛑️ Native workspace editing has no external IDE launch request or process capability. */
test("external source editor delegation is absent and rejected before all effects",async()=>{
 const {Database}=await import("bun:sqlite"),db=new Database(":memory:"),project=JSON.parse(readFileSync(join(owner,"📋️project.json"),"utf8")),script=readFileSync(join(owner,"📜️script.ts"),"utf8");try{for(const row of fixture.retiredSourceCases){let accepted=true;try{readDeveloperRequest(row.request);}catch{accepted=false;}expect(accepted).toBe(row.accepted);expect(oracle(row.request)).toBe(row.accepted);const declared=db.query("select count(*) n from json_each(?1,'$.oneOf') where json_extract(value,'$.properties.command.const')=?2").get(JSON.stringify(schema),row.request.command)as{n:number};expect(declared.n).toBe(0);let effects=0;const controller=new AbortController();await expect(executeDeveloperRequest(root,row.request,{run:async()=>{effects++;}},controller.signal,row.platform,{SEMIO_SOURCE_OPENER:'["must-never-run"]'})).rejects.toThrow("developer.request.invalid");expect(effects).toBe(0);expect(()=>parseDeveloperArguments(["open-source","--file","app/space 🧪 file.ts"])).toThrow();}expect(project.targets["open-source"]).toBeUndefined();expect(script).not.toContain('"open-source"');expect(script).not.toContain("detachedDesktop");}finally{db.close();}
});
