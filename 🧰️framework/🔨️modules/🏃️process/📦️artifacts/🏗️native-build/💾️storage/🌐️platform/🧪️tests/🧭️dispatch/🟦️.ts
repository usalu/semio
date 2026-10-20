import {expect,test} from "bun:test";
import Ajv from "ajv";
import {existsSync,readdirSync,readFileSync,statSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import fixture from "./🧫️fixtures/🔣️.json";
import {nativeStoragePlatformV1} from "../../🟦️.ts";

const platformRoot=resolve(import.meta.dir,"../.."),storageRoot=resolve(platformRoot,"..");
const repositoryRoot=(()=>{let current=import.meta.dir;for(let depth=0;depth<24;depth++){if(existsSync(join(current,".devcontainer","devcontainer.json")))return current;const parent=dirname(current);if(parent===current)break;current=parent;}return undefined;})();
const walk=(directory:string):string[]=>readdirSync(directory,{withFileTypes:true}).flatMap(entry=>{const path=join(directory,entry.name);return entry.isDirectory()?walk(path):/\.(ts|mjs|json|py|feature)$/u.test(entry.name)?[path]:[];});

test("the declared platform selects its backend: macOS the mounted image, Linux and Windows the directory partition",()=>{
 const check=new Ajv({strict:true}).compile({type:"object",additionalProperties:false,required:["platform","backend","opens","retires"],properties:{platform:{enum:["darwin","linux","win32"]},backend:{enum:["macos","linux","windows"]},opens:{type:"string"},retires:{type:"string"}}});
 const source=readFileSync(join(platformRoot,"🟦️.ts"),"utf8");
 for(const row of fixture.platforms){
  expect(check(row),JSON.stringify(check.errors)).toBe(true);expect(nativeStoragePlatformV1(row.platform),row.platform).toBe(row.backend);
  const image=row.backend==="macos";
  expect(source,row.platform).toContain(`const image=await(backend==="macos"?openNativeImageStorageV1(options):openNativeDirectoryImageStorageV1(options));`);
  expect(source,row.platform).toContain(`return backend==="macos"?retireNativeImageStorageV1(options,admitted):retireNativeDirectoryImageStorageV1(options,admitted);`);
  expect(image?row.opens==="openNativeImageStorageV1":row.opens==="openNativeDirectoryImageStorageV1",row.platform).toBe(true);
 }
 for(const platform of fixture.unsupported)expect(()=>nativeStoragePlatformV1(platform),platform).toThrow("Declared native fixed storage platform required");
});

test("only the macOS branch reaches the mounted-image module; the directory module refuses macOS and the image module refuses every other host",()=>{
 const directory=readFileSync(join(storageRoot,"📁️image","🟦️.ts"),"utf8"),image=readFileSync(join(storageRoot,"🍎️image","🟦️.ts"),"utf8");
 expect(directory).toContain('const directoryHost=()=>process.platform==="linux"||process.platform==="win32";');
 expect(directory.match(/Directory partition backend unavailable on this platform/gu)?.length).toBe(2);
 expect(image.match(/process\.platform!=="darwin"\)throw Error\("macOS fixed image enforcement is unavailable on this platform"\)/gu)?.length).toBeGreaterThanOrEqual(3);
 const dispatch=readFileSync(join(platformRoot,"🟦️.ts"),"utf8");
 expect(dispatch.match(/openNativeImageStorageV1/gu)?.length).toBe(2);expect(dispatch.match(/retireNativeImageStorageV1/gu)?.length).toBe(2);
});

test("no source below the storage module names a retired loop-device, image-file or privileged-mount identifier",()=>{
 const own=join(platformRoot,"🧪️tests","🧭️dispatch"),offenders:string[]=[];
 for(const path of walk(storageRoot)){
  if(path.startsWith(own))continue;
  const text=readFileSync(path,"utf8");
  for(const word of fixture.retiredIdentifiers)if(text.includes(word))offenders.push(path.slice(storageRoot.length)+": "+word);
 }
 expect(offenders).toEqual([]);
});

test.skipIf(repositoryRoot===undefined)("the development container needs no privilege, capability, device or loop mount for the native partitions",()=>{
 const compose=readFileSync(join(repositoryRoot!,".devcontainer","docker-compose.yml"),"utf8").split("\n").filter(line=>!/^\s*#/u.test(line)),definition=readFileSync(join(repositoryRoot!,".devcontainer","devcontainer.json"),"utf8");
 for(const key of fixture.unprivilegedKeys)expect(compose.some(line=>new RegExp("^\\s*"+key+"\\s*:","u").test(line)),"docker-compose.yml: "+key).toBe(false);
 for(const word of fixture.devcontainerForbidden)expect(definition.includes(word),"devcontainer.json: "+word).toBe(false);
 expect(statSync(join(repositoryRoot!,".devcontainer")).isDirectory()).toBe(true);
});
