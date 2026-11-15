/** 🎯️ Law: every project that applies the input precision rules records the facts of the rules file, whichever project of the same process asked first, so a record inferred by a farm worker in a fresh process holds the same rows as one inferred in the parent. While the conformance stamps of the engine and the package managers exist, the stamp rows stand for their code and no row names it; without them (a process that could not settle them) the engine, its schema and the manager sources are recorded as content again. Independent oracle: the sha256 Python computes for each file. */
import{expect,test}from"bun:test";
import{dirname,join,resolve}from"node:path";
import{fileURLToPath}from"node:url";
import{mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{observeDetachedV1}from"../../../../👁️observation/🟨️.mjs";
import{libraryBootstrap}from"../../🟨️.mjs";
import{readProjectInputPrecisionV1}from"../🟨️.mjs";

const here=dirname(fileURLToPath(import.meta.url)),library=resolve(here,"../../../../.."),python=testPythonV1();
const files=["🏗️build/📥️inputs/🎯️precision/🟦️.ts","🏗️build/📥️inputs/🎯️precision/🧬️schema/🔣️.json","🏗️build/📥️inputs/🎯️precision/🔣️.json"].map(path=>join(library,path));

const engine=files.slice(0,2),rules=files[2]!,managers=join(library,"⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers")+"/";
const rowsOf=async()=>{const{rows,layers}=await observeDetachedV1(async()=>readProjectInputPrecisionV1());return[...rows,...layers.flat()]as{kind:string;path:string;fact:{sha256?:string}}[];};

test("each of two projects in one process records the facts of the precision rules file it applies, equal to the independent digest, and no row for the code a conformance stamp stands for",async()=>{
 await libraryBootstrap;
 const expected=JSON.parse(Bun.spawnSync([python!,"-I","-c","import hashlib,json,sys;print(json.dumps([hashlib.sha256(open(p,'rb').read()).hexdigest() for p in sys.argv[1:]]))",rules]).stdout.toString());
 const project=async()=>{const all=await rowsOf(),{value}=await observeDetachedV1(async()=>readProjectInputPrecisionV1());return{value,digest:all.find(row=>row.kind==="file"&&row.path===rules)?.fact?.sha256,code:all.filter(row=>row.kind==="file"&&(engine.includes(row.path)||row.path.startsWith(managers))).map(row=>row.path)};};
 const first=await project(),second=await project();
 expect(first.value).toBeDefined();expect(second.value).toBe(first.value);
 expect([first.digest]).toEqual(expected);expect([second.digest]).toEqual(expected);
 expect(first.code).toEqual([]);expect(second.code).toEqual([]);
},60000);

test("a process that cannot settle the conformance stamps records the engine, its schema, the rules and every manager source as content, like before",async()=>{
 const directory=mkdtempSync(join(realpathSync(tmpdir()),"precision-fallback-")),script=join(directory,"child.mjs"),plugin=join(library,"🏗️build/🕸️graph/📥️inference/🟨️.mjs");
 try{
  writeFileSync(script,`const stamps=await import(${JSON.stringify(join(library,"🏗️build/🕸️graph/📥️inference/🔖️stamps/🟨️.mjs"))}),{observeDetachedV1}=await import(${JSON.stringify(join(library,"🏗️build/👁️observation/🟨️.mjs"))}),plugin=await import(${JSON.stringify(plugin)}),assembly=await import(${JSON.stringify(join(library,"🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs"))});await plugin.libraryBootstrap;const broken=async()=>{throw Error("broken");};await stamps.settleConformanceStampsV1({libraryRoot:${JSON.stringify(library)},directory:${JSON.stringify(join(directory,"records"))},runners:{"input-precision":broken,"package-managers":broken}});const{rows,layers}=await observeDetachedV1(async()=>assembly.readProjectInputPrecisionV1());console.log(JSON.stringify([...rows,...layers.flat()].filter(row=>row.kind==="file").map(row=>row.path)));`);
  const run=Bun.spawnSync([process.execPath,script],{cwd:directory,env:{...process.env,SEMIO_GRAPH_PHASES:""}});
  expect(run.exitCode,run.stderr.toString().slice(0,1500)).toBe(0);
  const paths=JSON.parse(run.stdout.toString().trim().split("\n").at(-1)!) as string[];
  for(const file of files)expect(paths,file).toContain(file);
  expect(paths.filter(path=>path.startsWith(managers)).length).toBeGreaterThanOrEqual(17);
 }finally{rmSync(directory,{recursive:true,force:true});}
},120000);
