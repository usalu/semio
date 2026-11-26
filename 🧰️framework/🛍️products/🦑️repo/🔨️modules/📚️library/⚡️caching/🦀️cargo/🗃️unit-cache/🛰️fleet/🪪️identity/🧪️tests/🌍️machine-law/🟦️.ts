import {expect,test} from "bun:test";
import {join} from "node:path";
import {testPythonV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {keyEnvironmentV1,normalizeRustcArgumentsV1,parseRustcUnitV1,sha256V1,type RustcUnitV1} from "../../../../🔏️key/🟦️.ts";
import {hostIndependentV1,neutralReleaseV1,platformOfV1,portableRootsV1,portableTextV1,remoteKeyV1,slotIdV1,slotPartsV1,type CompilerReleaseV1,type MachineV1} from "../../🟦️.ts";
import {unitToolchainOfV1} from "../../../🟦️.ts";

/** 🌍️ Law: the key of one unit is a function of what the unit is, never of the machine that asks. Three simulated machines (macOS at ~/Documents/semio, Linux at /home/deck/semio, Windows at C:\semio) with different homes, host names, Cargo and Rust homes and path spellings key the same unit. An output that cannot depend on the machine (a WebAssembly library) is keyed alike by all of them; an output that can (native code, proc-macros, linked artifacts, native-CPU code) is keyed alike only by machines of one platform. */
type Simulated=Readonly<{name:string;machine:MachineV1;host:string;separator:"/"|"\\";root:string;home:string;hostname:string;cargoHome:string;rustupHome:string}>;
const machines:readonly Simulated[]=[
 {name:"mac",machine:{os:"darwin",arch:"arm64",darwinMajor:"27",cpuModel:"Apple M4",cpuInfo:undefined},host:"aarch64-apple-darwin",separator:"/",root:"/Users/ueli/Documents/semio",home:"/Users/ueli",hostname:"ueli-mac.fritz.box",cargoHome:"/Users/ueli/.cargo",rustupHome:"/Users/ueli/.rustup"},
 {name:"deck",machine:{os:"linux",arch:"x64",darwinMajor:"",cpuModel:"AMD Custom APU 0405",cpuInfo:undefined},host:"x86_64-unknown-linux-gnu",separator:"/",root:"/home/deck/semio",home:"/home/deck",hostname:"steamdeck",cargoHome:"/home/deck/.cargo",rustupHome:"/home/deck/.rustup"},
 {name:"win",machine:{os:"win32",arch:"x64",darwinMajor:"",cpuModel:"AMD Ryzen 9",cpuInfo:undefined},host:"x86_64-pc-windows-msvc",separator:"\\",root:"C:\\semio",home:"C:\\Users\\deck",hostname:"DESKTOP-7QK2",cargoHome:"C:\\Users\\deck\\.cargo",rustupHome:"C:\\Users\\deck\\.rustup"},
 {name:"mac2",machine:{os:"darwin",arch:"arm64",darwinMajor:"27",cpuModel:"Apple M4",cpuInfo:undefined},host:"aarch64-apple-darwin",separator:"/",root:"/Volumes/Work/ci/checkout/semio",home:"/Users/builder",hostname:"build-07",cargoHome:"/Volumes/Work/cargo",rustupHome:"/Volumes/Work/rustup"},
];
const releaseText=(host:string,commit="abc123def456")=>`rustc 1.97.0-nightly (${commit} 2026-07-19)\nbinary: rustc\ncommit-hash: ${commit}0000\ncommit-date: 2026-07-19\nhost: ${host}\nrelease: 1.97.0-nightly\nLLVM version: 22.1.0\n`;
const releaseOf=(machine:Simulated,commit?:string):CompilerReleaseV1=>{const text=releaseText(machine.host,commit);return{text,host:machine.host,digest:sha256V1(text),neutral:sha256V1(text.split("\n").filter(line=>!line.startsWith("host: ")).join("\n"))};};

type Kind={target?:string;crateType:string;extra?:readonly string[];emit?:string};
function unitOn(machine:Simulated,kind:Kind,options:{commit?:string;source?:string;home?:boolean}={}){
 const s=machine.separator,at=(...parts:string[])=>[machine.root,...parts].join(s),relative=["crates","core","src","lib.rs"].join(s);
 const args=["--crate-name","core","--edition=2021",relative,"--error-format=json","--json=diagnostic-rendered-ansi,artifacts,future-incompat","--crate-type",kind.crateType,`--emit=${kind.emit??"dep-info,metadata,link"}`,"-C","embed-bitcode=no","-C","debuginfo=0","-C","metadata=0a1b2c3d4e5f6071","-C","extra-filename=-0a1b2c3d4e5f6071","--out-dir",at(".cache","b","t","deps"),...kind.target?["--target",kind.target]:[],"-L","dependency="+at(".cache","b","t","deps"),"--extern","dep="+at(".cache","b","t","deps","libdep-1122334455667788.rlib"),`--remap-path-prefix=${machine.root}=.`,`--remap-path-prefix=${machine.cargoHome}${s}registry${s}src=/registry`,...kind.extra??[]];
 const environment={CARGO_MANIFEST_DIR:at("crates","core"),CARGO_PKG_NAME:"core",CARGO_PKG_VERSION:"0.1.0",CARGO_PKG_DESCRIPTION:"The core.\nSecond line.",CARGO_CRATE_NAME:"core",HOME:machine.home,USERPROFILE:machine.home,HOSTNAME:machine.hostname,COMPUTERNAME:machine.hostname,CARGO_HOME:machine.cargoHome,RUSTUP_HOME:machine.rustupHome,PWD:machine.root};
 const parsed=parseRustcUnitV1(args),release=releaseOf(machine,options.commit),seen=hostIndependentV1({release,unit:parsed,args})?neutralReleaseV1(release):release,platform=platformOfV1({release:seen,unit:parsed,args,environment,toolchain:"launcher-digest-nightly-2026-07-20",machine:machine.machine});
 const toolchain=unitToolchainOfV1(release,parsed,args,()=>release.text);
 const roots=portableRootsV1({cwd:machine.root,workspace:machine.root,hints:[environment.CARGO_MANIFEST_DIR,environment.PWD]});
 const input={toolchain,normalized:normalizeRustcArgumentsV1(args,new Map([[at(".cache","b","t","deps","libdep-1122334455667788.rlib"),"e".repeat(64)]]),undefined),sourceSha256:sha256V1(options.source??"pub fn id()->u32{1}\n"),externs:[{name:"dep",sha256:"e".repeat(64)}],environment:keyEnvironmentV1(environment,undefined)};
 const slot=slotIdV1(slotPartsV1({release:seen,unit:parsed,args,environment,cwd:machine.root}));
 return{key:remoteKeyV1(platform,input,roots,[]),platform,slot,parsed,input,roots};
}
const keys=(kind:Kind,options?:Parameters<typeof unitOn>[2])=>Object.fromEntries(machines.map(machine=>[machine.name,unitOn(machine,kind,options).key]));
const wasm:Kind={target:"wasm32-wasip2",crateType:"lib"};

test("a WebAssembly library has one key on macOS, Linux and Windows machines with different homes, host names, Cargo homes and worktree roots, and one slot",()=>{
 const k=keys(wasm);
 expect(new Set(Object.values(k)).size,JSON.stringify(k)).toBe(1);
 expect(new Set(machines.map(machine=>unitOn(machine,wasm).slot)).size).toBe(1);
 const platform=unitOn(machines[0]!,wasm).platform;
 expect(platform).toMatchObject({os:"any",arch:"any",host:"any",linker:"",cpu:"",toolchain:""});
 for(const machine of machines)expect(unitOn(machine,wasm).platform.rustc).toBe(releaseOf(machines[0]!).neutral!);
});

test("what the key must still see: a changed source, another compiler build, another crate type or target, native-CPU code, and a different relative position of the crate in the worktree",()=>{
 const reference=keys(wasm).mac!;
 expect(keys(wasm,{source:"pub fn id()->u32{2}\n"}).mac).not.toBe(reference);
 expect(keys(wasm,{commit:"ffffffffffff"}).mac,"another compiler build").not.toBe(reference);
 expect(keys({...wasm,target:"wasm32-unknown-unknown"}).mac).not.toBe(reference);
 expect(keys({...wasm,extra:["-C","target-cpu=native"]}).mac).not.toBe(reference);
 expect(keys({...wasm,crateType:"rlib"}).mac).not.toBe(reference);
 for(const bound of[{...wasm,crateType:"cdylib"},{...wasm,crateType:"proc-macro"},{...wasm,crateType:"bin"},{...wasm,crateType:"staticlib"}]){
  expect(hostIndependentV1({release:releaseOf(machines[0]!),unit:parseRustcUnitV1(["--crate-name","x","--crate-type",bound.crateType,"--target","wasm32-wasip2","a.rs"]),args:[]}),bound.crateType).toBe(false);
 }
 expect(hostIndependentV1({release:releaseOf(machines[0]!),unit:parseRustcUnitV1(["--crate-name","x","--crate-type","lib","--target","aarch64-apple-darwin","a.rs"]),args:[]}),"the host's own target").toBe(false);
});

test("native code is keyed by platform: equal on machines of one platform whatever their paths, names and homes, different between operating systems and architectures",()=>{
 const native:Kind={crateType:"lib"},k=keys(native);
 expect(k.mac,"two macOS machines with different roots, homes and names").toBe(k.mac2);
 expect(new Set([k.mac,k.deck,k.win]).size).toBe(3);
 for(const kind of[{crateType:"proc-macro"},{...wasm,crateType:"cdylib"},{...wasm,crateType:"lib",extra:["-C","target-cpu=native"]}] as Kind[]){
  const row=keys(kind);expect(row.mac,JSON.stringify(kind)).toBe(row.mac2);expect(new Set([row.mac,row.deck,row.win]).size,JSON.stringify(kind)).toBe(3);
 }
 expect(unitOn(machines[0]!,native).platform).toMatchObject({os:"darwin",arch:"arm64",host:"aarch64-apple-darwin"});
 expect(unitOn(machines[2]!,native).platform).toMatchObject({os:"win32",host:"x86_64-pc-windows-msvc"});
});

test("the portable text of a Windows root agrees with Python's Windows path library: the root becomes the token and the rest is spelled with forward slashes, whatever the separator",()=>{
 const python=testPythonV1();if(python===null)return;
 const vectors=[["C:\\semio","C:\\semio\\crates\\core"],["C:\\semio","C:\\semio\\crates\\core\\src\\lib.rs"],["C:\\Work\\semio ws","C:\\Work\\semio ws\\a"],["C:\\semio","C:\\semio"],["C:\\semio","C:\\semiofoo\\x"],["\\\\server\\share\\semio","\\\\server\\share\\semio\\crates\\core"]];
 const script="import json,sys\nfrom pathlib import PureWindowsPath\nrows=json.load(sys.stdin)\nout=[]\nfor root,path in rows:\n    r=PureWindowsPath(root);p=PureWindowsPath(path)\n    try:\n        rel=p.relative_to(r).as_posix()\n        out.append('@@semio-cwd@@' if rel=='.' else '@@semio-cwd@@/'+rel)\n    except ValueError:\n        out.append(path)\nprint(json.dumps(out))";
 const done=Bun.spawnSync([python,"-I","-c",script],{stdin:new TextEncoder().encode(JSON.stringify(vectors)),stdout:"pipe",stderr:"pipe"});expect(done.exitCode,done.stderr.toString()).toBe(0);
 const expected=JSON.parse(done.stdout.toString()) as string[];
 const ours=vectors.map(([root,path])=>portableTextV1(path!,portableRootsV1({cwd:root!,workspace:undefined})));
 expect(ours).toEqual(expected);
 expect(portableTextV1(join("/x","y"),portableRootsV1({cwd:"C:\\semio",workspace:undefined}))).toBe(join("/x","y"));
});

