/** 🧷️ Contract: a process that holds an obsolete implementation (its declared format stamps differ from the stamps declared on disk now) yields every exclusive resource of a producer without producing, readers refuse the newer artifact with the same EN/DE message, and the outermost command re-executes once. */
import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import {fileURLToPath,pathToFileURL} from "node:url";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {acquireQueuedResourceLease,acquireResourceLease} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {createFormatStampsV1,exitIfStaleImplementationV1,formatStampLanguagesV1,formatStampProtocolV1,formatStampTextV1,formatStampsV1,registerFormatStampGuardV1,renderStaleImplementationV1,staleImplementationOfV1,StaleImplementationErrorV1} from "../🟨️.mjs";
import {runWithStaleReexecV1} from "../🔄️reexec/🟨️.mjs";
import {staleImplementationV1} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";

const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"🐍️.py"),schemaPath=join(here,"..","🧬️schema","🔣️.json");
const library=resolve(here,"../../.."),protocol=formatStampProtocolV1(),scratches:string[]=[];
const scratch=()=>{const path=mkdtempSync(join(realpathSync(tmpdir()),"fs-"));scratches.push(path);return path;};
afterAll(()=>{for(const path of scratches)rmSync(path,{recursive:true,force:true});});
const ask=(...args:string[])=>JSON.parse(Bun.spawnSync([python!,"-I",oracle,...args]).stdout.toString());
const writeAtomic=(path:string,value:unknown)=>{const temporary=path+".tmp";writeFileSync(temporary,JSON.stringify(value));renameSync(temporary,path);};
const readJson=(path:string)=>JSON.parse(readFileSync(path,"utf8"));
let counter=0;
const lease=(directory:string,resource:string,mode:"shared"|"exclusive"="exclusive",timeoutMs?:number)=>acquireQueuedResourceLease({directory,resource,mode,owner:"t"+process.pid+"-"+(counter++),signal:new AbortController().signal,...(timeoutMs===undefined?{}:{timeoutMs})});
const unguarded=(directory:string,resource:string)=>acquireResourceLease({directory,resource,mode:"exclusive",signal:new AbortController().signal,timeoutMs:0});

test("the declared table is closed under its schema and the fixture under its own, validated by an independent implementation",()=>{
 const ajv=new Ajv({strict:false}),entry=ajv.compile({$defs:schema.$defs,$ref:"#/$defs/Entry"}),binding=ajv.compile({$defs:schema.$defs,$ref:"#/$defs/Binding"});
 for(const row of protocol.stamps)expect(entry(row),JSON.stringify(entry.errors)).toBe(true);
 for(const row of protocol.resources)expect(binding(row),JSON.stringify(binding.errors)).toBe(true);
 const ids=protocol.stamps.map(row=>row.id);
 expect(new Set(ids).size).toBe(ids.length);
 expect(new Set(protocol.resources.map(row=>row.prefix)).size).toBe(protocol.resources.length);
 for(const row of protocol.resources)for(const id of row.stamps)expect(ids,row.prefix+" binds "+id).toContain(id);
 expect(protocol.exitCode).toBe(75);expect(protocol.maximumReexecutions).toBe(6);
 expect(new Ajv({strict:true}).validate(fixtureSchema,fixture),"fixture").toBe(true);
});

test("every message exists in EN and DE with the same placeholders and a different text",()=>{
 const placeholders=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort().join();
 expect(Object.keys(protocol.texts.en).sort()).toEqual(Object.keys(protocol.texts.de).sort());
 for(const key of Object.keys(protocol.texts.en)){expect(placeholders(protocol.texts.en[key]!),key).toBe(placeholders(protocol.texts.de[key]!));expect(protocol.texts.en[key]).not.toBe(protocol.texts.de[key]);}
 expect(protocol.languages).toEqual(["en","de"]);
 expect(formatStampLanguagesV1({SEMIO_LANGUAGE:"de"})).toEqual(["de"]);expect(formatStampLanguagesV1({LANG:"en_US.UTF-8"})).toEqual(["en"]);expect(formatStampLanguagesV1({LANG:"C"})).toEqual(["en","de"]);expect(formatStampLanguagesV1({SEMIO_LANGUAGE:"fr",LANG:"de_CH.UTF-8"}),"an unoffered explicit choice chooses nothing").toEqual(["en","de"]);
 expect(()=>formatStampTextV1("fr","moved",{})).toThrow();
 expect(()=>formatStampTextV1("en","moved",{id:"x"})).toThrow("placeholder");
});

