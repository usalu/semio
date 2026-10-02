import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv/dist/2020.js";

const owner=resolve(import.meta.dir,"../.."), output=process.env.SEMIO_TEST_ARTIFACT_DIR;
if(!output) throw Error("Caller-owned SEMIO_TEST_ARTIFACT_DIR is required");
mkdirSync(output,{recursive:true});
const corpus=JSON.parse(readFileSync(join(owner,"🧫️fixtures/🔣️.json"),"utf8")) as {nativeCases:{id:string;source:string;accept:boolean;output?:string}[]};
const root=mkdtempSync(join(output,"rust-binding-native-"));
const provider=join(root,"provider.rs"), library=join(root,"libdifferent_library.rlib");
writeFileSync(provider,"pub const VALUE:u32=19;");
const invoke=(args:string[])=>{const result=Bun.spawnSync(args,{cwd:root,stdout:"pipe",stderr:"pipe",timeout:4_000});if(result.stdout===undefined||result.stderr===undefined)throw Error("Native binding oracle requires both captured streams");return {...result,stdout:result.stdout,stderr:result.stderr};};
const compiled=invoke(["rustc","--edition=2021","--crate-name=different_library","--crate-type=rlib",provider,"-o",library]);
writeFileSync(join(root,"provider.stderr"),compiled.stderr);
if(compiled.exitCode!==0) throw Error("Native provider compilation failed: "+compiled.stderr.toString());
test("closed native binding corpus",()=>{
 expect(new Ajv({strict:true}).compile(JSON.parse(readFileSync(join(owner,"🧬️schema/🔣️.json"),"utf8")))(corpus)).toBe(true);
 expect(new Set(corpus.nativeCases.map(row=>row.id)).size).toBe(corpus.nativeCases.length);
});
for(const row of corpus.nativeCases) test(row.id,()=>{
 const source=join(root,row.id+".rs"), binary=join(root,row.id+(process.platform==="win32"?".exe":""));
 writeFileSync(source,row.source);
 const built=invoke(["rustc","--edition=2021","--crate-name=consumer",source,"--extern",`wire=${library}`,"-o",binary]);
 writeFileSync(join(root,row.id+".stderr"),built.stderr);
 expect(built.exitCode===0).toBe(row.accept);
 if(row.accept){const result=invoke([binary]);writeFileSync(join(root,row.id+".stdout"),result.stdout);expect(result.exitCode).toBe(0);expect<string | undefined>(result.stdout.toString()).toBe(row.output);}
 else expect(built.stderr.toString()).toMatch(/error\[E0(?:433|599)\]/u);
});
