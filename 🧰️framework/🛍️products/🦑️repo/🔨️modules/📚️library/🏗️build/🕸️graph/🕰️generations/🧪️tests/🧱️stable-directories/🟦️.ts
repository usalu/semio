import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,symlinkSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import Ajv from"ajv";
import fixture from"./🔣️.json";
import fixtureSchema from"./🧬️schema/🔣️.json";

const generations=await import(new URL("../../🟨️.mjs",import.meta.url).href) as typeof import("../../🟨️.mjs");
const observation=await import(new URL("../../../../👁️observation/🟨️.mjs",import.meta.url).href) as typeof import("../../../../👁️observation/🟨️.mjs");
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});

test("the fixture is closed under its schema",()=>{
 expect(new Ajv({strict:true}).compile(fixtureSchema)(fixture)).toBe(true);
});

test("stable directories equal the independent Python model on every case",()=>{
 for(const row of fixture.cases)expect([...generations.stableDirectoriesV1(row.before as never,row.after as never,{racyMilliseconds:fixture.racyMilliseconds})].sort()).toEqual([...row.stable].sort());
 const result=Bun.spawnSync(["python3","-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))]);
 expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});
});

/** 🧪️ A workspace with a stable-by-claim directory `quiet` and a directory `busy` the proof does not cover. */
function workspace(){
 const parent=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();mkdirSync(parent,{recursive:true});
 const root=mkdtempSync(join(realpathSync(parent),"stable-"));roots.push(root);
 for(const name of["quiet","busy"]){mkdirSync(join(root,name));writeFileSync(join(root,name,"present.txt"),"x");}
 return root;
}

const rows=(root:string,directory:string)=>[
 {kind:"readdir",path:join(root,directory),fact:undefined as unknown},
 {kind:"readdirNames",path:join(root,directory),fact:undefined as unknown},
 {kind:"lstat",path:join(root,directory,"present.txt"),fact:"file"},
 {kind:"exists",path:join(root,directory,"missing.txt"),fact:false},
 {kind:"fileExists",path:join(root,directory,"missing.txt"),fact:false},
 {kind:"directoryExists",path:join(root,directory,"missing"),fact:false},
];

/** 📸️ Records the facts of the directory as they are now, through the same probes the verifier replays. */
async function record(root:string,directory:string){
 const list=rows(root,directory),ledger=await import("node:fs");
 const entries=ledger.readdirSync(join(root,directory),{withFileTypes:true}),typed=entries.map(entry=>[entry.name,entry.isDirectory()?"directory":entry.isFile()?"file":"other"]).sort();
 const {createHash}=await import("node:crypto");
 const sha=(value:unknown)=>createHash("sha256").update(JSON.stringify(value)).digest("hex");
 list[0]!.fact=sha(typed.map(([name,type])=>[String(name),type]).sort());
 list[1]!.fact=sha(entries.map(entry=>entry.name).sort());
 return list;
}

test("rows about the entries of a stable directory hold without a probe, the others are probed",async()=>{
 const root=workspace(),quiet=await record(root,"quiet"),busy=await record(root,"busy");
 const stable=new Set([join(root,"quiet")]);
 expect(await observation.observationsHold([...quiet,...busy],4,stable,true)).toBe(true);
 for(const name of["quiet","busy"]){writeFileSync(join(root,name,"missing.txt"),"appeared");mkdirSync(join(root,name,"missing"));}
 expect(await observation.observationsHold(quiet,4,stable,true)).toBe(true);
 for(const row of busy.filter(row=>row.kind!=="lstat"))expect(await observation.observationsHold([row],4,stable,true)).toBe(false);
 expect(await observation.observationsHold(busy.filter(row=>row.kind==="lstat"),4,stable,true)).toBe(true);
});

test("a present entry is probed when links are possible, because a link target can vanish without touching its directory",async()=>{
 const root=workspace(),target=join(root,"target.txt");writeFileSync(target,"x");
 symlinkSync(target,join(root,"quiet","link.txt"));
 const row={kind:"exists",path:join(root,"quiet","link.txt"),fact:true},stable=new Set([join(root,"quiet")]);
 expect(await observation.observationsHold([row],1,stable,true)).toBe(true);
 rmSync(target);
 expect(await observation.observationsHold([row],1,stable,true)).toBe(false);
 expect(await observation.observationsHold([row],1,stable,new Set())).toBe(true);
});

test("the host proves directories stable from the earlier frontier before it verifies, and never trusts a present entry to be no link",()=>{
 const text=readFileSync(fileURLToPath(new URL("../../../🟦️.ts",import.meta.url)),"utf8");
 const proof=text.indexOf("stableDirectoriesV1(previous.frontier,live.frontier)"),verify=text.indexOf("observationV1.observationsHold(plain,64,state.stable,true,signal)");
 expect(proof).toBeGreaterThan(0);expect(verify).toBeGreaterThan(0);
 expect(text.slice(proof-200,proof)).toContain("seams.membership(previous,signal)");
 expect(text).toContain("previous?.frontier?");
});
