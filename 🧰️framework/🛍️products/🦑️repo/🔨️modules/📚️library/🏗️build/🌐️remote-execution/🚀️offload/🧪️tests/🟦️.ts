import {expect,test} from "bun:test";
import Ajv from "ajv";
import {existsSync,readdirSync} from "node:fs";
import {isAbsolute,join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {dependencyClosureV1,walkFilesV1} from "../🌳️inputs/🟦️.ts";
import {includedSourcesV1,offloadRealTextV1,planOffloadV1,type OffloadPlanV1} from "../🗺️paths/🟦️.ts";
import {OFFLOAD_POLICY_V1,extractOffloadFlagV1,offloadModeV1,offloadTextV1} from "../📐️policy/🟦️.ts";
import {offloadPlatformV1,parseRustcIdentityV1} from "../📜️action/🟦️.ts";
import {createTestCheckoutV1,externHash,testHostProbeV1,testRustEnvironmentV1} from "./🧰️workspace/🟦️.ts";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {availableParallelism,tmpdir} from "node:os";
import {BUILD_HOST_CAPACITY_POLICY_V1} from "../../../🌡️host-capacity/🎛️control/🟦️.ts";
import {localUnitAdmissionV1,offloadFanOutJobsV1,readUnitCostV1,workspaceRootFromV1,worthARoundTripV1,writeUnitCostV1,readOffloadMemoV1,slotQueueMillisecondsV1,writeOffloadMemoV1} from "../🚦️admission/🟦️.ts";
import {releaseImplicitTokenV1} from "../🎟️token/🟦️.ts";
import {bumpOffloadCountersV1,readOffloadCountersV1,readOffloadReasonCountersV1} from "../📊️counters/🟦️.ts";
import {tryClaimV1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🔒️lock/🟦️.ts";

const ajv=new Ajv({strict:false});ajv.addSchema(schema);

function chain(label:string){
 const checkout=createTestCheckoutV1(label);
 checkout.compile("a","pub fn a()->u32{1}\n");
 checkout.compile("b","pub fn b()->u32{a::a()+1}\n",["a"]);
 checkout.crate("c","pub fn c()->u32{b::b()+1}\n");
 return checkout;
}
const plan=(checkout:ReturnType<typeof chain>):OffloadPlanV1=>{
 const result=planOffloadV1({args:checkout.arguments("c",["b"]),environment:{...checkout.environment("c"),HOME:"/Users/someone",PATH:"/x",SEMIO_UNIT_CACHE_ROOT:"/c",CARGO_MAKEFLAGS:"-j2"},cwd:checkout.root});
 if("declined" in result)throw Error(result.declined+" "+result.detail);
 return result;
};

test("the policy and the messages validate against the closed schema, in both validators, and both languages carry the same texts",()=>{
 const ours=validateJsonSchemaSubset({$ref:"#/definitions/Policy"},schema.policy,schema as never);
 expect(ours).toEqual([]);
 expect(ajv.validate({$ref:`${schema.$id}#/definitions/Policy`},schema.policy),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate({$ref:`${schema.$id}#/definitions/Policy`},{...schema.policy,extra:1})).toBe(false);
 expect(Object.keys(schema.messages.en).sort()).toEqual(Object.keys(schema.messages.de).sort());
 for(const key of ["no-idle-machine","queue-timeout","infrastructure","path-dependent-output"])expect(offloadTextV1("de",`reason.${key}`)).not.toBe(offloadTextV1("en",`reason.${key}`));
 expect(ajv.validate({$ref:`${schema.$id}#/definitions/Event`},{version:1,atMs:1,pid:2,kind:"offload-run",crate:"a",key:null,milliseconds:3,waitedMs:0,origin:"w"})).toBe(true);
});

test("the mode reads always, auto and never and falls back to auto",()=>{
 expect(offloadModeV1({SEMIO_OFFLOAD:"always"})).toBe("always");expect(offloadModeV1({SEMIO_OFFLOAD:"never"})).toBe("never");
 expect(offloadModeV1({})).toBe("auto");expect(offloadModeV1({SEMIO_OFFLOAD:"sometimes"})).toBe("auto");
});

test("the compiler identity is the commit, the host and the first 16 hex of the sha-256 of the exact text; platform properties are sorted and carry the scheduler's control properties",()=>{
 const text=spawnSync("rustc",["-vV"],{encoding:"utf8",env:testRustEnvironmentV1()}).stdout,identity=parseRustcIdentityV1(text)!;
 expect(identity.commit).toMatch(/^[0-9a-f]{40}$/u);expect(identity.host).toMatch(/-/u);
 const python=spawnSync("python3",["-I","-c","import hashlib,sys;print(hashlib.sha256(sys.stdin.buffer.read()).hexdigest()[:16])"],{input:text,encoding:"utf8"}).stdout.trim();
 expect(identity.digest).toBe(python);
 const rows=offloadPlatformV1([{name:"OSFamily",value:"macos"},{name:"Arch",value:"arm64"},{name:"semio-target",value:"stale"}],{...identity,targets:[]},"wasm32-wasip2","g1-m03");
 expect(rows.map(row=>row.name)).toEqual([...rows.map(row=>row.name)].sort());
 expect(Object.fromEntries(rows.map(row=>[row.name,row.value]))).toEqual({Arch:"arm64",OSFamily:"macos","semio-offload":"if-idle","semio-origin":"g1-m03","semio-rustc-target-wasm32-wasip2":"1","semio-target":identity.host,"semio-toolchain":identity.digest});
 expect(parseRustcIdentityV1("rustc nothing")).toBeUndefined();
});

test("the dependency closure follows the extra-filename each artifact records, so a direct extern brings its transitive dependencies and the rmeta beside an rlib",()=>{
 const checkout=chain("closure");
 const closure=dependencyClosureV1(checkout.deps,[join(checkout.deps,`libb-${externHash("b")}.rlib`)])!;
 expect(closure).toContain(`liba-${externHash("a")}.rlib`);expect(closure).toContain(`liba-${externHash("a")}.rmeta`);
 expect(closure).toContain(`libb-${externHash("b")}.rmeta`);
 expect(dependencyClosureV1(checkout.deps,[join(checkout.deps,`liba-${externHash("a")}.rlib`)])).not.toContain(`libb-${externHash("b")}.rlib`);
 expect(dependencyClosureV1(join(checkout.base,"missing"),[])).toBeUndefined();
});

test("walking a package is ordered, skips ignored directories and refuses beyond its bounds",()=>{
 const checkout=chain("walk"),files=walkFilesV1(join(checkout.root,"pk","c"),["target"])!;
 expect([...files.keys()]).toEqual(["Cargo.toml","src/lib.rs"]);
 expect(walkFilesV1(join(checkout.root,"pk","c"),[],1)).toBeUndefined();
 expect(walkFilesV1(join(checkout.root,"pk","c"),[],10,5)).toBeUndefined();
});

test("a plan is path-independent: two checkouts at different absolute paths yield the same arguments, environment and virtual files, and no requester path remains",()=>{
 const left=chain("left"),right=chain("right"),a=plan(left),b=plan(right);
 expect(left.root).not.toBe(right.root);
 expect(a.arguments).toEqual(b.arguments);expect(a.environment).toEqual(b.environment);expect([...a.files.keys()]).toEqual([...b.files.keys()]);
 for(const text of [...a.arguments,...Object.values(a.environment)]){expect(text.includes(left.base),text).toBe(false);}
 expect(a.arguments).toContain(OFFLOAD_POLICY_V1.virtual.out);
 expect(a.arguments.some(argument=>argument.startsWith("dependency="+OFFLOAD_POLICY_V1.virtual.deps))||a.arguments.includes("dependency="+OFFLOAD_POLICY_V1.virtual.deps)).toBe(true);
 expect(a.arguments.filter(argument=>isAbsolute(argument))).toEqual([]);
 expect(a.arguments.join(" ")).not.toContain("-C incremental");
 expect(a.environment.PATH).toBe(OFFLOAD_POLICY_V1.environment.path);expect(a.environment.SEMIO_TOOL_RUSTC).toBe("@tool");
 expect(a.environment.SEMIO_ABSOLUTE).toBe("CARGO_MANIFEST_DIR");expect(a.environment.SEMIO_REL_CARGO_MANIFEST_DIR).toBe("pk/c");
 for(const denied of ["HOME","SEMIO_UNIT_CACHE_ROOT","CARGO_MAKEFLAGS"])expect(a.environment[denied],denied).toBeUndefined();
 expect(a.environment.CARGO_PKG_NAME).toBe("c");expect(a.environment.RUST_MIN_STACK).toBe("8388608");
 const files=[...a.files.keys()];
 expect(files).toContain("pk/c/src/lib.rs");expect(files).toContain(`${OFFLOAD_POLICY_V1.virtual.deps}/libb-${externHash("b")}.rlib`);expect(files).toContain(`${OFFLOAD_POLICY_V1.virtual.deps}/liba-${externHash("a")}.rmeta`);
 for(const real of a.files.values())expect(existsSync(real),real).toBe(true);
});

test("units that cannot be made path-independent are declined with a reason instead of being half-mapped",()=>{
 const checkout=chain("decline"),base={environment:checkout.environment("c"),cwd:checkout.root};
 const declined=(args:readonly string[],environment=base.environment)=>{const result=planOffloadV1({args,environment,cwd:checkout.root});return "declined" in result?result.declined:"planned";};
 expect(declined([...checkout.arguments("c",["b"]),"-l","static=foo"])).toBe("not-portable");
 expect(declined([...checkout.arguments("c",["b"]),"-L","native=/x"])).toBe("not-portable");
 expect(declined([...checkout.arguments("c",["b"]),"-o","/x"])).toBe("unmapped-path");
 expect(declined([...checkout.arguments("c",["b"]),"-C","linker=/Users/x/ld"])).toBe("unmapped-path");
 expect(declined([...checkout.arguments("c",["b"]),"-C",`link-arg=${checkout.root}/x`])).toBe("unmapped-path");
 expect(declined(checkout.arguments("c",["b"]),{...base.environment,SOMETHING:`${checkout.root}/x`})).toBe("unmapped-path");
 expect(declined(checkout.arguments("c",["b"]),{...base.environment,CARGO_MANIFEST_DIR:undefined as never})).toBe("unmapped-path");
 expect(declined(checkout.arguments("c",["missing"]))).toBe("unreadable-input");
});

test("text from the other machine is translated back in one pass, longest names first, with its root and the remapped root",()=>{
 const mappings=[{real:"/r/target/deps",virtual:".semio-offload/d"},{real:"/r/other",virtual:".semio-offload/d1"},{real:"target/deps",virtual:".semio-offload/o"}];
 const text=offloadRealTextV1('{"artifact":".semio-offload/o/liba.rmeta"} .semio-offload/d1/x .semio-offload/d/y /w/o/root/.semio-offload/d/z /semio/root/pk/c .semio-offload/dx',mappings,"/w/o/root","/r");
 expect(text).toBe('{"artifact":"target/deps/liba.rmeta"} /r/other/x /r/target/deps/y /r/target/deps/z /r/pk/c .semio-offload/dx');
 expect(offloadRealTextV1("plain",[],undefined,"/r")).toBe("plain");
 expect(readdirSync(".").length).toBeGreaterThan(0);
});

const probeOf=(name:"critical-pressure-gigabyte-swap"|"quiet-no-swap")=>testHostProbeV1(name,name==="quiet-no-swap");

test("the machine's single admission decision says wait under critical pressure and admit on a calm host, through the injected probe",async()=>{
 const roomy={...BUILD_HOST_CAPACITY_POLICY_V1,load:{...BUILD_HOST_CAPACITY_POLICY_V1.load,perCoreCeiling:100000}};
 const pressured=await localUnitAdmissionV1({probe:probeOf("critical-pressure-gigabyte-swap"),policy:roomy});
 expect(pressured.verdict).toBe("wait");expect(["pressure-critical","swap","swap-activity"]).toContain(pressured.reason);
 const calm=await localUnitAdmissionV1({probe:probeOf("quiet-no-swap"),policy:roomy});
 expect(calm.verdict).toBe("admit");
 const loaded=await localUnitAdmissionV1({probe:probeOf("quiet-no-swap"),policy:{...BUILD_HOST_CAPACITY_POLICY_V1,load:{...BUILD_HOST_CAPACITY_POLICY_V1.load,perCoreCeiling:0}}});
 expect(loaded.verdict).toBe("wait");expect(loaded.reason).toBe("load");
});

test("verdicts are shared within their window and expire after it",()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"offload-memo-")));
 writeOffloadMemoV1(root,"declined",{atMs:1000});
 expect(readOffloadMemoV1(root,"declined",1500,3000)).toEqual({atMs:1000});
 expect(readOffloadMemoV1(root,"declined",4001,3000)).toBeUndefined();expect(readOffloadMemoV1(root,"declined",500,3000)).toBeUndefined();expect(readOffloadMemoV1(root,"other",1500,3000)).toBeUndefined();
 rmSync(root,{recursive:true,force:true});
});

