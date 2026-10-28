import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{pinnedRustToolchainV1,pinnedTestToolsV1,pinnedTestToolVariableV1}from"../🟦️.ts";
import{systemToolHostV1}from"../../../../🧰️tools/🔎️resolve/🟦️.ts";
import{existsSync}from"node:fs";
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
 if(!python)throw Error("Independent Python oracle required");
 const answer=Bun.spawnSync([python,"-c","import re,sys;t=open(sys.argv[1],encoding='utf8').read();m=re.search(r'^\\s*channel\\s*=\\s*\"([^\"]+)\"',t,re.M);print(m.group(1))",join(root,"rust-toolchain.toml")],{stdout:"pipe"});
 expect(answer.stdout.toString().trim()).toBe(own);
});

const clean=Object.fromEntries(Object.entries(process.env).filter(([name,value])=>name!=="RUSTUP_TOOLCHAIN"&&value!==undefined))as Record<string,string>,run=(cwd:string,environment:Record<string,string>)=>{try{return Bun.spawnSync(["rustc","-vV"],{cwd,env:environment,stdout:"pipe",stderr:"pipe"});}catch{return undefined;}},inside=run(root,clean);

test.skipIf(inside===undefined||inside.exitCode!==0)("Pinned Rust toolchain: a compiler started from a scratch directory with the pinned selection is the compiler rustup selects inside the checkout (needs rustc)",()=>{
 const scratch=process.env.SEMIO_TEST_ARTIFACT_DIR??process.env.TMPDIR??"/tmp",outside=run(scratch,{...clean,...pinnedRustToolchainV1(root,{}).environment});
 expect(outside?.exitCode).toBe(0);expect(outside!.stdout.toString()).toBe(inside!.stdout.toString());
});

const hostWith=(environment:Record<string,string>,present:readonly string[])=>({...systemToolHostV1("/repo",environment),exists:(path:string)=>present.includes(path),isExecutable:(path:string)=>present.includes(path)});

test("Pinned test tools: every named tool becomes SEMIO_TEST_<TOOL>_BINARY from the registry resolution, the same one the product uses",()=>{
 const environment={SEMIO_TOOL_BAZEL:"/pin/bazel",SEMIO_TOOL_WASM_OPT:"/pin/wasm-opt"};
 expect(pinnedTestToolsV1(["bazel","wasm-opt"],"/repo",environment,hostWith(environment,["/pin/bazel","/pin/wasm-opt"]))).toEqual({SEMIO_TEST_BAZEL_BINARY:"/pin/bazel",SEMIO_TEST_WASM_OPT_BINARY:"/pin/wasm-opt"});
 expect(pinnedTestToolsV1([],"/repo",{},hostWith({},[]))).toEqual({});
 expect(pinnedTestToolVariableV1("wasm-opt")).toBe("SEMIO_TEST_WASM_OPT_BINARY");
});

test("Pinned test tools: a tool that is unknown or not provisioned is refused in the chosen language, never skipped",()=>{
 const none=hostWith({},[]);
 expect(()=>pinnedTestToolsV1(["bazel"],"/repo",{SEMIO_LANGUAGE:"en"},none)).toThrow("is not provisioned on this host");
 expect(()=>pinnedTestToolsV1(["bazel"],"/repo",{SEMIO_LANGUAGE:"de"},none)).toThrow("nicht bereitgestellt");
 expect(()=>pinnedTestToolsV1(["no-such-tool"],"/repo",{SEMIO_LANGUAGE:"en"},none)).toThrow("is not registered");
 expect(()=>pinnedTestToolsV1(["no-such-tool"],"/repo",{SEMIO_LANGUAGE:"de"},none)).toThrow("nicht registriert");
});

test("Pinned test tools: on this checkout the pinned Bazel is the file an independent Python reading of the registry and the release pin names, or the resolution refuses",async()=>{
 const python=testPythonV1();if(python===null)throw Error("Independent Python oracle required");
 const answer=Bun.spawnSync([python,join(import.meta.dirname,"🐍️oracle/🐍️.py"),root],{stdout:"pipe",stderr:"pipe"});
 expect(answer.exitCode,answer.stderr.toString()).toBe(0);
 const expected=answer.stdout.toString().trim(),provisioned=existsSync(expected);
 if(provisioned)expect(pinnedTestToolsV1(["bazel"],root,{}).SEMIO_TEST_BAZEL_BINARY).toBe(expected);
 else expect(()=>pinnedTestToolsV1(["bazel"],root,{SEMIO_LANGUAGE:"en"})).toThrow("is not provisioned on this host");
});
