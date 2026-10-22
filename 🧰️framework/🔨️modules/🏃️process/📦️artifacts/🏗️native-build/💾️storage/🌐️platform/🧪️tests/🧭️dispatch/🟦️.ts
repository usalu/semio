import {expect,test} from "bun:test";
import Ajv from "ajv";
import {existsSync,readdirSync,readFileSync,statSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import fixture from "./🧫️fixtures/🔣️.json";
import {selectNativeStorageBackendV1} from "../../../🔌️backends/🧭️selection/🟦️.ts";

const platformRoot=resolve(import.meta.dir,"../.."),storageRoot=resolve(platformRoot,"..");
const repositoryRoot=(()=>{let current=import.meta.dir;for(let depth=0;depth<24;depth++){if(existsSync(join(current,".devcontainer","devcontainer.json")))return current;const parent=dirname(current);if(parent===current)break;current=parent;}return undefined;})();
const walk=(directory:string):string[]=>readdirSync(directory,{withFileTypes:true}).flatMap(entry=>{const path=join(directory,entry.name);return entry.isDirectory()?walk(path):/\.(ts|mjs|json|py|feature)$/u.test(entry.name)?[path]:[];});

test("the declared selection policy chooses the mounted image on macOS and the directory partition on Linux and Windows",()=>{
 const check=new Ajv({strict:true}).compile({type:"object",additionalProperties:false,required:["platform","backend"],properties:{platform:{enum:["darwin","linux","win32"]},backend:{enum:["mounted-image","directory-partition"]}}});
 for(const row of fixture.platforms){expect(check(row),JSON.stringify(check.errors)).toBe(true);expect(selectNativeStorageBackendV1({platform:row.platform}).id,row.platform).toBe(row.backend);}
 for(const platform of fixture.unsupported)expect(()=>selectNativeStorageBackendV1({platform}),platform).toThrow("Declared native storage platform required");
});

test("the platform facade names no backend module and no platform: every operation routes through the bound registry",()=>{
 const source=readFileSync(join(platformRoot,"🟦️.ts"),"utf8");
 for(const forbidden of fixture.facadeForbiddenImports)expect(source.includes(forbidden+"/"),forbidden).toBe(false);
 expect(source).not.toContain("process.platform");
 expect(source).not.toContain('=== "macos"');
 expect(source.match(/nativeStorageOperationsV1\(/gu)?.length).toBeGreaterThanOrEqual(11);
 expect(source).toContain('import "../🔌️backends/🧩️builtin/🟦️.ts";');
});

test("the backends refuse hosts they are not declared for, and the image module refuses every host but macOS",()=>{
 const directory=readFileSync(join(storageRoot,"📁️image","🟦️.ts"),"utf8"),image=readFileSync(join(storageRoot,"🍎️image","🟦️.ts"),"utf8");
 expect(directory).toContain("const directoryHost=()=>directoryPartitionHostV1();");
 expect(directory.match(/Directory partition backend unavailable on this platform/gu)?.length).toBe(2);
 expect(image.match(/process\.platform!=="darwin"\)throw Error\("macOS fixed image enforcement is unavailable on this platform"\)/gu)?.length).toBeGreaterThanOrEqual(3);
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
