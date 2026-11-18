import {describe,expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,statSync,symlinkSync,writeFileSync} from "node:fs";
import {hostname,tmpdir} from "node:os";
import {join} from "node:path";
import {TOOL_SETUP_V1} from "../../../../🏗️build/🧰️tool-setup/🟦️.ts";
import {READY_STAGES_V1,runReadyStageV1} from "../../../../🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts";
import {SLICE_LAB_HOME_V1 as H,sliceLabAllStagesV1,sliceLabLauncherBuildV1,sliceLabStageEnvironmentV1,sliceLabRegistryRefusalV1,sliceLabWarmBuildV1,sliceLabWarmItemsV1,sliceLabWarmV1,sliceLabCloneV1,sliceLabHomeEnvironmentV1,sliceLabHomeSeedsV1,sliceLabKindsOfSliceV1,sliceLabHomeReportV1,sliceLabLeaksV1,sliceLabLinkHomeV1,sliceLabLiveDiffV1,sliceLabLiveSnapshotV1,sliceLabObservedStagesV1,sliceLabPinnedGoSeedsV1,sliceLabSeedHomeV1,sliceLabSeedToolsV1,sliceLabSkipStagesV1,sliceLabShimsV1,sliceLabStageTableV1,sliceLabToolsForKindsV1} from "../🟦️.ts";

