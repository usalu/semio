import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {homedir,tmpdir} from "node:os";
import {dirname,join,relative} from "node:path";
import {manifestClosureMemoV1,manifestClosureV1} from "../🟦️.ts";

/** 🧮️ Contract: the closure derived from Cargo.toml files alone names exactly the local crates `cargo metadata` names, for a workspace with globbed and excluded members, `workspace = true` inheritance, renamed, optional, build, target-specific and development path dependencies and a path dependency outside the workspace. */
const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
function findCargo():string|undefined{
 if(process.env.SEMIO_TEST_CARGO)return process.env.SEMIO_TEST_CARGO;
 for(const root of[process.env.SEMIO_TEST_CHECKOUT,"/Users/ueli/Documents/semio"]){
  if(root===undefined)continue;
  try{
   const channel=/^channel\s*=\s*"([^"]+)"/mu.exec(readFileSync(join(root,"rust-toolchain.toml"),"utf8"))?.[1],base=join(process.env.RUSTUP_HOME??join(homedir(),".rustup"),"toolchains"),name=readdirSync(base).find(entry=>channel!==undefined&&entry.startsWith(channel));
   if(name!==undefined&&existsSync(join(base,name,"bin","cargo")))return join(base,name,"bin","cargo");
  }catch{}
 }
 return undefined;
}
const cargo=findCargo(),real=cargo===undefined?test.skip:test;
const plainEnvironment=(home:string)=>({...Object.fromEntries(Object.entries(process.env).filter(([name])=>!/^(RUSTC|RUSTC_WRAPPER|RUSTC_WORKSPACE_WRAPPER|RUSTFLAGS|RUSTDOC|CARGO_.*|SEMIO_.*)$/u.test(name))),PATH:dirname(cargo!)+":/usr/bin:/bin",RUSTC:join(dirname(cargo!),"rustc"),CARGO_HOME:home,HOME:home});

function crate(root:string,path:string,name:string,body="",extra=""):void{
 mkdirSync(join(root,path,"src"),{recursive:true});
 writeFileSync(join(root,path,"Cargo.toml"),`[package]\nname="${name}"\nversion="0.1.0"\nedition="2021"\n${extra}\n${body}\n`);
 writeFileSync(join(root,path,"src","lib.rs"),"");
}

function fixture():string{
 const root=realpathSync(mkdtempSync(join(process.platform==="win32"?tmpdir():"/tmp","gr4-closure-")));scratch.push(root);
 writeFileSync(join(root,"Cargo.toml"),`[workspace]\nresolver="2"\nmembers=["crates/*","tools/t?"]\nexclude=["crates/skip"]\n[workspace.dependencies]\nshared={path="libs/shared"}\n`);
 crate(root,"crates/a","a",`[dependencies]\nb={path="../b"}\nshared={workspace=true}\nbren={path="../../libs/ren",package="ren-real"}\nopt={path="../../libs/opt",optional=true}\n[build-dependencies]\nbld={path="../../libs/bld"}\n[dev-dependencies]\ndevonly={path="../../libs/devonly"}\n[target.'cfg(windows)'.dependencies]\nwin={path="../../libs/win"}\n`);
 crate(root,"crates/b","b",`[dependencies]\nleaf={path="../../ext/leaf"}\n`);
 crate(root,"crates/skip","skip");
 crate(root,"tools/t1","t1");
 crate(root,"libs/shared","shared");crate(root,"libs/ren","ren-real");crate(root,"libs/opt","opt");crate(root,"libs/bld","bld");crate(root,"libs/devonly","devonly");crate(root,"libs/win","win");
 crate(root,"ext/leaf","leaf",`[dev-dependencies]\nleafdev={path="../leafdev"}\n`);crate(root,"ext/leafdev","leafdev");
 return root;
}

function oracle(root:string,args:readonly string[]=[]):{members:string[];packages:string[];reach:(name:string,dev:boolean)=>string[]}{
 const run=spawnSync(cargo!,["metadata","--format-version=1","--offline","--manifest-path",join(root,"Cargo.toml"),...args],{encoding:"utf8",maxBuffer:64*1024*1024,cwd:root,env:plainEnvironment(join(root,".home"))});
 if(run.status!==0)throw Error(run.stderr);
 const data=JSON.parse(run.stdout),directory=(path:string)=>relative(root,dirname(path))||".",byId=new Map<string,any>(data.packages.map((row:any)=>[row.id,row]));
 const nodes=new Map<string,any>(data.resolve.nodes.map((row:any)=>[row.id,row]));
 return{
  members:data.workspace_members.map((id:string)=>directory(byId.get(id).manifest_path)).sort(),
  packages:data.packages.filter((row:any)=>row.source===null).map((row:any)=>directory(row.manifest_path)).sort(),
  reach(name,dev){
   const start=data.packages.find((row:any)=>row.name===name).id,seen=new Set([start]),queue=[start];
   while(queue.length){const at=queue.pop()!;for(const edge of nodes.get(at).deps)if(edge.dep_kinds.some((kind:any)=>at===start&&dev||kind.kind===null||kind.kind==="build")&&!seen.has(edge.pkg)){seen.add(edge.pkg);queue.push(edge.pkg);}}
   return[...seen].map(id=>directory(byId.get(id).manifest_path)).sort();
  },
 };
}

