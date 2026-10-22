/** 📜 Contract: the TOML boundary of the native build inputs (`readToml`, backed by the Bun parser) reads the documents Cargo manifests use as TOML 1.0 defines them, equal to the authored values in `./🔣️.json` and to two independent parsers (smol-toml and @iarna/toml); it refuses what they refuse with an observation that names the file, its hash and the failing line, and the documented divergences of the Bun parser are pinned so that a parser upgrade shows up here. */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {parse as iarna} from "@iarna/toml";
import Ajv from "ajv";
import {parse as smol} from "smol-toml";
import {readToml} from "../../🟨️.mjs";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

let directory="";
beforeAll(()=>{directory=realpathSync(mkdtempSync(join(tmpdir(),"toml-boundary-")));});
afterAll(()=>{rmSync(directory,{recursive:true,force:true});});
const plain=(value:unknown)=>JSON.parse(JSON.stringify(value));
const attempt=<T>(action:()=>T):{value?:T;error?:Error}=>{try{return{value:action()};}catch(error){return{error:error as Error};}};
let counter=0;
const fileOf=(text:string)=>{const path=join(directory,`m${counter++}.toml`);writeFileSync(path,text);return path;};

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,documents:[{...corpus.documents[0],bun:"maybe"}]})).toBe(false);
});

test("the authored values are what two independent TOML parsers read, and they refuse the same documents",()=>{
 for(const row of corpus.documents){
  const first=attempt(()=>plain(smol(row.text))),second=attempt(()=>plain(iarna(row.text)));
  if(row.expected===null){expect(first.error,row.name+" (smol-toml)").toBeDefined();expect(second.error,row.name+" (@iarna/toml)").toBeDefined();}
  else{expect(first.error,row.name+" "+first.error?.message).toBeUndefined();expect(first.value,row.name+" (smol-toml)").toEqual(row.expected);expect(second.error,row.name+" "+second.error?.message).toBeUndefined();expect(second.value,row.name+" (@iarna/toml)").toEqual(row.expected);}
 }
});

test("the boundary reads every document the Bun parser reads as TOML does and refuses the documents TOML refuses",()=>{
 for(const row of corpus.documents.filter(row=>row.bun==="same")){
  const got=attempt(()=>plain(readToml(fileOf(row.text))));
  if(row.expected===null)expect(got.error,row.name).toBeDefined();
  else{expect(got.error,row.name+" "+got.error?.message).toBeUndefined();expect(got.value,row.name).toEqual(row.expected);}
 }
});

test("the Bun parser's known divergences from TOML 1.0 stay pinned",()=>{
 for(const row of corpus.documents.filter(row=>row.bun!=="same")){
  const got=attempt(()=>plain(readToml(fileOf(row.text))));
  if(row.bun==="refuses")expect(got.error,row.name).toBeDefined();
  else{expect(got.error,row.name).toBeUndefined();if(row.expected!==null)expect(got.value,row.name).not.toEqual(row.expected);}
 }
});

test("a refused file reports an observation naming the file, its hash, its size and the failing line",()=>{
 for(const row of corpus.refusals){
  const path=fileOf(row.text),got=attempt(()=>readToml(path)),error=got.error as Error&{code?:string;tomlObservation?:Record<string,unknown>;source?:string;sha256?:string};
  expect(error,row.name).toBeDefined();
  expect(error.code).toBe("BUILD_SOURCE_UNPARSEABLE");
  expect(error.message.startsWith("Observed TOML parse refused: ")).toBe(true);
  const observation=JSON.parse(error.message.slice("Observed TOML parse refused: ".length));
  expect(observation).toEqual(error.tomlObservation!);
  expect(observation.path).toBe(path);expect(error.source).toBe(path);
  expect(observation.sha256).toBe(createHash("sha256").update(row.text).digest("hex"));expect(error.sha256).toBe(observation.sha256);
  expect(observation.bytes).toBe(Buffer.byteLength(row.text));
  expect(observation.version).toBe(1);expect(String(observation.namespace)).toMatch(/^toml-parse-v1:/u);
  expect(String(observation.sourceLine).length).toBeLessThanOrEqual(512);
  expect(row.text.split(/\r?\n/u)[Number(observation.line)]).toBe(observation.sourceLine);
 }
});