test("a unit queues here when every slot is held: the estimate is the age of the oldest live waiter, and zero while a slot is free",()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"offload-slots-"))),slots=join(root,"s"),tickets=join(root,"q");
 mkdirSync(tickets,{recursive:true});
 expect(slotQueueMillisecondsV1(root,2,5000)).toBe(0);expect(slotQueueMillisecondsV1(root,0,5000)).toBe(0);
 const first=tryClaimV1(slots,"0")!,second=tryClaimV1(slots,"1")!;
 expect(slotQueueMillisecondsV1(root,2,5000)).toBe(1);
 writeFileSync(join(tickets,`${String(2000).padStart(15,"0")}-abcd1234-${process.pid}`),"{}");
 writeFileSync(join(tickets,`${String(1000).padStart(15,"0")}-abcd1234-999999999`),"{}");
 expect(slotQueueMillisecondsV1(root,2,5000)).toBe(3000);
 first.release();expect(slotQueueMillisecondsV1(root,2,5000)).toBe(0);
 second.release();rmSync(root,{recursive:true,force:true});
});

test("the job token goes back to the build's named pool while the unit is away and is taken back afterwards, and pools that cannot be served keep the token",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"offload-token-"))),fifo=join(root,"pool");
 expect(spawnSync("mkfifo",[fifo]).status).toBe(0);
 const opened=await import("node:fs").then(fs=>fs.openSync(fifo,fs.constants.O_RDWR|fs.constants.O_NONBLOCK));
 const {writeSync,readSync,closeSync}=await import("node:fs");
 writeSync(opened,"++");
 const count=()=>{const buffer=Buffer.alloc(16);let total=0;const taken:Buffer[]=[];try{for(;;){const n=readSync(opened,buffer,0,1,null);if(n<=0)break;total+=n;taken.push(Buffer.from("+"));}}catch{}if(total>0)writeSync(opened,"+".repeat(total));return total;};
 expect(count()).toBe(2);
 const lease=releaseImplicitTokenV1({CARGO_MAKEFLAGS:`-j3 --jobserver-fds=fifo:${fifo} --jobserver-auth=fifo:${fifo}`},"darwin")!;
 expect(lease).toBeDefined();expect(count()).toBe(3);
 expect(await lease.reacquire(1000)).toBe(true);expect(count()).toBe(2);
 expect(await lease.reacquire(1000)).toBe(true);expect(count()).toBe(2);
 lease.close();
 const empty=join(root,"empty");expect(spawnSync("mkfifo",[empty]).status).toBe(0);
 const idle=releaseImplicitTokenV1({MAKEFLAGS:`--jobserver-auth=fifo:${empty}`},"darwin")!;
 const started=Date.now();expect(await idle.reacquire(150)).toBe(true);expect(Date.now()-started).toBeLessThan(140);
 const none=releaseImplicitTokenV1({MAKEFLAGS:`--jobserver-auth=fifo:${empty}`},"darwin")!;
 expect(await none.reacquire(100)).toBe(true);
 expect(releaseImplicitTokenV1({},"darwin")).toBeUndefined();expect(releaseImplicitTokenV1({MAKEFLAGS:"--jobserver-auth=3,4"},"darwin")).toBeUndefined();expect(releaseImplicitTokenV1({MAKEFLAGS:"--jobserver-auth=fifo:/nope"},"darwin")).toBeUndefined();expect(releaseImplicitTokenV1({MAKEFLAGS:`--jobserver-auth=fifo:${fifo}`},"win32")).toBeUndefined();
 closeSync(opened);rmSync(root,{recursive:true,force:true});
});

