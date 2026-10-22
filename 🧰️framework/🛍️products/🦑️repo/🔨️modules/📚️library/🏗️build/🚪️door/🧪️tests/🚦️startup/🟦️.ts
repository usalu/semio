import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdtempSync,readFileSync,rmSync,statSync,writeFileSync} from "node:fs";
import {createRequire} from "node:module";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import {build} from "esbuild";
import fixture from "./🔣️.json" with {type:"json"};

const scratch=mkdtempSync(join(tmpdir(),"startup-law-"));
afterAll(()=>rmSync(scratch,{recursive:true,force:true}));
const entry=resolve(import.meta.dir,"../../📜️script.ts"),doorCore=resolve(import.meta.dir,"../../🟦️.ts"),groups=fixture.groups;
const plugin=join(scratch,"loaded-modules.ts"),driver=join(scratch,"import-driver.ts");
writeFileSync(plugin,'import {readFileSync,writeFileSync} from "node:fs";const seen:string[]=[];Bun.plugin({name:"loaded-modules",setup(build){build.onLoad({filter:/\\.(ts|mjs)$/},args=>{seen.push(args.path);return {contents:readFileSync(args.path,"utf8"),loader:args.path.endsWith(".ts")?"ts":"js"};});}});process.on("exit",()=>{writeFileSync(process.env.LOADED_MODULES_OUT!,JSON.stringify(seen));});\n');
writeFileSync(driver,'await import(process.argv[2]!);\n');

/** 🚦️ Names the heavy groups a loaded module list reaches, by path substring. */
const reached=(loaded:readonly string[],selected:readonly string[]):Record<string,string[]>=>Object.fromEntries(groups.filter(group=>selected.includes(group.id)).map(group=>[group.id,loaded.filter(path=>group.match.some(part=>path.includes(part)))]).filter(([,paths])=>(paths as string[]).length>0));

const load=(row:{id:string;kind:string;args?:readonly string[];module?:string}):string[]=>{
 const out=join(scratch,`${row.id}.json`),arguments_=row.kind==="door"?[entry,...row.args!]:[driver,resolve(import.meta.dir,row.module!)],result=spawnSync(process.execPath,["--preload",plugin,...arguments_],{encoding:"utf8",timeout:60_000,env:{...process.env,SEMIO_LANGUAGE:"en",LOADED_MODULES_OUT:out}});
 if(result.status!==0&&result.status!==2)throw Error(`${row.id}: exit ${result.status}: ${result.stderr.slice(0,400)}`);
 return JSON.parse(readFileSync(out,"utf8")) as string[];
};

test("a light command loads no heavy module group and stays within its module and byte budget; a negative control proves the trace sees the groups",()=>{
 for(const row of fixture.rows){
  const loaded=load(row),forbidden=(row as {forbid?:string[]}).forbid??[],expected=(row as {expectViolations?:string[]}).expectViolations,found=reached(loaded,forbidden);
  if(expected){expect(Object.keys(found).sort(),row.id).toEqual([...expected].sort());continue;}
  expect(found,`${row.id} reached heavy groups`).toEqual({});
  const bytes=loaded.reduce((sum,path)=>sum+statSync(path).size,0);
  expect(loaded.length,`${row.id} modules`).toBeLessThanOrEqual((row as {maximumModules:number}).maximumModules);
  expect(bytes,`${row.id} bytes`).toBeLessThanOrEqual((row as {maximumBytes:number}).maximumBytes);
 }
});

test("the group matcher flags exactly the modules whose path names a group",()=>{
 const base="/w/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
 const loaded=[`${base}/🏗️build/🕸️graph/🟦️.ts`,`${base}/🏗️build/⚙️operations/📖️help/🟦️.ts`,`/w/🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🟦️.ts`];
 expect(reached(loaded,groups.map(group=>group.id))).toEqual({graph:[loaded[0]!],"test-harness":[loaded[2]!]});
 expect(reached(loaded,["caller"])).toEqual({});
});

test("the traced front door equals the static closure an independent bundler computes",async()=>{
 const traced=load({id:"closure-help",kind:"door",args:["repo","--help"]}).filter(path=>path!==entry&&!path.endsWith("/⚙️runtime/🏁️main/🟦️.ts")&&!path.endsWith("/📤️output/🚪️entry/🟦️.ts")).sort();
 const result=await build({entryPoints:[doorCore],bundle:true,format:"esm",platform:"node",write:false,outdir:join(scratch,"bundle"),metafile:true,external:["node:*","bun:*"],logLevel:"silent",plugins:[{name:"light-entries",setup(builder){builder.onResolve({filter:/.*/},args=>args.path.includes("⌨️command")?{path:args.path,external:true}:undefined);}}]});
 const inputs=Object.keys(result.metafile.inputs).filter(path=>!path.endsWith(".json")).map(path=>resolve(process.cwd(),path)).sort();
 expect(traced).toEqual([...new Set(inputs)].sort());
});

test("the Cargo execution contract the build caller loads lazily still resolves from the caller and exports its issuer, and the caller keeps no static import of it",()=>{
 const caller=resolve(import.meta.dir,"../../../📥️caller/🟦️.ts"),source=readFileSync(caller,"utf8"),literals=[...source.matchAll(/createRequire\(import\.meta\.url\)\("([^"]+)"\)/gu)].map(match=>match[1]!);
 expect(literals.length).toBe(1);
 expect(/^import[^\n]*from\s*["'][^"']*🦀️cargo\/🏃️execution\//mu.test(source)).toBe(false);
 const loaded=createRequire(caller)(resolve(dirname(caller),literals[0]!)) as {issueRepositoryCargoExecutionBackendV1?:unknown};
 expect(typeof loaded.issueRepositoryCargoExecutionBackendV1).toBe("function");
});

test("the performance table states the startup objective this front door is held to",()=>{
 const table=resolve(import.meta.dir,"../../../📏️incremental/🏎️bench/🎯️slo/🔣️.json");
 if(!existsSync(table))return;
 const {slos,probes}=JSON.parse(readFileSync(table,"utf8")) as {slos:{id:string;probe:string;statistic:string;goal:number|null;gate:number|null;enforcement:string;tools:string[]}[];probes:{id:string;kind:string}[]};
 const row=slos.find(candidate=>candidate.id==="cli-help-p50");
 expect(row).toBeDefined();
 expect([row!.statistic,row!.tools,row!.enforcement]).toEqual(["p50",["neutral"],"blocking"]);
 expect(probes.find(candidate=>candidate.id===row!.probe)?.kind).toBe("cli-help");
 expect(row!.goal!).toBeLessThanOrEqual(100);
 expect(row!.gate!).toBeLessThan(1000);
});
