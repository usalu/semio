import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {parseDepInfoV1,withoutIncrementalV1} from "../../🟦️.ts";
import {keyGapV1,sourceEnvironmentV1,sourceKeyV1,unitSourcesV1,SOURCE_KEY_VERSION_V1} from "../🟦️.ts";

/** 🔗️ The key of a unit covers every file the compiler reads and every environment name it asks for: the scan before the compile finds what rustc's own dependency file lists afterwards, a change of any module, include or build output moves the key, the same crate in another directory has the same key, and a file the scan cannot find is reported as a gap. */
const scratch:string[]=[],toolchain={...process.env,RUSTUP_TOOLCHAIN:process.env.RUSTUP_TOOLCHAIN??"nightly-2026-07-20"};
const directory=()=>{const path=realpathSync(mkdtempSync(join(tmpdir(),"sources-")));scratch.push(path);return path;};
const digest=(path:string):string|null=>{try{return createHash("sha256").update(readFileSync(path)).digest("hex");}catch{return null;}};
const CRATE:Record<string,string>={
 "src/lib.rs":'mod a;\nmod b;\n#[path = "odd/name.rs"]\nmod c;\npub const D:&str=include_str!("data.txt");\npub const E:&str=env!("SEMIO_SCAN_FOO");\npub const F:Option<&str>=option_env!("SEMIO_SCAN_BAR");\npub fn f()->u32{a::g()+b::h()+c::m()}\n',
 "src/a.rs":"pub fn g()->u32{1}\n","src/b/mod.rs":"mod deep;\npub fn h()->u32{deep::k()}\n","src/b/deep.rs":"pub fn k()->u32{2}\n","src/odd/name.rs":"pub fn m()->u32{3}\n","src/data.txt":"data\n"};
const crate=(root:string,overrides:Record<string,string>={})=>{for(const [path,text] of Object.entries({...CRATE,...overrides})){mkdirSync(join(root,path,".."),{recursive:true});writeFileSync(join(root,path),text);}return join(root,"src","lib.rs");};
const compile=(entry:string,out:string,environment:Record<string,string>)=>{mkdirSync(out,{recursive:true});const run=spawnSync("rustc",["--crate-name","x","--edition=2021",entry,"--crate-type","lib","--emit=dep-info","--out-dir",out],{cwd:resolve(entry,"..","..",".."),encoding:"utf8",env:{...toolchain,...environment}});expect(run.status,run.stderr).toBe(0);return readFileSync(join(out,"x.d"),"utf8");};

test("the source key equals an independent Python serialisation and moves with the entry and with every other file",()=>{
 const rows:[string,string][]=[["a.rs","1".repeat(64)],["b/mod.rs","2".repeat(64)]],python=spawnSync("python3",["-I","-c","import hashlib,json,sys;print(hashlib.sha256(json.dumps([sys.argv[1],sys.argv[2],json.loads(sys.argv[3])],separators=(',',':'),ensure_ascii=False).encode()).hexdigest())",SOURCE_KEY_VERSION_V1,"e".repeat(64),JSON.stringify(rows)],{encoding:"utf8"});
 if(python.error===undefined)expect(python.stdout.trim()).toBe(sourceKeyV1("e".repeat(64),rows));
 const base=sourceKeyV1("e".repeat(64),rows);
 expect(new Set([base,sourceKeyV1("f".repeat(64),rows),sourceKeyV1("e".repeat(64),[rows[0]!,["b/mod.rs","3".repeat(64)]]),sourceKeyV1("e".repeat(64),[rows[0]!]),sourceKeyV1("e".repeat(64),[["a2.rs","1".repeat(64)],rows[1]!])]).size).toBe(5);
});