test("every decline and fall-back is counted under its reason beside the three totals, so the refusals can be told apart by cause",()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"offload-reasons-")));
 for(const [add,reason,times] of [[{refused:1},"no-idle-machine",5],[{refused:1},"queue-timeout",2],[{fellBack:1},"worker-policy",3],[{fellBack:1},"incomplete-inputs",4],[{},"unmapped-path",6]] as const)for(let index=0;index<times;index++)bumpOffloadCountersV1(root,add,reason);
 bumpOffloadCountersV1(root,{out:1});bumpOffloadCountersV1(root,{refused:1},"../escape");bumpOffloadCountersV1(root,{},"Bad Reason");
 expect(readOffloadReasonCountersV1(root)).toEqual({"no-idle-machine":5,"queue-timeout":2,"worker-policy":3,"incomplete-inputs":4,"unmapped-path":6});
 expect(readOffloadCountersV1(root)).toEqual({out:1,refused:8,fellBack:7});
 const counted=readOffloadReasonCountersV1(root);
 expect(counted["no-idle-machine"]!+counted["queue-timeout"]!,"the reasons of the refused total").toBe(7);expect(readOffloadCountersV1(root).refused-1,"one refused event had an unusable reason name and is counted in the total only").toBe(7);
 expect(readOffloadReasonCountersV1(join(root,"nothing"))).toEqual({});
 rmSync(root,{recursive:true,force:true});
});