test("admission verdicts, moved rows and EN/DE reports equal the independent oracle on every fixture case",()=>{
 const expected=ask("cases",join(here,"🔣️.json"),schemaPath) as {name:string;admission:string;moved:unknown[];en?:string;de?:string}[];
 expect(expected.length).toBe(fixture.cases.length);
 fixture.cases.forEach((row,index)=>{
  const files=new Map(Object.entries({...fixture.files,...row.live}) as [string,unknown][]);
  const registry=createFormatStampsV1({base:"/fixture",table:{...protocol,...fixture.table} as never,environment:{},read:path=>{const document=files.get(path.slice("/fixture/".length));if(document===undefined)throw Error("absent");return document;}});
  const initial=fixture.files as Record<string,unknown>;
  for(const id of row.declared){const entry=fixture.table.stamps.find(stamp=>stamp.id===id)!;registry.declare(id,initial[entry.schema]);}
  let outcome:unknown;try{registry.guard({resource:row.resource,mode:row.mode as "shared"|"exclusive"});outcome=undefined;}catch(error){outcome=error;}
  const verdict=outcome===undefined?"pass":"stale";
  expect(verdict,row.name).toBe(row.admission);expect(expected[index]!.admission,row.name).toBe(row.admission);expect(expected[index]!.moved,row.name).toEqual(row.moved);
  if(outcome instanceof StaleImplementationErrorV1){
   expect(outcome.moved,row.name).toEqual(row.moved);expect(outcome.code).toBe("SEMIO_STALE_IMPLEMENTATION");
   const lines=outcome.message.split("\n");expect(lines.length).toBe(2);
   expect(lines[0],row.name).toBe(expected[index]!.en!);expect(lines[1],row.name).toBe(expected[index]!.de!);
   if(row.en!==undefined){expect(lines[0]).toBe(row.en);expect(lines[1]).toBe(row.de!);}
   if(row.hidden){expect(lines[0]!.split("moved from").length-1).toBe(row.shown);expect(lines[0]).toContain(row.enMore!);expect(lines[1]).toContain(row.deMore!);}
  }
 });
});

test("a reader report says it neither reads nor overwrites; one language when chosen, both when not",()=>{
 const rows=[{id:"cache.graph-image",own:"graph-records-v2",live:"graph-records-v3"}];
 expect(renderStaleImplementationV1(["en"],rows,"graph cache image","refuse")).toBe("This command runs an outdated implementation: the declared format stamp cache.graph-image moved from graph-records-v2 to graph-records-v3. It neither reads nor overwrites graph cache image.");
 expect(renderStaleImplementationV1(["de"],rows,"graph cache image","refuse")).toBe("Dieser Befehl führt eine veraltete Implementierung aus: der deklarierte Formatstempel cache.graph-image wechselte von graph-records-v2 zu graph-records-v3. Er liest und überschreibt graph cache image nicht.");
 expect(new StaleImplementationErrorV1(rows,{context:"graph cache image",kind:"refuse",languages:["en","de"]}).message.split("\n").length).toBe(2);
});

test("the live table resolves every declared stamp exactly as the independent oracle does",()=>{
 const live=ask("live",schemaPath,library) as Record<string,unknown>;
 expect(Object.keys(live).sort()).toEqual(protocol.stamps.map(row=>row.id).sort());
 for(const row of protocol.stamps)expect(formatStampsV1.live(row.id),row.id).toEqual(live[row.id]);
 expect(()=>formatStampsV1.live("no.such-stamp")).toThrow("No stamp");
});