setDefaultTimeout(60000);
/** 🏠️ The home of a lab machine: the toolchain a slice needs comes from the declared kinds of its owners, the environment of a machine names no path of the live account, a download is refused and logged, a seed is a copy-on-write clone, a stage and a seed are told in a table, and a change outside the seam is found by a before and after listing. */
const live="/Users/live",root="/Users/live/work/checkout",home="/tmp/slw-ab12/m1/h";
const base={HOME:live,PATH:`${live}/.cargo/bin:/opt/homebrew/bin:/usr/bin:${live}/.local/bin`,CARGO_HOME:`${live}/.cargo`,RUSTUP_HOME:`${live}/.rustup`,GOPATH:`${live}/go`,XDG_DATA_HOME:`${live}/.local/share`,BUN_INSTALL:`${live}/.bun`,CARGO_TARGET_DIR:`${root}/.cache/target`,KEEP:"1",SEMIO_WORKSPACE_ROOT:root};
describe("slice lab home",()=>{
 test("the tools of a slice follow the declared kinds of its owners, never more",()=>{
  expect(sliceLabToolsForKindsV1(["go"])).toEqual(["bun","go"]);
  expect(sliceLabToolsForKindsV1(["go"]).some(id=>/rust|cargo/u.test(id))).toBe(false);
  expect(sliceLabToolsForKindsV1(["typescript","rust","go"])).toEqual(["bun","node","rustup","rust","cargo-nextest","cargo-llvm-cov","go"]);
  expect(sliceLabToolsForKindsV1(undefined)).toBe("all");
  expect(()=>sliceLabToolsForKindsV1(["cobol"])).toThrow(/unknown toolchain kind cobol[\s\S]*unbekannte Werkzeugart cobol/u);
  const known=new Set(TOOL_SETUP_V1.groups.all);
  for(const ids of Object.values(H.kinds))for(const id of ids)expect(known.has(id)).toBe(true);
  for(const seed of H.seeds)for(const id of seed.tools)expect(known.has(id)).toBe(true);
 });
 test("the kinds of a slice are read from the declared probe table, an undeclared owner makes them unknown",()=>{
  const probes=[{name:"a",kind:"go"},{name:"b",kind:"rust"}];
  expect(sliceLabKindsOfSliceV1(["a"],probes)).toEqual(["go"]);
  expect(sliceLabKindsOfSliceV1(["a","b"],probes)).toEqual(["go","rust"]);
  expect(sliceLabKindsOfSliceV1(["a","zzz"],probes)).toBeUndefined();
  expect(sliceLabKindsOfSliceV1(["zzz"],probes,["go"])).toEqual(["go"]);
 });
 test("the seeds of a Go slice hold no Rust, .NET or Python tree, the seeds of a Rust slice hold Rust",()=>{
  const names=(tools:readonly string[]|"all")=>sliceLabHomeSeedsV1(tools,live,home).map(seed=>seed.id);
  expect(names(["bun","go"])).toEqual(["launcher","runtimes","go"]);
  expect(names(["bun","rustup","rust"])).toEqual(["launcher","runtimes","cargo","rustup"]);
  expect(names("all")).toEqual(H.seeds.map(seed=>seed.id));
  const [first]=sliceLabHomeSeedsV1(["bun"],live,home);
  expect(first).toEqual({id:"launcher",source:`${live}/.local/share/semio/launcher`,target:`${home}/.local/share/semio/launcher`,kind:"tree"});
 });
 test("the environment of a machine names no path of the live account and drops every tool home of it",()=>{
  const env=sliceLabHomeEnvironmentV1(base,{home,live,downloads:"/tmp/slw-ab12/m1/downloads.log"});
  expect(env.HOME).toBe(home);
  for(const name of H.droppedEnvironment)expect(env[name]).toBeUndefined();
  expect(env.KEEP).toBe("1");
  expect(env.PATH!.split(":")).toEqual(["/opt/homebrew/bin","/usr/bin"]);
  expect(env.HTTPS_PROXY).toBeUndefined();expect(env.GOTOOLCHAIN).toBe("local");expect(env.GOPROXY).toBe("off");expect(env.CARGO_NET_OFFLINE).toBe("true");
  const proxied=sliceLabHomeEnvironmentV1({...base,SEMIO_SLICE_LAB_PROXY:"http://127.0.0.1:9"},{home,live,downloads:"/x"});expect(proxied.HTTPS_PROXY).toBe("http://127.0.0.1:9");for(const host of["127.0.0.1","localhost","[::1]","0.0.0.0",hostname()])expect(proxied.NO_PROXY!.split(",")).toContain(host);
  expect(env[H.downloadEnvironment]).toBe("/tmp/slw-ab12/m1/downloads.log");
  expect(sliceLabLeaksV1(env,live,root)).toEqual([]);
  expect(sliceLabLeaksV1({...env,STRAY:`${live}/.cargo/bin`,CACHE:`${root}/.cache/b`},live,root)).toEqual(["CACHE","STRAY"]);
  expect(sliceLabLeaksV1({...env,SEMIO_WORKSPACE_ROOT:root},live,root,["SEMIO_WORKSPACE_ROOT"])).toEqual([]);
 });
 test("a refused download is logged with its program and arguments and exits with the refusal status",()=>{
  const directory=realpathSync(mkdtempSync(join(tmpdir(),"slh-"))),machine=join(directory,"h"),log=join(directory,"downloads.log");
  try{
   sliceLabShimsV1(machine,log);
   for(const name of H.refusedPrograms){expect(existsSync(join(machine,H.shimDirectory,name))).toBe(true);expect(statSync(join(machine,H.shimDirectory,name)).mode&0o111).not.toBe(0);}
   const run=spawnSync(join(machine,H.shimDirectory,"curl"),["-fsSL","https://example.invalid/x.tar.gz"],{env:{PATH:"/usr/bin:/bin",[H.downloadEnvironment]:log},encoding:"utf8"});
   expect(run.status).toBe(H.refusedExit);
   expect(run.stderr).toContain("download refused by the slice lab");expect(run.stderr).toContain("Download vom Slice-Labor abgelehnt");
   expect(readFileSync(log,"utf8")).toContain("curl -fsSL https://example.invalid/x.tar.gz");
  }finally{rmSync(directory,{recursive:true,force:true});}
 });
 test("a seed is a copy-on-write clone: the copy has the bytes, a write to it leaves the source alone",()=>{
  const directory=realpathSync(mkdtempSync(join(tmpdir(),"slh-"))),source=join(directory,"a"),target=join(directory,"b");
  try{
   mkdirSync(join(source,"x","y"),{recursive:true});writeFileSync(join(source,"x","y","f.txt"),"one");writeFileSync(join(source,"g.bin"),Buffer.alloc(100000,7));symlinkSync("g.bin",join(source,"link"));
   const row=sliceLabCloneV1(source,target);
   expect(["clonefile","reflink","copy"]).toContain(row.method);expect(row.seconds).toBeGreaterThanOrEqual(0);
   expect(readFileSync(join(target,"x","y","f.txt"),"utf8")).toBe("one");expect(readFileSync(join(target,"link")).length).toBe(100000);
   writeFileSync(join(target,"x","y","f.txt"),"two");expect(readFileSync(join(source,"x","y","f.txt"),"utf8")).toBe("one");
   const rows=sliceLabSeedHomeV1([{id:"t",source,target:join(directory,"c","d"),kind:"tree"},{id:"gone",source:join(directory,"none"),target:join(directory,"e"),kind:"tree"}]);
   expect(rows.map(item=>[item.id,item.ok])).toEqual([["t",true],["gone",false]]);expect(readFileSync(join(directory,"c","d","g.bin")).length).toBe(100000);
  }finally{rmSync(directory,{recursive:true,force:true});}
 });
 test("a change outside the seam shows in the listing diff, an untouched tree and a reread do not",()=>{
  const directory=realpathSync(mkdtempSync(join(tmpdir(),"slh-"))),account=join(directory,"acct"),checkout=join(directory,"co");
  try{
   mkdirSync(join(account,".cargo","bin"),{recursive:true});mkdirSync(join(account,".local","bin"),{recursive:true});writeFileSync(join(account,".cargo","bin","cargo"),"x");writeFileSync(join(account,".zshrc"),"a");mkdirSync(join(checkout,".cache","b","t"),{recursive:true});mkdirSync(join(checkout,".cache","f"),{recursive:true});
   const before=sliceLabLiveSnapshotV1(account,checkout);
   expect(sliceLabLiveDiffV1(before,sliceLabLiveSnapshotV1(account,checkout))).toEqual([]);
   writeFileSync(join(account,".zshrc"),"changed now");writeFileSync(join(account,".cargo","bin","wasm-pack"),"y");mkdirSync(join(checkout,".cache","b","z"));
   const changed=sliceLabLiveDiffV1(before,sliceLabLiveSnapshotV1(account,checkout));
   expect(changed.some(path=>path.endsWith(".zshrc"))).toBe(true);expect(changed.some(path=>path.endsWith(join(".cargo","bin","wasm-pack")))).toBe(true);expect(changed.some(path=>path.endsWith(join(".cache","b","z")))).toBe(true);
  }finally{rmSync(directory,{recursive:true,force:true});}
 });
 test("the stage table names every stage with its seconds and its verdict",()=>{
  const stages=H.stageNames.map(name=>({name}));
  const rows=sliceLabStageTableV1(stages,[{stage:"native",seconds:5.2,outcome:"ok"},{stage:"repo-mcp",seconds:12,outcome:"ok"},{stage:"dependencies",seconds:30.04,outcome:"stage-failed"}],{stage:"dependencies",code:"stage-failed",exit:1});
  expect(rows).toEqual([["native","5.2","ok"],["repo-mcp","12","ok"],["dependencies","30","failed (stage-failed, exit 1)"],["tools","-","not reached"],["workspace","-","not reached"]]);
 });
 test("the whole plan is the real plan of a worker: same names, same bootstrap and repo kinds, only tools and workspace are the slice's",()=>{
  const workspace={name:"workspace",kind:"repo" as const,arguments:["run-many","--projects=repo-mcp","--targets=build","--dry-run"],timeoutMilliseconds:1000,inputs:[],bootstrapInput:false};
  const plan=sliceLabAllStagesV1(READY_STAGES_V1,workspace,["bun","go"]);
  expect(plan.map(stage=>stage.name)).toEqual([...H.stageNames]);
  for(const name of ["native","repo-mcp","dependencies"])expect(plan.find(stage=>stage.name===name)).toEqual(READY_STAGES_V1.find(stage=>stage.name===name)!);
  expect(plan.find(stage=>stage.name==="tools")!.arguments).toEqual(["tools","setup","toolchains","bun","go"]);
  expect(plan.find(stage=>stage.name==="workspace")).toEqual(workspace);
  expect(sliceLabAllStagesV1(READY_STAGES_V1,workspace,"all").find(stage=>stage.name==="tools")!.arguments).toEqual(["tools","setup","all"]);
 });

 test("a stage of the real runner with a lab home and a lab state root touches nothing of the account it was not given",async()=>{
  const directory=realpathSync(mkdtempSync(join("/tmp","slh-"))),account=join(directory,"live"),checkout=join(directory,"co"),lab=join(directory,"lab"),labHome=join(lab,"h"),state=join(lab,"s");
  try{
   mkdirSync(join(account,".cargo","bin"),{recursive:true});mkdirSync(join(account,".local","bin"),{recursive:true});writeFileSync(join(account,".zshrc"),"a");mkdirSync(join(checkout,".cache","b"),{recursive:true});mkdirSync(join(checkout,".cache","f"),{recursive:true});
   mkdirSync(join(labHome,".local","bin"),{recursive:true});mkdirSync(join(state,"f"),{recursive:true});
   const repo=join(labHome,".local","bin","repo");
   writeFileSync(repo,`#!/bin/sh\nprintf '%s\\n' "$*" > "$HOME/argv.txt"\nmkdir -p "$HOME/.cargo" && printf x > "$HOME/.cargo/marker"\nprintf y > "${join(state,"f","wrote.txt")}"\n`);chmodSync(repo,0o755);
   const before=sliceLabLiveSnapshotV1(account,checkout),lines:string[]=[];
   const environment=sliceLabHomeEnvironmentV1({PATH:"/usr/bin:/bin",SEMIO_WORKSPACE_ROOT:checkout},{home:labHome,live:account,downloads:join(lab,"d.log")});
   const declared=process.env.SEMIO_MACHINE_STATE_DIRECTORY;process.env.SEMIO_MACHINE_STATE_DIRECTORY=state;
   const result=await runReadyStageV1({root:checkout,environment:{...environment,SEMIO_MACHINE_STATE_DIRECTORY:state},stage:{name:"tools",kind:"repo",arguments:["tools","setup","bun"],timeoutMilliseconds:60000,inputs:[],bootstrapInput:false},signal:AbortSignal.timeout(50000),jobs:1,home:labHome,onLine:line=>lines.push(line)}).finally(()=>{if(declared===undefined)delete process.env.SEMIO_MACHINE_STATE_DIRECTORY;else process.env.SEMIO_MACHINE_STATE_DIRECTORY=declared;});
   expect(result.exit).toBe(0);
   expect(readFileSync(join(labHome,"argv.txt"),"utf8").trim()).toBe("tools setup bun");expect(existsSync(join(labHome,".cargo","marker"))).toBe(true);expect(existsSync(join(state,"f","ready","tools.log"))).toBe(true);
   expect(sliceLabLiveDiffV1(before,sliceLabLiveSnapshotV1(account,checkout))).toEqual([]);expect(existsSync(join(checkout,".cache","f","ready"))).toBe(false);
  }finally{rmSync(directory,{recursive:true,force:true});}
 });

 test("a run that ended before its result shows the stages from its samples, the last one as running",()=>{
  const samples=[{machine:"m1",stage:"native"},{machine:"m1",stage:"native"},{machine:"m1",stage:"repo-mcp"},{machine:"m2",stage:"native"},{machine:"m1",stage:null}];
  const observed=sliceLabObservedStagesV1(samples,"m1",2);
  expect(observed).toEqual({seconds:{native:4,"repo-mcp":2},last:"repo-mcp"});
  expect(sliceLabStageTableV1(H.stageNames.map(name=>({name})),[],null,observed)).toEqual([["native","4","passed (seen in samples)"],["repo-mcp","2","running"],["dependencies","-","not reached"],["tools","-","not reached"],["workspace","-","not reached"]]);
 });
 test("the report tells tools, seeds, stage tables, changes and refused downloads in both languages",()=>{
  const report={tools:["bun","go"] as readonly string[],kinds:["go"],seeds:[{machine:"m1",id:"launcher",ok:true,method:"clonefile",seconds:0.1},{machine:"m1",id:"go",ok:false,method:"-",seconds:0}],stageTables:{m1:[["native","5","ok"] as const]},liveChanges:["/a/.zshrc"],downloads:["curl -fsSL https://x.invalid/a"]};
  const en=sliceLabHomeReportV1("en",report).join("\n"),de=sliceLabHomeReportV1("de",report).join("\n");
  expect(en).toContain("tools prepared for this slice: bun go");expect(de).toContain("für diesen Ausschnitt vorbereitete Werkzeuge: bun go");
  expect(en).toContain("| launcher | clonefile | 0.1 | m1 |");expect(en).toContain("| go | absent | 0 | m1 |");expect(en).toContain("| native | 5 | ok |");expect(de).toContain("Stufe | Sekunden | Urteil");
  expect(en).toContain("changed during the run: 1: /a/.zshrc");expect(en).toContain("refused: 1: curl -fsSL https://x.invalid/a");
  expect(sliceLabHomeReportV1("en",{...report,tools:"all"}).join("\n")).toContain("tools prepared: all");
 });

 test("the real native stage cannot be sliced: with all stages its tools are seeded whatever the slice is",()=>{
  expect(sliceLabSeedToolsV1(["bun","go"],"workspace")).toEqual(["bun","go"]);
  expect(sliceLabSeedToolsV1(["bun","go"],"all")).toEqual(["bun","go","node","rustup","rust","dotnet","uv"]);
  expect(sliceLabSeedToolsV1("all","all")).toBe("all");
  expect(sliceLabHomeSeedsV1(sliceLabSeedToolsV1(["bun","go"],"all"),live,home).map(seed=>seed.id)).toEqual(["launcher","runtimes","cargo","rustup","dotnet","go","uv-bin","uvx-bin","uv"]);
  for(const id of H.nativeTools)expect(new Set(TOOL_SETUP_V1.groups.all).has(id)).toBe(true);
 });

 test("the pinned Go of the tools partition is seeded where the launcher installer looks for it",()=>{
  const directory=realpathSync(mkdtempSync(join(tmpdir(),"slh-")));
  try{
   mkdirSync(join(directory,".cache","b","t","m","native","go","1.26.9","darwin-arm64-abc","bin"),{recursive:true});
   expect(sliceLabPinnedGoSeedsV1(directory,"/h")).toEqual([{id:"go-pinned",source:join(directory,".cache","b","t","m","native","go","1.26.9","darwin-arm64-abc"),target:"/h/.local/share/semio/go/v1.26.9",kind:"tree"}]);
   expect(sliceLabPinnedGoSeedsV1(join(directory,"none"),"/h")).toEqual([]);
  }finally{rmSync(directory,{recursive:true,force:true});}
 });

 test("the user tool links are made again into the lab home, only for prepared tools and present targets",()=>{
  const directory=realpathSync(mkdtempSync(join(tmpdir(),"slh-")));
  try{
   mkdirSync(join(directory,".dotnet"),{recursive:true});writeFileSync(join(directory,".dotnet","dotnet"),"x");mkdirSync(join(directory,".cargo","bin"),{recursive:true});writeFileSync(join(directory,".cargo","bin","cargo"),"x");
   mkdirSync(join(directory,".local","share","semio","launcher","bin"),{recursive:true});writeFileSync(join(directory,".local","share","semio","launcher","bin","repo"),"r");
   expect(sliceLabLinkHomeV1(["dotnet"],directory)).toEqual(["repo","dotnet"]);
   expect(readFileSync(join(directory,".local","bin","dotnet"),"utf8")).toBe("x");expect(existsSync(join(directory,".local","bin","cargo"))).toBe(false);
   expect(sliceLabLinkHomeV1("all",directory)).toEqual(["cargo"]);
  }finally{rmSync(directory,{recursive:true,force:true});}
 });

 test("a stage can be skipped by name, only by a name the plan has",()=>{
  const plan=sliceLabSkipStagesV1(READY_STAGES_V1,["native"]);
  expect(plan.map(stage=>stage.name)).toEqual(READY_STAGES_V1.map(stage=>stage.name));
  expect(plan[0]).toMatchObject({name:"native",kind:"repo",arguments:["--help"]});expect(plan[1]).toEqual(READY_STAGES_V1[1]!);
  expect(sliceLabSkipStagesV1(READY_STAGES_V1,undefined)).toEqual(READY_STAGES_V1);
  expect(()=>sliceLabSkipStagesV1(READY_STAGES_V1,["nope"])).toThrow(/unknown stage nope/u);
 });
});