test("counters add up concurrent events exactly and keep the unit cache fields of the shared file",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"offload-counters-"))),file=join(root,".cache","f","build-counters.json");
 mkdirSync(join(root,".cache","f"),{recursive:true});writeFileSync(file,JSON.stringify({version:1,wallMs:5,unit:{localHit:1,remoteHit:2,miss:3,stored:4},offload:{out:0,refused:0,fellBack:0}}));
 await Promise.all(Array.from({length:40},(_,index)=>Promise.resolve().then(()=>bumpOffloadCountersV1(root,index%4===0?{refused:1}:index%4===1?{fellBack:1}:{out:1}))));
 bumpOffloadCountersV1(root,{out:1});
 const value=JSON.parse(readFileSync(file,"utf8"));
 expect(value.unit).toEqual({localHit:1,remoteHit:2,miss:3,stored:4});expect(value.wallMs).toBe(5);expect(value.offload).toEqual({out:21,refused:10,fellBack:10});
 expect(readOffloadCountersV1(root)).toEqual({out:21,refused:10,fellBack:10});
 rmSync(root,{recursive:true,force:true});
});

test("the --offload flag is taken out before the delimiter, in both spellings, and refused when unknown or repeated",()=>{
 expect(extractOffloadFlagV1(["run","x:build","--offload=always","--","--offload=never"])).toEqual({args:["run","x:build","--","--offload=never"],mode:"always"});
 expect(extractOffloadFlagV1(["run","--offload","never","y"])).toEqual({args:["run","y"],mode:"never"});
 expect(extractOffloadFlagV1(["run","y"])).toEqual({args:["run","y"]});
 for(const bad of [["--offload=sometimes"],["--offload"],["--offload=auto","--offload=never"]])expect(()=>extractOffloadFlagV1(bad)).toThrow(/--offload takes always, auto or never, once\.\n--offload nimmt always, auto oder never, genau einmal\./u);
});

