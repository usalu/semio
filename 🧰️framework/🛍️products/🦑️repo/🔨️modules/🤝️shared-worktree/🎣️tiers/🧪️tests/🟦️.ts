import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,statSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {testPythonV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {ReplicaBookV1} from "../../🔁️replica/🟦️.ts";
import {DirectoryTableV1,ensureHintsV1,hintBytesLimitV1,hookBytesLimitV1,hookCommandV1,HotSetV1,hintsPathV1,LatencyV1,nextDelayV1,silentLatencyV1,TierClocksV1,normaliseHintPathV1,parseHintTextV1,parseHookInputV1,percentileV1,takeHintsV1,writeHintV1} from "../🟦️.ts";

const ajv=new Ajv({strict:true,allowUnionTypes:true}),python=testPythonV1(),scratch:string[]=[],posixTest=process.platform==="win32"?test.skip:test;
const temp=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`tiers-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

test("the fixture is valid against the schema",()=>{
 const check=ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:"#/definitions/Fixture"});
 expect(check(fixture),JSON.stringify(check.errors?.[0])).toBe(true);
});

test("the Python oracle reproduces every path, hint, hook and percentile vector with numpy and jsonschema",()=>{
 const result=Bun.spawnSync([python!,"-E",join(import.meta.dirname,"🐍️.py")],{stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stdout.toString()+result.stderr.toString()).toBe(0);
 expect(result.stdout.toString()).toContain("0 mismatches");
});

for(const row of fixture.paths)test(`hint path vector: ${row.name}`,async()=>{
 const parent=temp("path"),root=join(parent,"worktree"),outside=join(parent,"outside");
 mkdirSync(root);mkdirSync(outside);
 expect(await normaliseHintPathV1(root,row.raw.replaceAll("{root}",root).replaceAll("{outside}",outside))).toBe(row.expect??undefined);
});

for(const row of fixture.hints)test(`hint text vector: ${row.name}`,()=>{expect(parseHintTextV1(row.text)).toEqual(row.expect);});
for(const row of fixture.hooks)test(`hook payload vector: ${row.name}`,()=>{expect(parseHookInputV1(row.text)).toEqual(row.expect);});

for(const row of fixture.percentiles)test(`percentile vector: ${row.name}`,()=>{
 expect(percentileV1(row.values,50)).toBe(row.p50);
 expect(percentileV1(row.values,95)).toBe(row.p95);
});

test("the latency sampler keeps the newest 100 samples, rounds, clamps small skew and drops edits older than ten minutes",()=>{
 const latency=new LatencyV1();
 expect(latency.report()).toEqual({publishMs:{p50:null,p95:null},applyMs:{p50:null,p95:null},samples:{publish:0,apply:0},tiers:silentLatencyV1().tiers});
 for(let value=1;value<=150;value++)latency.publish(value);
 latency.apply(-30);latency.apply(12.6);latency.apply(700_000);latency.apply(-700_000);latency.apply(Number.NaN);latency.apply(Infinity);
 expect(latency.report()).toEqual({publishMs:{p50:100,p95:145},applyMs:{p50:0,p95:13},samples:{publish:100,apply:2},tiers:new LatencyV1().report().tiers});
});

test("the hot set keeps the newest paths up to its limit, renews a path that is noted again, expires after the ttl and ignores bulk notes",()=>{
 const hot=new HotSetV1(3,1000);
 hot.note("a",0);hot.note("b",10);hot.note("c",20);hot.note("a",30);hot.note("d",40);
 expect(hot.snapshot(40)).toEqual(["c","a","d"]);
 expect(hot.snapshot(1025)).toEqual(["a","d"]);
 expect(hot.snapshot(1100)).toEqual([]);
 hot.noteMany(["x","y"],501,1100);
 expect(hot.has("x")).toBe(false);
 hot.noteMany(["x","y"],2,1100);
 expect(hot.snapshot(1100)).toEqual(["x","y"]);
 expect(new HotSetV1().limit).toBe(20_000);expect(new HotSetV1().ttlMilliseconds).toBe(600_000);
});

test("the directory table visits every present directory once per pass, forgets deleted ones, and learns a deleted directory again",()=>{
 const book=new ReplicaBookV1(),table=new DirectoryTableV1(book),names=(): string[]=>{const found:string[]=[];table.begin();for(let id=table.next();id>=0;id=table.next())found.push(table.nameAt(id));return found;};
 for(let index=0;index<3000;index++)table.set(`d${index}`,index);
 table.set("",5);
 expect(table.size).toBe(3001);expect(table.get("d7")).toBe(7);expect(table.has("nope")).toBe(false);
 expect(names()).toHaveLength(3001);
 for(let index=0;index<3000;index+=2)table.delete(`d${index}`);
 table.delete("never-known");
 expect(table.size).toBe(1501);expect(table.get("d2")).toBeUndefined();
 const kept=names();
 expect(kept).toHaveLength(1501);expect(kept).toContain("");expect(kept).not.toContain("d4");
 table.set("d4",99);
 expect(table.get("d4")).toBe(99);
 const again=names();
 expect(again).toHaveLength(1502);expect(new Set(again).size).toBe(1502);
 table.set("d4",100);
 expect(table.get("d4")).toBe(100);expect(table.size).toBe(1502);
});

test("a hint file round-trips: the dot-named temp file is invisible, paths are normalised and deduplicated in file order, outside and sealed paths are dropped, and every file is consumed",async()=>{
 const parent=temp("hints"),root=join(parent,"worktree");
 mkdirSync(join(root,"a"),{recursive:true});
 const first=await writeHintV1(root,["a/one.txt",join(root,"a","two.txt"),"../outside.txt",".cache/f/state.json"]);
 await Bun.sleep(2);
 const second=await writeHintV1(root,["a/one.txt","three.txt"]);
 const directory=hintsPathV1(root);
 expect(first.endsWith(".hint")).toBe(true);expect(readdirSync(directory).sort()).toEqual([first,second].sort());
 expect(readFileSync(join(directory,first),"utf8")).toBe(["a/one.txt",join(root,"a","two.txt"),"../outside.txt",".cache/f/state.json"].join("\0"));
 writeFileSync(join(directory,".unfinished"),"half");
 expect(await takeHintsV1(root)).toEqual(["a/one.txt","a/two.txt","three.txt"]);
 expect(readdirSync(directory)).toEqual([".unfinished"]);
 expect(await takeHintsV1(root)).toEqual([]);
 const old=(Date.now()-120_000)/1000;
 utimesSync(join(directory,".unfinished"),old,old);
 await takeHintsV1(root);
 expect(readdirSync(directory)).toEqual([]);
});

test("a hint file above the size limit is removed unread, the take stops at the path limit, and a missing directory yields nothing",async()=>{
 const root=temp("limits");
 expect(await takeHintsV1(root)).toEqual([]);
 const directory=await ensureHintsV1(root);
 writeFileSync(join(directory,"big.hint"),"x".repeat(hintBytesLimitV1+1));
 for(const name of ["a.txt","b.txt","c.txt"]){await writeHintV1(root,[name]);await Bun.sleep(3);}
 expect(await takeHintsV1(root,2)).toEqual(["a.txt","b.txt"]);
 expect(readdirSync(directory).filter(name=>name.endsWith(".hint")).sort()).toHaveLength(2);
 expect(await takeHintsV1(root)).toEqual(["c.txt"]);
 expect(existsSync(join(directory,"big.hint"))).toBe(false);
});

posixTest("the drop directory is open to every local account and an absolute path through a symbolic link of the worktree is found",async()=>{
 const parent=temp("alias"),real=join(parent,"real"),alias=join(parent,"alias");
 mkdirSync(join(real,"d"),{recursive:true});symlinkSync(real,alias);
 writeFileSync(join(real,"d","f.txt"),"x");
 const directory=await ensureHintsV1(alias);
 expect(statSync(directory).mode&0o7777).toBe(0o1777);
 expect(await normaliseHintPathV1(alias,join(real,"d","f.txt"))).toBe("d/f.txt");
 expect(await normaliseHintPathV1(real,join(alias,"d","f.txt"))).toBe("d/f.txt");
 expect(await normaliseHintPathV1(alias,join(parent,"elsewhere","f.txt"))).toBeUndefined();
});

test("a latency sample leaves the report one hour after it was taken, so an outlier of a blocked period does not describe a quiet machine",()=>{
 let clock=1_000_000;
 const latency=new LatencyV1(100,600_000,3_600_000,()=>clock);
 latency.publish(491_421);latency.apply(9000);
 expect(latency.report()).toMatchObject({publishMs:{p50:491_421,p95:491_421},applyMs:{p50:9000,p95:9000},samples:{publish:1,apply:1}});
 clock+=1_800_000;
 for(const milliseconds of [40,50,60])latency.publish(milliseconds);
 expect(latency.report().publishMs.p95).toBe(491_421);
 clock+=1_800_001;
 latency.publish(70);
 expect(latency.report()).toMatchObject({publishMs:{p50:50,p95:70},applyMs:{p50:null,p95:null},samples:{publish:4,apply:0}});
 clock+=3_600_001;
 expect(latency.report()).toMatchObject({publishMs:{p50:null,p95:null},samples:{publish:0,apply:0}});
});

test("a tier waits the larger of its interval and the time that keeps its duty cycle, and the clocks report cost and duty",()=>{
 expect(nextDelayV1(2000,100,10)).toBe(2000);
 expect(nextDelayV1(2000,1000,10)).toBe(9000);
 expect(nextDelayV1(90_000,42_000,5)).toBe(798_000);
 expect(nextDelayV1(300,60,8)).toBe(690);
 const clocks=new TierClocksV1();
 expect(clocks.report().sweep).toEqual({passes:0,lastMs:null,dutyPercent:null});
 clocks.record("sweep",400,nextDelayV1(2000,400,10));clocks.record("sweep",1000,nextDelayV1(2000,1000,10));
 expect(clocks.report().sweep).toEqual({passes:2,lastMs:1000,dutyPercent:10});
 clocks.record("walk",42_000,nextDelayV1(90_000,42_000,5));
 expect(clocks.report().walk).toEqual({passes:1,lastMs:42_000,dutyPercent:5});
 expect(new LatencyV1().report().tiers.view).toEqual({passes:0,lastMs:null,dutyPercent:null});
});

test("the directory table names the root and the directories up to three levels deep as shallow",()=>{
 const table=new DirectoryTableV1(new ReplicaBookV1());
 for(const path of ["","a","a/b","a/b/c","a/b/c/d","x/y/z/w/v"])table.set(path,1);
 expect(table.shallow().sort()).toEqual(["","a","a/b","a/b/c"]);
 table.delete("a/b");
 expect(table.shallow().sort()).toEqual(["","a","a/b/c"]);
});

test("a hook payload file is parsed by the daemon side, even when the payload is far above the hint limit, and the shell command writes exactly such a file",async()=>{
 const root=temp("hookfile"),directory=await ensureHintsV1(root);
 const payload=JSON.stringify({hook_event_name:"PostToolUse",tool_name:"Write",tool_input:{file_path:join(root,"big","new.ts"),content:"x".repeat(hintBytesLimitV1*2)}});
 expect(payload.length).toBeGreaterThan(hintBytesLimitV1);expect(payload.length).toBeLessThan(hookBytesLimitV1);
 writeFileSync(join(directory,"one.hook"),payload);
 writeFileSync(join(directory,"two.hook"),"{not json");
 writeFileSync(join(directory,"three.hook"),JSON.stringify({tool_input:{notebook_path:"n/b.ipynb"}}));
 expect((await takeHintsV1(root)).sort()).toEqual(["big/new.ts","n/b.ipynb"]);
 expect(readdirSync(directory)).toEqual([]);
 if(process.platform==="win32")return;
 const run=(input:string,project:string)=>Bun.spawnSync(["bash","-c",hookCommandV1],{stdin:Buffer.from(input),env:{...process.env,CLAUDE_PROJECT_DIR:project},stdout:"pipe",stderr:"pipe"});
 const first=run(JSON.stringify({tool_input:{file_path:join(root,"sh","a b.ts")}}),root);
 expect(first.exitCode).toBe(0);expect(first.stderr.toString()).toBe("");
 expect(readdirSync(directory).filter(name=>name.endsWith(".hook"))).toHaveLength(1);
 expect(readdirSync(directory).filter(name=>name.startsWith("."))).toEqual([]);
 expect(await takeHintsV1(root)).toEqual(["sh/a b.ts"]);
 expect(run("anything",temp("nodir")).exitCode).toBe(0);
 expect(run("",root).exitCode).toBe(0);
 expect(await takeHintsV1(root)).toEqual([]);
});

test("the checked-in project settings register the hook command of the tiers module for every file-writing tool",()=>{
 let directory=import.meta.dirname;
 while(!existsSync(join(directory,".claude","settings.json"))&&join(directory,"..")!==directory)directory=join(directory,"..");
 const file=join(directory,".claude","settings.json");
 if(!existsSync(file))return;
 const settings=JSON.parse(readFileSync(file,"utf8")) as {hooks:{PostToolUse:{matcher:string;hooks:Record<string,unknown>[]}[]}};
 const entry=settings.hooks.PostToolUse.find(item=>item.matcher==="Write|Edit|MultiEdit|NotebookEdit")!;
 expect(entry.hooks[0]).toMatchObject({type:"command",command:hookCommandV1,shell:"bash",async:true});
});