describe("slice lab home: registry, warm partitions, tool selection",()=>{
 test("the launcher of a lab machine is built from the sources of the tree and replaces the seeded binary",()=>{
  const goBinary=spawnSync("/bin/sh",["-c","command -v go"],{encoding:"utf8"}).stdout.trim();
  if(goBinary===""){expect(sliceLabLauncherBuildV1("/nonexistent","/nonexistent")).toMatchObject({ok:false,reason:"no-go"});return;}
  const directory=realpathSync(mkdtempSync(join(tmpdir(),"slw-t161-"))),root=join(directory,"root"),home=join(directory,"h"),module=join(root,"🧰️framework","🛍️products","🦑️repo","🔨️modules","🚀️launcher","📦️packages","🐹️go");
  try{
   mkdirSync(join(module,"📦️main"),{recursive:true});mkdirSync(join(module,"🧭️workspace-root"),{recursive:true});
   writeFileSync(join(module,"🐹️.go"),"package launcher\n");writeFileSync(join(module,"🧭️workspace-root","🐹️.go"),'package launcher\n\nfunc Marker() string { return "built-from-sources" }\n');
   writeFileSync(join(module,"📦️main","🐹️.go"),'package main\n\nimport (\n\t"fmt"\n\tlauncher "github.com/usalu/semio/repo/launcher"\n)\n\nfunc main() { fmt.Println(launcher.Marker()) }\n');
   mkdirSync(join(home,".local","share","semio","launcher","bin"),{recursive:true});writeFileSync(join(home,".local","share","semio","launcher","bin","repo"),"seeded");
   expect(sliceLabLauncherBuildV1(join(directory,"empty"),home,goBinary)).toMatchObject({ok:false,reason:"no-sources"});
   const row=sliceLabLauncherBuildV1(root,home,goBinary);
   expect(row).toMatchObject({ok:true,reason:"built"});
   const target=join(home,".local","share","semio","launcher","bin","repo");expect(spawnSync(target,[],{encoding:"utf8"}).stdout.trim()).toBe("built-from-sources");
  }finally{rmSync(directory,{recursive:true,force:true});}
 });
 test("the dependencies stage runs without the offline switches and behind a refusing proxy, every other stage keeps the machine's environment",()=>{
  const base={CARGO_NET_OFFLINE:"true",RUSTUP_OFFLINE:"1",GOPROXY:"off",PATH:"/usr/bin"};
  const dependencies=sliceLabStageEnvironmentV1("dependencies",base);
  expect(dependencies.CARGO_NET_OFFLINE).toBeUndefined();expect(dependencies.RUSTUP_OFFLINE).toBeUndefined();expect(dependencies.GOPROXY).toBe("off");
  for(const name of["HTTPS_PROXY","HTTP_PROXY","ALL_PROXY"])expect(dependencies[name]).toBe("http://127.0.0.1:9");
  expect(dependencies.NO_PROXY).toContain("127.0.0.1");expect(base.CARGO_NET_OFFLINE).toBe("true");
  for(const name of["native","repo-mcp","tools","workspace"])expect(sliceLabStageEnvironmentV1(name,base)).toEqual(base);
 });
 test("no Go flag is forced: -mod=mod is refused in workspace mode and would let go rewrite go.mod, the offline guard is GOPROXY and GOTOOLCHAIN",()=>{
  expect(Object.keys(H.forcedEnvironment)).not.toContain("GOFLAGS");expect(H.forcedEnvironment.GOPROXY).toBe("off");expect(H.forcedEnvironment.GOTOOLCHAIN).toBe("local");
  const directory=realpathSync(mkdtempSync(join(tmpdir(),"slw-t161-")));
  try{
   const run=spawnSync("/bin/sh",["-c","command -v go >/dev/null && go env GOFLAGS || echo none"],{encoding:"utf8",env:{...process.env,...sliceLabHomeEnvironmentV1({PATH:process.env.PATH},{home:directory,live:"/nonexistent",downloads:join(directory,"d.log")})}});
   expect(run.stdout.trim()).not.toContain("-mod=mod");
  }finally{spawnSync("chmod",["-R","u+w",directory]);rmSync(directory,{recursive:true,force:true});}
 });
 test("the tools stage of a slice names the toolchains command (the bare ids are read as a build backend and refused)",async()=>{
  const{requireBuildBackendIdV1}=await import("../../../../🏗️build/🔌️backends/🟦️.ts");
  expect(()=>requireBuildBackendIdV1("bun")).toThrow();
  const workspace={name:"workspace",kind:"repo" as const,arguments:["run-many"],timeoutMilliseconds:1000,inputs:[],bootstrapInput:false};
  expect(sliceLabAllStagesV1(READY_STAGES_V1,workspace,["bun","go"]).find(stage=>stage.name==="tools")!.arguments).toEqual(["tools","setup","toolchains","bun","go"]);
  const{parseToolSetupSelectionV1}=await import("../../../../🏗️build/🧰️tool-setup/⌨️command/🟦️.ts");
  expect(parseToolSetupSelectionV1(["tools","setup","toolchains","bun","go"])).toEqual(["bun","go"]);
 });
 test("the refused registry is honoured by the pinned bun: its request goes to the loopback address of the files, nowhere else",async()=>{
  const directory=realpathSync(mkdtempSync(join(tmpdir(),"slw-t161-")));
  try{
   const seen:string[]=[],server=Bun.serve({port:0,hostname:"127.0.0.1",fetch:request=>{seen.push(new URL(request.url).pathname);return new Response("{}",{status:404});}});
   mkdirSync(join(directory,"h"));mkdirSync(join(directory,"p"));writeFileSync(join(directory,"p","package.json"),JSON.stringify({name:"g",dependencies:{"semio-r161-nonexistent-zzz":"1.0.0"}}));
   const made=sliceLabRegistryRefusalV1(join(directory,"h"));expect(made).toEqual([".npmrc",".bunfig.toml"]);
   expect(readFileSync(join(directory,"h",".npmrc"),"utf8")).toBe("registry=http://127.0.0.1:9/\n");
   for(const name of made)writeFileSync(join(directory,"h",name),readFileSync(join(directory,"h",name),"utf8").replaceAll("http://127.0.0.1:9/",`http://127.0.0.1:${server.port}/`));
   const run=Bun.spawn([process.execPath,"install","--cache-dir",join(directory,"c")],{cwd:join(directory,"p"),env:{PATH:process.env.PATH!,HOME:join(directory,"h"),HTTPS_PROXY:"http://127.0.0.1:9",HTTP_PROXY:"http://127.0.0.1:9",NO_PROXY:"127.0.0.1"},stdout:"pipe",stderr:"pipe"});
   await run.exited;server.stop();
   expect(seen).toEqual(["/semio-r161-nonexistent-zzz"]);
  }finally{rmSync(directory,{recursive:true,force:true});}
 });
 test("the warm table names known tools and stages, and a stage gets the items of its tools only",()=>{
  const known=new Set(TOOL_SETUP_V1.groups.all);
  for(const row of H.warm){expect(H.stageNames).toContain(row.stage);for(const item of row.items)for(const id of item.tools)expect(known.has(id)).toBe(true);}
  expect(sliceLabWarmItemsV1("dependencies",["bun","go"]).map(item=>item.id)).toEqual(["bun-cache"]);
  expect(sliceLabWarmItemsV1("tools",["bun","go"]).map(item=>item.id)).toEqual(["downloads","repo-watch","bun","go"]);
  expect(sliceLabWarmItemsV1("tools","all").length).toBe(H.warm.find(row=>row.stage==="tools")!.items.length);
  expect(sliceLabWarmItemsV1("workspace","all")).toEqual([]);
 });
 test("warming copies exactly the source tree into a mounted partition, leaves a filled target, and names why it skips",()=>{
  const directory=realpathSync(mkdtempSync(join(tmpdir(),"slw-t161-"))),source=join(directory,"warm"),state=join(directory,"s");
  try{
   mkdirSync(join(source,"d","bun-cache","@scope","pkg"),{recursive:true});writeFileSync(join(source,"d","bun-cache","@scope","pkg","a.txt"),"a");writeFileSync(join(source,"d","bun-cache","b b.txt"),"b");
   mkdirSync(join(state,"b","d","m","bun-cache"),{recursive:true});
   const mounted=()=>true;
   const first=sliceLabWarmV1({state,source,stage:"dependencies",tools:["bun","go"],mounted});
   expect(first.map(row=>[row.id,row.ok,row.method])).toEqual([["bun-cache",true,"copy"]]);
   const diff=spawnSync("diff",["-r",join(source,"d","bun-cache"),join(state,"b","d","m","bun-cache")],{encoding:"utf8"});expect(diff.status).toBe(0);
   expect(sliceLabWarmV1({state,source,stage:"dependencies",tools:["bun"],mounted}).map(row=>row.method)).toEqual(["present"]);
   rmSync(join(state,"b","d","m","bun-cache"),{recursive:true});mkdirSync(join(state,"b","d","m","bun-cache"));
   expect(sliceLabWarmV1({state,source,stage:"dependencies",tools:["bun"],mounted:()=>false}).map(row=>row.method)).toEqual(["skipped: not-mounted"]);
   expect(sliceLabWarmV1({state,source,stage:"tools",tools:["bun"],mounted}).map(row=>[row.id,row.method])).toEqual([["downloads","skipped: no-source"],["repo-watch","skipped: no-source"],["bun","skipped: no-source"]]);
   expect(sliceLabWarmV1({state,source:join(directory,"nowhere"),stage:"dependencies",tools:"all",mounted}).map(row=>row.method)).toEqual(["skipped: no-source"]);
   expect(()=>statSync(join(source,"d","bun-cache","@scope","pkg","a.txt"))).not.toThrow();
  }finally{rmSync(directory,{recursive:true,force:true});}
 });
 test("the real partition test tells a plain directory from a mounted one by the device of its mount directory",()=>{
  const directory=realpathSync(mkdtempSync(join(tmpdir(),"slw-t161-"))),source=join(directory,"warm");
  try{
   mkdirSync(join(source,"d","bun-cache"),{recursive:true});writeFileSync(join(source,"d","bun-cache","x"),"x");mkdirSync(join(directory,"s","b","d","m","bun-cache"),{recursive:true});
   expect(sliceLabWarmV1({state:join(directory,"s"),source,stage:"dependencies",tools:["bun"]}).map(row=>row.method)).toEqual(["skipped: not-mounted"]);
  }finally{rmSync(directory,{recursive:true,force:true});}
 });
 test("the warm directory is built read only from the live partitions and an item already there is kept",()=>{
  const directory=realpathSync(mkdtempSync(join(tmpdir(),"slw-t161-"))),live=join(directory,"live"),out=join(directory,"out");
  try{
   mkdirSync(join(live,".cache","b","d","m","bun-cache"),{recursive:true});writeFileSync(join(live,".cache","b","d","m","bun-cache","y"),"y");
   const rows=sliceLabWarmBuildV1(live,out);
   expect(rows.find(row=>row.id==="bun-cache")).toMatchObject({ok:true,method:"copy"});
   expect(readFileSync(join(out,"d","bun-cache","y"),"utf8")).toBe("y");
   expect(rows.filter(row=>row.id!=="bun-cache").every(row=>row.method==="skipped: no-source")).toBe(true);
   expect(sliceLabWarmBuildV1(live,out).find(row=>row.id==="bun-cache")!.method).toBe("present");
   expect(readFileSync(join(live,".cache","b","d","m","bun-cache","y"),"utf8")).toBe("y");
  }finally{rmSync(directory,{recursive:true,force:true});}
 });
});