test("repo run and repo cargo take --offload out of their arguments and hand the mode to the build environment",async()=>{
 const {resolveBuildModeV1}=await import("../../../🧭️invocation/🟦️.ts");
 const resolved=resolveBuildModeV1({args:["cargo","--store=shared","--offload=always","--configuration=prototype","--","check","-p","x","--offload=never"],env:{}},{});
 expect(resolved.env.SEMIO_OFFLOAD).toBe("always");expect(resolved.args).toEqual(["cargo","--store=shared","--configuration=prototype","--","check","-p","x","--offload=never"]);
 const plain=resolveBuildModeV1({args:["cargo","--store=shared","--configuration=prototype","--","check"],env:{}},{});
 expect(plain.env.SEMIO_OFFLOAD).toBeUndefined();
 expect(()=>resolveBuildModeV1({args:["run","a:build","--offload=maybe"],env:{}},{})).toThrow(/--offload nimmt/u);
});

test("Cargo's CARGO_MANIFEST_PATH is mapped like the manifest directory, and a wrong one is declined",()=>{
 const left=chain("manifest-left"),right=chain("manifest-right");
 const planned=(checkout:ReturnType<typeof chain>,path=join(checkout.root,"pk","c","Cargo.toml"))=>planOffloadV1({args:checkout.arguments("c",["b"]),environment:{...checkout.environment("c"),CARGO_MANIFEST_PATH:path},cwd:checkout.root});
 const a=planned(left),b=planned(right);
 if("declined" in a||"declined" in b)throw Error("planned");
 expect(a.environment.SEMIO_ABSOLUTE).toBe("CARGO_MANIFEST_DIR CARGO_MANIFEST_PATH");expect(a.environment.SEMIO_REL_CARGO_MANIFEST_PATH).toBe("pk/c/Cargo.toml");
 expect(a.environment).toEqual(b.environment);expect(a.environment.CARGO_MANIFEST_PATH).toBeUndefined();
 const wrong=planned(left,join(left.root,"elsewhere","Cargo.toml"));expect("declined" in wrong&&wrong.declined).toBe("unmapped-path");
});

test("the repository's own layouts are mapped: an absolute source in the unit's own directory (registry crates) and a manifest below the directory that holds the source (module layout), both path-independent",()=>{
 const planned=(checkout:ReturnType<typeof chain>,mode:"registry"|"module")=>{
  const base=join(checkout.root,"pk"),manifest=mode==="registry"?join(base,"c"):join(base,"m","packages","rust");
  mkdirSync(join(manifest,"src"),{recursive:true});writeFileSync(join(manifest,"Cargo.toml"),"[package]\nname = \"c\"\n");
  const source=mode==="registry"?join(manifest,"src","lib.rs"):join(base,"m","lib.rs");writeFileSync(source,"pub fn c()->u32{b::b()+1}\n");
  const cwd=mode==="registry"?manifest:checkout.root,args=checkout.arguments("c",["b"]).map(argument=>argument==="pk/c/src/lib.rs"?(mode==="registry"?source:"pk/m/lib.rs"):argument);
  return{cwd,result:planOffloadV1({args,environment:{...checkout.environment("c"),CARGO_MANIFEST_DIR:manifest},cwd})};
 };
 for(const mode of ["registry","module"] as const){
  const left=chain("layout-left-"+mode),right=chain("layout-right-"+mode),a=planned(left,mode),b=planned(right,mode);
  if("declined" in a.result||"declined" in b.result)throw Error(mode+" declined: "+JSON.stringify(a.result)+JSON.stringify(b.result));
  expect(a.result.arguments,mode).toEqual(b.result.arguments);expect(a.result.environment,mode).toEqual(b.result.environment);expect([...a.result.files.keys()],mode).toEqual([...b.result.files.keys()]);
  expect(a.result.arguments.some(argument=>argument.includes(left.base)),mode).toBe(false);
  if(mode==="registry"){expect(a.result.environment.SEMIO_SOURCE,"an absolute source stays absolute on the other machine, rooted at its work directory").toBe("src/lib.rs");expect(a.result.arguments).not.toContain("src/lib.rs");}
  else{expect([...a.result.files.keys()]).toContain("pk/m/lib.rs");expect([...a.result.files.keys()]).toContain("pk/m/packages/rust/Cargo.toml");expect(a.result.environment.SEMIO_REL_CARGO_MANIFEST_DIR).toBe("pk/m/packages/rust");}
 }
 const checkout=chain("layout-outside"),manifest=join(checkout.root,"pk","c"),result=planOffloadV1({args:checkout.arguments("c",["b"]).map(argument=>argument==="pk/c/src/lib.rs"?"/tmp/outside/lib.rs":argument),environment:{...checkout.environment("c"),CARGO_MANIFEST_DIR:manifest},cwd:checkout.root});
 expect("declined" in result&&result.declined).toBe("unmapped-path");
});