real("the workspace closure equals the local packages of cargo metadata",()=>{
 const root=fixture(),answer=manifestClosureV1(root,["."],{scope:"workspace"}),expected=oracle(root);
 expect(answer.crates.map(row=>row.directory)).toEqual(expected.packages);
 expect(answer.crates.filter(row=>row.member).map(row=>row.directory)).toEqual(expected.members);
 expect(answer.crates.some(row=>row.directory==="crates/skip")).toBe(false);
 expect(manifestClosureV1(root,["crates/a"]).crates.some(row=>row.directory==="ext/leafdev")).toBe(false);
 expect(answer.unsupported).toEqual([]);
});

real("a package closure equals the cargo metadata resolve below that package, with and without its test dependencies",()=>{
 const root=fixture();
 for(const tests of[false,true])expect(manifestClosureV1(root,["crates/a"],{tests}).crates.map(row=>row.directory)).toEqual(oracle(root,["--all-features"]).reach("a",tests));
 expect(manifestClosureV1(root,["crates/b"]).crates.map(row=>row.directory)).toEqual(oracle(root,["--all-features"]).reach("b",false));
});

real("the proof workspace of the fleet ticket matches cargo metadata",()=>{
 const proof=join(import.meta.dirname,"../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122-dist/proof");
 if(!existsSync(join(proof,"Cargo.toml")))return;
 const home=realpathSync(mkdtempSync(join(tmpdir(),"gr4-proof-")));scratch.push(home);
 const run=spawnSync(cargo!,["metadata","--format-version=1","--offline","--locked","--manifest-path",join(proof,"Cargo.toml")],{encoding:"utf8",maxBuffer:64*1024*1024,cwd:home,env:plainEnvironment(home)});
 if(run.status!==0)return;
 const expected=JSON.parse(run.stdout).packages.filter((row:any)=>row.source===null).map((row:any)=>relative(proof,dirname(row.manifest_path))||".").sort();
 expect(manifestClosureV1(proof,["."],{scope:"workspace"}).crates.map(row=>row.directory)).toEqual(expected);
});

test("a memoized closure is served while every manifest digest holds and recomputed when one changes",()=>{
 const root=fixture(),memo=manifestClosureMemoV1(root,join(root,".memo")),first=manifestClosureV1(root,["crates/a"],{memo}),second=manifestClosureV1(root,["crates/a"],{memo});
 expect(second).toEqual(first);
 writeFileSync(join(root,"crates/b/Cargo.toml"),`[package]\nname="b"\nversion="0.1.0"\nedition="2021"\n[dependencies]\n`);
 const third=manifestClosureV1(root,["crates/a"],{memo});
 expect(third.crates.map(row=>row.directory)).not.toContain("ext/leaf");
 writeFileSync(join(root,"Cargo.toml"),readFileSync(join(root,"Cargo.toml"),"utf8").replace('"crates/*"','"crates/*","more/*"'));
 mkdirSync(join(root,"more/x"),{recursive:true});writeFileSync(join(root,"more/x/Cargo.toml"),`[package]\nname="x"\nversion="0.1.0"\n`);
 expect(manifestClosureV1(root,["."],{scope:"workspace",memo}).crates.map(row=>row.directory)).toContain("more/x");
});

test("a path dependency outside the repository, a missing manifest and a patch table are refused or reported",()=>{
 const root=fixture();crate(root,"crates/esc","esc",`[dependencies]\nout={path="../../../outside"}\n`);
 expect(()=>manifestClosureV1(root,["crates/esc"])).toThrow(/leaves the repository/u);
 crate(root,"crates/gone","gone",`[dependencies]\nnone={path="../nothing"}\n`);
 expect(()=>manifestClosureV1(root,["crates/gone"])).toThrow(/has no manifest/u);
 crate(root,"crates/patched","patched",`[patch.crates-io]\nx={path="../../libs/shared"}\n`);
 expect(manifestClosureV1(root,["crates/patched"]).unsupported).toEqual(["patch"]);
});
