import{test,expect}from"bun:test";
import{mkdirSync,mkdtempSync,writeFileSync,readFileSync,utimesSync,existsSync,rmSync}from"node:fs";
import{join,dirname}from"node:path";
import fg from"fast-glob";
import fixture from"../🧫️fixtures/🔣️.json";
import{acquireCargoBuildLeaseV1}from"../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";

test("Incremental compaction rescans under compiler ownership and preserves current, working and foreign-profile sessions",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;expect(output).toBeTruthy();mkdirSync(output!,{recursive:true});const root=mkdtempSync(join(output!,"incremental-compact-")),build=join(root,"build"),leases=join(root,"leases"),controller=new AbortController();let held:any,run:Promise<any>|undefined;
 try{
  for(const row of fixture.sessions){const path=join(build,row.path);mkdirSync(path,{recursive:true});writeFileSync(join(path,"data"),"x".repeat(row.bytes));utimesSync(join(path,"data"),row.time/1000,row.time/1000);const lock=join(dirname(path),row.path.split("/").at(-1)!.replace(/-[^-]+$/u,".lock"));writeFileSync(lock,"");utimesSync(lock,row.time/1000,row.time/1000);}
  const oracle=fg.sync("**/s-*/data",{cwd:build,onlyFiles:true}).map(path=>path.slice(0,-5)).sort();expect(oracle).toEqual(fixture.sessions.map(row=>row.path).sort());
  const api=await import("../../🟦️.ts");expect(typeof(api as any).compactCargoIncrementalSessionsV1).toBe("function");
  const options={buildDirectory:build,leaseDirectory:leases,profile:fixture.profile,signal:controller.signal,dryRun:true};held=await acquireCargoBuildLeaseV1({directory:leases,buildDirectory:build,args:["--profile",fixture.profile],signal:controller.signal});
  let complete=false;run=(api as any).compactCargoIncrementalSessionsV1(options).then(value=>{complete=true;return value;});await Bun.sleep(100);expect(complete).toBe(false);expect(fg.sync("**/data",{cwd:build})).toHaveLength(fixture.sessions.length);held.release();held=undefined;
  const planned=await run;run=undefined;expect(planned.paths).toEqual(fixture.removed);expect(fg.sync("**/data",{cwd:build})).toHaveLength(fixture.sessions.length);
  const result=await(api as any).compactCargoIncrementalSessionsV1({...options,dryRun:false});expect(result.paths).toEqual(fixture.removed);expect(result.bytes).toBe(3);expect(fg.sync("**/s-*/data",{cwd:build}).map(path=>path.slice(0,-5)).sort()).toEqual([...fixture.retained].sort());
  for(const path of fixture.removed)expect(existsSync(join(build,path))).toBe(false);for(const path of fixture.retained)expect(readFileSync(join(build,path,"data")).length).toBeGreaterThan(0);
  console.log("[DEBUG] Incremental compaction compiler exclusion=true dryRun=unchanged staleRemoved=1 newestWorkingForeign=retained");
 }finally{controller.abort();held?.release();await run?.catch(()=>{});rmSync(root,{recursive:true,force:true});}
},30000);
