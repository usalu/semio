/** 🪪️ Contract: the install identity of the Nx tooling (which names no authorizer manifest) and the key of the authorizer bundle beside it are each the sha256 the corpus in `./🔣️.json` states and an independent Python hashlib implementation computes, depends on every part of the platform triple and on the order and exact bytes of the files, and cannot be forged by moving bytes across the name and content boundary. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {nxAuthorizerBundleIdentityV1,nxToolingInstallIdentityV1} from "../../🟦️.ts";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py"),encoder=new TextEncoder();
const ask=(rows:unknown[]):string[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({rows}),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const mapOf=(files:readonly(readonly[string,string])[]):Map<string,Uint8Array>=>new Map(files.map(([name,text])=>[name,encoder.encode(text)]));

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,cases:[{...corpus.cases[0],expected:"abc"}]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,boundaries:[{...corpus.boundaries[0],left:[["a"]]}]})).toBe(false);
});

test("every case equals the authored key, the Python key and a direct hash of the documented byte layout",()=>{
 const rows=corpus.cases.map(row=>({environment:row.environment,files:row.files.map(file=>[file.name,file.text])})),answers=ask(rows);
 for(const[index,row]of corpus.cases.entries()){
  const actual=nxToolingInstallIdentityV1(row.environment,mapOf(rows[index]!.files as never));
  expect(actual,row.name).toBe(row.expected);expect(answers[index],row.name).toBe(row.expected);
  const direct=createHash("sha256").update("nx-tooling-install-v5\0"),frame=(value:Uint8Array|string)=>{const bytes=typeof value==="string"?encoder.encode(value):value;direct.update(bytes.byteLength+":").update(bytes);};
  for(const value of[row.environment.platform,row.environment.architecture,row.environment.libc])frame(value);
  for(const file of row.files){frame(file.name);frame(file.text);}
  expect(direct.digest("hex"),row.name).toBe(row.expected);
 }
 expect(new Set(corpus.cases.map(row=>row.expected)).size).toBe(corpus.cases.length);
});

test("every authorizer bundle key equals the authored key, the Python key and a direct hash, and moves with each part",()=>{
 const rows=corpus.bundles.map(row=>({dependencies:row.dependencies,files:row.files.map(file=>[file.name,file.text])})),answers=ask(rows),encode=(value:string)=>encoder.encode(value);
 for(const[index,row]of corpus.bundles.entries()){
  const actual=nxAuthorizerBundleIdentityV1(row.dependencies,mapOf(rows[index]!.files as never));
  expect(actual,row.name).toBe(row.expected);expect(answers[index],row.name).toBe(row.expected);
  const direct=createHash("sha256").update("nx-authorizer-bundle-v1\0"),frame=(value:Uint8Array|string)=>{const bytes=typeof value==="string"?encode(value):value;direct.update(bytes.byteLength+":").update(bytes);};
  frame(row.dependencies);for(const file of row.files){frame(file.name);frame(file.text);}
  expect(direct.digest("hex"),row.name).toBe(row.expected);
 }
 expect(new Set(corpus.bundles.map(row=>row.expected)).size).toBe(corpus.bundles.length);
 expect(()=>nxAuthorizerBundleIdentityV1("short",new Map())).toThrow();
});

test("the installation key never sees an authorizer manifest and the bundle key never sees an installation file",()=>{
 const environment={platform:"linux",architecture:"x64",libc:"glibc"},installation=[["package.json","{}\n"],["bun.lock","{}\n"]] as const,manifests=[[".semio-authorizer-source.json","a"],[".semio-authorizer-compiler.json","b"]] as const;
 const base=nxToolingInstallIdentityV1(environment,mapOf(installation as never)),dependencies=base;
 const first=nxAuthorizerBundleIdentityV1(dependencies,mapOf(manifests as never)),second=nxAuthorizerBundleIdentityV1(dependencies,mapOf([[".semio-authorizer-source.json","a2"],manifests[1]] as never));
 expect(nxToolingInstallIdentityV1(environment,mapOf(installation as never))).toBe(base);expect(second).not.toBe(first);
 expect(nxAuthorizerBundleIdentityV1(nxToolingInstallIdentityV1(environment,mapOf([["package.json","{\"a\":1}\n"],installation[1]] as never)),mapOf(manifests as never))).not.toBe(first);
});

test("boundaries between names and contents never collide",()=>{
 const rows=corpus.boundaries.flatMap(row=>[{environment:row.environment,files:row.left},{environment:row.environment,files:row.right}]),answers=ask(rows);
 for(const[index,row]of corpus.boundaries.entries()){
  const left=nxToolingInstallIdentityV1(row.environment,mapOf(row.left as never)),right=nxToolingInstallIdentityV1(row.environment,mapOf(row.right as never));
  expect(left,row.name).not.toBe(right);expect([left,right],row.name).toEqual([answers[index*2],answers[index*2+1]]);
 }
});

test("every platform, architecture and libc gives its own key, equal to Python's, and the same inputs always give the same key",()=>{
 const axes=corpus.environments,rows:{environment:{platform:string;architecture:string;libc:string};files:string[][]}[]=[];
 for(const platform of axes.platforms)for(const architecture of axes.architectures)for(const libc of axes.libcs)rows.push({environment:{platform,architecture,libc},files:[["package.json","{}\n"]]});
 const answers=ask(rows),keys=rows.map(row=>nxToolingInstallIdentityV1(row.environment,mapOf(row.files as never)));
 expect(keys).toEqual(answers);expect(new Set(keys).size).toBe(rows.length);
 expect(nxToolingInstallIdentityV1(rows[0]!.environment,mapOf(rows[0]!.files as never))).toBe(keys[0]!);
 expect(keys.every(key=>/^[a-f0-9]{64}$/u.test(key))).toBe(true);
});