test("the scan before the compile finds the files and the environment names that rustc's own dependency file lists after it, and the key has no gap",async()=>{
 const root=directory(),entry=crate(root),listing=compile(entry,join(root,"out"),{SEMIO_SCAN_FOO:"1"}),oracle=parseDepInfoV1(listing);
 const sources=(await unitSourcesV1(entry,join(root,"src"),undefined,digest))!;
 expect(new Set(sources.files.map(file=>resolve(file)))).toEqual(new Set(oracle.dependencies.map(path=>resolve(root,path))));
 expect([...sources.names]).toEqual(oracle.environment.map(row=>row.name).sort());
 expect(sources.rows.map(row=>row[0])).toEqual(["a.rs","b/deep.rs","b/mod.rs","data.txt","odd/name.rs"]);
 expect(keyGapV1({cwd:root,covered:new Set(sources.files.map(file=>resolve(file))),keyedNames:new Set(sources.names),depInfo:listing})).toEqual([]);
});

test("a change of a module, a nested module, a path module or an included file moves the key, an unchanged copy in another directory has the same key, and an environment name moves the key by its value",async()=>{
 const first=directory(),second=directory(),entry=crate(first),copy=crate(second),key=async(path:string)=>(await unitSourcesV1(path,resolve(path,".."),undefined,digest))!.sha256;
 const base=await key(entry);expect(await key(copy)).toBe(base);
 for(const [path,text] of [["src/a.rs","pub fn g()->u32{9}\n"],["src/b/deep.rs","pub fn k()->u32{9}\n"],["src/odd/name.rs","pub fn m()->u32{9}\n"],["src/data.txt","changed\n"],["src/b/mod.rs","mod deep;\npub fn h()->u32{deep::k()+0}\n"]] as const){
  const other=directory();expect(await key(crate(other,{[path]:text})),path).not.toBe(base);
 }
 const names=["SEMIO_SCAN_FOO","SEMIO_SCAN_BAR"],row=(environment:Record<string,string>)=>sourceEnvironmentV1(names,environment,new Set(),(_,text)=>text);
 expect(row({SEMIO_SCAN_FOO:"1"})).toEqual([["SEMIO_SCAN_FOO","1"],["SEMIO_SCAN_BAR","⟨unset⟩"]]);expect(row({SEMIO_SCAN_FOO:"2"})).not.toEqual(row({SEMIO_SCAN_FOO:"1"}));expect(row({SEMIO_SCAN_FOO:"1",SEMIO_SCAN_BAR:"x"})).not.toEqual(row({SEMIO_SCAN_FOO:"1"}));
 expect(sourceEnvironmentV1(names,{},new Set(["SEMIO_SCAN_FOO"]),(_,text)=>text)).toEqual([["SEMIO_SCAN_BAR","⟨unset⟩"]]);
});

test("the build script output directory is covered, and a file the scan cannot find (an include named by a computed path) or an environment name nobody scanned is a gap",async()=>{
 const root=directory(),out=join(root,"out-dir");mkdirSync(out,{recursive:true});writeFileSync(join(out,"gen.rs"),"pub const G:u32=1;\n");
 const entry=crate(root,{"src/lib.rs":'include!(concat!(env!("OUT_DIR"),"/gen.rs"));\npub const D:&str=include_str!(concat!("da","ta.txt"));\npub const E:&str=env!("SEMIO_SCAN_FOO");\n'}),listing=compile(entry,join(root,"o"),{OUT_DIR:out,SEMIO_SCAN_FOO:"1"});
 const sources=(await unitSourcesV1(entry,join(root,"src"),out,digest))!;
 expect(sources.rows.map(row=>row[0])).toEqual(["⟨out⟩/gen.rs"]);
 const gap=keyGapV1({cwd:root,covered:new Set(sources.files.map(file=>resolve(file))),keyedNames:new Set(sources.names),depInfo:listing});
 expect(gap.some(path=>path.endsWith("data.txt")),JSON.stringify(gap)).toBe(true);expect(gap.some(path=>path.endsWith("gen.rs"))).toBe(false);
 expect(keyGapV1({cwd:root,covered:new Set(sources.files.map(file=>resolve(file))),keyedNames:new Set(),depInfo:listing}).includes("env:SEMIO_SCAN_FOO")).toBe(true);
 const text=listing+"# env-dep:SEMIO_NOBODY_ASKED=1\n";expect(keyGapV1({cwd:root,covered:new Set(sources.files.map(file=>resolve(file))),keyedNames:new Set(sources.names),depInfo:text})).toContain("env:SEMIO_NOBODY_ASKED");
 expect(keyGapV1({cwd:root,covered:new Set(sources.files.map(file=>resolve(file))),keyedNames:new Set(sources.names),depInfo:listing+"# env-dep:CARGO_PKG_NAME=x\n"}).some(item=>item.includes("CARGO_PKG_NAME"))).toBe(false);
});