test("a one-line shim that includes the crate's code from the module directory above its manifest ships that directory (and the files that its files include in turn)",()=>{
 const planned=(checkout:ReturnType<typeof chain>)=>{
  const base=join(checkout.root,"pk"),manifest=join(base,"m","packages","rust");
  mkdirSync(manifest,{recursive:true});writeFileSync(join(manifest,"Cargo.toml"),"[package]\nname = \"c\"\n");
  writeFileSync(join(manifest,"shim.rs"),"#[path = \"../../lib.rs\"]\nmod component;\npub use component::*;\n");
  writeFileSync(join(base,"m","lib.rs"),"mod part;\npub fn c()->u32{b::b()+include_str!(\"data/v.txt\").len() as u32}\n");writeFileSync(join(base,"m","part.rs"),"\n");
  mkdirSync(join(base,"m","data"),{recursive:true});writeFileSync(join(base,"m","data","v.txt"),"1");
  const args=checkout.arguments("c",["b"]).map(argument=>argument==="pk/c/src/lib.rs"?"pk/m/packages/rust/shim.rs":argument);
  return planOffloadV1({args,environment:{...checkout.environment("c"),CARGO_MANIFEST_DIR:manifest},cwd:checkout.root});
 };
 const left=planned(chain("shim-left")),right=planned(chain("shim-right"));
 if("declined" in left||"declined" in right)throw Error(JSON.stringify(left)+JSON.stringify(right));
 const names=[...left.files.keys()];
 for(const wanted of["pk/m/packages/rust/shim.rs","pk/m/lib.rs","pk/m/part.rs","pk/m/data/v.txt","pk/m/packages/rust/Cargo.toml"])expect(names).toContain(wanted);
 expect(names).toEqual([...right.files.keys()]);expect(left.arguments).toEqual(right.arguments);
 expect(includedSourcesV1("/nonexistent/entry.rs",()=>undefined)).toEqual([]);
 const chainRead=(path:string)=>path==="/x/a.rs"?'#[path = "b.rs"] mod b; include!("c.rs");':path==="/x/b.rs"?'#[path = "a.rs"] mod a;':undefined;
 expect(includedSourcesV1("/x/a.rs",chainRead)).toEqual([]);
});

