import {expect,test} from "bun:test";
import Ajv from "ajv";
import {spawn,spawnSync} from "node:child_process";
import {appendFileSync,existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import {CATALOG_LANGUAGES_V1,placeholdersV1,textV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import catalogue from "../../⚙️operations/🔣️catalog.json" with {type:"json"};
import door from "../../🚪️door/🔣️.json" with {type:"json"};
import selection from "../../⌨️command/🧭️selection/🔣️.json" with {type:"json"};
import policy from "../🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import texts from "../📖️texts/🔣️.json" with {type:"json"};
import {landOverlayBaseV1,landSlotOverrunV1,landAreaOfV1,landArgumentsV1,landApplyOrderV1,landApplyV1,landBaseProblemsV1,landChangesetV1,landCheckoutV1,landDeleteV1,landGenerationV1,landHashV1,landOptionNamesV1,landRebaseV1,landSlotStateV1,readLandPolicyV1,validateLandDocumentV1,type LandPolicyV1} from "../🟦️.ts";
import {landBindDependenciesV1,landManifestFoldersV1,landMirrorDirectoriesV1,landFailureSignatureV1,landDanglingImportsV1,landDeclaredTestsV1,landTouchedCratesV1,landSpecifiersV1} from "../🔬️gates/🟦️.ts";
import {landResultProblemsV1,landRunV1,type LandFlowPortsV1,type LandResultV1} from "../🧭️flow/🟦️.ts";
import {renderLandResultV1,runLandCliV1} from "../⌨️command/🟦️.ts";

const checkout=(()=>{let directory=import.meta.dir;while(!existsSync(join(directory,"MODULE.bazel"))&&dirname(directory)!==directory)directory=dirname(directory);return directory;})();
const ajv=new Ajv({strict:false,allowUnionTypes:true}),validator=(name:string)=>ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${name}`});
const scratch=(run:(directory:string)=>void|Promise<void>)=>async()=>{const directory=mkdtempSync(join(tmpdir(),"semio-land-"));try{await run(directory);}finally{rmSync(directory,{recursive:true,force:true});}};
const write=(directory:string,file:string,text:string)=>{mkdirSync(dirname(join(directory,file)),{recursive:true});writeFileSync(join(directory,file),text);};
const read=(directory:string,file:string)=>readFileSync(join(directory,file),"utf8");
const sleepSync=(milliseconds:number)=>{Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,milliseconds);};

const mini=(directory:string,overrides:Partial<Record<string,unknown>>={}):{root:string;limits:LandPolicyV1;cache:string;stage:string}=>{
 const root=join(directory,"live"),cache=join(directory,"cache"),stage=join(directory,"stage");
 mkdirSync(join(root,"locks"),{recursive:true});
 write(root,"tsconfig.json",JSON.stringify({compilerOptions:{target:"ESNext",module:"ESNext",moduleResolution:"bundler",strict:true,noEmit:true,skipLibCheck:true,allowImportingTsExtensions:true,types:[]}}));
 write(root,"package.json","{}");
 symlinkSync(resolve(checkout,"node_modules"),join(root,"node_modules"),"junction");
 write(root,"landings.md","# landings\n");
 const limits={...policy,areas:[{prefix:"lib",area:"derive",slot:true}],locks:{...policy.locks,root:"locks",waitMilliseconds:50,pollMilliseconds:1},landings:{...policy.landings,file:"../landings.md"},overlay:{...policy.overlay,roots:["lib"],rustRoots:[],rootFiles:["tsconfig.json","package.json"],links:["node_modules"]},...overrides} as unknown as LandPolicyV1;
 return {root,limits,cache,stage};
};
const stub:LandFlowPortsV1={bench:async()=>({passed:true,lines:[]}),typecheck:async()=>({passed:true,lines:[]}),tests:async()=>({passed:true,lines:[]}),rust:async()=>({passed:true,lines:[]})};
const request=(stage:string,options:Partial<ReturnType<typeof landArgumentsV1>>={},mode:"land"|"check"|"rollback"="land",id?:string)=>({mode,directory:stage,id,options:{...landArgumentsV1(["x"]),lane:"T1",title:"Test landing",wave:"Wave T",...options},language:"en" as const,environment:{} as NodeJS.ProcessEnv});
const stageEdit=(env:{root:string;stage:string;limits:LandPolicyV1},edits:Record<string,string|null>)=>{
 for(const [file,content] of Object.entries(edits)){
  if(content===null){landDeleteV1(env.root,env.stage,[file],env.root,env.limits);continue;}
  landCheckoutV1(env.root,env.stage,[file],{lane:"T1",title:"Test landing"},env.root,env.limits);
  write(join(env.stage,"tree"),file,content);
 }
};
const inSlot=()=>new Date(2026,10,27,12,3,0).getTime(),outOfSlot=()=>new Date(2026,10,27,12,30,0).getTime();

test("the policy, the stage and the result satisfy their closed schema under the in-repo validator and under Ajv, and both reject the same mutations",()=>{
 const check=validator("Policy");
 expect(check(policy),JSON.stringify(check.errors?.[0])).toBe(true);
 expect(validateLandDocumentV1("Policy",policy)).toEqual([]);
 expect(readLandPolicyV1()).toBe(policy as never);
 const mutations:Record<string,(value:any)=>void>={extra:value=>{value.extra=1;},noAreas:value=>{delete value.areas;},badMinute:value=>{value.slot.startMinute=60;},badLimit:value=>{value.limits.maximumEntries=0;},badArea:value=>{value.fallbackArea="Nope";},badOffload:value=>{value.gates.offloadEnvironment.X=1;}};
 for(const [name,mutate] of Object.entries(mutations)){const copy=JSON.parse(JSON.stringify(policy));mutate(copy);expect(check(copy),name).toBe(false);expect(validateLandDocumentV1("Policy",copy).length,name).toBeGreaterThan(0);}
 const stage={version:1,id:"a1",lane:"L",title:"t",bases:{"a/b.ts":"0".repeat(64),"c.ts":null},deletes:[]};
 expect(validator("Stage")(stage)).toBe(true);expect(validateLandDocumentV1("Stage",stage)).toEqual([]);
 for(const bad of [{...stage,id:"-x"},{...stage,bases:{"a":"zz"}},{...stage,extra:1}]){expect(validator("Stage")(bad)).toBe(false);expect(validateLandDocumentV1("Stage",bad).length).toBeGreaterThan(0);}
});

test("every message exists in English and German with the same placeholders, and the usage names every mode",()=>{
 for(const [key,leaf] of Object.entries(texts.messages)){
  expect([...placeholdersV1(leaf.en)].sort(),key).toEqual([...placeholdersV1(leaf.de)].sort());
  expect(leaf.en.length&&leaf.de.length,key).toBeGreaterThan(0);
 }
 expect([...CATALOG_LANGUAGES_V1]).toEqual(["en","de"]);
 const usage=textV1(texts as never,"usage","de");
 for(const word of ["checkout","delete","check","apply","rollback","status"])expect(usage).toContain(word);
 expect(()=>landArgumentsV1(["--nope","x"])).toThrow();
 expect(()=>landArgumentsV1(["--lane"])).toThrow();
 expect(landArgumentsV1(["--lane=a","--outage","--","dir"])).toMatchObject({lane:"a",outage:true,words:["dir"]});
});

test("the operations catalogue, the front door and the command selection declare land exactly as the parser accepts it",()=>{
 const operation=(catalogue as any).operations.find((row:any)=>row.name==="land");
 expect(operation,"operation land").toBeDefined();
 const names=landOptionNamesV1(),declared=(catalogue as any).flags.filter((flag:any)=>flag.operations?.includes("land")).map((flag:any)=>flag.name);
 for(const name of [...names.valued,...names.plain])expect(operation.flags.includes(name)||name==="--lang","flag "+name).toBe(true);
 for(const name of operation.flags)expect(declared.includes(name)||(catalogue as any).flags.some((flag:any)=>flag.name===name&&!flag.operations),name).toBe(true);
 expect(door.light.find((row:any)=>row.operation==="land")).toMatchObject({export:"runLandCliV1"});
 expect(selection.graph.freeVerbs).toContain("land");
});

test("an area is the first folder below a build or caching root, tests and documents are never slot-bound, manifests always are",()=>{
 const B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",C="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/";
 expect(landAreaOfV1(B+"🕸️graph/🟦️.ts")).toEqual({area:"graph",slot:true});
 expect(landAreaOfV1(B+"🌐️remote-execution/🚀️offload/x.ts")).toEqual({area:"remote-execution",slot:true});
 expect(landAreaOfV1(C+"🦀️cargo/🟦️.ts")).toEqual({area:"caching-cargo",slot:true});
 expect(landAreaOfV1("🧰️framework/🔨️modules/🏃️process/🚦️ready/🟦️.ts")).toEqual({area:"process",slot:true});
 expect(landAreaOfV1(B+"🕸️graph/🧪️tests/🟦️.ts")).toEqual({area:"graph",slot:false});
 expect(landAreaOfV1(B+"🕸️graph/README.md").slot).toBe(false);
 expect(landAreaOfV1("some/Cargo.toml")).toEqual({area:"other",slot:true});
 expect(landAreaOfV1("some/other/file.ts")).toEqual({area:"other",slot:false});
});

test("the slot is open in minutes 0 to 10 and names the next opening otherwise",()=>{
 expect(landSlotStateV1(new Date(2026,10,27,12,3,0))).toMatchObject({open:true,next:"now"});
 expect(landSlotStateV1(new Date(2026,10,27,12,10,59))).toMatchObject({open:true});
 expect(landSlotStateV1(new Date(2026,10,27,12,11,0))).toMatchObject({open:false,next:"13:00"});
 expect(landSlotStateV1(new Date(2026,10,27,23,40,0))).toMatchObject({open:false,next:"00:00"});
});

test("a landing that began in the slot may finish in the overrun but not one that began outside it",()=>{
 const at=(minute:number)=>new Date(2026,10,27,12,minute,0).getTime();
 expect(landSlotOverrunV1(at(5),at(24))).toBe(true);
 expect(landSlotOverrunV1(at(5),at(27))).toBe(false);
 expect(landSlotOverrunV1(at(20),at(21))).toBe(false);
});

test("staging records the base, drops unchanged files and carries new files and deletions",scratch(directory=>{
 const env=mini(directory);
 write(env.root,"lib/a.ts","export const a=1;\n");write(env.root,"lib/b.ts","export const b=1;\n");write(env.root,"lib/c.ts","export const c=1;\n");
 landCheckoutV1(env.root,env.stage,["lib/a.ts","lib/b.ts","lib/new.ts"],{lane:"L",title:"T"},env.root,env.limits);
 write(join(env.stage,"tree"),"lib/a.ts","export const a=2;\n");write(join(env.stage,"tree"),"lib/new.ts","export const n=1;\n");
 landDeleteV1(env.root,env.stage,["lib/c.ts"],env.root,env.limits);
 const changeset=landChangesetV1(env.stage,env.limits);
 expect(changeset.entries.map(entry=>[entry.path,entry.op,entry.base===null])).toEqual([["lib/a.ts","write",false],["lib/c.ts","delete",false],["lib/new.ts","write",true]]);
 expect(changeset.entries[0]!.base).toBe(landHashV1(new TextEncoder().encode("export const a=1;\n")));
 expect(landBaseProblemsV1(env.root,changeset)).toEqual([]);
 expect(()=>landCheckoutV1(env.root,env.stage,["../outside.ts"],{lane:"L",title:"T"},env.root,env.limits)).toThrow();
 expect(()=>landCheckoutV1(env.root,env.stage,["node_modules/x.ts"],{lane:"L",title:"T"},env.root,env.limits)).toThrow();
}));

test("a stale base is refused and nothing is applied",scratch(async directory=>{
 const env=mini(directory);
 write(env.root,"lib/a.ts","export const a=1;\n");
 stageEdit(env,{"lib/a.ts":"export const a=2;\n"});
 write(env.root,"lib/a.ts","export const a=99;\n");
 const result=await landRunV1(env.root,request(env.stage,{outage:true}),{...stub,cacheDirectory:env.cache,now:inSlot},env.limits);
 expect(result.status).toBe("refused");expect(result.reason).toContain("lib/a.ts");
 expect(read(env.root,"lib/a.ts")).toBe("export const a=99;\n");
 expect(read(env.root,"landings.md")).toBe("# landings\n");
 expect(readdirSync(join(env.root,"locks"))).toEqual([]);
 expect(landResultProblemsV1(result)).toEqual([]);expect(validator("Result")(result)).toBe(true);
}));

test("a new file that appeared and a deletion of a changed file are refused too",scratch(async directory=>{
 const env=mini(directory);
 write(env.root,"lib/d.ts","export const d=1;\n");
 stageEdit(env,{"lib/new.ts":"export const n=1;\n","lib/d.ts":null});
 write(env.root,"lib/new.ts","export const other=1;\n");write(env.root,"lib/d.ts","export const d=2;\n");
 const result=await landRunV1(env.root,request(env.stage,{outage:true}),{...stub,cacheDirectory:env.cache,now:inSlot},env.limits);
 expect(result.status).toBe("refused");expect(result.reason).toContain("lib/new.ts");expect(result.reason).toContain("lib/d.ts");
 expect(read(env.root,"lib/new.ts")).toBe("export const other=1;\n");
}));

test("a type error in the overlay refuses the landing with the real compiler and leaves the live tree untouched",scratch(async directory=>{
 const env=mini(directory),cacheDirectory=join(directory,"typecheck-cache");
 process.env.SEMIO_TYPECHECK_CACHE=cacheDirectory;
 try{
  write(env.root,"lib/a.ts","export const a:number=1;\n");write(env.root,"lib/user.ts","import {a} from \"./a.ts\";\nexport const u:number=a;\n");
  stageEdit(env,{"lib/a.ts":"export const a:number=\"one\";\n"});
  const red=await landRunV1(env.root,request(env.stage,{outage:true}),{cacheDirectory:env.cache,now:inSlot,tests:stub.tests,rust:stub.rust},env.limits);
  expect(red.status).toBe("failed");
  const gate=red.gates.find(row=>row.id==="typecheck")!;
  expect(gate.status).toBe("failed");expect(gate.detail.join("\n")).toContain("lib/a.ts");
  expect(read(env.root,"lib/a.ts")).toBe("export const a:number=1;\n");
  expect(read(env.root,"landings.md")).toBe("# landings\n");
  expect(readdirSync(join(env.root,"locks"))).toEqual([]);
  write(join(env.stage,"tree"),"lib/a.ts","export const a:number=2;\n");
  const green=await landRunV1(env.root,request(env.stage,{outage:true}),{cacheDirectory:env.cache,now:inSlot,tests:stub.tests,rust:stub.rust},env.limits);
  expect(green.status,JSON.stringify(green.gates)).toBe("landed");
  expect(green.gates.find(row=>row.id==="typecheck")!.status).toBe("passed");
  expect(read(env.root,"lib/a.ts")).toBe("export const a:number=2;\n");
 }finally{delete process.env.SEMIO_TYPECHECK_CACHE;}
}),180000);

test("a temporary debug line in a written file under a checked root refuses the landing; --bench runs the latency check and its breach refuses too",scratch(async directory=>{
 const env=mini(directory),marker="[DEB"+"UG] ";
 write(env.root,"lib/a.ts","export const a=1;\n");
 stageEdit(env,{"lib/a.ts":"export const a=2;\nconsole.log(\""+marker+"x\");\n"});
 const red=await landRunV1(env.root,request(env.stage,{outage:true}),{...stub,cacheDirectory:env.cache,now:inSlot},env.limits);
 expect(red.status).toBe("failed");expect(red.gates.find(row=>row.id==="debug-lines")!.detail.join("\n")).toContain("lib/a.ts:2");
 expect(read(env.root,"lib/a.ts")).toBe("export const a=1;\n");
 write(join(env.stage,"tree"),"lib/a.ts","export const a=2;\n");
 let asked=0;
 const skipped=await landRunV1(env.root,request(env.stage,{},"check"),{...stub,bench:async()=>{asked++;return {passed:false,lines:["breach"]};},cacheDirectory:env.cache,now:inSlot},env.limits);
 expect(skipped.status).toBe("checked");expect(asked).toBe(0);expect(skipped.gates.find(row=>row.id==="bench")!.status).toBe("skipped");
 const breach=await landRunV1(env.root,request(env.stage,{bench:true,outage:true}),{...stub,bench:async()=>{asked++;return {passed:false,lines:["fleet-status: breach"]};},cacheDirectory:env.cache,now:inSlot},env.limits);
 expect(breach.status).toBe("failed");expect(asked).toBe(1);expect(read(env.root,"lib/a.ts")).toBe("export const a=1;\n");
}));

test("a test that fails in the overlay exactly as it fails on the live tree is reported as pre-existing and does not refuse; a new or different failure does",scratch(async directory=>{
 const env=mini(directory);
 write(env.root,"lib/a.ts","export const a=1;\n");write(env.root,"lib/🧪️tests/🟦️.ts","x");
 stageEdit(env,{"lib/a.ts":"export const a=2;\n"});
 const failing=(signature:string)=>async()=>({passed:false,lines:["lib/🧪️tests/🟦️.ts: failed"],failures:[{file:"lib/🧪️tests/🟦️.ts",signature}]});
 const calls:string[]=[];
 const same=await landRunV1(env.root,request(env.stage,{outage:true}),{...stub,tests:async req=>{calls.push(req.overlay.endsWith("live")?"live":"overlay");return failing("red [1.2s]".replace(/\[.*\]/,"")+(req.overlay.includes("overlay")?"":""))();},cacheDirectory:env.cache,now:inSlot},env.limits);
 expect(same.status).toBe("landed");expect(calls).toEqual(["overlay","live"]);
 expect(same.gates.find(row=>row.id==="tests")!.detail.join("\n")).toContain("pre-existing");
 write(env.root,"lib/a.ts","export const a=1;\n");
 stageEdit(env,{"lib/a.ts":"export const a=3;\n"});
 const different=await landRunV1(env.root,request(env.stage,{outage:true}),{...stub,tests:async req=>failing(req.overlay.endsWith("live")?"old failure":"new failure")(),cacheDirectory:env.cache,now:inSlot},env.limits);
 expect(different.status).toBe("failed");
}));

test("a failure signature names the failing tests and ignores roots, timings and runner noise",()=>{
 const one="x /ov/a.ts: (fail) the law holds [1.2s] | [script] build:x remains active (shared, 29 s)",two="x /live/a.ts: (fail) the law holds [88.7ms]\n[budget] busy";
 expect(landFailureSignatureV1(one,["/ov"])).toBe(landFailureSignatureV1(two,["/live"]));
 expect(landFailureSignatureV1(one,[])).toBe("failed: the law holds");
 expect(landFailureSignatureV1("boom 12 s at /ov/x",["/ov"])).toBe(landFailureSignatureV1("boom 9 s at /live/x",["/live"]));
 expect(landFailureSignatureV1("(fail) a\n(fail) b",[])).not.toBe(landFailureSignatureV1("(fail) a",[]));
});

test("a dangling import and an importer of a deleted file are refused before anything lands",scratch(async directory=>{
 const env=mini(directory);
 write(env.root,"lib/a.ts","export const a=1;\n");write(env.root,"lib/user.ts","import {a} from \"./a.ts\";\nexport const u=a;\n");
 stageEdit(env,{"lib/new.ts":"import {x} from \"./missing.ts\";\nexport const n=x;\n"});
 const first=await landRunV1(env.root,request(env.stage,{outage:true}),{...stub,cacheDirectory:env.cache,now:inSlot},env.limits);
 expect(first.status).toBe("failed");expect(first.gates.find(row=>row.id==="dangling-imports")!.detail.join("\n")).toContain("./missing.ts");
 expect(existsSync(join(env.root,"lib/new.ts"))).toBe(false);
 const second=mini(join(directory,"second"));
 write(second.root,"lib/a.ts","export const a=1;\n");write(second.root,"lib/user.ts","import {a} from \"./a.ts\";\nexport const u=a;\n");
 stageEdit(second,{"lib/a.ts":null});
 const deleted=await landRunV1(second.root,request(second.stage,{outage:true}),{...stub,cacheDirectory:second.cache,now:inSlot},second.limits);
 expect(deleted.status).toBe("failed");expect(deleted.gates.find(row=>row.id==="dangling-imports")!.detail.join("\n")).toContain("lib/user.ts");
 expect(existsSync(join(second.root,"lib/a.ts"))).toBe(true);
}));

test("the dangling-import scan agrees with the coordinator's Python script on a corpus of tricky sources",scratch(directory=>{
 const python=spawnSync("python3",["--version"],{encoding:"utf8"});
 const script=join(import.meta.dir,"🐍️dangling-imports.py");
 write(directory,"src/ok.ts","export const ok=1;\n");
 write(directory,"src/tricky.ts",["import {ok} from \"./ok.ts\";","import \"./missing-side.ts\";","export * from \"../up/missing-up.ts\";","const lazy=()=>import(\"./lazy-missing.ts\");","const text=\"import x from './in-string.ts'\";","// import y from './in-comment.ts'","const tpl=`${ok} import z from './in-template.ts'`;","const re=/import w from '.\\/in-regex.ts'/;","const dyn=require(\"./req-missing\");","const json=await import(\"./ok.js\");"].join("\n"));
 const ours=landDanglingImportsV1(directory,["src/ok.ts","src/tricky.ts"]).map(row=>row.file+" -> "+row.specifier).sort();
 expect(ours).toEqual(["src/tricky.ts -> ../up/missing-up.ts","src/tricky.ts -> ./lazy-missing.ts","src/tricky.ts -> ./missing-side.ts","src/tricky.ts -> ./req-missing"]);
 expect(landSpecifiersV1("import a from \"./a.ts\";export {b} from './b';x.other('./c')")).toEqual(["./a.ts","./b"]);
 if(python.status!==0)return;
 const run=spawnSync("python3",["-I",script,join(directory,"src")],{encoding:"utf8"});
 const theirs=run.stdout.split("\n").filter(Boolean).map(line=>{const [file,specifier]=line.split(" -> ");return "src/"+file!.split("/src/")[1]+" -> "+specifier;}).sort();
 expect(theirs).toEqual(ours);
}));

test("a landing in the slot applies atomically, records a rollback point, appends the landings entry and releases the locks",scratch(async directory=>{
 const env=mini(directory);
 write(env.root,"lib/graph/a.ts","import {b} from \"./b.ts\";\nexport const a=b;\n");write(env.root,"lib/graph/b.ts","export const b=1;\n");
 stageEdit(env,{"lib/graph/b.ts":"export const b=2;\nexport const c=3;\n","lib/graph/new.ts":"export const n=1;\n"});
 const notices:string[]=[],result=await landRunV1(env.root,request(env.stage),{...stub,cacheDirectory:env.cache,now:inSlot,notice:line=>notices.push(line)},env.limits);
 expect(result.status,result.reason).toBe("landed");expect(result.areas).toEqual(["graph"]);expect(result.slot).toMatchObject({bound:true,open:true});
 expect(read(env.root,"lib/graph/b.ts")).toBe("export const b=2;\nexport const c=3;\n");expect(existsSync(join(env.root,"lib/graph/new.ts"))).toBe(true);
 expect(readdirSync(join(env.root,"locks")).filter(name=>name.startsWith("landing.lock."))).toEqual([]);
 expect(landGenerationV1(join(env.root,"locks"),"graph",env.limits)%2).toBe(0);expect(landGenerationV1(join(env.root,"locks"),"graph",env.limits)).toBeGreaterThan(0);
 const entry=read(env.root,"landings.md");
 expect(entry).toContain("## Wave T T1: Test landing");expect(entry).toContain(result.rollback);expect(entry).toContain("graph");expect(entry.startsWith("# landings\n")).toBe(true);
 expect(notices.some(line=>line.includes("Applying 2 change"))).toBe(true);
 expect(existsSync(join(env.cache,"rollback",result.rollback,"rollback.json"))).toBe(true);
 expect(validator("RollbackPoint")(JSON.parse(read(join(env.cache,"rollback",result.rollback),"rollback.json")))).toBe(true);
 const back=await landRunV1(env.root,request(env.stage,{},"rollback",result.rollback),{...stub,cacheDirectory:env.cache,now:inSlot},env.limits);
 expect(back.status,back.reason).toBe("rolled-back");
 expect(read(env.root,"lib/graph/b.ts")).toBe("export const b=1;\n");expect(existsSync(join(env.root,"lib/graph/new.ts"))).toBe(false);
 expect(read(env.root,"landings.md")).toContain("Rollback of Test landing");
}));

test("a rollback refuses when a landed file changed afterwards",scratch(async directory=>{
 const env=mini(directory);
 write(env.root,"lib/a.ts","export const a=1;\n");
 stageEdit(env,{"lib/a.ts":"export const a=2;\n"});
 const landed=await landRunV1(env.root,request(env.stage,{outage:true}),{...stub,cacheDirectory:env.cache,now:inSlot},env.limits);
 write(env.root,"lib/a.ts","export const a=3;\n");
 await expect(landRunV1(env.root,request(env.stage,{},"rollback",landed.rollback),{...stub,cacheDirectory:env.cache,now:inSlot},env.limits)).rejects.toMatchObject({key:"land.rollback.diverged"});
 expect(read(env.root,"lib/a.ts")).toBe("export const a=3;\n");
}));

test("outside the slot a slot-bound landing is queued without --outage, lands with it, and test-only files land any time",scratch(async directory=>{
 const env=mini(directory);
 write(env.root,"lib/a.ts","export const a=1;\n");write(env.root,"lib/🧪️tests/t.ts","export const t=1;\n");
 stageEdit(env,{"lib/a.ts":"export const a=2;\n"});
 const queued=await landRunV1(env.root,request(env.stage,{noWait:true}),{...stub,cacheDirectory:env.cache,now:outOfSlot},env.limits);
 expect(queued.status).toBe("queued");expect(queued.slot).toMatchObject({bound:true,open:false,next:"13:00"});
 expect(read(env.root,"lib/a.ts")).toBe("export const a=1;\n");expect(read(env.root,"locks/landing.slot-queue")).toContain("T1");
 const outage=await landRunV1(env.root,request(env.stage,{outage:true}),{...stub,cacheDirectory:env.cache,now:outOfSlot},env.limits);
 expect(outage.status).toBe("landed");expect(read(env.root,"lib/a.ts")).toBe("export const a=2;\n");
 const second=mini(join(directory,"second"));
 write(second.root,"lib/🧪️tests/t.ts","export const t=1;\n");
 stageEdit(second,{"lib/🧪️tests/t.ts":"export const t=2;\n"});
 const test_only=await landRunV1(second.root,request(second.stage),{...stub,cacheDirectory:second.cache,now:outOfSlot},second.limits);
 expect(test_only.status).toBe("landed");expect(test_only.slot.bound).toBe(false);
}));

test("without --no-wait the landing waits for the slot, checks again once it is open and then lands",scratch(async directory=>{
 const env=mini(directory);
 write(env.root,"lib/a.ts","export const a=1;\n");
 stageEdit(env,{"lib/a.ts":"export const a=2;\n"});
 let clock=outOfSlot(),checks=0;
 const sleeps:number[]=[],notices:string[]=[];
 const result=await landRunV1(env.root,request(env.stage),{...stub,typecheck:async()=>{checks++;return {passed:true,lines:[]};},cacheDirectory:env.cache,now:()=>clock,sleep:async milliseconds=>{sleeps.push(milliseconds);clock+=milliseconds;},notice:line=>notices.push(line)},env.limits);
 expect(result.status,result.reason).toBe("landed");expect(sleeps.length).toBeGreaterThan(1);expect(checks).toBe(2);
 expect(notices.some(line=>line.includes("Waiting for the landing slot"))).toBe(true);expect(notices.some(line=>line.includes("slot is open"))).toBe(true);
 expect(new Date(clock).getMinutes()).toBeLessThanOrEqual(10);
}));

test("a held lock is waited for, queued and then refused naming its owner; a stale lock is reported, never stolen; locks are taken in alphabetical order",scratch(async directory=>{
 const env=mini(directory);
 write(env.root,"lib/a/x.ts","export const x=1;\n");write(env.root,"lib/b/y.ts","export const y=1;\n");
 stageEdit(env,{"lib/a/x.ts":"export const x=2;\n","lib/b/y.ts":"export const y=2;\n"});
 mkdirSync(join(env.root,"locks","landing.lock.b"));write(env.root,"locks/landing.lock.b/owner","H99 12:00:00\n");
 let clock=Date.now();
 const busy=landRunV1(env.root,request(env.stage,{outage:true}),{...stub,cacheDirectory:env.cache,now:()=>clock,sleep:async milliseconds=>{clock+=milliseconds;}},env.limits);
 await expect(busy).rejects.toMatchObject({key:"land.lock.busy",values:{area:"b",owner:"H99 12:00:00"}});
 expect(readdirSync(join(env.root,"locks")).sort()).toEqual(["landing.lock.b","landing.queue.b"]);
 expect(read(env.root,"locks/landing.queue.b")).toContain("T1 land");expect(read(env.root,"lib/a/x.ts")).toBe("export const x=1;\n");
 await expect(landRunV1(env.root,request(env.stage,{outage:true}),{...stub,cacheDirectory:env.cache,now:()=>Date.now()+30*60000,sleep:async()=>{}},env.limits)).rejects.toMatchObject({key:"land.lock.stale"});
 expect(readdirSync(join(env.root,"locks")).sort()).toEqual(["landing.lock.b","landing.queue.b"]);
}));

test("cancelling before the apply applies nothing and leaves no lock",scratch(async directory=>{
 const env=mini(directory),controller=new AbortController();
 write(env.root,"lib/a.ts","export const a=1;\n");
 stageEdit(env,{"lib/a.ts":"export const a=2;\n"});
 const result=await landRunV1(env.root,request(env.stage,{outage:true}),{...stub,typecheck:async()=>{controller.abort();return {passed:true,lines:[]};},signal:controller.signal,cacheDirectory:env.cache,now:inSlot},env.limits);
 expect(result.status).toBe("refused");expect(read(env.root,"lib/a.ts")).toBe("export const a=1;\n");expect(readdirSync(join(env.root,"locks"))).toEqual([]);
}));

test("--rebase merges a non-overlapping live change three-way and refuses a conflicting one",scratch(async directory=>{
 if(spawnSync("git",["--version"]).status!==0)return;
 const env=mini(directory),original=Array.from({length:12},(_,index)=>"line "+index).join("\n")+"\n";
 write(env.root,"lib/a.txt",original);
 stageEdit(env,{"lib/a.txt":original.replace("line 1\n","line 1 mine\n")});
 write(env.root,"lib/a.txt",original.replace("line 10\n","line 10 theirs\n"));
 const refused=await landRunV1(env.root,request(env.stage,{outage:true}),{...stub,cacheDirectory:env.cache,now:inSlot},env.limits);
 expect(refused.status).toBe("refused");
 const merged=await landRunV1(env.root,request(env.stage,{outage:true,rebase:true}),{...stub,cacheDirectory:env.cache,now:inSlot},env.limits);
 expect(merged.status,merged.reason).toBe("landed");
 expect(read(env.root,"lib/a.txt")).toContain("line 1 mine");expect(read(env.root,"lib/a.txt")).toContain("line 10 theirs");
 const conflict=mini(join(directory,"c"));
 write(conflict.root,"lib/a.txt",original);
 stageEdit(conflict,{"lib/a.txt":original.replace("line 3\n","line 3 mine\n")});
 write(conflict.root,"lib/a.txt",original.replace("line 3\n","line 3 theirs\n"));
 const refusedAgain=await landRunV1(conflict.root,request(conflict.stage,{outage:true,rebase:true}),{...stub,cacheDirectory:conflict.cache,now:inSlot},conflict.limits);
 expect(refusedAgain.status).toBe("refused");expect(read(conflict.root,"lib/a.txt")).toContain("line 3 theirs");
 const unavailable=landRebaseV1(conflict.root,conflict.stage,landChangesetV1(conflict.stage,conflict.limits),landBaseProblemsV1(conflict.root,landChangesetV1(conflict.stage,conflict.limits)),()=>undefined);
 expect(unavailable.unavailable).toBe(true);
}));

test("the apply order puts new files first, every imported file before its importer and deletions last",scratch(()=>{
 const encode=(text:string)=>new TextEncoder().encode(text),entries=[
  {path:"lib/top.ts",op:"write" as const,base:"1".repeat(64),after:"2".repeat(64),area:"x"},
  {path:"lib/mid.ts",op:"write" as const,base:"1".repeat(64),after:"3".repeat(64),area:"x"},
  {path:"lib/new-leaf.ts",op:"write" as const,base:null,after:"4".repeat(64),area:"x"},
  {path:"lib/old.ts",op:"delete" as const,base:"5".repeat(64),after:null,area:"x"},
 ],bytes=new Map([["lib/top.ts",encode("import {m} from \"./mid.ts\";import {n} from \"./new-leaf.ts\";")],["lib/mid.ts",encode("export const m=1;")],["lib/new-leaf.ts",encode("export const n=1;")]]);
 const order=landApplyOrderV1({stage:{version:1,id:"o",lane:"L",title:"",bases:{},deletes:[]},entries,bytes}).map(entry=>entry.path);
 expect(order).toEqual(["lib/new-leaf.ts","lib/mid.ts","lib/top.ts","lib/old.ts"]);
}));

test("a failing rename restores every file already replaced and leaves the generation even",scratch(()=>{
 const directory=mkdtempSync(join(tmpdir(),"semio-land-fail-"));
 try{
  const root=join(directory,"live"),locks=join(directory,"locks");mkdirSync(locks,{recursive:true});
  for(const name of ["a","b","c"])write(root,"lib/"+name+".ts","old "+name);
  const encode=(text:string)=>new TextEncoder().encode(text),entries=["a","b","c"].map(name=>({path:"lib/"+name+".ts",op:"write" as const,base:landHashV1(encode("old "+name)),after:landHashV1(encode("new "+name)),area:"x"})),bytes=new Map(["a","b","c"].map(name=>["lib/"+name+".ts",encode("new "+name)] as const));
  let renames=0;
  expect(()=>landApplyV1(root,{stage:{version:1,id:"f",lane:"L",title:"",bases:{},deletes:[]},entries,bytes},{lockRoot:locks,rename:(from,to)=>{if(++renames===3)throw Error("disk full");renameSyncReal(from,to);}},policy as never)).toThrow();
  for(const name of ["a","b","c"])expect(read(root,"lib/"+name+".ts")).toBe("old "+name);
  expect(readdirSync(join(root,"lib")).filter(name=>name.endsWith(".tmp"))).toEqual([]);
  expect(landGenerationV1(locks,"x",policy as never)%2).toBe(0);
 }finally{rmSync(directory,{recursive:true,force:true});}
}));
import {renameSync as renameSyncReal} from "node:fs";

const READER=`
const fs=require("fs"),path=require("path");
const [root,generation,count,wait]=process.argv.slice(1),n=Number(count);
const gen=()=>{try{return Number(fs.readFileSync(generation,"utf8"));}catch{return 0;}};
const read=f=>{try{return fs.readFileSync(path.join(root,f),"utf8");}catch{return null;}};
let snapshots=0,mixedUnprotected=0,mixedProtected=0,torn=0,order=0,reads=0;
const until=Date.now()+Number(wait);
console.log("ready");
while(Date.now()<until){
 const g1=gen();
 const view=[];for(let i=0;i<n;i++)view.push(read("lib/m"+i+".ts"));
 const importer=read("lib/top.ts"),leaf=read("lib/leaf.ts");
 const g2=gen();reads++;
 for(const text of [importer,...view]){if(text!==null&&!/^(old .+|new .+|import .*)$/.test(text))torn++;}
 if(importer!==null&&importer.includes("new")&&leaf===null)order++;
 const kinds=new Set(view.map(t=>t===null?"x":t.startsWith("old")?"old":"new"));
 if(kinds.size>1)mixedUnprotected++;
 if(g1===g2&&g1%2===0){snapshots++;if(kinds.size>1||(importer!==null&&importer.includes("new")!==(kinds.has("new")))){mixedProtected++;}}
}
console.log(JSON.stringify({snapshots,mixedUnprotected,mixedProtected,torn,order,reads}));
`;

test("a concurrent reader never sees a torn file or a missing import, and a generation-reading reader never sees a half-applied changeset",scratch(async directory=>{
 const root=join(directory,"live"),locks=join(directory,"locks"),count=60;
 mkdirSync(locks,{recursive:true});
 write(root,"lib/top.ts","old top");
 for(let index=0;index<count;index++)write(root,`lib/m${index}.ts`,`old ${index}`);
 const encode=(text:string)=>new TextEncoder().encode(text),bytes=new Map<string,Uint8Array>(),entries=[];
 for(let index=0;index<count;index++){const path=`lib/m${index}.ts`;bytes.set(path,encode(`new ${index}`));entries.push({path,op:"write" as const,base:landHashV1(encode(`old ${index}`)),after:landHashV1(encode(`new ${index}`)),area:"x"});}
 bytes.set("lib/top.ts",encode("import {n} from \"./leaf.ts\"; new top"));entries.push({path:"lib/top.ts",op:"write" as const,base:landHashV1(encode("old top")),after:landHashV1(encode("import {n} from \"./leaf.ts\"; new top")),area:"x"});
 bytes.set("lib/leaf.ts",encode("export const n=1;"));entries.push({path:"lib/leaf.ts",op:"write" as const,base:null,after:landHashV1(encode("export const n=1;")),area:"x"});
 const child=spawn(process.execPath,["-e",READER,root,join(locks,"landing.generation.x"),String(count),"1500"],{stdio:["ignore","pipe","inherit"]});
 let output="";child.stdout.on("data",chunk=>{output+=chunk;});
 const closed=new Promise<void>(done=>child.on("close",()=>done()));
 for(let waited=0;!output.includes("ready")&&waited<200;waited++)await new Promise(done=>setTimeout(done,25));
 await new Promise(done=>setTimeout(done,100));
 landApplyV1(root,{stage:{version:1,id:"r",lane:"L",title:"",bases:{},deletes:[]},entries,bytes},{lockRoot:locks,before:step=>{if(step.phase==="rename")sleepSync(4);}},policy as never);
 await closed;
 const report=JSON.parse(output.trim().split("\n").pop()!) as {snapshots:number;mixedUnprotected:number;mixedProtected:number;torn:number;order:number;reads:number};
 expect(report.reads).toBeGreaterThan(50);expect(report.snapshots).toBeGreaterThan(0);
 expect(report.torn).toBe(0);expect(report.order).toBe(0);expect(report.mixedProtected).toBe(0);
 expect(report.mixedUnprotected,"the reader must have overlapped the apply window").toBeGreaterThan(0);
 for(let index=0;index<count;index++)expect(read(root,`lib/m${index}.ts`)).toBe(`new ${index}`);
}),60000);

test("the gates find the touched crates, the declared tests of a member and nothing for plain data",scratch(directory=>{
 write(directory,"crates/a/Cargo.toml","[package]\nname = \"semio-a\"\nversion = \"0.1.0\"\n");write(directory,"crates/a/src/🦀️.rs","fn main(){}");
 write(directory,"member/🧪️tests/🟦️.ts","");write(directory,"member/sub/code.ts","export {};");write(directory,"member/🧪️tests/🔣️.json","{}");
 const entry=(path:string)=>({path,op:"write" as const,base:null,after:"0".repeat(64),area:"x"}),set=(...paths:string[])=>({stage:{version:1 as const,id:"g",lane:"L",title:"",bases:{},deletes:[]},entries:paths.map(entry),bytes:new Map()});
 expect(landTouchedCratesV1(directory,set("crates/a/src/🦀️.rs","other/readme.md"))).toEqual(["semio-a"]);
 expect(landDeclaredTestsV1(directory,set("member/sub/code.ts","member/🧪️tests/🔣️.json","plain/data.json"))).toEqual(["member/🧪️tests/🟦️.ts"]);
}));

test("the command stages, checks and lands from the shell interface and prints the verdicts in German on request",scratch(async directory=>{
 const env=mini(directory),lines:string[]=[],notices:string[]=[];
 write(env.root,"lib/a.ts","export const a=1;\n");
 const base={directory:env.root,policy:env.limits,notice:(line:string)=>notices.push(line),run:{...stub,cacheDirectory:env.cache,now:inSlot}};
 expect(await runLandCliV1(env.root,["--lang=de","--lane","T1","checkout",env.stage,"lib/a.ts"],{},line=>lines.push(line),base)).toBe(0);
 expect(lines.join("\n")).toContain("Arbeitskopie von lib/a.ts angelegt");
 write(join(env.stage,"tree"),"lib/a.ts","export const a=2;\n");
 lines.length=0;
 expect(await runLandCliV1(env.root,["--lang=en","--outage","--title","CLI","check",env.stage],{},line=>lines.push(line),base)).toBe(0);
 expect(lines.join("\n")).toContain("All gates passed");expect(read(env.root,"lib/a.ts")).toBe("export const a=1;\n");
 lines.length=0;
 expect(await runLandCliV1(env.root,["--lang=de","--outage","--json",env.stage],{},line=>lines.push(line),base)).toBe(0);
 const parsed=JSON.parse(lines.join("\n")) as LandResultV1;
 expect(parsed.status).toBe("landed");expect(read(env.root,"lib/a.ts")).toBe("export const a=2;\n");
 expect(renderLandResultV1(parsed,"de")).toContain("gelandet");
 lines.length=0;
 expect(await runLandCliV1(env.root,["--lang=en",env.stage],{},line=>lines.push(line),base)).toBe(2);
 expect(await runLandCliV1(env.root,["--lang=en","--no-wait","rollback","nope"],{},line=>lines.push(line),base)).toBe(2);
 expect(await runLandCliV1(env.root,["--lang=de","--bogus"],{},line=>lines.push(line),base)).toBe(2);
 expect(notices.join("\n")).toContain("Unbekannte Option");
}));

test("the overlay lives below a short per-machine folder so Unix socket paths stay below SUN_LEN, and package imports resolve to the overlay, never to the live tree",scratch(async directory=>{
 const base=landOverlayBaseV1("/Users/someone/Documents/semio",{},readLandPolicyV1(),"darwin","/Users/someone");
 expect(base).toBe(join("/Users/someone",".cache","semio","l",base.split("/").pop()!));expect(base.length+"/H89".length).toBeLessThan(45);
 expect(landOverlayBaseV1("/r",{SEMIO_LAND_CACHE:"/x"},readLandPolicyV1(),"linux","/h").startsWith("/x/o/")).toBe(true);
 const live=join(directory,"live"),overlay=join(directory,"overlay"),generation=join(directory,"generation");
 write(live,"🧰️framework/pkg/package.json","{}");write(overlay,"🧰️framework/pkg/package.json","{}");write(generation,"node_modules/third/index.js","");
 mkdirSync(join(generation,"node_modules/@scope"),{recursive:true});symlinkSync(join(live,"🧰️framework/pkg"),join(generation,"node_modules/@scope/pkg"),"junction");symlinkSync(join(generation,"node_modules/third"),join(generation,"node_modules/@scope/third"),"junction");
 symlinkSync(join(generation,"node_modules"),join(live,"node_modules"),"junction");
 landBindDependenciesV1(live,overlay,readLandPolicyV1());
 expect(lstatSync(join(overlay,"node_modules")).isSymbolicLink()).toBe(false);
 expect(realpathSync(join(overlay,"node_modules/@scope/pkg"))).toBe(realpathSync(join(overlay,"🧰️framework/pkg")));
 expect(realpathSync(join(overlay,"node_modules/third"))).toBe(realpathSync(join(generation,"node_modules/third")));
 expect(realpathSync(join(overlay,"node_modules/@scope/third"))).toBe(realpathSync(join(generation,"node_modules/third")));
}));

test("a test bound to the live workspace is deferred past the overlay, runs on the live tree after the apply, and a red there rolls the landing back",scratch(async directory=>{
 const env=mini(directory);
 write(env.root,"lib/a.ts","export const a=1;\n");write(env.root,"lib/🧪️tests/🟦️.ts","x");
 stageEdit(env,{"lib/a.ts":"export const a=2;\n"});
 const bound=async()=>({passed:false,lines:["lib/🧪️tests/🟦️.ts: failed"],failures:[{file:"lib/🧪️tests/🟦️.ts",signature:"failed: x",bound:true}]});
 const calls:string[]=[];
 const green=await landRunV1(env.root,request(env.stage,{outage:true}),{...stub,tests:async req=>{calls.push(req.overlay===env.root?"live":"overlay");return req.overlay===env.root?{passed:true,lines:[]}:bound();},cacheDirectory:env.cache,now:inSlot},env.limits);
 expect(green.status).toBe("landed");expect(calls).toEqual(["overlay","live","live"]);expect(read(env.root,"lib/a.ts")).toBe("export const a=2;\n");
 expect(green.gates.find(row=>row.id==="tests")!.detail.join("\n")).toContain("bound to the live workspace");
 rmSync(env.stage,{recursive:true,force:true});stageEdit(env,{"lib/a.ts":"export const a=3;\n"});
 const red=await landRunV1(env.root,request(env.stage,{outage:true}),{...stub,tests:async req=>req.overlay===env.root?{passed:false,lines:["lib/🧪️tests/🟦️.ts: failed"],failures:[{file:"lib/🧪️tests/🟦️.ts",signature:"failed: y"}]}:bound(),cacheDirectory:env.cache,now:inSlot},env.limits);
 expect(red.status).toBe("failed");expect(read(env.root,"lib/a.ts")).toBe("export const a=2;\n");
}));


test("every top-level folder outside the cloned roots keeps its manifests and locks in the overlay, nothing else",scratch(async directory=>{
 const live=join(directory,"live"),overlay=join(directory,"overlay");
 write(live,"hub/Cargo.toml","[workspace]\n");write(live,"hub/Cargo.lock","# lock\n");write(live,"hub/src/big.rs","x");write(live,"empty/notes.txt","n");write(live,"node_modules/x/package.json","{}");mkdirSync(join(overlay,"lib"),{recursive:true});write(live,"lib/Cargo.toml","");
 landManifestFoldersV1(live,overlay,readLandPolicyV1());
 expect(read(overlay,"hub/Cargo.toml")).toBe("[workspace]\n");expect(read(overlay,"hub/Cargo.lock")).toBe("# lock\n");
 expect(existsSync(join(overlay,"hub/src"))).toBe(false);expect(existsSync(join(overlay,"empty"))).toBe(false);expect(existsSync(join(overlay,"node_modules"))).toBe(false);expect(existsSync(join(overlay,"lib/Cargo.toml"))).toBe(false);
}));

test("a failing test file that does not exist on the live tree is not run there: it is a true red of the changeset",scratch(async directory=>{
 const env=mini(directory);
 write(env.root,"lib/a.ts","export const a=1;\n");
 stageEdit(env,{"lib/a.ts":"export const a=2;\n"});
 write(env.stage,"tree/lib/🧪️tests/🟦️.ts","x");
 const calls:string[]=[];
 const result=await landRunV1(env.root,request(env.stage,{outage:true},"check"),{...stub,tests:async req=>{calls.push(req.overlay===env.root?"live":"overlay");return {passed:false,lines:["lib/🧪️tests/🟦️.ts: failed"],failures:[{file:"lib/🧪️tests/🟦️.ts",signature:"failed: n"}]};},cacheDirectory:env.cache,now:inSlot},env.limits);
 expect(result.status).toBe("failed");expect(calls).toEqual(["overlay"]);
}));


test("the tests of the overlay get the live machine state declared (the one indirection of the tool layer), the live baseline does not",scratch(async directory=>{
 const env=mini(directory);
 write(env.root,"lib/a.ts","export const a=1;\n");write(env.root,"lib/🧪️tests/🟦️.ts","x");
 stageEdit(env,{"lib/a.ts":"export const a=2;\n"});
 const seen:{overlay:boolean;state:string|undefined}[]=[];
 await landRunV1(env.root,request(env.stage,{outage:true},"check"),{...stub,tests:async req=>{seen.push({overlay:req.overlay!==env.root,state:req.state});return {passed:false,lines:["lib/🧪️tests/🟦️.ts: failed"],failures:[{file:"lib/🧪️tests/🟦️.ts",signature:"failed: n"}]};},cacheDirectory:env.cache,now:inSlot},env.limits);
 expect(seen[0]).toEqual({overlay:true,state:join(realpathSync(env.root),".cache")});expect(seen[1]).toEqual({overlay:false,state:undefined});
}));