test("through the wrapper: the same unit is a hit, and a change of a module file, of an included file or of a nested module is a miss under another key (the entry file never changed)",()=>{
 const root=directory(),entry=crate(root),cache=join(root,"cache"),script=resolve(fileURLToPath(new URL(".",import.meta.url)),"../../../📜️script.ts");
 const run=()=>{rmSync(join(root,"out"),{recursive:true,force:true});mkdirSync(join(root,"out"),{recursive:true});const result=spawnSync(process.execPath,[script,"rustc","--crate-name","x","--edition=2021",entry,"--error-format=json","--json=artifacts,future-incompat","--crate-type","lib","--emit=dep-info,metadata,link","--out-dir",join(root,"out"),"-C","metadata=a","-C","extra-filename=-a"],{cwd:root,encoding:"utf8",env:{...toolchain,SEMIO_UNIT_CACHE_ROOT:cache,SEMIO_UNIT_CACHE_LANE:"l1",SEMIO_UNIT_CACHE_LANE_ROOT:join(cache,"l","l1"),SEMIO_UNIT_CACHE_TOOLCHAIN:"t",SEMIO_UNIT_CACHE_INNER:JSON.stringify(["rustc"]),SEMIO_CAS_ROOT:join(root,"cas"),CARGO_MANIFEST_DIR:join(root,"src"),CARGO_PKG_NAME:"x",SEMIO_SCAN_FOO:"1"}});expect(result.status,result.stderr).toBe(0);const rows=readFileSync(join(cache,"x","l1.jsonl"),"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line) as {kind:string;key:string|null});return rows.filter(row=>row.kind==="miss"||row.kind==="hit").at(-1)!;};
 const first=run(),again=run();expect([first.kind,again.kind]).toEqual(["miss","hit"]);expect(again.key).toBe(first.key);
 const keys=new Set([first.key]);
 for(const [path,text] of [["src/a.rs","pub fn g()->u32{9}\n"],["src/data.txt","changed\n"],["src/b/deep.rs","pub fn k()->u32{9}\n"]] as const){writeFileSync(join(root,path),text);const row=run();expect(row.kind,path).toBe("miss");keys.add(row.key);}
 expect(keys.size).toBe(4);
},120000);

test("an incremental compilation has other bytes on every run, one without a session has the same bytes every time, and the command line without the session is that compilation",()=>{
 expect(withoutIncrementalV1(["a","-C","incremental=/x","-Cincremental=/y","-C","opt-level=1","-Cdebuginfo=2","b"])).toEqual(["a","-C","opt-level=1","-Cdebuginfo=2","b"]);
 const root=directory();writeFileSync(join(root,"l.rs"),Array.from({length:120},(_,index)=>`pub fn f${index}(x:u64)->u64{ x.wrapping_mul(${index+3}).rotate_left(${index%13}) }\n`).join(""));
 const build=(label:string,session:boolean,strip=false)=>{const out=join(root,label);mkdirSync(out,{recursive:true});const args=["--crate-name","l","--edition=2021","l.rs","--crate-type","lib","--emit=link","-C","debuginfo=2","-C","metadata=a","-C","extra-filename=-a","--out-dir",out,...session?["-C","incremental="+join(root,"session-"+label)]:[]],run=spawnSync("rustc",strip?[...withoutIncrementalV1(args)]:args,{cwd:root,encoding:"utf8",env:toolchain});expect(run.status,run.stderr).toBe(0);return createHash("sha256").update(readFileSync(join(out,"libl-a.rlib"))).digest("hex");};
 const plain=[build("p1",false),build("p2",false)],stripped=build("s1",true,true),sessions=[build("i1",true),build("i2",true)];
 expect(plain[0]).toBe(plain[1]);expect(stripped,"the stripped command line is the compilation without a session").toBe(plain[0]);
 expect(sessions[0]).not.toBe(sessions[1]);
});

afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
