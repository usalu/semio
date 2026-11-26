import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import {mkdirSync,readFileSync,readdirSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {cargoInputClosureV1,HERMETIC_TEXTS_V1,hermeticRootsV1,hermeticStateRootV1,runHermeticCargoV1,runHermeticV1,type HermeticOutcomeV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {removeV1,shortScratchV1,snapshotV1} from "../../🧪️tests/🧰️world/🟦️.ts";

const cargo=Bun.which("cargo"),pinned="nightly-2026-07-20",usable=cargo!==null&&Bun.spawnSync([cargo,"--version"],{env:{...process.env,RUSTUP_TOOLCHAIN:pinned}}).exitCode===0;
const scratch:string[]=[];
const world=(tag:string)=>{const root=shortScratchV1(tag);scratch.push(root);return root;};
afterAll(()=>{for(const root of scratch)removeV1(root);});
const put=(root:string,files:Record<string,string>)=>{for(const[path,text]of Object.entries(files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const sha=(path:string)=>createHash("sha256").update(readFileSync(path)).digest("hex");
const done=(outcome:HermeticOutcomeV1)=>{expect(outcome.kind,JSON.stringify(outcome)).toBe("done");return outcome as Extract<HermeticOutcomeV1,{kind:"done"}>;};
const refused=(outcome:HermeticOutcomeV1,code:string)=>{expect(outcome.kind,JSON.stringify(outcome)).toBe("refused");const row=outcome as Extract<HermeticOutcomeV1,{kind:"refused"}>;expect<unknown>(row.code).toBe(code);expect(row.en.length).toBeGreaterThan(10);expect(row.de.length).toBeGreaterThan(10);expect(row.de).not.toBe(row.en);return row;};

const crate=(root:string,trim=true)=>{
 put(root,{
  "Cargo.toml":`${trim?'cargo-features=["trim-paths"]\n':""}[package]\nname="hello"\nversion="0.1.0"\nedition="2021"\n[profile.dev]\ndebug=false\n${trim?'trim-paths="all"\n':""}`,
  "src/main.rs":'fn main(){println!("{}",include_str!("data.txt"));}\n',"src/data.txt":"hello\n","outside.txt":"not declared\n","README.md":"readme\n",
 });
 const run=Bun.spawnSync([cargo!,"generate-lockfile","--offline"],{cwd:root,env:{...process.env,RUSTUP_TOOLCHAIN:pinned},stdout:"pipe",stderr:"pipe"});
 expect(run.exitCode,run.stderr.toString()).toBe(0);
};
const BUILD={artifacts:["debug/hello"],toolchain:pinned};

test("schema law: the receipt and the policy validate against the schema, every text exists in English and German",()=>{
 const ajv=new Ajv({strict:false}),check=ajv.compile({...schema,$ref:"#/definitions/Receipt"});
 expect(check({version:1,hash:"a".repeat(64),key:"b".repeat(64),outputs:[{name:"x",digest:"c".repeat(64),bytes:1}]}),JSON.stringify(check.errors)).toBe(true);
 expect(check({version:1,hash:"a",key:"b",outputs:[]})).toBe(false);
 expect(Object.keys(HERMETIC_TEXTS_V1.en).sort()).toEqual(Object.keys(HERMETIC_TEXTS_V1.de).sort());
 for(const key of Object.keys(HERMETIC_TEXTS_V1.en))expect(HERMETIC_TEXTS_V1.de[key as keyof typeof HERMETIC_TEXTS_V1.de]).not.toBe(HERMETIC_TEXTS_V1.en[key as keyof typeof HERMETIC_TEXTS_V1.en]);
});

test.skipIf(!usable)("closure law: the cargo input closure is the manifests, the lock, the config and the package sources, never target, never a file beside the package",async()=>{
 const live=world("hm-live");crate(live);put(live,{"target/debug/x":"x",".cargo/config.toml":"[build]\n","rust-toolchain.toml":"[toolchain]\nchannel=\"x\"\n"});
 const closure=await cargoInputClosureV1(live,{RUSTUP_TOOLCHAIN:pinned});
 expect(closure.outside).toEqual([]);
 expect([...closure.paths]).toEqual([".cargo/config.toml","Cargo.lock","Cargo.toml","rust-toolchain.toml","src/data.txt","src/main.rs"]);
},60000);

test.skipIf(!usable)("view law: a real crate builds in a view; the output lands in the declared place only; the live tree, the view and the state root hold nothing else",async()=>{
 const live=world("hm-live"),state=world("hm-state");crate(live);
 const before=snapshotV1(live);
 const outcome=done(await runHermeticCargoV1({workspace:live,stateRoot:state,...BUILD}));
 expect(snapshotV1(live),"the replicated tree is untouched: no target directory, no lock change").toEqual(before);
 expect(outcome.outputs.map(row=>row.name)).toEqual(["debug/hello"]);
 expect(readdirSync(outcome.directory).sort()).toEqual(["debug%2Fhello","receipt.json"]);
 const roots=hermeticRootsV1(state);
 expect(outcome.directory.startsWith(roots.outputs)).toBe(true);
 expect(readdirSync(join(roots.target,"debug")).includes("hello")).toBe(true);
 expect(sha(join(outcome.directory,"debug%2Fhello"))).toBe(outcome.outputs[0]!.digest);
 const ajv=new Ajv({strict:false});expect(ajv.compile({...schema,$ref:"#/definitions/Receipt"})(JSON.parse(readFileSync(join(outcome.directory,"receipt.json"),"utf8")))).toBe(true);
 const run=Bun.spawnSync([join(outcome.directory,"debug%2Fhello")],{stdout:"pipe"});expect(run.stdout.toString()).toBe("hello\n\n");
},120000);

test.skipIf(!usable)("undeclared-input law: a read of a live file that is not declared is refused and names the file; declaring it makes it a different task",async()=>{
 const live=world("hm-live"),state=world("hm-state");crate(live);
 writeFileSync(join(live,"src/main.rs"),'fn main(){println!("{}{}",include_str!("data.txt"),include_str!("../outside.txt"));}\n');
 const before=snapshotV1(live);
 const row=refused(await runHermeticCargoV1({workspace:live,stateRoot:state,...BUILD}),"undeclared-input");
 expect(row.paths).toEqual(["outside.txt"]);
 expect(snapshotV1(live)).toEqual(before);
 expect(readdirSync(hermeticRootsV1(state).outputs).filter(name=>!name.startsWith(".tmp-"))).toEqual([]);
 const closure=await cargoInputClosureV1(live,{RUSTUP_TOOLCHAIN:pinned});
 const declared=done(await runHermeticCargoV1({workspace:live,stateRoot:state,...BUILD,paths:[...closure.paths,"outside.txt"]}));
 const plain=world("hm-live2");crate(plain);
 expect(declared.hash).not.toBe(done(await runHermeticCargoV1({workspace:plain,stateRoot:world("hm-state"),...BUILD})).hash);
},180000);

test.skipIf(!usable)("absolute-read law: an absolute path to the live tree is found in the dependency info and refused",async()=>{
 const live=world("hm-live"),state=world("hm-state");crate(live);
 writeFileSync(join(live,"src/main.rs"),`fn main(){println!("{}",include_str!(${JSON.stringify(join(live,"outside.txt"))}));}\n`);
 const row=refused(await runHermeticCargoV1({workspace:live,stateRoot:state,...BUILD}),"read-outside-view");
 expect(row.paths[0]).toContain("outside.txt");
},120000);

test.skipIf(!usable)("determinism law: two runs from different state roots give the same hash, key, output bytes and receipt; the view path is in no output",async()=>{
 const live=world("hm-live"),one=world("hm-state"),two=world("hm-state-long-name");crate(live);
 const a=done(await runHermeticCargoV1({workspace:live,stateRoot:one,...BUILD})),b=done(await runHermeticCargoV1({workspace:live,stateRoot:two,...BUILD}));
 expect(hermeticRootsV1(one).views).not.toBe(hermeticRootsV1(two).views);
 expect(a.hash).toBe(b.hash);expect(a.key).toBe(b.key);
 expect(a.outputs.map(row=>row.digest)).toEqual(b.outputs.map(row=>row.digest));
 expect(readFileSync(join(a.directory,"receipt.json"),"utf8")).toBe(readFileSync(join(b.directory,"receipt.json"),"utf8"));
 const again=done(await runHermeticCargoV1({workspace:live,stateRoot:one,...BUILD}));
 expect(again.key).toBe(a.key);expect(again.reused).toBe(true);
 const plain=world("hm-plain");crate(plain);
 const direct=Bun.spawnSync([cargo!,"build","--locked","--offline"],{cwd:plain,env:{...process.env,RUSTUP_TOOLCHAIN:pinned,CARGO_TARGET_DIR:join(plain,"tt"),CARGO_INCREMENTAL:"0"},stdout:"pipe",stderr:"pipe"});
 expect(direct.exitCode,direct.stderr.toString()).toBe(0);
 expect(sha(join(plain,"tt/debug/hello")),"equals a plain cargo build of a pristine copy elsewhere").toBe(a.outputs[0]!.digest);
 const python=Bun.which("python3");
 if(python!==null){
  const oracle=Bun.spawnSync([python,"-I","-c","import hashlib,sys;print(hashlib.sha256(open(sys.argv[1],'rb').read()).hexdigest())",join(a.directory,"debug%2Fhello")],{stdout:"pipe"});
  expect(oracle.stdout.toString().trim(),"independent digest (Python hashlib)").toBe(a.outputs[0]!.digest);
 }
},240000);

test.skipIf(!usable)("path-leak law (control): a build that keeps the view path in its output is refused as not shareable",async()=>{
 const live=world("hm-live"),state=world("hm-state");crate(live,false);
 writeFileSync(join(live,"src/main.rs"),'fn main(){println!("{}",env!("CARGO_MANIFEST_DIR"));}\n');
 const row=refused(await runHermeticCargoV1({workspace:live,stateRoot:state,...BUILD,artifacts:["debug/hello"]}),"path-leak");
 expect(row.paths.length).toBe(1);
},120000);

test.skipIf(!usable)("ambient-config law: a state root below a Cargo configuration that is not declared is refused before anything runs",async()=>{
 const live=world("hm-live"),outer=world("hm-outer");crate(live);put(outer,{".cargo/config.toml":"[build]\njobs=1\n"});
 const row=refused(await runHermeticCargoV1({workspace:live,stateRoot:join(outer,"s"),...BUILD}),"ambient-config");
 expect(row.paths[0]).toContain(".cargo");
},60000);

test("fallback law: a state root too long for a view is a typed refusal, the task never runs on the live tree",async()=>{
 const live=world("hm-live"),state=join(world("hm-state"),"x".repeat(130));put(live,{"a.txt":"a"});let ran=false;
 const row=refused(await runHermeticV1({workspace:live,stateRoot:state,paths:["a.txt"],outputs:[],command:()=>{ran=true;return{file:process.execPath,args:["-e","0"]};}}),"unavailable");
 expect(ran).toBe(false);expect(row.en).toContain("existing path");
});

const TS={"src/index.ts":'import {answer} from "./lib.ts";\nconsole.log(answer());\n',"src/lib.ts":"export const answer=():number=>42;\n","extra/undeclared.ts":"export const secret=1;\n"};
const bundle=(live:string,state:string,extra:Partial<Parameters<typeof runHermeticV1>[0]>={})=>runHermeticV1({workspace:live,stateRoot:state,paths:["src/index.ts","src/lib.ts"],command:context=>({file:process.execPath,args:["build","src/index.ts","--outfile",join(context.outputDirectory,"index.js")]}),outputs:[{name:"index.js",from:"staging",path:"index.js"}],...extra});

test("ts law: a bundle task runs in a view, outputs only its declared file, and two state roots give the same bytes and key",async()=>{
 const live=world("hm-live"),one=world("hm-state"),two=world("hm-state-other");put(live,TS);
 const before=snapshotV1(live);
 const a=done(await bundle(live,one)),b=done(await bundle(live,two));
 expect(snapshotV1(live)).toEqual(before);
 expect(readdirSync(a.directory).sort()).toEqual(["index.js","receipt.json"]);
 expect(a.hash).toBe(b.hash);expect(a.key).toBe(b.key);expect(a.outputs[0]!.digest).toBe(b.outputs[0]!.digest);
 const run=Bun.spawnSync([process.execPath,join(a.directory,"index.js")],{stdout:"pipe"});expect(run.stdout.toString().trim()).toBe("42");
},120000);

test("ts law: an import of an undeclared live file is refused and named; a write into the view is detected",async()=>{
 const live=world("hm-live"),state=world("hm-state");put(live,{...TS,"src/lib.ts":'export {secret as answer} from "../extra/undeclared.ts";\n'});
 const row=refused(await bundle(live,state),"undeclared-input");
 expect(row.paths).toEqual(["extra/undeclared.ts"]);
 const plain=world("hm-live");put(plain,TS);
 const stray=refused(await bundle(plain,state,{command:context=>({file:process.execPath,args:["-e",`require("fs").writeFileSync("stray.txt","x");require("fs").writeFileSync(${JSON.stringify(join(context.outputDirectory,"index.js"))},"x")`]})}),"wrote-into-view");
 expect(stray.paths).toEqual(["stray.txt"]);
 const extra=refused(await bundle(plain,state,{command:context=>({file:process.execPath,args:["-e",`require("fs").writeFileSync(${JSON.stringify(join(context.outputDirectory,"index.js"))},"x");require("fs").writeFileSync(${JSON.stringify(join(context.outputDirectory,"other.js"))},"x")`]})}),"output-unexpected");
 expect(extra.paths).toEqual(["other.js"]);
 refused(await bundle(plain,state,{command:()=>({file:process.execPath,args:["-e","0"]})}),"output-missing");
 refused(await bundle(plain,state,{command:()=>({file:process.execPath,args:["-e","process.exit(3)"]})}),"failed");
});

test("cancel law: an aborted task is a typed refusal and leaves no output",async()=>{
 const live=world("hm-live"),state=world("hm-state");put(live,TS);const controller=new AbortController();
 const pending=bundle(live,state,{signal:controller.signal,command:()=>({file:process.execPath,args:["-e","setTimeout(()=>0,30000)"]})});
 setTimeout(()=>controller.abort(),400);
 refused(await pending,"aborted");
 expect(readdirSync(hermeticRootsV1(state).outputs).filter(name=>!name.startsWith(".tmp-"))).toEqual([]);
},30000);

test("sweep law: a stray file of an earlier run is not visible to the next run in the same view",async()=>{
 const live=world("hm-live"),state=world("hm-state");put(live,TS);
 refused(await bundle(live,state,{command:context=>({file:process.execPath,args:["-e",`require("fs").writeFileSync("stray.txt","x");require("fs").mkdirSync("junk");require("fs").writeFileSync(${JSON.stringify(join(context.outputDirectory,"index.js"))},"x")`]})}),"wrote-into-view");
 const seen=done(await bundle(live,state,{command:context=>({file:process.execPath,args:["-e",`const fs=require("fs");fs.writeFileSync(${JSON.stringify(join(context.outputDirectory,"index.js"))},JSON.stringify(fs.readdirSync(".").sort()))`]})}));
 expect(readFileSync(join(seen.directory,"index.js"),"utf8")).toBe('["src"]');
});

test("oracle law: the hermetic bundle equals a plain `bun build` of a pristine copy in another directory",async()=>{
 const live=world("hm-live"),plain=world("hm-plain"),state=world("hm-state");put(live,TS);put(plain,TS);
 const hermetic=done(await bundle(live,state)),direct=Bun.spawnSync([process.execPath,"build","src/index.ts","--outfile",join(plain,"out.js")],{cwd:plain,stdout:"pipe",stderr:"pipe"});
 expect(direct.exitCode,direct.stderr.toString()).toBe(0);
 expect(hermetic.outputs[0]!.digest).toBe(sha(join(plain,"out.js")));
});

test("state-root law: the environment names the state root, the default is the machine state of the workspace, the roots are siblings",()=>{
 expect(hermeticStateRootV1("/w",{SEMIO_HERMETIC_STATE_ROOT:"/s/x"})).toBe("/s/x");
 expect(hermeticStateRootV1("/w",{})).toEndWith("/f/hm");
 const roots=hermeticRootsV1("/s/x");
 expect([roots.views,roots.target,roots.home,roots.outputs]).toEqual(["/s/x/v","/s/x/t","/s/x/h","/s/x/o"]);
});
