import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{resolve}from"node:path";
import{spawnSync}from"node:child_process";
import * as TOML from"@iarna/toml";
import Ajv from"ajv";
import{selectRustSourceV1}from"../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import law from"./🔣️.json";
import schema from"./🧬️schema/🔣️.json";
const root=process.cwd(),channel=(TOML.parse(readFileSync(resolve(root,"rust-toolchain.toml"),"utf8"))as any).toolchain.channel;
function rustParse(source:string){const result=spawnSync(process.platform==="win32"?"repo.exe":"repo",["tools","exec","--light","rustup","--","run",channel,"rustc","-Z","parse-crate-root-only","--crate-type=lib","-"],{cwd:root,env:{...process.env,SEMIO_LANGUAGE:"en"},input:source,encoding:"utf8",timeout:30000,maxBuffer:32768});if(result.error)throw result.error;expect(result.signal).toBeNull();if(result.status!==0)expect(result.stderr).toContain("unclosed delimiter");return result.status===0;}
test("native history implementation boundaries match the independent Rust parser",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 for(const row of law.cases){let accepted=true;try{selectRustSourceV1(row.source,[])}catch(error){expect(String(error)).toContain("Unbalanced Rust source delimiter");accepted=false}expect(accepted,row.id).toBe(row.accepted);expect(rustParse(row.source),row.id).toBe(row.accepted);}
},{timeout:120000});
test("the actual history command has closed top-level implementation boundaries",()=>{
 const source=readFileSync(resolve(import.meta.dir,"../../🦀️.rs"),"utf8");
 expect(()=>selectRustSourceV1(source,[])).not.toThrow();expect(rustParse(source)).toBe(true);
},{timeout:30000});
