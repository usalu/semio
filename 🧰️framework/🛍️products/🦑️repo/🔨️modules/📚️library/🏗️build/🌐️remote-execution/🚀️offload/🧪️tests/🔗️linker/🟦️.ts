import {afterEach,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {placementCompatibleV1,placementRequiredV1} from "../../../🗓️scheduler/⚖️placement/🟦️.ts";
import {offloadPlatformV1} from "../../📜️action/🟦️.ts";
import {OFFLOAD_POLICY_V1} from "../../📐️policy/🟦️.ts";
import {detectWorkerRustcV1,offloadLinkerIdentityV1} from "../../🧰️toolchain/🟦️.ts";

const scratch:string[]=[];
afterEach(()=>{for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});});
const host="aarch64-apple-darwin",commit="a".repeat(40);

const shim=`import os, sys
if sys.argv[1:] == ["-vV"]:
    sys.stdout.write("rustc 1.0.0\\ncommit-hash: ${commit}\\nhost: ${host}\\n")
elif sys.argv[1:] == ["--print", "sysroot"]:
    print(os.environ["SEMIO_TEST_RUSTC_SYSROOT"])
else:
    sys.exit(64)
`;

function sysroot(files:Readonly<Record<string,string>>):string{
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"linker-"));scratch.push(root);
 const bin=join(root,"lib","rustlib",host,"bin");mkdirSync(join(bin,"gcc-ld"),{recursive:true});mkdirSync(join(root,"lib","rustlib",host,"lib"),{recursive:true});mkdirSync(join(root,"bin"),{recursive:true});
 for(const [name,text] of Object.entries(files))writeFileSync(join(bin,name),text);
 writeFileSync(join(root,"bin","rustc"),shim,{mode:0o755});
 return root;
}
const healthy={"rust-lld":"lld 21.1.0 bitcode 0xdec04342","wasm-component-ld":"component linker 0.5.20","gcc-ld/wasm-ld":"wrapper wasm","gcc-ld/ld.lld":"wrapper elf"};
const oracle=(path:string):string=>{const run=spawnSync(testPythonV1()??"python3",["-I",join(import.meta.dir,"🐍️.py"),path],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return run.stdout.trim();};

test("the linker identity is the digest of the linker files and equals the independent Python oracle; the optional LLVM tools beside them do not count",()=>{
 const one=sysroot(healthy),two=sysroot({...healthy,llc:"optional tool",opt:"optional tool"});
 const identity=offloadLinkerIdentityV1(one);
 expect(identity).toMatch(/^[0-9a-f]{16}$/u);
 expect(identity).toBe(oracle(one));
 expect(offloadLinkerIdentityV1(two)).toBe(identity);
 expect<unknown>(oracle(two)).toBe(identity);
});

test("a compiler with a different or damaged linker has another identity although its version text is the same; a sysroot without linkers has none; the memo follows a changed file",()=>{
 const good=sysroot(healthy),older=sysroot({...healthy,"rust-lld":"lld 20.1.0 bitcode 0xdec04341"}),cut=sysroot({...healthy,"rust-lld":""}),none=sysroot({});
 expect(offloadLinkerIdentityV1(older)).not.toBe(offloadLinkerIdentityV1(good));
 expect(offloadLinkerIdentityV1(cut)).not.toBe(offloadLinkerIdentityV1(good));
 expect(offloadLinkerIdentityV1(older)).toBe(oracle(older));
 rmSync(join(none,"lib","rustlib",host,"bin"),{recursive:true,force:true});
 expect(offloadLinkerIdentityV1(none)).toBeUndefined();
 const memo=mkdtempSync(join(tmpdir(),"linker-memo-"));scratch.push(memo);
 const before=offloadLinkerIdentityV1(good,memo);
 expect(JSON.parse(readFileSync(join(memo,"offload-linker.json"),"utf8")).identity).toBe(before);
 expect(offloadLinkerIdentityV1(good,memo)).toBe(before);
 writeFileSync(join(good,"lib","rustlib",host,"bin","rust-lld"),"lld 22.0.0 something longer than before");
 expect(offloadLinkerIdentityV1(good,memo)).not.toBe(before);
 expect(offloadLinkerIdentityV1(good,memo)).toBe(oracle(good));
});

test("a worker offers the identity of its linkers and a linked unit that requires it is never placed on a worker whose linker differs (the g1-m06 case: bitcode that rust-lld could not read)",()=>{
 const good=sysroot(healthy),damaged=sysroot({...healthy,"rust-lld":"lld 19 cannot read this bitcode"});
 const offer=(root:string)=>detectWorkerRustcV1(root,{...process.env,SEMIO_TEST_RUSTC_SYSROOT:root},[testPythonV1()??"python3","-I",join(root,"bin","rustc")]);
 const goodOffer=offer(good)!,damagedOffer=offer(damaged)!,name=OFFLOAD_POLICY_V1.property.linker;
 const linkerOf=(properties:readonly {name:string;value:string}[])=>properties.find(row=>row.name===name)?.value;
 expect(linkerOf(goodOffer.properties)).toBe(offloadLinkerIdentityV1(good));
 expect(linkerOf(damagedOffer.properties)).toBe(offloadLinkerIdentityV1(damaged));
 expect(linkerOf(goodOffer.properties)).not.toBe(linkerOf(damagedOffer.properties));
 const toolchain={...goodOffer.toolchain},requester=offloadPlatformV1([{name:"Arch",value:"arm64"},{name:"OSFamily",value:"macos"}],toolchain,toolchain.host,"g1-server",2600,{queue:true,linker:linkerOf(goodOffer.properties)!});
 expect(linkerOf(requester)).toBe(linkerOf(goodOffer.properties));
 const worker=(id:string,offered:typeof goodOffer)=>({id,name:id,origin:id,properties:[{name:"Arch",value:"arm64"},{name:"OSFamily",value:"macos"},{name:"semio-toolchain",value:toolchain.digest},{name:"semio-target",value:toolchain.host},...offered.properties],concurrency:2,running:0,waiting:true,draining:false,refusedUntilMs:0});
 const operation={id:"o",platform:requester,tried:new Set<string>(),requester:"g1-server"},required=placementRequiredV1(requester,undefined);
 expect(placementCompatibleV1(operation,worker("g1-m12",goodOffer),required)).toBe(true);
 expect(placementCompatibleV1(operation,worker("g1-m06",damagedOffer),required)).toBe(false);
 const plain=offloadPlatformV1([{name:"Arch",value:"arm64"},{name:"OSFamily",value:"macos"}],toolchain,undefined,"g1-server",0,{});
 expect(plain.some(row=>row.name===name),"a unit that links nothing does not ask for a linker").toBe(false);
 expect(placementCompatibleV1({...operation,platform:plain},worker("g1-m06",damagedOffer),placementRequiredV1(plain,undefined))).toBe(true);
});
