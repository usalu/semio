import {expect,test} from "bun:test";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,symlinkSync,unlinkSync,writeFileSync} from "node:fs";
import {homedir,tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {startRemoteCacheServerV1} from "../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {routeNxThroughFlightV1} from "../../../../🌐️remote-cache/🧶️nx/🚦️single-flight/🟦️.ts";
import {splitNxPlatformInputsV1,type PlatformProjectV1} from "../../🟦️.ts";

const worktree=resolve(import.meta.dir,"../../../../../../../../../..");
const candidates=()=>{const base=join(worktree,".cache/b/d/m/j");return existsSync(base)?readdirSync(base).map(name=>join(base,name,"node_modules")).filter(path=>existsSync(join(path,"nx/package.json"))):[];};
const modules=process.env.SEMIO_TEST_NX_NODE_MODULES??candidates()[0],plain=process.env.SEMIO_TEST_NX_PACKAGE??join(homedir(),".bun/install/cache/nx@23.2.0@@@1");
const real=Bun.which("node")??"",row={runtime:'node -p "process.platform.concat(process.arch)"'};

for(const useSplit of[true,false])test.skipIf(!real||!modules||!existsSync(plain)||process.platform==="win32")(useSplit?"two real Nx 23 builds on two platforms in two checkouts: a platform-independent task is computed once and fetched, a platform-bound task is computed on each platform, and the fetched result equals a clean local run":"before the split (the cause): the same independent task on two platforms is computed twice because the platform is in its hash",async()=>{
 const scratch=mkdtempSync(join(tmpdir(),"nx-platform-e2e-")),marks=join(scratch,"marks.txt"),stops:Array<()=>Promise<void>>=[],links:string[]=[];
 try{
  const server=await startRemoteCacheServerV1({directory:join(scratch,"store"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"e2e-token"}});stops.push(()=>server.stop());
  const target=(kind:string)=>({executor:"nx:run-commands",cache:true,inputs:["{projectRoot}/src.txt",row],outputs:["{projectRoot}/out"],options:{command:`mkdir -p out && echo built-${kind} > out/${kind}.txt && echo ${kind} >> ${marks}`,cwd:"libs/a"}});
  const projects={a:{namedInputs:{},targets:{build:target("build"),test:target("test")}}} as Record<string,PlatformProjectV1>,split=(useSplit?splitNxPlatformInputsV1(projects):projects).a!.targets!;
  expect(JSON.stringify(split.build!.inputs).includes("process.platform")).toBe(!useSplit);expect(JSON.stringify(split.test!.inputs)).toContain("process.platform");
  const make=(name:string,platform:string)=>{
   const root=join(scratch,name,"checkout");mkdirSync(join(root,"libs/a"),{recursive:true});mkdirSync(join(root,"node_modules"));mkdirSync(join(scratch,name,"bin"));
   for(const entry of readdirSync(modules!))if(entry!=="nx"){const link=join(root,"node_modules",entry);symlinkSync(join(modules!,entry),link);links.push(link);}
   const nx=join(root,"node_modules","nx");symlinkSync(plain,nx);links.push(nx);
   writeFileSync(join(scratch,name,"bin","node"),`#!/bin/sh\nif [ "$1" = "-p" ]; then echo ${platform}; else exec ${real} "$@"; fi\n`);chmodSync(join(scratch,name,"bin","node"),0o755);
   writeFileSync(join(scratch,name,"bin","repo"),`#!/bin/sh\necho ${platform}\n`);chmodSync(join(scratch,name,"bin","repo"),0o755);
   writeFileSync(join(root,"nx.json"),JSON.stringify({cacheDirectory:".nx-cache",useDaemonProcess:false,analytics:false}));
   writeFileSync(join(root,"libs/a/project.json"),JSON.stringify({name:"a",root:"libs/a",targets:split}));writeFileSync(join(root,"libs/a/src.txt"),"a\n");
   return {root,bin:join(scratch,name,"bin")};
  };
  const one=make("darwin","darwinarm64"),two=make("linux","linuxx64"),clean=make("clean","windowsx64");
  const run=async(machine:{root:string;bin:string},remote:boolean)=>{
   const flight=remote?await routeNxThroughFlightV1({NX_SELF_HOSTED_REMOTE_CACHE_SERVER:`${server.url}/main`,NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN:"e2e-token"}):undefined;if(flight)stops.push(()=>flight.stop());
   const child=Bun.spawn([real,join(machine.root,"node_modules/nx/dist/bin/nx.js"),"run-many","-t","build","test"],{cwd:machine.root,stdout:"pipe",stderr:"pipe",env:{PATH:`${machine.bin}:${process.env.PATH!}`,HOME:process.env.HOME!,NODE_OPTIONS:"--preserve-symlinks --preserve-symlinks-main",NX_DAEMON:"false",NX_NO_CLOUD:"true",NX_TUI:"false",CI:"1",...(flight?flight.environment():{})}});
   const [out,err,status]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
   return {out,err,status};
  };
  const first=await run(one,true);expect(first.status,first.err+first.out).toBe(0);
  const second=await run(two,true);expect(second.status,second.err+second.out).toBe(0);
  const ran=readFileSync(marks,"utf8").trim().split("\n").sort();
  expect(ran,"test runs once per platform; build once only when the platform is out of its hash").toEqual(useSplit?["build","test","test"]:["build","build","test","test"]);
  const local=await run(clean,false);expect(local.status,local.err+local.out).toBe(0);
  for(const kind of["build","test"]){
   const fetched=readFileSync(join(two.root,"libs/a/out",`${kind}.txt`),"utf8"),computed=readFileSync(join(one.root,"libs/a/out",`${kind}.txt`),"utf8"),fresh=readFileSync(join(clean.root,"libs/a/out",`${kind}.txt`),"utf8");
   expect(fetched,"the fetched result equals the clean local run").toBe(fresh);expect(computed).toBe(fresh);
  }
 }finally{for(const stop of stops)await stop().catch(()=>{});for(const link of links)try{unlinkSync(link);}catch{}rmSync(scratch,{recursive:true,force:true});}
},400000);
