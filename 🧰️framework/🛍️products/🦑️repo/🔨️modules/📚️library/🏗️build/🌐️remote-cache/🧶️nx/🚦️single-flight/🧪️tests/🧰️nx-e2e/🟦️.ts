import {expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {homedir,tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {startRemoteCacheServerV1} from "../../../../🖥️server/🟦️.ts";
import {routeNxThroughFlightV1} from "../../🟦️.ts";

const worktree=resolve(import.meta.dir,"../../../../../../../../../../..");
const candidates=()=>{const base=join(worktree,".cache/b/d/m/j");return existsSync(base)?readdirSync(base).map(name=>join(base,name,"node_modules")).filter(path=>existsSync(join(path,"nx/package.json"))):[];};
const modules=process.env.SEMIO_TEST_NX_NODE_MODULES??candidates()[0],plain=process.env.SEMIO_TEST_NX_PACKAGE??join(homedir(),".bun/install/cache/nx@23.2.0@@@1");

test.skipIf(!modules||!existsSync(plain)||process.platform==="win32")("two real Nx 23 builds in two checkouts at different paths start the same cold task at once: the body runs once, the other build is served the result, and a later run runs nothing",async()=>{
 const scratch=mkdtempSync(join(tmpdir(),"nx-flight-e2e-")),marks=join(scratch,"marks.txt"),stops:Array<()=>Promise<void>>=[];
 try{
  const server=await startRemoteCacheServerV1({directory:join(scratch,"store"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"e2e-token"}});stops.push(()=>server.stop());
  const roots=["alpha/checkout","bravo/deeper/place","charlie/third"].map(name=>join(scratch,name));
  for(const root of roots){
   mkdirSync(join(root,"libs/a"),{recursive:true});mkdirSync(join(root,"node_modules"));
   for(const entry of readdirSync(modules!))if(entry!=="nx")symlinkSync(join(modules!,entry),join(root,"node_modules",entry));symlinkSync(plain,join(root,"node_modules","nx"));
   writeFileSync(join(root,"nx.json"),JSON.stringify({cacheDirectory:".nx-cache",useDaemonProcess:false,analytics:false,targetDefaults:{build:{cache:true,inputs:["{projectRoot}/src.txt"],outputs:["{projectRoot}/out"]}}}));
   writeFileSync(join(root,"libs/a/project.json"),JSON.stringify({name:"a",root:"libs/a",targets:{build:{executor:"nx:run-commands",options:{command:`sleep 4 && mkdir -p out && echo built-a > out/a.txt && echo run >> ${marks}`,cwd:"libs/a"}}}}));
   writeFileSync(join(root,"libs/a/src.txt"),"a\n");
  }
  const run=async(root:string)=>{
   const flight=await routeNxThroughFlightV1({NX_SELF_HOSTED_REMOTE_CACHE_SERVER:`${server.url}/main`,NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN:"e2e-token"});stops.push(()=>flight!.stop());
   const child=Bun.spawn(["node",join(root,"node_modules/nx/dist/bin/nx.js"),"run-many","-t","build"],{cwd:root,stdout:"pipe",stderr:"pipe",env:{PATH:process.env.PATH!,HOME:process.env.HOME!,NODE_OPTIONS:"--preserve-symlinks --preserve-symlinks-main",NX_DAEMON:"false",NX_NO_CLOUD:"true",NX_TUI:"false",CI:"1",...flight!.environment()}});
   const [out,err,status]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
   return {out,err,status,counters:flight!.counters()};
  };
  const [one,two]=await Promise.all(roots.slice(0,2).map(run));
  expect(one.status,one.err+one.out).toBe(0);expect(two.status,two.err+two.out).toBe(0);
  expect(readFileSync(marks,"utf8").trim().split("\n"),"the task body ran exactly once").toEqual(["run"]);
  for(const root of roots.slice(0,2))expect(readFileSync(join(root,"libs/a/out/a.txt"),"utf8")).toBe("built-a\n");
  expect([one.counters.claimed,two.counters.claimed].sort(),"one claimed, one did not").toEqual([0,1]);
  expect(one.counters.served+two.counters.served).toBe(1);
  const third=await run(roots[0]!),fourth=await run(roots[2]!);
  expect(third.status).toBe(0);expect(third.out).toContain("1/1 hit");expect(fourth.out).toContain("1/1 hit");expect(fourth.out).toContain("remote cache");
  expect(readFileSync(marks,"utf8").trim().split("\n"),"later runs run nothing").toEqual(["run"]);
 }finally{for(const stop of stops)await stop().catch(()=>{});rmSync(scratch,{recursive:true,force:true});}
},240000);

test.skipIf(!modules||!existsSync(plain)||process.platform==="win32")("after real Nx runs the slot settlement claims the touched tasks at the cache and removes the superseded entry from a real Nx cache directory and database, and Nx runs on",async()=>{
 const {settleNxSlotsV1,nxCacheDirectoryV1}=await import("../../../🪢️slot-settle/🟦️.ts"),{Database}=await import("bun:sqlite");
 const scratch=mkdtempSync(join(tmpdir(),"nx-slot-e2e-")),stops:Array<()=>Promise<void>>=[];
 try{
  const server=await startRemoteCacheServerV1({directory:join(scratch,"store"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"e2e-token"}});stops.push(()=>server.stop());
  const root=join(scratch,"checkout"),data=join(root,".nx","data");
  mkdirSync(join(root,"libs/a"),{recursive:true});mkdirSync(join(root,"node_modules"));
  for(const entry of readdirSync(modules!))if(entry!=="nx")symlinkSync(join(modules!,entry),join(root,"node_modules",entry));symlinkSync(plain,join(root,"node_modules","nx"));
  writeFileSync(join(root,"nx.json"),JSON.stringify({cacheDirectory:".nx-cache",useDaemonProcess:false,analytics:false,targetDefaults:{build:{cache:true,inputs:["{projectRoot}/src.txt"],outputs:["{projectRoot}/out"]}}}));
  writeFileSync(join(root,"libs/a/project.json"),JSON.stringify({name:"a",root:"libs/a",targets:{build:{executor:"nx:run-commands",options:{command:"mkdir -p out && cp src.txt out/a.txt",cwd:"libs/a"}}}}));
  const run=async(content:string)=>{
   writeFileSync(join(root,"libs/a/src.txt"),content);
   const flight=await routeNxThroughFlightV1({NX_SELF_HOSTED_REMOTE_CACHE_SERVER:`${server.url}/main`,NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN:"e2e-token"});stops.push(()=>flight!.stop());
   const environment={PATH:process.env.PATH!,HOME:process.env.HOME!,NODE_OPTIONS:"--preserve-symlinks --preserve-symlinks-main",NX_DAEMON:"false",NX_NO_CLOUD:"true",NX_TUI:"false",CI:"1",NX_WORKSPACE_DATA_DIRECTORY:data,...flight!.environment()},sinceMs=Date.now();
   const child=Bun.spawn(["node",join(root,"node_modules/nx/dist/bin/nx.js"),"run-many","-t","build"],{cwd:root,stdout:"pipe",stderr:"pipe",env:environment});
   const [out,err,status]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
   return {out,err,status,flight:flight!,environment,sinceMs};
  };
  const first=await run("one\n");expect(first.status,first.err+first.out).toBe(0);
  const second=await run("two\n");expect(second.status,second.err+second.out).toBe(0);
  const cache=nxCacheDirectoryV1(root),before=readdirSync(cache).filter(name=>/^[0-9]+$/u.test(name));
  expect(before.length,"two entries of one task slot in the real cache directory: "+readdirSync(cache).join()).toBe(2);
  const settled=await settleNxSlotsV1({root,environment:second.environment,flight:second.flight,sinceMs:0,graceMilliseconds:0,nowMs:Date.now()+60000});
  expect(settled.local?.scanned,"both entries are known to the real database").toBe(2);expect(settled.local?.removed).toHaveLength(1);
  const after=readdirSync(cache).filter(name=>/^[0-9]+$/u.test(name));
  expect(after).toHaveLength(1);expect(before).toContain(after[0]!);
  const database=new Database(join(data,readdirSync(data).find(name=>name.endsWith(".db"))!),{readonly:true}),kept=database.query("SELECT hash FROM cache_outputs").all() as {hash:string}[];database.close();
  expect(kept.map(row=>row.hash)).toEqual(after);
  expect(settled.registered===undefined||settled.registered.claimed+settled.registered.missing>=1).toBe(true);
  const third=await run("one\n");expect(third.status,third.err+third.out).toBe(0);
  const fourth=await run("one\n");expect(fourth.out).toContain("1/1 hit");
 }finally{for(const stop of stops)await stop().catch(()=>{});rmSync(scratch,{recursive:true,force:true});}
},240000);
