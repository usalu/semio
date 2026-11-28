import {expect,test} from "bun:test";
import {buildKeyEnvironmentV1,keyEnvironmentV1,normalizeRustcArgumentsV1,unitKeyV1} from "../../🔏️key/🟦️.ts";
import {rustSourceRemapArgumentsV1} from "../../🧭️remap/🟦️.ts";
import {unitThreadsArgumentsV1,unitThreadsVerdictV1} from "../../🧵️threads/🟦️.ts";

type HostV1=Readonly<{name:string;home:string;checkout:string;sysroot:string;flags:string|undefined;threads:readonly string[]}>;

const COMMIT="9f36de775bc636c8e88c31a173c2bcb48b9d9f1f",GUARD={level:"calm" as const,availableMegabytes:9000,guardMegabytes:1637};
const cargoArguments=(host:HostV1,extra:readonly string[])=>["--crate-name","dep","--edition=2021",host.checkout+"/dep/src/lib.rs","--error-format=json","--json=diagnostic-rendered-ansi,artifacts","--crate-type","lib","--emit=dep-info,metadata,link","-C","embed-bitcode=no","-C","metadata=abc","-C","extra-filename=-abc","--out-dir",host.checkout+"/.cache/t/debug/deps","-L","dependency="+host.checkout+"/.cache/t/debug/deps","--remap-path-prefix="+host.home+"/.cargo/registry/src=",...extra];

const calm:HostV1={name:"calm macOS host",home:"/Users/ueli",checkout:"/Users/ueli/Documents/semio",sysroot:"/Users/ueli/.rustup/toolchains/nightly-2026-07-20",flags:"-Z\u001fthreads=0\u001f-Z\u001fthreads=0",threads:[]};
const pressed:HostV1={name:"pressed Linux host",home:"/home/deck",checkout:"/home/deck/semio",sysroot:"/opt/toolchains/n",flags:undefined,threads:unitThreadsArgumentsV1(unitThreadsVerdictV1({args:["--crate-name","dep","--out-dir","x","-Z","threads=0"],budgetMegabytes:1536,host:{...GUARD,level:"danger",availableMegabytes:900}}))};

const argumentsOf=(host:HostV1,extra:readonly string[]=[])=>{
 const given=[...cargoArguments(host,[]),...host.flags===undefined?[]:["-Z","threads=0","-Z","threads=0"],...extra];
 return[...rustSourceRemapArgumentsV1(given,host.sysroot,COMMIT),...given,...host.threads];
};
const keyOf=(host:HostV1,extra:readonly string[]=[],environment:Record<string,string|undefined>={})=>unitKeyV1({toolchain:"t",normalized:normalizeRustcArgumentsV1(argumentsOf(host,extra),new Map(),host.checkout),sourceSha256:"a".repeat(64),externs:[],environment:keyEnvironmentV1({CARGO_PKG_NAME:"dep",CARGO_MANIFEST_DIR:host.checkout+"/dep",...environment},host.checkout)});

test("the host that launches Cargo with flags in the environment and the host that does not give one unit key",()=>{
 expect(pressed.threads,"the pressed host really asks for another front-end thread count").toEqual(["-Z","threads=1"]);
 const withFlags=keyOf(calm,[],{CARGO_ENCODED_RUSTFLAGS:calm.flags}),without=keyOf(pressed);
 expect(keyEnvironmentV1({CARGO_ENCODED_RUSTFLAGS:calm.flags!,CARGO_PKG_NAME:"dep"},undefined).map(row=>row[0]),"a compile unit's flags are its arguments, never its environment").toEqual(["CARGO_PKG_NAME"]);
 expect(withFlags,"launch path, memory pressure, home, checkout and sysroot differ; the key does not").toBe(without);
 expect(keyOf(pressed,[],{CARGO_ENCODED_RUSTFLAGS:""})).toBe(without);
});

test("the front-end thread count never splits a unit, whatever the count, spelling or place",()=>{
 const plain=keyOf(calm);
 for(const extra of[["-Z","threads=1"],["-Zthreads=8"],["-Z","threads=0","-Z","threads=2"]])expect(keyOf(calm,extra),extra.join(" ")).toBe(plain);
 expect(keyOf(calm,["-C","target-cpu=native"]),"real flags are still inputs").not.toBe(plain);
 expect(keyOf(calm,["--cfg","threads=\"0\""]),"a cfg named threads is no front-end flag").not.toBe(plain);
});

test("the flags a build script run observes are the same on every host, with the machine parts and the thread count removed",()=>{
 const flagsOf=(flags:string|undefined)=>buildKeyEnvironmentV1({TARGET:"t",HOST:"h",CARGO_ENCODED_RUSTFLAGS:flags},undefined);
 const reference=flagsOf("");
 for(const flags of["-Z\u001fthreads=0\u001f-Z\u001fthreads=0","-Z\u001fthreads=1","-Zthreads=4"])expect(flagsOf(flags),JSON.stringify(flags)).toEqual(reference);
 const remapped=(home:string)=>flagsOf("-Z\u001fthreads=0\u001f--remap-path-prefix="+home+"/.cargo/registry/src=\u001f-C\u001flink-arg=-zstack-size=8388608");
 expect(remapped("/Users/ueli")).toEqual(remapped("/home/deck"));
 expect(remapped("/Users/ueli"),"the link arguments are inputs").not.toEqual(reference);
});
