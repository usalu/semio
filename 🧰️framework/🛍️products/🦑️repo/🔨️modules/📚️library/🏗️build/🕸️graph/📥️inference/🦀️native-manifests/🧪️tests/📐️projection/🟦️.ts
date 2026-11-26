/** 📐️ Contract: the projection of a Cargo manifest onto the part one role depends on. The shape readers consume is preserved (the dependency reader gives the same answer on the projection as on the whole manifest, 200 seeded manifests), an independent Python model of the rules derives the same projection for the same manifests, an edit of any other part of a manifest leaves the digest of the `field` row and the verdict of the judge, an edit of the part moves both, and a dependent's record holds while a dependency edits what does not concern it. */
import{afterAll,beforeAll,expect,setDefaultTimeout,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{parse,stringify}from"smol-toml";
import{requireTestToolV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{nativeDependencies,nativeDependencyRoots,readCargoManifestRoleV1,wire}from"../../🟨️.mjs";
import{CARGO_MANIFEST_ROLES_V1,cargoManifestProjectionV1}from"../../📐️projection/🟨️.mjs";
import{fieldFactV1}from"../../../../../../🏗️build/👁️observation/🧩️field/🟨️.mjs";
import{captureObservedSourceInputsV1,observationsHold}from"../../../../../../🏗️build/👁️observation/🟨️.mjs";

setDefaultTimeout(120000);
const python=requireTestToolV1("python"),oracle=join(import.meta.dir,"🐍️.py");
let root="";
beforeAll(()=>{root=realpathSync(mkdtempSync(join(tmpdir(),"manifest-projection-")));wire({sourceAnalyses:{prepareSourceAnalysesV1:()=>({}),RUST_SOURCE_CACHE:{},RUST_INPUT_SNAPSHOTS:new WeakMap()},coordinates:{buildPath:(path:string)=>path.split("\\").join("/"),SCRIPT_BASENAME:"x"},targetPolicy:{targetExecutionKindV1:()=>undefined,executionTargetV1:()=>undefined}});});
afterAll(()=>{rmSync(root,{recursive:true,force:true});});

const seeded=(seed:number)=>()=>{seed|=0;seed=seed+0x6D2B79F5|0;let t=Math.imul(seed^seed>>>15,1|seed);t=t+Math.imul(t^t>>>7,61|t)^t;return((t^t>>>14)>>>0)/4294967296;};
const names=["a","b","c","d","e","f"];
function manifest(seed:number):any{
 const next=seeded(seed),pick=<T,>(list:T[])=>list[Math.floor(next()*list.length)]!,chance=(p:number)=>next()<p;
 const entry=():any=>pick([()=>"1.0",()=>({version:"1",features:["x"]}),()=>({path:"../"+pick(names)}),()=>({path:"../"+pick(names),package:pick(names),version:"2"}),()=>({workspace:true,features:["y"]}),()=>({workspace:true,optional:true}),()=>({git:"https://example.test/r"})])();
 const tables=():any=>{const out:any={};for(const kind of["dependencies","build-dependencies","dev-dependencies"])if(chance(.6)){out[kind]={};for(const alias of names)if(chance(.4))out[kind][alias]=entry();}return out;};
 const m:any={package:{name:"p"+seed,version:"0.1."+seed,edition:"2021",...chance(.3)?{workspace:"../.."}:{},...chance(.2)?{build:pick(["build.rs",false,"gen.rs"])}:{},...chance(.2)?{metadata:{semio:{"build-script":{script:"x.rs",files:[]}},other:{k:seed}}}:{}},...tables()};
 if(chance(.4)){m.target={};for(const cfg of["cfg(unix)","cfg(windows)"])if(chance(.6))m.target[cfg]=tables();}
 if(chance(.5))m.lib={path:pick(["src/lib.rs","x.rs"]),name:"p"};
 for(const kind of["bin","test","bench","example"])if(chance(.4))m[kind]=Array.from({length:1+Math.floor(next()*3)},(_,i)=>({name:kind+i,...chance(.8)?{path:"t/"+kind+i+".rs"}:{}}));
 if(chance(.3)){m.workspace={members:["x","y"],...chance(.7)?{dependencies:Object.fromEntries(names.filter(()=>chance(.5)).map(alias=>[alias,pick([()=>"1",()=>({path:"crates/"+alias}),()=>({path:"crates/"+alias,package:"q"+alias}),()=>({version:"1",features:["z"]})])()]))}:{}};}
 if(chance(.3))m.features={x:[],y:["x"]};
 return JSON.parse(JSON.stringify(m));
}
const canonical=(value:unknown):string=>JSON.stringify(value,(_,v)=>v&&typeof v==="object"&&!Array.isArray(v)?Object.fromEntries(Object.keys(v).sort().map(k=>[k,v[k]])):v);

test("200 seeded manifests: the projection equals the independent Python model for every role",()=>{
 const manifests=Array.from({length:200},(_,i)=>manifest(i+1));
 const run=spawnSync(python,["-I",oracle],{input:JSON.stringify({manifests}),encoding:"utf8",maxBuffer:1<<26});
 expect(run.status,run.stderr).toBe(0);
 const model=JSON.parse(run.stdout);let compared=0;
 for(const[index,m]of manifests.entries())for(const role of CARGO_MANIFEST_ROLES_V1){expect(canonical(cargoManifestProjectionV1(m,role)),`${index} ${role}`).toBe(canonical(model[index][role]));compared++;}
 expect(compared).toBe(800);
});

test("200 seeded manifests: the dependency reader answers the same on the projection as on the whole manifest",()=>{
 for(let seed=1;seed<=200;seed++){
  const m=manifest(seed),w=manifest(seed+1000);w.workspace??={};
  for(const tests of[false,true]){
   const role=tests?"graph-tests":"graph",whole=nativeDependencies(m,w.workspace),projected=nativeDependencies(cargoManifestProjectionV1(m,role),cargoManifestProjectionV1(w,"graph").workspace??{});
   const expected=tests?whole:whole.filter(row=>row.kind!=="dev-dependencies");
   expect(projected,`${seed} ${role}`).toEqual(expected);
  }
 }
});

const digest=(m:any,role:string)=>fieldFactV1("/x/Cargo.toml","cargo-manifest",role,()=>stringify(m)).digest;
const clone=(m:any)=>JSON.parse(JSON.stringify(m));

test("an edit of anything the role does not read leaves the digest, an edit it reads moves it",()=>{
 let seen=0;
 for(let seed=1;seed<=200;seed++){
  const m=manifest(seed);m.dependencies??={};
  const edits:[string,(x:any)=>void,string[]][]=[
   ["version",x=>{x.package.version="9.9.9";},[]],
   ["registry dependency",x=>{x.dependencies["zz"]="3";},[]],
   ["features",x=>{x.features={...x.features,added:["x"]};},[]],
   ["metadata other",x=>{x.package.metadata={...x.package.metadata,other:{k:"new"+seed}};},[]],
   ["workspace members",x=>{if(x.workspace)x.workspace.members.push("new");},[]],
   ["path dependency",x=>{x.dependencies["zz"]={path:"../zz"};},["graph","graph-tests"]],
   ["workspace dependency",x=>{x.dependencies["zz"]={workspace:true};},["graph","graph-tests"]],
   ["dev path dependency",x=>{x["dev-dependencies"]={...x["dev-dependencies"],zz:{path:"../zz"}};},["graph-tests"]],
   ["package alias",x=>{x.dependencies["zz"]={path:"../zz",package:"other"};},["graph","graph-tests"]],
   ["lib path",x=>{x.lib={path:"zz/lib.rs"};},["sources","sources-tests"]],
   ["bin path",x=>{x.bin=[...(x.bin??[]),{name:"zz",path:"zz.rs"}];},["sources","sources-tests"]],
   ["test target",x=>{x.test=[...(x.test??[]),{name:"zz",path:"zz.rs"}];},["sources-tests"]],
  ];
  for(const[name,edit,moves]of edits){
   const after=clone(m);edit(after);
   for(const role of CARGO_MANIFEST_ROLES_V1){
    const moved=digest(m,role)!==digest(after,role);
    if(!moves.includes(role)&&moved)throw Error(`${seed} ${name}: ${role} moved`);
    if(moves.includes(role)){if(!moved)throw Error(`${seed} ${name}: ${role} did not move`);seen++;}
   }
  }
 }
 expect(seen).toBeGreaterThan(1000);
});

test("failures stay distinct from every value: absent, unparsable and unknown role",()=>{
 expect(fieldFactV1("/x/Cargo.toml","cargo-manifest","graph",()=>{throw Error("absent");}).digest).toBe("!absent-file");
 expect(fieldFactV1("/x/Cargo.toml","cargo-manifest","graph",()=>"[package\n").digest).toBe("!unparsable");
 expect(fieldFactV1("/x/Cargo.toml","cargo-manifest","nonsense",()=>"[package]\nname='a'\n").digest).toBe("!unparsable");
 expect(()=>cargoManifestProjectionV1({},"nonsense")).toThrow();
});

function write(path:string,text:string){mkdirSync(join(path,".."),{recursive:true});writeFileSync(path,text);}
function crate(name:string,body:string){write(join(root,name,"Cargo.toml"),`[package]\nname = "${name}"\nversion = "0.1.0"\n\n${body}`);write(join(root,name,"src/lib.rs"),"");}

test("a dependent's record holds while a dependency edits what does not concern it, the producer's row and the judge agree",async()=>{
 write(join(root,"Cargo.toml"),'[workspace]\nmembers = ["a","b","c"]\n');
 crate("c",'[dependencies]\nserde = "1"\n');
 crate("b",'[dependencies]\nc = { path = "../c" }\n');
 crate("a",'[dependencies]\nb = { path = "../b" }\n');
 const captured=await captureObservedSourceInputsV1(async()=>nativeDependencyRoots("a",root,false,new Map(),new Map()),undefined,false);
 expect(captured.value).toEqual(["a","b","c"]);
 const rows=captured.observations;
 expect(rows.filter((row:any)=>row.kind==="file"&&/[\\/][bc][\\/]Cargo\.toml$/u.test(row.path)),"no whole-file row of a dependency manifest").toEqual([]);
 const fields=rows.filter((row):row is Extract<(typeof rows)[number],{kind:"field"}>=>row.kind==="field"&&row.fact.format==="cargo-manifest");
 expect(fields.length).toBeGreaterThanOrEqual(3);
 for(const row of fields)expect(row.fact.digest).toBe(fieldFactV1(row.path,"cargo-manifest",row.fact.pointer,()=>require("node:fs").readFileSync(row.path,"utf8")).digest);
 expect(await observationsHold(rows)).toBe(true);
 crate("c",'[dependencies]\nserde = "2"\nregex = "1"\n\n[dev-dependencies]\nx = { path = "../x" }\n\n[[test]]\nname = "t"\npath = "t.rs"\n\n[features]\nf = []\n');
 expect(await observationsHold(rows),"an edit of a test target, features, versions and a dev-dependency of a dependency").toBe(true);
 crate("c",'[dependencies]\nserde = "2"\nnew = { path = "../new" }\n');
 expect(await observationsHold(rows),"a new path dependency of a dependency").toBe(false);
 crate("c",'[dependencies]\nserde = "1"\n');
 expect(await observationsHold(rows)).toBe(true);
 crate("b",'[dependencies]\nc = { path = "../c" }\nd = { path = "../c" }\n');
 expect(await observationsHold(rows),"an edit of the dependency list of b").toBe(false);
});

test("the sources role holds while a manifest edits what is not an entry point, and moves with an entry point",async()=>{
 crate("e",'[dependencies]\nserde = "1"\n');
 const path=join(root,"e","Cargo.toml");
 const captured=await captureObservedSourceInputsV1(async()=>readCargoManifestRoleV1(path,"sources-tests"),undefined,false);
 expect(captured.value).toEqual({});
 const rows=captured.observations;
 expect(rows.some((row:any)=>row.kind==="file")).toBe(false);
 expect(await observationsHold(rows)).toBe(true);
 crate("e",'[dependencies]\nserde = "2"\n\n[dev-dependencies]\nz = "1"\n\n[features]\nf = []\n');
 expect(await observationsHold(rows),"versions, dev-dependencies, features").toBe(true);
 crate("e",'[[test]]\nname = "t"\npath = "t.rs"\n');
 expect(await observationsHold(rows),"a test target moves the sources of a test closure").toBe(false);
 const plain=await captureObservedSourceInputsV1(async()=>readCargoManifestRoleV1(path,"sources"),undefined,false);
 crate("e",'[[test]]\nname = "t"\npath = "t2.rs"\n');
 expect(await observationsHold(plain.observations),"and not the sources of a production closure").toBe(true);
 crate("e",'[lib]\npath = "src/other.rs"\n');
 expect(await observationsHold(plain.observations),"the library entry point").toBe(false);
});

test("both readers of a dependency manifest go through the projection",()=>{
 const text=require("node:fs").readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8") as string;
 expect(text).toContain('readCargoManifestRoleV1(manifest, includeTests ? "sources-tests" : "sources")');
 expect(text).toContain('readCargoManifestRoleV1(path, role)');
 expect(text).toContain('includeTests ? "graph-tests" : "graph"');
});

test("an absent or unparsable manifest keeps the whole-file refusal of the reader",()=>{
 expect(()=>readCargoManifestRoleV1(join(root,"missing","Cargo.toml"),"graph")).toThrow();
 write(join(root,"broken","Cargo.toml"),"[package\n");
 expect(()=>readCargoManifestRoleV1(join(root,"broken","Cargo.toml"),"graph")).toThrow(/Observed TOML parse refused|parse/iu);
 void parse;
});
