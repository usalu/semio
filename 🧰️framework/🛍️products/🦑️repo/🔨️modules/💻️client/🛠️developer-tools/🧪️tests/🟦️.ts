import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020";
import {readFileSync,mkdirSync,mkdtempSync,writeFileSync,symlinkSync} from "node:fs";
import {join,resolve} from "node:path";
import {spawnSync} from "node:child_process";
import {planDeveloperRequest,parseDeveloperArguments,executeDeveloperRequest,readDeveloperRequest,developerDebuggers} from "../🟦️.ts";
const owner=resolve(import.meta.dir,".."),fixture=JSON.parse(readFileSync(join(owner,"🧫️fixtures/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(owner,"🧬️schema/🔣️.json"),"utf8"));
const oracle=new Ajv({strict:true}).compile(schema),generated=process.env.SEMIO_TEST_ARTIFACT_DIR!;
if(!generated)throw Error("Ticket-owned test artifact directory required");
const root=mkdtempSync(join(generated,"dt-")),file=join(root,"app/space 🧪 file.ts");mkdirSync(join(root,"app"));writeFileSync(file,"export const value=1;\n");
for(const row of fixture.sourceCases)test(`source ${row.platform} ${row.request.editor}`,async()=>{
 expect(oracle(row.request)).toBe(true);const plan=await planDeveloperRequest(root,row.request,row.platform,row.environment??{});
 expect(plan.command).toBe(row.command);expect(plan.args).toEqual(row.args.map((value:string)=>value.replace("{file}",file)));expect(plan.cwd).toBe(root);
});
for(const row of fixture.gitCases)test(`read-only ${JSON.stringify(row.request)}`,async()=>{
 expect(oracle(row.request)).toBe(true);const plan=await planDeveloperRequest(root,row.request,process.platform,{});
 expect(plan.command).toBe("git");expect(plan.args.slice(-row.tail.length)).toEqual(row.tail);expect(plan.args).toContain("--no-optional-locks");expect(plan.args).toContain("core.hooksPath=");expect(plan.args).toContain("diff.external=");
});
for(const request of fixture.refusals)test(`refuses ${JSON.stringify(request)}`,async()=>{await expect(planDeveloperRequest(root,request,process.platform,{})).rejects.toBeDefined();});
test("physical source cannot cross a symlink",async()=>{
 const foreign=mkdtempSync(join(generated,"df-"));writeFileSync(join(foreign,"foreign.ts"),"");symlinkSync(foreign,join(root,"linked"),process.platform==="win32"?"junction":"dir");
 await expect(planDeveloperRequest(root,{command:"open-source",file:"linked/foreign.ts",editor:"system"},process.platform,{})).rejects.toBeDefined();
});
test("original cancellation and recorded process port are retained",async()=>{
 const controller=new AbortController(),records:unknown[]=[];const port={run:async(plan:unknown,signal:AbortSignal)=>{expect(signal).toBe(controller.signal);records.push(plan);},open:async(plan:unknown,signal:AbortSignal)=>{expect(signal).toBe(controller.signal);records.push(plan);}};
 await executeDeveloperRequest(root,{command:"open-source",file:"app/space 🧪 file.ts",editor:"system"},port,controller.signal,process.platform,{});expect(records.length).toBe(1);controller.abort();
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

for(const row of fixture.runtimeCases)test(`native Git ${row.name}`,async()=>{
 const workspace=resolve(owner,"../../../../../.."),plan=await planDeveloperRequest(workspace,row.request,process.platform,{}),environment={...process.env};
 for(const key of Object.keys(environment))if(key.startsWith("GIT_"))delete environment[key];environment.GIT_OPTIONAL_LOCKS="0";environment.GIT_TERMINAL_PROMPT="0";
 const result=spawnSync(plan.command,plan.args,{cwd:workspace,env:environment,encoding:"utf8",timeout:30_000,maxBuffer:16*1024*1024,windowsHide:true});
 expect(result.status,result.stderr).toBe(0);for(const text of row.contains??[])expect(result.stdout).toContain(text);
});

for(const row of fixture.terminalCases)test(`terminal ${row.platform} ${row.request.shell}`,async()=>{
 expect(oracle(row.request)).toBe(true);const plan=await planDeveloperRequest(root,row.request,row.platform,{});expect(plan.command).toBe(row.command);expect(plan.args).toEqual(row.args);expect(plan.cwd).toBe(resolve(root,row.request.directory??"."));expect(plan.detached).toBe(false);
});
test("localized owner declarations validate against the independent registry schema",()=>{
 const registrySchema=JSON.parse(readFileSync(resolve(owner,"../../🎛️dashboard/🧬️schema/🎮️registry/🔣️.json"),"utf8")),project=JSON.parse(readFileSync(join(owner,"📋️project.json"),"utf8"));
 const validate=new Ajv({strict:false}).compile({...registrySchema,$ref:"#/$defs/ProjectManifest"});expect(validate(project),JSON.stringify(validate.errors)).toBe(true);
 for(const branch of schema.oneOf){const command=branch.properties.command.const;expect(project.targets[command]?.options.command).toBe(`bun ./📜️script.ts ${command}`);}
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
 const controller=new AbortController(),records:unknown[]=[];await executeDeveloperRequest(root,captured,{run:async(plan,signal)=>{expect(signal).toBe(controller.signal);records.push(plan);},open:async()=>{throw Error("Debugger must remain supervised");}},controller.signal,process.platform,{});expect(records).toHaveLength(1);controller.abort();await expect(executeDeveloperRequest(root,captured,{run:async()=>{throw Error("Cancelled debugger executed");},open:async()=>{}},controller.signal,process.platform,{})).rejects.toBeDefined();
});


for(const profile of["node-inspect","python-pdb"] as const)test.skipIf(!developerDebuggers(process.platform,program=>Bun.which(program)).find(entry=>entry.profile===profile)?.installed)(`debug runtime ${profile}`,async()=>{
 const program=profile==="node-inspect"?"app/debug-target.js":"app/debug-target.py";writeFileSync(join(root,program),profile==="node-inspect"?fixture.debugPrograms.node:fixture.debugPrograms.python);
 const arguments_=["value with spaces",'quote"mark',"trailing\\","; $(literal)","🧪"],plan=await planDeveloperRequest(root,{command:"debug",profile,program,arguments:arguments_},process.platform,{}),{runOwnedCommand}=await import("../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts"),controller=new AbortController(),output:string[]=[];
 const run=runOwnedCommand(plan.command,plan.args,plan.cwd,"debugger-fixture",15_000,{signal:controller.signal,input:profile==="node-inspect"?"":"p 6*7\nc\nq\n",stdout:"ignore",output:{maximumBytes:65536,maximumLines:1000},outputConsumer:bytes=>{output.push(Buffer.from(bytes).toString());if(profile==="node-inspect"&&output.join("").includes("Break on start"))controller.abort();},onProgress:()=>{}});
 if(profile==="node-inspect"){await expect(run).rejects.toThrow("cancelled");expect(output.join("")).toContain("Debugger attached");expect(output.join("")).toContain("Break on start");console.log("[DEBUG] Actual Node inspector attached on a random local port, paused the fixture and was cancelled with its child");}
 else{await run;expect(output.join("")).toMatch(/(?:^|\n)(?:\(Pdb\) )?42\r?(?:\n|$)/u);expect(output.join("")).not.toContain("UnicodeEncodeError");const encoded=/DEBUGGER_ARGV:([^\r\n]+)/u.exec(output.join(""));expect(encoded).not.toBeNull();expect(JSON.parse(encoded![1]!)).toEqual(arguments_);console.log("[DEBUG] Actual Python Pdb evaluated 6*7 as42 and exited with Unicode workspace paths");}
},20_000);

test.skipIf(!Bun.which("gdb")||(process.platform!=="win32"&&!Bun.which("gcc")))(process.platform==="win32"?"debug runtime gdb installed frontend":"debug runtime gdb native breakpoint and arguments",async()=>{
 const {runOwnedCommand}=await import("../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts"),controller=new AbortController(),output:string[]=[];
 if(process.platform==="win32"){
  await runOwnedCommand("gdb",["--nx","--quiet"],root,"debugger-native-frontend",15_000,{signal:controller.signal,input:"print 6*7\nquit\n",stdout:"ignore",output:{maximumBytes:65536,maximumLines:1000},outputConsumer:bytes=>{output.push(Buffer.from(bytes).toString());},onProgress:()=>{}});expect(output.join("")).toMatch(/\$1 = 42/u);console.log("[DEBUG] Installed Windows GDB frontend evaluated42; native Unicode targets are refused with LLDB guidance");return;
 }
 const source=join(root,"app/debug-native.c"),program="app/debug-native";writeFileSync(source,fixture.debugPrograms.native);
 await runOwnedCommand("gcc",["-g","-O0","app/debug-native.c","-o",program],root,"debugger-native-build",20_000,{signal:controller.signal,output:{maximumBytes:65536,maximumLines:1000},onProgress:()=>{}});
 const plan=await planDeveloperRequest(root,{command:"debug",profile:"gdb",program,arguments:["value with spaces"]},process.platform,{});
 await runOwnedCommand(plan.command,plan.args,plan.cwd,"debugger-native-fixture",20_000,{signal:controller.signal,input:"break main\nrun\nprint argc\nprint 6*7\ncontinue\nquit\n",stdout:"ignore",output:{maximumBytes:131072,maximumLines:2000},outputConsumer:bytes=>{output.push(Buffer.from(bytes).toString());},onProgress:()=>{}});
 expect(output.join("")).toMatch(/Breakpoint 1, main/u);expect(output.join("")).toMatch(/\$1 = 2/u);expect(output.join("")).toMatch(/\$2 = 42/u);expect(output.join("")).toContain("DEBUGGER_FIXTURE_OK");console.log("[DEBUG] Actual GDB hit main, preserved a spaced argument, evaluated42 and continued the native fixture");
},45_000);


test.skipIf(/[^\u0020-\u007e]/u.test(process.cwd()))("debug Windows GDB keeps an ASCII workspace target and root cwd without relocation",async()=>{
 const workspace=process.cwd(),plan=await planDeveloperRequest(workspace,{command:"debug",profile:"gdb",program:"package.json",directory:"."},"win32",{});expect(plan.args).toEqual(["--nx","--quiet","--args","./package.json"]);expect(plan.cwd).toBe(workspace);expect(plan.detached).toBe(false);
});
