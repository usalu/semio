import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{pinnedRustToolchainV1}from"../🟦️.ts";
import{toolRootV1}from"../../../../🧰️tools/🔎️resolve/🟦️.ts";
import{testPythonV1}from"../../🟦️.ts";
import fixture from"./🧫️fixtures/🔣️.json";

const port=(file:string|null)=>({read:(path:string)=>path.endsWith("rust-toolchain.toml")&&file!==null?file:undefined});
const root=toolRootV1(import.meta.dirname);

test("Pinned Rust toolchain: the channel is the channel of the pin file for every selection the environment may carry",()=>{
 for(const row of fixture.selection){
  const result=pinnedRustToolchainV1("/repo",row.environment,port(row.file));
  expect(result.channel,row.name).toBe(row.channel);expect(result.environment,row.name).toEqual({RUSTUP_TOOLCHAIN:row.channel});
 }
});

test("Pinned Rust toolchain: a missing pin or a disagreeing selection is refused in the chosen language",()=>{
 for(const row of fixture.refusals)expect(()=>pinnedRustToolchainV1("/repo",row.environment,port(row.file)),row.name).toThrow(row.message);
});

test("Pinned Rust toolchain: on this checkout the channel equals the channel an independent reader finds in the pin file",()=>{
 const text=readFileSync(join(root,"rust-toolchain.toml"),"utf8"),own=pinnedRustToolchainV1(root,{}).channel;
 expect(own).toBe((Bun.TOML.parse(text)as{toolchain:{channel:string}}).toolchain.channel);
 const python=testPythonV1();
 if(python===null)return;
 const answer=Bun.spawnSync([python,"-c","import re,sys;t=open(sys.argv[1],encoding='utf8').read();m=re.search(r'^\\s*channel\\s*=\\s*\"([^\"]+)\"',t,re.M);print(m.group(1))",join(root,"rust-toolchain.toml")],{stdout:"pipe"});
 expect(answer.stdout.toString().trim()).toBe(own);
});

test("Pinned Rust toolchain: a compiler started from a scratch directory with the pinned selection is the compiler rustup selects inside the checkout",()=>{
 const clean=Object.fromEntries(Object.entries(process.env).filter(([name,value])=>name!=="RUSTUP_TOOLCHAIN"&&value!==undefined))as Record<string,string>,run=(cwd:string,environment:Record<string,string>)=>{try{return Bun.spawnSync(["rustc","-vV"],{cwd,env:environment,stdout:"pipe",stderr:"pipe"});}catch{return undefined;}};
 const inside=run(root,clean);
 if(inside===undefined||inside.exitCode!==0){process.stderr.write("[toolchain] rustc is not available here; the selection comparison is skipped.\n");return;}
 const scratch=process.env.SEMIO_TEST_ARTIFACT_DIR??process.env.TMPDIR??"/tmp",outside=run(scratch,{...clean,...pinnedRustToolchainV1(root,{}).environment});
 expect(outside?.exitCode).toBe(0);expect(outside!.stdout.toString()).toBe(inside.stdout.toString());
});
