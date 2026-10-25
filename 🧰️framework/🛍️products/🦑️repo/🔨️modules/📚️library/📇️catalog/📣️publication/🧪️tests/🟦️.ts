/** 📣️ Contract: catalog publication resolves strict RFC 6901 pointers without prototype aliases, reads the one admitted `manifest` path of the owner publication subtable and records bounded component admission without following symlinks; the corpus is language-agnostic, `fast-json-patch` and `@iarna/toml` are the independent third-party oracles for pointers and the TOML subtable, and `🐍️.py` re-derives both. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {getValueByPointer} from "fast-json-patch";
import {parse} from "@iarna/toml";
import {afterAll} from "bun:test";
import {closeSync,ftruncateSync,mkdirSync,mkdtempSync,openSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {ownerPublicationManifestPath,publicationComponentAdmission,publicationPointer} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const scratch=(files:Record<string,string>):string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"catpub-")));roots.push(root);for(const[path,text]of Object.entries(files)){const target=join(root,...path.split("/"));mkdirSync(dirname(target),{recursive:true});writeFileSync(target,text);}return root;};
for(const row of corpus.pointerOk)test("pointer "+JSON.stringify(row.pointer),()=>{expect(publicationPointer(corpus.document,row.pointer)).toEqual(row.expected);});
for(const pointer of corpus.pointerMissing)test("pointer "+JSON.stringify(pointer)+" is missing",()=>{expect(()=>publicationPointer(corpus.document,pointer)).toThrow("Missing publication pointer: "+pointer);});
for(const pointer of corpus.pointerInvalid)test("pointer "+JSON.stringify(pointer)+" is invalid",()=>{expect(()=>publicationPointer(corpus.document,pointer)).toThrow("Invalid publication pointer: "+pointer);});
test("fast-json-patch resolves every valid pointer to the same value and finds none of the missing ones",()=>{
 for(const row of corpus.pointerOk)expect(getValueByPointer(corpus.document,row.pointer),row.pointer).toEqual(row.expected);
 for(const pointer of corpus.pointerMissing.filter(item=>!["/toString","/__proto__","/constructor","/a/x/01"].includes(item))){let found:unknown;try{found=getValueByPointer(corpus.document,pointer);}catch{found=undefined;}expect(found,pointer).toBeUndefined();}
});
for(const row of corpus.tomlOk)test("owner manifest path: "+row.name,()=>{expect(ownerPublicationManifestPath(row.text)).toBe(row.expected??undefined);});
for(const row of corpus.tomlErrors)test("owner manifest path fails on "+row.name,()=>{expect(()=>ownerPublicationManifestPath(row.text)).toThrow(row.message);});
test("@iarna/toml reads the same manifest path from every valid document",()=>{
 for(const row of corpus.tomlOk){const parsed=parse(row.text) as {package?:{metadata?:{semio?:{publications?:{manifest?:string}}}}};expect(parsed.package?.metadata?.semio?.publications?.manifest,row.name).toBe(row.expected??undefined);}
});
for(const row of corpus.admission.cases)test("component admission of "+row.target,()=>{const root=scratch(corpus.admission.files);expect<unknown>(publicationComponentAdmission(root,row.target)).toEqual(row.expected);});
for(const row of corpus.admission.errors)test("component admission refuses "+row.target,()=>{const root=scratch(corpus.admission.files);expect(()=>publicationComponentAdmission(root,row.target)).toThrow(row.message);});
test("an absolute target outside the workspace and a symlink are refused, an oversized file is flagged",()=>{
 const root=scratch(corpus.admission.files),outside=scratch({"x":"y"});expect(()=>publicationComponentAdmission(root,join(outside,"x"))).toThrow("Publication component escapes workspace");
 if(process.platform==="win32")return;
 symlinkSync(join(root,"comp"),join(root,"link"));expect(()=>publicationComponentAdmission(root,"link/ok.wasm")).toThrow("no-follow regular ancestry");
 const fd=openSync(join(root,"comp","big.wasm"),"w");ftruncateSync(fd,corpus.admission.exceedsBytes);closeSync(fd);
 expect(publicationComponentAdmission(root,"comp/big.wasm")).toEqual({status:"exceeds-limit",limitBytes:67108864,byteLength:corpus.admission.exceedsBytes,path:"comp/big.wasm"});
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