test("every declared stamp has a declaring producer and every bound resource name occurs in the sources",()=>{
 const sources:string[]=[],walk=(directory:string)=>{for(const entry of readdirSync(directory,{withFileTypes:true})){if(entry.name==="node_modules"||entry.name.startsWith(".")||entry.name==="🧪️tests")continue;const path=join(directory,entry.name);if(entry.isDirectory())walk(path);else if(/\.(?:ts|mjs|json)$/u.test(entry.name))sources.push(path);}};
 walk(library);
 const declared=new Set<string>(),text=sources.map(path=>readFileSync(path,"utf8"));
 for(const body of text)for(const match of body.matchAll(/declareFormatStampV1\(\s*"([a-z][a-z0-9.-]*)"/gu))declared.add(match[1]!);
 expect([...declared].sort()).toEqual(protocol.stamps.map(row=>row.id).sort());
 for(const row of protocol.resources)expect(text.some(body=>body.includes(row.prefix)),row.prefix).toBe(true);
 for(const row of protocol.stamps)expect(existsSync(join(library,...row.schema.split("/"))),row.schema).toBe(true);
});

test("the wiring stays in place: lease admission and wait hook, owner declarations, readers that refuse, child exit code and the outermost restart",()=>{
 const text=(...parts:string[])=>readFileSync(join(library,...parts),"utf8"),leases=readFileSync(join(library,"../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts"),"utf8");
 expect(leases).toContain('Symbol.for("'+protocol.admissionGuardRegistry+'")');
 expect(leases).toContain('runAdmissionGuardsV1(options,"waiting")');expect(leases).toContain("return admitResourceLeaseV1(options,upgraded)");expect(leases).toContain("return admitResourceLeaseV1(options,await acquire())");expect(leases).toContain("return admitResourceLeaseV1(options,lease)");
 const graph=text("🏗️build","🕸️graph","🟦️.ts");
 expect(graph).toContain('declareFormatStampV1("graph.generation",generationSchemaV1)');expect(graph).toContain('requireCurrentFormatStampsV1(graphFormatStampsV1,"graph generation","refuse")');expect(graph).toContain('requireCurrentFormatStampsV1(graphFormatStampsV1,"graph generation");return discoverBuildGraphPartV1');
 expect(text("🏗️build","👁️observation","🟨️.mjs")).toContain('declareFormatStampV1("graph.record",sourceAnalysisSchemaV1)');
 expect(text("🏗️build","🕸️graph","📥️inference","🔖️stamps","🟨️.mjs")).toContain('declareFormatStampV1("graph.inferrers", schema)');
 expect(text("⚡️caching","🚀️bootstrap","📦️dependencies","🚦️concurrent","🟦️.ts")).toContain('declareFormatStampV1("dependency.walk",generationSchema)');
 const storage=text("⚡️caching","🚀️bootstrap","📦️dependencies","🧾️generation","💾️storage","🟦️.ts");
 expect(storage).toContain('requireCurrentFormatStampsV1(["dependency.walk"],"dependency generation receipt","refuse")');expect(storage).toContain("staleImplementationOfV1(error)");
 const format=text("⚡️caching","💾️storage","🔒️enforcement","📦️budget","📥️opening","🏪️store","📇️format","🟦️.ts");
const opening=text("⚡️caching","💾️storage","🔒️enforcement","📦️budget","📥️opening","🟦️.ts");expect(opening).toContain('declareFormatStampV1("native.publication",nativeStorageSchema)');expect(opening).toContain('export async function openRepositoryCachePartitionAttemptV1(');const recovering=text("⚡️caching","💾️storage","🔒️enforcement","📦️budget","📥️opening","♻️reset","🟦️.ts");expect(recovering).toContain('openRepositoryCachePartitionAttemptV1(workspace,id,control,()=>openRepositoryCachePartitionV1(workspace,"leases",control))');
 expect(format).toContain('declareFormatStampV1("cache.graph-image",schema)');expect(format).toContain('requireCurrentFormatStampsV1(["cache.graph-image"],"graph cache image","refuse")');
 expect(text("🏗️build","⌨️command","📥️entry","🟦️.ts")).toContain("exitIfStaleImplementationV1(error);throw error;}");
 const script=text("🏗️build","📜️script.ts");
 expect(text("🏗️build","🚪️door","🔄️restart","🟦️.ts")).toContain("outermost=environment[invocationEnvironmentV1]===undefined");expect(script).toContain("if(isMainModuleV1(import.meta.url)){");expect(script).toContain("runBuildMainV1(findWorkspaceRoot(import.meta.dirname)");
 expect(text("🏗️build","🕸️graph","📥️inference","🔖️stamps","🧬️schema","🔣️.json")).toContain('"🏗️build/🧷️format-stamps/"');
  const door=text("🏗️build","🚪️door","🏃️main","🟦️.ts"),restart=text("🏗️build","🚪️door","🔄️restart","🟦️.ts");expect(door).toContain("runBuildWithRestartV1(async()=>{const{runBuildCliV1}=await import(\"../../📜️script.ts\");");expect(restart).toContain("useProcessWideImplementationPinV1()");expect(restart).toContain("runWithStaleReexecV1(run,{environment,outermost,yielded:");expect(script).toContain("runBuildWithRestartV1(()=>runBuildCliV1(root,argv,environment),argv,environment)");expect(script).toContain("export async function runBuildCliWithRestartV1(");

});

test("an unreadable or incomplete table is an error that names the file in EN and DE, never a silent pass",()=>{
 const base=scratch(),table={...protocol,readAttempts:2,readRetryMilliseconds:1,stamps:[{id:"p.format",schema:"s.json",pointer:"/f/stamp"}],resources:[{prefix:"producer:",stamps:["p.format"]}]};
 writeFileSync(join(base,"s.json"),JSON.stringify({f:{stamp:"A"}}));
 const registry=createFormatStampsV1({base,table:table as never,environment:{}});registry.declare("p.format",{f:{stamp:"A"}});
 writeFileSync(join(base,"s.json"),"{ torn");
 expect(()=>registry.guard({resource:"producer:r",mode:"exclusive"})).toThrow("kann nicht gelesen werden");
 writeAtomic(join(base,"s.json"),{f:{}});
 expect(()=>registry.guard({resource:"producer:r",mode:"exclusive"})).toThrow("fehlt bei");
 rmSync(join(base,"s.json"));
 expect(()=>registry.guard({resource:"producer:r",mode:"exclusive"})).toThrow("cannot be read");
 expect(()=>registry.declare("unknown.stamp",{})).toThrow("No stamp");
});

test("a process captured with stamp A that is admitted after the live schema moved to B yields the lease and never publishes A",async()=>{
 const base=scratch(),directory=join(base,"leases");mkdirSync(directory);
 const table={...protocol,stamps:[{id:"p.format",schema:"s.json",pointer:"/f/stamp"}],resources:[{prefix:"producer:",stamps:["p.format"]}]};
 writeAtomic(join(base,"s.json"),{f:{stamp:"A"}});
 const registry=createFormatStampsV1({base,table:table as never,environment:{SEMIO_LANGUAGE:"en"}});
 registry.declare("p.format",readJson(join(base,"s.json")));
 const published:string[]=[],produce=async(resource:string)=>{const admitted=await lease(directory,resource);try{published.push(registry.owned().get("p.format") as string);}finally{admitted.release();}};
 const unregister=registerFormatStampGuardV1(registry);
 try{
  await produce("producer:graph");
  expect(published).toEqual(["A"]);
  writeAtomic(join(base,"s.json"),{f:{stamp:"B"}});
  const failure=await produce("producer:graph").then(()=>undefined,error=>error);
  expect(failure).toBeInstanceOf(StaleImplementationErrorV1);
  expect(failure.moved).toEqual([{id:"p.format",own:"A",live:"B"}]);
  expect(failure.message).toContain("producer:graph");
  expect(staleImplementationOfV1(new Error("wrapped",{cause:failure}))?.origin).toBe("error");
  expect(staleImplementationOfV1(new AggregateError([new Error("x"),failure]))?.moved).toEqual(failure.moved);
  expect(published).toEqual(["A"]);
  (await lease(directory,"producer:graph","shared")).release();
 }finally{unregister();}
 const free=await unguarded(directory,"producer:graph");free.release();
 writeAtomic(join(base,"s.json"),{f:{stamp:"A"}});
 const again=registerFormatStampGuardV1(registry);try{await produce("producer:graph");expect(published).toEqual(["A","A"]);}finally{again();}
});

test("a queued exclusive request that waits while the stamp moves withdraws its ticket, and the one at the head of the queue is refused at its admission",async()=>{
 const base=scratch(),directory=join(base,"leases");mkdirSync(directory);
 const table={...protocol,stamps:[{id:"p.format",schema:"s.json",pointer:"/f/stamp"}],resources:[{prefix:"producer:",stamps:["p.format"]}]};
 writeAtomic(join(base,"s.json"),{f:{stamp:"A"}});
 const registry=createFormatStampsV1({base,table:table as never,environment:{SEMIO_LANGUAGE:"de"}});registry.declare("p.format",readJson(join(base,"s.json")));
 const holder=await unguarded(directory,"producer:wait"),unregister=registerFormatStampGuardV1(registry);
 let head:Promise<unknown>|undefined;
 try{
  head=lease(directory,"producer:wait").then(()=>undefined,error=>error);
  await new Promise(accept=>setTimeout(accept,300));
  writeAtomic(join(base,"s.json"),{f:{stamp:"B"}});
  const waiting=await lease(directory,"producer:wait").then(()=>undefined,error=>error);
  expect(waiting).toBeInstanceOf(StaleImplementationErrorV1);
  expect(waiting.message).toContain("Dieser Befehl führt eine veraltete Implementierung aus");
  holder.release();
  const admitted=await head;
  expect(admitted).toBeInstanceOf(StaleImplementationErrorV1);
 }finally{unregister();try{holder.release();}catch{}await head;}
 const free=await lease(directory,"producer:wait",undefined,0);free.release();
});

test("the publication gate of a producer is guarded as well: a stamp that moves during the build stops the publish",async()=>{
 const base=scratch(),directory=join(base,"leases");mkdirSync(directory);
 const table={...protocol,stamps:[{id:"p.format",schema:"s.json",pointer:"/f/stamp"}],resources:[{prefix:"producer:",stamps:["p.format"]}]};
 writeAtomic(join(base,"s.json"),{f:{stamp:"A"}});
 const registry=createFormatStampsV1({base,table:table as never,environment:{}});registry.declare("p.format",readJson(join(base,"s.json")));
 const unregister=registerFormatStampGuardV1(registry),pointer:string[]=[];
 try{
  const failure=await (async()=>{const build=await lease(directory,"producer:build");try{writeAtomic(join(base,"s.json"),{f:{stamp:"B"}});const gate=await lease(directory,"producer:publish");try{pointer.push("A");}finally{gate.release();}}finally{build.release();}})().then(()=>undefined,error=>error);
  expect(failure).toBeInstanceOf(StaleImplementationErrorV1);expect(pointer).toEqual([]);
 }finally{unregister();}
 const free=await unguarded(directory,"producer:build");free.release();const gate=await unguarded(directory,"producer:publish");gate.release();
});

test("the outermost command re-executes once with the current implementation; nested commands pass the exit code on; the restart is bounded",async()=>{
 const stale=()=>new StaleImplementationErrorV1([{id:"x.y",own:"1",live:"2"}],{languages:["en"]});
 const exitFailure=()=>Object.assign(new Error("repository:command failed (75)"),{name:"OwnedCommandFailure",outcome:{version:1,status:75,signal:null}});
 const calls:{args:readonly string[];environment:Record<string,string|undefined>}[]=[],written:string[]=[];
 const launch=async(options:{execPath:string;args:readonly string[];environment:Record<string,string|undefined>})=>{calls.push({args:options.args,environment:options.environment});return 0;};
 const options={environment:{SEMIO_LANGUAGE:"en"},argv:["bun","/repo/script.ts","repo","show","targets"],execPath:"bun",execArgv:[],launch,write:(text:string)=>{written.push(text);}};
 expect(await runWithStaleReexecV1(async()=>{},{...options,outermost:true})).toEqual({status:"completed"});
 expect(await runWithStaleReexecV1(async()=>{throw stale();},{...options,outermost:true})).toEqual({status:"reexecuted",code:0});
 expect(calls.length).toBe(1);expect(calls[0]!.args).toEqual(["/repo/script.ts","repo","show","targets"]);expect(calls[0]!.environment.SEMIO_STALE_REEXEC).toBe("1");
 expect(written.join("")).toContain("Restarting with the current implementation (1 of "+protocol.maximumReexecutions+").");
 expect(await runWithStaleReexecV1(async()=>{throw exitFailure();},{...options,outermost:true})).toEqual({status:"reexecuted",code:0});expect(calls.length).toBe(2);
 expect(await runWithStaleReexecV1(async()=>{throw stale();},{...options,environment:{SEMIO_LANGUAGE:"de",SEMIO_STALE_REEXEC:String(protocol.maximumReexecutions)},outermost:true})).toEqual({status:"exhausted",code:75});
 expect(calls.length).toBe(2);expect(written.join("")).toContain("Die Stempel wechselten nach dem Neustart erneut");
 expect(await runWithStaleReexecV1(async()=>{throw stale();},{...options,outermost:false})).toEqual({status:"yielded",code:75});expect(calls.length).toBe(2);
 await expect(runWithStaleReexecV1(async()=>{throw new Error("ordinary");},{...options,outermost:true})).rejects.toThrow("ordinary");expect(calls.length).toBe(2);
 let exited:number|undefined;exitIfStaleImplementationV1(stale(),{write:()=>{},exit:code=>{exited=code;}});expect(exited).toBe(75);
 exited=undefined;exitIfStaleImplementationV1(new Error("ordinary"),{write:()=>{},exit:code=>{exited=code;}});expect(exited).toBeUndefined();
});

test("across real processes: the first run holds stamp A while the schema moves to B, exits 75 without publishing, the replacement publishes B once",async()=>{
 const base=scratch(),directory=join(base,"leases"),output=join(base,"published.txt"),child=join(base,"child.mjs");mkdirSync(directory);
 writeAtomic(join(base,"s.json"),{f:{stamp:"A"}});
 const table={...protocol,stamps:[{id:"p.format",schema:"s.json",pointer:"/f/stamp"}],resources:[{prefix:"producer:",stamps:["p.format"]}]};
 writeFileSync(join(base,"table.json"),JSON.stringify(table));
 writeFileSync(child,`import{appendFileSync,readFileSync,renameSync,writeFileSync}from"node:fs";
import{createFormatStampsV1,exitIfStaleImplementationV1,registerFormatStampGuardV1}from${JSON.stringify(pathToFileURL(join(library,"🏗️build","🧷️format-stamps","🟨️.mjs")).href)};
import{acquireResourceLease}from${JSON.stringify(pathToFileURL(join(dirname(dirname(dirname(dirname(library)))),"🔨️modules","🏃️process","🔒️leases","🟦️.ts")).href)};
const[base,directory,output]=process.argv.slice(2),attempt=process.env.SEMIO_STALE_REEXEC??"0";
const registry=createFormatStampsV1({base,table:JSON.parse(readFileSync(base+"/table.json","utf8")),environment:{SEMIO_LANGUAGE:"en"}});
registry.declare("p.format",JSON.parse(readFileSync(base+"/s.json","utf8")));
registerFormatStampGuardV1(registry);
if(attempt==="0"){writeFileSync(base+"/s.tmp",JSON.stringify({f:{stamp:"B"}}));renameSync(base+"/s.tmp",base+"/s.json");}
try{const held=await acquireResourceLease({directory,resource:"producer:graph",mode:"exclusive",signal:new AbortController().signal,timeoutMs:0});try{appendFileSync(output,registry.owned().get("p.format")+"\\n");}finally{held.release();}}
catch(error){exitIfStaleImplementationV1(error);throw error;}
`);
 const runChild=async()=>{const run=Bun.spawnSync([process.execPath,child,base,directory,output],{env:{...process.env,SEMIO_STALE_REEXEC:process.env.SEMIO_STALE_REEXEC},stdout:"pipe",stderr:"pipe"});captured.push(run.stderr.toString());if(run.exitCode!==0)throw Object.assign(new Error("repository:command failed ("+run.exitCode+")"),{name:"OwnedCommandFailure",outcome:{version:1,status:run.exitCode,signal:null}});};
 const captured:string[]=[],written:string[]=[];
 const outcome=await runWithStaleReexecV1(runChild,{environment:{...process.env,SEMIO_LANGUAGE:"en"},outermost:true,argv:[process.execPath,child,base,directory,output],execPath:process.execPath,execArgv:[],write:text=>{written.push(text);}});
 expect(outcome).toEqual({status:"reexecuted",code:0});
 expect(captured[0]).toContain("moved from A to B");
 expect(written.join("")).toContain("(1 of "+protocol.maximumReexecutions+")");
 expect(readFileSync(output,"utf8")).toBe("B\n");
},60000);

test("a reader whose own source changed on disk after it loaded it is a stale implementation: the outermost command starts again, bounded like a moved stamp",async()=>{
 const error=staleImplementationV1("Stale loaded Source revision reader; restart required");
 expect(staleImplementationOfV1(error)).toEqual({origin:"error",moved:[],message:error.message});
 const calls:{environment:Record<string,string|undefined>}[]=[],written:string[]=[];
 const options={environment:{SEMIO_LANGUAGE:"en"},argv:["bun","/repo/script.ts","repo","run","a:b"],execPath:"bun",execArgv:[],launch:async(row:{environment:Record<string,string|undefined>})=>{calls.push(row);return 0;},write:(text:string)=>{written.push(text);},outermost:true};
 expect(await runWithStaleReexecV1(async()=>{throw error;},options)).toEqual({status:"reexecuted",code:0});
 expect(calls[0]!.environment.SEMIO_STALE_REEXEC).toBe("1");expect(written.join("")).toContain("Stale loaded Source revision reader; restart required");expect(written.join("")).toContain("(1 of "+protocol.maximumReexecutions+")");
 expect(await runWithStaleReexecV1(async()=>{throw error;},{...options,environment:{SEMIO_LANGUAGE:"en",SEMIO_STALE_REEXEC:String(protocol.maximumReexecutions)}})).toEqual({status:"exhausted",code:75});
 expect(calls.length).toBe(1);
});
