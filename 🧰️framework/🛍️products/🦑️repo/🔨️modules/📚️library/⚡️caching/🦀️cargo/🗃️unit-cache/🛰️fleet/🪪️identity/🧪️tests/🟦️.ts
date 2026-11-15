import {expect,test} from "bun:test";
import Ajv from "ajv";
import {mkdtempSync,mkdirSync,realpathSync,rmSync,symlinkSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {testPythonV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {slotIdV1 as slotOfC} from "../../../../../../🏗️build/🪢️slots/🟦️.ts";
import {expandRustcArgumentsV1,keyEnvironmentV1,normalizeRustcArgumentsV1,parseRustcUnitV1,unitKeyV1} from "../../../🔏️key/🟦️.ts";
import {cpuIdentityV1,FLEET_POLICY_V1,localTextV1,platformOfV1,portableRootsV1,portableTextV1,remoteKeyV1,slotIdV1,slotPartsV1,type CompilerReleaseV1} from "../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

/** 🪪️ Identity of a unit across machines: the slot equals package C's function and an independent Python derivation on four real-shaped command lines; paths of the working directory, the workspace and every spelling of them become tokens only where the path ends; the fleet key is equal for equal units in different checkouts and moves with the platform and with every key input. */
const python=testPythonV1(),oracle=join(import.meta.dir,"🐍️oracle/🐍️.py");
const run=(command:string,input:unknown):unknown=>{const done=Bun.spawnSync([python!,"-I",oracle,command],{stdin:new TextEncoder().encode(JSON.stringify(input)),stdout:"pipe",stderr:"pipe"});expect(done.exitCode,done.stderr.toString()).toBe(0);return JSON.parse(done.stdout.toString());};
const release=(rustc:string,host:string):CompilerReleaseV1=>({text:"rustc x\nhost: "+host+"\n",host,digest:rustc});

test("slot vectors validate against the schema and the slot equals package C's function",()=>{
 const ajv=new Ajv({strict:false});ajv.addSchema(schema);const validate=ajv.getSchema(`${schema.$id}#/definitions/SlotVector`)!;
 for(const vector of fixture.slots){
  expect(validate(vector),vector.name).toBe(true);
  expect(slotIdV1(vector.parts),vector.name).toBe(slotOfC("rust-unit",vector.parts));
  expect(slotIdV1(vector.parts)).toBe(vector.slot);
 }
});

test("the slot parts of each vector equal the Python derivation and the stored expectation",()=>{
 for(const vector of fixture.slots){
  const unit=parseRustcUnitV1(expandRustcArgumentsV1(vector.arguments,vector.cwd));
  expect(slotPartsV1({release:release(vector.rustc,vector.host),unit,args:vector.arguments,environment:vector.environment,cwd:vector.cwd}),vector.name).toEqual(vector.parts);
 }
 if(python){
  const derived=run("slots",fixture.slots) as {parts:string[];slot:string}[];
  derived.forEach((row,index)=>{expect(row.parts).toEqual(fixture.slots[index]!.parts);expect(row.slot).toBe(fixture.slots[index]!.slot);});
 }
});

test("the slot ignores content (a feature order, the output directory) and separates what it must (features, profile, target, kind, package)",()=>{
 const [library,member]=fixture.slots as [typeof fixture.slots[0],typeof fixture.slots[0]];
 const of=(vector:typeof library,extra:string[]=[],environment:Record<string,string>={})=>{const args=[...vector.arguments,...extra];return slotIdV1(slotPartsV1({release:release(vector.rustc,vector.host),unit:parseRustcUnitV1(args),args,environment:{...vector.environment,...environment},cwd:vector.cwd}));};
 expect(of(library,["--out-dir","/elsewhere"])).toBe(of(library));
 expect(of(library,["--cfg","feature=\"extra\""])).not.toBe(of(library));
 expect(of(library,["-C","opt-level=3"])).not.toBe(of(library));
 expect(of(library,["--target","wasm32-unknown-unknown"])).not.toBe(of(library));
 expect(of(library,[],{CARGO_PKG_VERSION:"1.0.230"})).not.toBe(of(library));
 expect(of(member)).not.toBe(of(library));
});

test.skipIf(!python)("portable text replaces a root only where it ends, in every spelling, and agrees with the Python replacement",()=>{
 const roots=[{path:"/work/ws",token:"@@semio-cwd@@"},{path:"/work/ws-other",token:"@@semio-ws@@"}],texts=["/work/ws/crates/a/src/lib.rs","/work/ws-other/x","/work/wsx/y","--cfg=a=/work/ws","C:\\\\work\\\\ws\\\\x","/work/ws.bak/z","env=/work/ws:/work/ws/b"];
 const expected=run("portable",{roots,texts}) as string[];
 expect(texts.map(text=>portableTextV1(text,roots))).toEqual(expected);
 expect(expected[0]).toBe("@@semio-cwd@@/crates/a/src/lib.rs");expect(expected[1]).toBe("@@semio-ws@@/x");expect(expected[2]).toBe("/work/wsx/y");expect(expected[5]).toBe("/work/ws.bak/z");expect(expected[6]).toBe("env=@@semio-cwd@@:@@semio-cwd@@/b");
});

test("every spelling of the working directory is replaced and the restored path is the spelling the environment uses",()=>{
 const directory=mkdtempSync(join(tmpdir(),"fleet-roots-")),real=realpathSync(directory),link=directory+"-link";
 try{
  mkdirSync(join(directory,"pkg"));symlinkSync(directory,link);
  const roots=portableRootsV1({cwd:real,workspace:undefined,hints:[join(link,"pkg")]});
  const text=[join(link,"pkg"),join(real,"pkg"),join(directory,"pkg")].map(path=>portableTextV1(path,roots));
  expect(text.every(row=>row==="@@semio-cwd@@/pkg"),JSON.stringify(text)).toBe(true);
  expect(localTextV1("@@semio-cwd@@/pkg",roots)).toBe(join(link,"pkg"));
  const plain=portableRootsV1({cwd:real,workspace:undefined});expect(localTextV1("@@semio-cwd@@/pkg",plain)).toBe(join(real,"pkg"));
  expect(portableRootsV1({cwd:"/a",workspace:undefined})).toEqual([]);
 }finally{rmSync(link,{force:true});rmSync(directory,{recursive:true,force:true});}
});

const baseInput={toolchain:"tc",normalized:["--crate-name","x","src/lib.rs","-C","metadata=abc"],sourceSha256:"a".repeat(64),externs:[{name:"dep",sha256:"b".repeat(64)}],environment:[["CARGO_MANIFEST_DIR","/work/ws/x"],["CARGO_PKG_NAME","x"]] as [string,string][]};
const library=parseRustcUnitV1(["--crate-name","x","src/lib.rs","--crate-type","lib","--emit=link"]),program=parseRustcUnitV1(["--crate-name","x","src/main.rs","--crate-type","bin","--emit=link"]);
const platform=(unit=library,args:string[]=[],compiler=release("c".repeat(64),"aarch64-apple-darwin"))=>platformOfV1({release:compiler,unit,args,environment:{},toolchain:"tc"});

test("the fleet key is equal in different checkouts and moves with the platform and with every key input",()=>{
 const one=[{path:"/work/ws",token:"@@semio-cwd@@"}],two=[{path:"/other/place/ws",token:"@@semio-cwd@@"}];
 const second={...baseInput,normalized:baseInput.normalized.map(a=>a),environment:[["CARGO_MANIFEST_DIR","/other/place/ws/x"],["CARGO_PKG_NAME","x"]] as [string,string][]};
 expect(remoteKeyV1(platform(),baseInput,one)).toBe(remoteKeyV1(platform(),second,two));
 expect(unitKeyV1(baseInput)).not.toBe(unitKeyV1(second));
 const reference=remoteKeyV1(platform(),baseInput,one),variants=[{...baseInput,toolchain:"other"},{...baseInput,normalized:[...baseInput.normalized,"-O"]},{...baseInput,sourceSha256:"c".repeat(64)},{...baseInput,externs:[{name:"dep",sha256:"d".repeat(64)}]},{...baseInput,environment:[["CARGO_MANIFEST_DIR","/work/ws/x"],["CARGO_PKG_NAME","y"]] as [string,string][]},{...baseInput,environment:[["CARGO_MANIFEST_DIR","/somewhere/else/x"],["CARGO_PKG_NAME","x"]] as [string,string][]}];
 for(const variant of variants)expect(remoteKeyV1(platform(),variant,one)).not.toBe(reference);
 expect(remoteKeyV1(platform(library,[],release("e".repeat(64),"aarch64-apple-darwin")),baseInput,one)).not.toBe(reference);
 expect(remoteKeyV1(platform(library,[],release("c".repeat(64),"x86_64-unknown-linux-gnu")),baseInput,one)).not.toBe(reference);
 expect(remoteKeyV1(platform(library,["-C","target-cpu=native"]),baseInput,one)).not.toBe(reference);
 const normalised=normalizeRustcArgumentsV1(["--crate-name","x","src/lib.rs"],new Map(),"/lane"),environment=keyEnvironmentV1({CARGO_PKG_NAME:"x"},"/lane");
 expect(remoteKeyV1(platform(),{...baseInput,normalized:normalised,environment},one)).toMatch(/^[a-f0-9]{64}$/u);
});

test("the platform names the linker world only for kinds that link and the CPU only for native code",()=>{
 const rlib=platform(library),exe=platform(program),native=platform(library,["-C","target-cpu=native"]);
 expect(rlib.linker).toBe("");expect(exe.linker).toBe(process.platform==="darwin"?"darwin-"+require("node:os").release().split(".")[0]:"");
 expect(rlib.cpu).toBe("");expect(native.cpu).not.toBe("");
 expect(rlib.os).toBe(process.platform);expect(rlib.arch).toBe(process.arch);expect(rlib.rustc).toBe("c".repeat(64));
 const ajv=new Ajv({strict:false});ajv.addSchema(schema);expect(ajv.getSchema(`${schema.$id}#/definitions/Platform`)!(exe)).toBe(true);
 expect(FLEET_POLICY_V1.tokens.cwd).toBe("@@semio-cwd@@");
});

test("a native-CPU unit is bound to the real CPU: the model where the system names one, else the ARM implementer, part and a digest of the feature list, so two different ARM Linux CPUs never share code",()=>{
 const block=(part:string,features:string)=>`processor\t: 0\nBogoMIPS\t: 50.00\nFeatures\t: ${features}\nCPU implementer\t: 0x41\nCPU architecture: 8\nCPU variant\t: 0x3\nCPU part\t: ${part}\nCPU revision\t: 1\n\nprocessor\t: 1\nCPU part\t: 0xdead\n`;
 expect(cpuIdentityV1("Apple M2 Pro",undefined)).toBe("Apple M2 Pro");
 const neoverse=cpuIdentityV1("unknown",block("0xd0c","fp asimd sve")),other=cpuIdentityV1("unknown",block("0xd40","fp asimd sve")),fewer=cpuIdentityV1(undefined,block("0xd0c","fp asimd"));
 expect(neoverse).toMatch(/^cpuinfo:0x41\.8\.0x3\.0xd0c:[0-9a-f]{12}$/u);
 expect(new Set([neoverse,other,fewer]).size).toBe(3);
 expect(cpuIdentityV1("unknown",block("0xd0c","fp asimd sve"))).toBe(neoverse);
 expect(cpuIdentityV1("unknown","processor\t: 0\nmodel name\t: Intel(R) Xeon(R)\nflags\t: fpu sse2 avx2\n")).toMatch(/^cpuinfo::[0-9a-f]{12}$/u);
 expect(cpuIdentityV1("unknown",undefined)).toBe("unknown");expect(cpuIdentityV1(undefined,"")).toBe("unknown");
});
