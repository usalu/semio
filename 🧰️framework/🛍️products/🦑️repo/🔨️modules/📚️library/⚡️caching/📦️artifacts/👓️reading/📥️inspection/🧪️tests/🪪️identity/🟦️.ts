import {test,expect} from "bun:test";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import {chmod,lstat,mkdir,mkdtemp,readdir,rm,writeFile} from "node:fs/promises";
import {dirname,join} from "node:path";
import type {RepositoryArtifactReaderOptionsV1} from "../../../📋️contract/🟦️.ts";
import {inspectArtifactV1,readArtifactManifestV1,readRepositoryArtifactManifestV1} from "../../🟦️.ts";
import corpus from "./🔣️.json" with {type:"json"};

const control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:()=>{},yieldContinuation:async()=>{}};
const schema={type:"object",required:["version","owner","maximumBytes","maximumFiles","files","cases"],properties:{cases:{type:"array",items:{type:"object",required:["name","mutation","stable"],additionalProperties:false,properties:{name:{type:"string"},mutation:{enum:["touch-directories","rewrite-same-size","add-entry","remove-file"]},stable:{type:"boolean"}}}}}};

async function publish(root:string,name:string):Promise<RepositoryArtifactReaderOptionsV1>{
 const path=join(root,name);await mkdir(path,{recursive:true});
 for(const[file,text]of Object.entries(corpus.files)){await mkdir(dirname(join(path,file)),{recursive:true});await writeFile(join(path,file),text);}
 await writeFile(join(path,".semio-artifact.json"),JSON.stringify({version:1,owner:corpus.owner,files:Object.keys(corpus.files).sort()}));
 return {storage:{backing:{root},recheck:async()=>{}},request:{version:1,path,owner:corpus.owner,maximumBytes:corpus.maximumBytes,maximumFiles:corpus.maximumFiles}}as unknown as RepositoryArtifactReaderOptionsV1;
}
async function directories(path:string):Promise<string[]>{const rows=[path];for(const row of await readdir(path,{withFileTypes:true}))if(row.isDirectory())rows.push(...await directories(join(path,row.name)));return rows;}
async function observe(options:RepositoryArtifactReaderOptionsV1){
 const manifest=await readArtifactManifestV1(options,control),declared=readRepositoryArtifactManifestV1(manifest.value,corpus.owner,corpus.maximumFiles);
 return {manifest,declared,original:await inspectArtifactV1(options,declared,control,manifest.fingerprint)};
}

test("artifact identity fixture is closed and its digests match an independent hasher",async()=>{
 expect(new Ajv({strict:false}).validate(schema,corpus)).toBe(true);
 const generated=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!generated)throw Error("Owned artifact identity Source Generated required");await mkdir(generated,{recursive:true});
 const root=await mkdtemp(join(generated,"identity-"));try{
  const options=await publish(root,"digest"),{original}=await observe(options);
  for(const[file,text]of Object.entries(corpus.files))expect(original.some(row=>row.startsWith(file+":")&&row.endsWith(":"+new Bun.CryptoHasher("sha256").update(text).digest("hex")))).toBe(true);
  expect(createHash("sha256").update(corpus.files["entry.js"]).digest("hex")).toBe(new Bun.CryptoHasher("sha256").update(corpus.files["entry.js"]).digest("hex"));
 }finally{await rm(root,{recursive:true,force:true});}
});

for(const row of corpus.cases)test("artifact identity "+row.name,async()=>{
 const generated=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!generated)throw Error("Owned artifact identity Source Generated required");await mkdir(generated,{recursive:true});
 const root=await mkdtemp(join(generated,"identity-"));try{
  const options=await publish(root,"artifact"),path=options.request.path,{manifest,declared,original}=await observe(options);
  if(row.mutation==="touch-directories")for(const directory of await directories(path)){const before=await lstat(directory,{bigint:true});await chmod(directory,Number(before.mode&0o7777n));if(process.platform!=="win32")expect((await lstat(directory,{bigint:true})).ctimeNs).not.toBe(before.ctimeNs);}
  if(row.mutation==="rewrite-same-size"){await writeFile(join(path,"entry.js"),corpus.files["entry.js"].replace("original","0riginal"));}
  if(row.mutation==="add-entry")await writeFile(join(path,"stray.txt"),"stray");
  if(row.mutation==="remove-file")await rm(join(path,"entry.js"));
  const after=await inspectArtifactV1(options,declared,control,manifest.fingerprint).catch(()=>undefined);
  if(row.stable)expect(after).toEqual(original);else expect(JSON.stringify(after)).not.toBe(JSON.stringify(original));
 }finally{await rm(root,{recursive:true,force:true});}
});
