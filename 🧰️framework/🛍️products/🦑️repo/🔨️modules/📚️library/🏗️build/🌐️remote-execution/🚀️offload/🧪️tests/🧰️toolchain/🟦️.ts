import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync,statSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {detectWorkerRustcV1,workerRustSourceRemapV1} from "../../🧰️toolchain/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

const shim=`import os, sys
if sys.argv[1:] == ["-vV"]:
    sys.stdout.buffer.write(os.environ["SEMIO_TEST_RUSTC_IDENTITY"].encode())
elif sys.argv[1:] == ["--print", "sysroot"]:
    print(os.environ["SEMIO_TEST_RUSTC_SYSROOT"])
else:
    sys.exit(64)
`;

for(const row of fixture.cases)test(row.name,()=>{
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"rc-"));
 try{
  const sysroot=join(root,"sys root"),compiler=join(sysroot,"bin",row.installed),python=testPythonV1()??"python3";
  mkdirSync(dirname(compiler),{recursive:true});writeFileSync(compiler,shim,{mode:0o755});
  for(const target of row.targets)mkdirSync(join(sysroot,"lib","rustlib",target,"lib"),{recursive:true});
  mkdirSync(join(sysroot,"lib","rustlib","components"),{recursive:true});
  const identity=["rustc 1.0.0","commit-hash: "+fixture.commit,"host: "+row.host,""].join(row.lineEnding);
  const environment={...process.env,SEMIO_TEST_RUSTC_IDENTITY:identity,SEMIO_TEST_RUSTC_SYSROOT:sysroot};
  const probe=spawnSync(python,["-I",compiler,"-vV"],{encoding:"utf8",env:environment,timeout:10000});
  expect(probe.status,probe.stderr).toBe(0);expect(probe.stdout).toBe(identity);
  const reported=spawnSync(python,["-I",compiler,"--print","sysroot"],{encoding:"utf8",env:environment,timeout:10000});
  expect(reported.status,reported.stderr).toBe(0);expect(reported.stdout.trim()).toBe(sysroot);
  const offer=detectWorkerRustcV1(root,environment,[python,"-I",compiler]);
  if(!row.offers){expect(offer).toBeUndefined();return;}
  const digest=spawnSync(python,["-I","-c","import hashlib, sys; print(hashlib.sha256(sys.stdin.buffer.read()).hexdigest()[:16])"],{input:identity,encoding:"utf8",timeout:10000});
  expect(digest.status,digest.stderr).toBe(0);
  const sdk=offer!.properties.filter(property=>property.name==="semio-sdk");
  expect(sdk.length,"only a macOS worker names its SDK").toBe(process.platform==="darwin"?1:0);
  expect({...offer!,properties:offer!.properties.filter(property=>property.name!=="semio-sdk")}).toEqual({
   tools:{rustc:{path:compiler,readable:[sysroot]}},
   toolchain:{commit:fixture.commit,host:row.host,digest:digest.stdout.trim(),targets:row.targets},
   properties:[{name:"semio-offload-format",value:"argv-1"},...row.targets.map(target=>({name:"semio-rustc-target-"+target,value:"1"}))],
  });
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a worker whose sysroot is spelled differently (or lacks rust-src) compiles the same bytes as every other worker",()=>{
 const probe=spawnSync("rustc",["--print","sysroot"],{encoding:"utf8"}),real=probe.status===0?probe.stdout.trim():"";
 if(real===""||!existsSync(join(real,"lib","rustlib","src","rust","library")))return;
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"ws-")));
 try{
  const bare=join(root,"bare");mkdirSync(join(bare,"lib","rustlib"),{recursive:true});
  for(const name of readdirSync(join(real,"lib","rustlib")))if(name!=="src"&&statSync(join(real,"lib","rustlib",name)).isDirectory())symlinkSync(join(real,"lib","rustlib",name),join(bare,"lib","rustlib",name));
  const without=join(root,"rustc-without");writeFileSync(without,`#!/bin/sh\nexec ${join(real,"bin","rustc")} --sysroot ${bare} "$@"\n`);chmodSync(without,0o755);
  const compile=(tool:string,label:string,fixed:boolean)=>{
   const home=join(root,label);mkdirSync(join(home,"src"),{recursive:true});mkdirSync(join(home,"out"),{recursive:true});
   writeFileSync(join(home,"src","lib.rs"),"pub fn f(v:&mut Vec<u32>){v.insert(5,1);}\npub fn g(v:&[u32])->u32{v[7]}\n");
   const args=["--crate-name","probe","--edition=2021","src/lib.rs","--crate-type","lib","--emit=link","-C","opt-level=1","-C","debuginfo=1","-C","metadata=abc","-C","extra-filename=-abc","--out-dir","out","--remap-path-prefix="+home+"=."];
   const run=spawnSync(tool,fixed?[...args,...workerRustSourceRemapV1(tool,args)]:args,{cwd:home,encoding:"utf8"});expect(run.stderr,label).toBe("");
   return createHash("sha256").update(readFileSync(join(home,"out","libprobe-abc.rlib"))).digest("hex");
  };
  const rustc=join(real,"bin","rustc");
  expect(compile(rustc,"plain-with",false),"the premise: rust-src present or absent gives other object code").not.toBe(compile(without,"plain-without",false));
  expect(compile(rustc,"fixed-with",true)).toBe(compile(without,"fixed-without",true));
  expect(workerRustSourceRemapV1(rustc,[`--remap-path-prefix=${join(real,"lib","rustlib","src","rust")}=/rustc/x`])).toEqual([]);
  expect(workerRustSourceRemapV1(join(root,"missing"),[])).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
});