test("a unit Cargo compiles in its package directory (absolute source, working directory = manifest directory) whose shim includes the module directory above the manifest is planned too: the module directory travels as the virtual package, the plan is the same in two checkouts, and an ancestor that is no module directory is still declined",()=>{
 const planned=(checkout:ReturnType<typeof chain>,depth:string)=>{
  const base=join(checkout.root,depth),manifest=join(base,"m","packages","rust");
  mkdirSync(manifest,{recursive:true});writeFileSync(join(manifest,"Cargo.toml"),"[package]\nname = \"c\"\n");
  writeFileSync(join(manifest,"shim.rs"),"#[path = \"../../lib.rs\"]\nmod component;\npub use component::*;\n");
  writeFileSync(join(base,"m","lib.rs"),"mod part;\npub fn c()->u32{b::b()+include_str!(\"data/v.txt\").len() as u32}\n");writeFileSync(join(base,"m","part.rs"),"\n");
  mkdirSync(join(base,"m","data"),{recursive:true});writeFileSync(join(base,"m","data","v.txt"),"1");
  const args=checkout.arguments("c",["b"]).map(argument=>argument==="pk/c/src/lib.rs"?join(manifest,"shim.rs"):argument).filter((argument,index,all)=>!argument.startsWith("--remap-path-prefix")&&all[index-1]!=="--remap-path-prefix");
  return{manifest,base,result:planOffloadV1({args:[...args,`--remap-path-prefix=${checkout.root}=.`],environment:{...checkout.environment("c"),CARGO_MANIFEST_DIR:manifest},cwd:manifest})};
 };
 const left=planned(chain("package-cwd-left"),"pk"),right=planned(chain("package-cwd-right"),"pk");
 if("declined" in left.result||"declined" in right.result)throw Error(JSON.stringify(left.result)+JSON.stringify(right.result));
 expect(left.result.arguments).toEqual(right.result.arguments);expect(left.result.environment).toEqual(right.result.environment);expect([...left.result.files.keys()]).toEqual([...right.result.files.keys()]);
 const names=[...left.result.files.keys()];
 for(const wanted of["lib.rs","part.rs","data/v.txt","packages/rust/shim.rs","packages/rust/Cargo.toml"])expect(names.map(name=>name.replace(/^\.semio-offload\/p\//u,"")),wanted).toContain(wanted);
 expect(left.result.environment.SEMIO_REL_CARGO_MANIFEST_DIR.endsWith("/packages/rust")).toBe(true);
 expect(left.result.arguments.some(argument=>argument.includes(left.base)||argument.includes(left.manifest))).toBe(false);
 expect(left.result.mappings.some(row=>row.real===left.base+"/m")).toBe(true);
 const narrow=chain("package-cwd-narrow"),narrowed=planned(narrow,"pk");
 expect("declined" in narrowed.result,"planned with the root remap").toBe(false);
 const lonely=chain("package-cwd-lonely"),base=join(lonely.root,"pk"),home=join(base,"m","packages","rust");
 mkdirSync(home,{recursive:true});writeFileSync(join(home,"Cargo.toml"),"[package]\nname = \"c\"\n");writeFileSync(join(home,"shim.rs"),"#[path = \"../../lib.rs\"]\nmod component;\n");writeFileSync(join(base,"m","lib.rs"),"pub fn c()->u32{1}\n");
 const uncovered=planOffloadV1({args:[...lonely.arguments("c",["b"]).filter((argument,index,all)=>!argument.startsWith("--remap-path-prefix")&&all[index-1]!=="--remap-path-prefix").map(argument=>argument==="pk/c/src/lib.rs"?join(home,"shim.rs"):argument),`--remap-path-prefix=${home}=.`],environment:{...lonely.environment("c"),CARGO_MANIFEST_DIR:home},cwd:home});
 expect("declined" in uncovered&&uncovered.declined,"a module directory the remaps do not cover would be recorded with this machine's path").toBe("unmapped-path");
 const shallow=chain("package-cwd-shallow"),manifest=join(shallow.root,"pk","c");
 const declined=planOffloadV1({args:shallow.arguments("c",["b"]).map(argument=>argument==="pk/c/src/lib.rs"?"/tmp/outside/lib.rs":argument),environment:{...shallow.environment("c"),CARGO_MANIFEST_DIR:manifest},cwd:manifest});
 expect("declined" in declined&&declined.declined).toBe("unmapped-path");
 expect("declined" in declined&&declined.detail).toContain("source outside CARGO_MANIFEST_DIR");
});

test("the requester's own temporary directories and path remaps are left behind, not carried to the other machine",()=>{
 const checkout=chain("remap"),remap=`--remap-path-prefix=${checkout.root}/.cache/cargo=/cargo`,foreign=`--remap-path-prefix=/usr/lib=/lib`;
 const result=planOffloadV1({args:[...checkout.arguments("c",["b"]),remap,"--remap-path-prefix",`${checkout.root}/t=/t`,foreign],environment:{...checkout.environment("c"),TEMP:`${checkout.root}/.tmp`,TMP:`${checkout.root}/.tmp`},cwd:checkout.root});
 if("declined" in result)throw Error(JSON.stringify(result));
 expect(result.arguments.some(argument=>argument.includes(checkout.root))).toBe(false);
 expect(result.arguments).toContain(foreign);expect(result.environment.TEMP).toBeUndefined();expect(result.environment.TMP).toBeUndefined();
});

test("an unreachable cache endpoint is a quick, silent decline and is remembered, never a hang",async()=>{
 const {offloadRustcUnitV1}=await import("../🟦️.ts");
 const checkout=chain("unreachable"),root=realpathSync(mkdtempSync(join(tmpdir(),"offload-unreachable-")));
 const environment={...checkout.environment("c"),...testRustEnvironmentV1(),SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://127.0.0.1:1",SEMIO_REMOTE_CACHE_ENDPOINT:"http://127.0.0.1:1",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 const started=Date.now(),first=await offloadRustcUnitV1({args:checkout.arguments("c",["b"]),environment,cwd:checkout.root,root,crate:"c",key:"a".repeat(64),compiler:["rustc"]});
 expect(first).toMatchObject({kind:"local",reason:"infrastructure"});expect(Date.now()-started).toBeLessThan(5000);
 const again=Date.now(),second=await offloadRustcUnitV1({args:checkout.arguments("c",["b"]),environment,cwd:checkout.root,root,crate:"c",key:"a".repeat(64),compiler:["rustc"]});
 expect(second,"the remembered decline keeps its own reason instead of posing as a full fleet").toMatchObject({kind:"local",reason:"infrastructure",detail:"memo cache endpoint unreachable"});expect(Date.now()-again).toBeLessThan(1000);
 const missing=await offloadRustcUnitV1({args:checkout.arguments("c",["b"]),environment:{...environment,SEMIO_REMOTE_EXECUTION_ENDPOINT:"",SEMIO_REMOTE_CACHE_ENDPOINT:""},cwd:checkout.root,root:join(root,"other"),crate:"c",key:"a".repeat(64),compiler:["rustc"]});
 expect(missing).toMatchObject({kind:"local",reason:"no-endpoint"});
 rmSync(root,{recursive:true,force:true});
});

test("a cache endpoint that answered is remembered for a while: a build of many units probes it about once, not once per decline window",async()=>{
 const {offloadRustcUnitV1}=await import("../🟦️.ts");
 let probes=0;
 const cache=Bun.serve({port:0,hostname:"127.0.0.1",fetch(request){if(new URL(request.url).pathname==="/healthz"){probes++;return new Response("{\"status\":\"ok\"}");}return new Response("no",{status:404});}});
 try{
  const checkout=chain("reachable"),root=realpathSync(mkdtempSync(join(tmpdir(),"offload-reachable-")));
  const environment={...checkout.environment("c"),...testRustEnvironmentV1(),SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://127.0.0.1:1",SEMIO_REMOTE_CACHE_ENDPOINT:`http://127.0.0.1:${cache.port}`,SEMIO_REMOTE_CACHE_INSTANCE:"main"};
  for(let index=0;index<4;index++){
   await offloadRustcUnitV1({args:checkout.arguments("c",["b"]),environment,cwd:checkout.root,root,crate:"c",key:String(index).repeat(64),compiler:["rustc"]});
   rmSync(join(root,"offload-declined.json"),{force:true});
  }
  expect(probes).toBe(1);
  rmSync(root,{recursive:true,force:true});
 }finally{cache.stop(true);}
});

test("an invocation that this machine cannot carry does not queue for it when the fleet can take units: the fan-out is granted only with offload on, a scheduler published and a machine that says wait",async()=>{
 const busy={probe:testHostProbeV1("critical-pressure-gigabyte-swap")},quiet={probe:testHostProbeV1("quiet-no-swap",true)},scheduler={SEMIO_WORKSPACE_ROOT:realpathSync(mkdtempSync(join(tmpdir(),"fanout-"))),SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://192.0.2.1:8980"};
 expect(await offloadFanOutJobsV1(scheduler,busy)).toBe(Math.min(OFFLOAD_POLICY_V1.limits.fanOutJobs,availableParallelism()));
 expect(await offloadFanOutJobsV1(scheduler,quiet),"a machine that admits one more unit queues normally").toBeUndefined();
 expect(await offloadFanOutJobsV1({...scheduler,SEMIO_OFFLOAD:"never"},busy)).toBeUndefined();
 expect(await offloadFanOutJobsV1({...scheduler,SEMIO_REMOTE_EXECUTION_ENDPOINT:""},busy),"no scheduler published").toBeUndefined();
 expect(await offloadFanOutJobsV1({...scheduler,SEMIO_OFFLOAD:"always"},busy)).toBeGreaterThan(1);
});

test("a unit whose recorded local cost is below a round trip stays here in auto mode; unknown or costly units go; the workspace root is found from any directory below it",()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"cost-")));
 expect(readUnitCostV1(root,"tiny")).toBeUndefined();expect(worthARoundTripV1(root,"tiny"),"no record: the first compile teaches the cost").toBe(true);
 writeUnitCostV1(root,"tiny",300);expect(readUnitCostV1(root,"tiny")).toBe(300);expect(worthARoundTripV1(root,"tiny")).toBe(false);
 writeUnitCostV1(root,"tiny",9300);expect(readUnitCostV1(root,"tiny"),"halfway to the new measurement").toBe(4800);expect(worthARoundTripV1(root,"tiny")).toBe(true);
 writeUnitCostV1(root,"a/b c",OFFLOAD_POLICY_V1.limits.roundTripMilliseconds);expect(worthARoundTripV1(root,"a/b c")).toBe(true);
 const checkout=join(root,"ws"),below=join(checkout,"x","y");mkdirSync(join(checkout,".cache","f"),{recursive:true});mkdirSync(below,{recursive:true});writeFileSync(join(checkout,".cache","f","machine.json"),"{}");
 expect(workspaceRootFromV1(below)).toBe(checkout);expect(workspaceRootFromV1(root)).toBeUndefined();
 rmSync(root,{recursive:true,force:true});
});

test("an extern given as an rlib ships its rmeta beside it (with embed-metadata=no the rlib alone is a stub: 'can't find crate' on the other machine), and the other way round",()=>{
 const checkout=chain("sibling"),stem=`libb-${externHash("b")}`;
 for(const extension of["rlib","rmeta"]){
  const args=checkout.arguments("c",["b"]).map(argument=>argument.startsWith("b=")?`b=${checkout.deps}/${stem}.${extension}`:argument);
  const planned=planOffloadV1({args,environment:checkout.environment("c"),cwd:checkout.root});
  if("declined" in planned)throw Error(JSON.stringify(planned));
  const names=[...planned.files.keys()];
  expect(names.some(name=>name.endsWith(`${stem}.rlib`))&&names.some(name=>name.endsWith(`${stem}.rmeta`)),extension).toBe(true);
 }
});
