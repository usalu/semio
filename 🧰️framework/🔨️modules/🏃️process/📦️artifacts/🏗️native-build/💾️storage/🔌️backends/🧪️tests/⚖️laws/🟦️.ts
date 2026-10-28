import {expect,test} from "bun:test";
import {readdirSync,readFileSync} from "node:fs";
import {dirname,join,relative,resolve,sep} from "node:path";
import fixture from "./🧫️fixtures/🔣️.json";
import {nativeStorageBackendsV1} from "../../🧭️selection/🟦️.ts";

const storageRoot=resolve(import.meta.dir,"../../.."),processRoot=resolve(storageRoot,"../../.."),composition="🔌️backends/🧩️builtin";
const walk=(directory:string):string[]=>readdirSync(directory,{withFileTypes:true}).flatMap(entry=>{if(entry.name==="node_modules")return[];const path=join(directory,entry.name);return entry.isDirectory()?walk(path):[path];});
const isTest=(path:string)=>path.includes(sep+"🧪️tests"+sep);
const code=(path:string)=>/\.(ts|mjs)$/u.test(path)&&!isTest(path);
const storageFiles=walk(storageRoot).filter(code).map(path=>relative(storageRoot,path));
const ownerOf=(path:string):string=>{let best:{id:string;length:number}|undefined;for(const row of nativeStorageBackendsV1)for(const module of row.modules)if((path===module||path.startsWith(module+"/"))&&(!best||module.length>best.length))best={id:row.id,length:module.length};return best?.id??"neutral";};
const withoutSpecifiers=(text:string)=>text.replace(/(?:from|import)\s*\(?\s*(["'`])[^"'`\n]*\1/gu,"");
const edges=(path:string)=>{const text=readFileSync(join(storageRoot,path),"utf8"),found=new Set<string>();for(const match of text.matchAll(/(?:from|import)\s*\(?\s*["'`]([^"'`\n]+)["'`]/gu)){const specifier=match[1]!;if(!specifier.startsWith("."))continue;const target=relative(storageRoot,resolve(storageRoot,dirname(path),specifier));if(!target.startsWith(".."))found.add(target);}return[...found];};

test("every declared module path exists and belongs to exactly one backend",()=>{
 const seen=new Map<string,string>();
 for(const row of nativeStorageBackendsV1)for(const module of row.modules){expect(walk(join(storageRoot,module)).length,module).toBeGreaterThan(0);expect(seen.get(module),module).toBeUndefined();seen.set(module,row.id);}
});

test("neutral modules reach a backend-owned module only through the registry; the remaining entanglements are pinned and may only shrink",()=>{
 const found:string[]=[];
 for(const path of storageFiles){
  const owner=ownerOf(path);
  for(const target of edges(path)){const targetOwner=ownerOf(target);if(targetOwner!=="neutral"&&targetOwner!==owner&&!path.startsWith(composition+sep)&&!path.startsWith(composition+"/"))found.push(path+" -> "+targetOwner+": "+target);}
 }
 expect(found.sort()).toEqual([...fixture.knownEntanglements].sort());
});

test("a backend never imports another backend's modules",()=>{
 const offenders:string[]=[];
 for(const path of storageFiles){const owner=ownerOf(path);if(owner==="neutral")continue;for(const target of edges(path)){const targetOwner=ownerOf(target);if(targetOwner!=="neutral"&&targetOwner!==owner)offenders.push(path+" -> "+target);}}
 expect(offenders.sort()).toEqual([...fixture.knownCrossBackend].sort());
});

test("the neutral contract, the registry and the facade import no backend and read no platform",()=>{
 for(const path of fixture.platformFree){
  const text=readFileSync(join(storageRoot,path),"utf8");
  expect(text.includes("process.platform"),path+" process.platform").toBe(false);
  for(const target of edges(path))expect(ownerOf(target),path+" -> "+target).toBe("neutral");
 }
 const facade=edges("🌐️platform/🟦️.ts");
 expect(facade.filter(target=>target.startsWith(composition))).toEqual([composition+"/🟦️.ts"]);
});

test("the image and signing tools are named in exactly one storage module each, inside the macOS adapter's modules",()=>{
 const files=walk(processRoot).filter(path=>/\.(ts|mjs|json|py)$/u.test(path)&&!isTest(path)).map(path=>relative(processRoot,path));
 const find=(pattern:RegExp)=>files.filter(path=>pattern.test(withoutSpecifiers(readFileSync(join(processRoot,path),"utf8")))).sort();
 const prefix="📦️artifacts/🏗️native-build/💾️storage/";
 expect(find(/["'`]\/usr\/bin\/hdiutil["'`]/u)).toEqual([...fixture.hdiutilLiteralsOutsideStorage,prefix+"🍎️image/🛠️hdiutil/🟦️.ts"].sort());
 expect(find(/["'`]\/usr\/bin\/codesign["'`]/u)).toEqual([prefix+"🔌️backends/🍎️mounted-image/✍️sealing/🟦️.ts"]);
 const mentions=find(/hdiutil/u).filter(path=>path.startsWith(prefix)&&ownerOf(path.slice(prefix.length))!=="mounted-image");
 expect(mentions).toEqual([]);
});

test("the platform-keyed gates of the directory partition read the declared platforms",()=>{
 const declared=nativeStorageBackendsV1.find(row=>row.id==="directory-partition")!.platforms;
 const source=readFileSync(join(storageRoot,"📤️publication/🧱️fixed/🟦️.ts"),"utf8"),gate=source.match(/!\[("[a-z0-9]+"(?:,"[a-z0-9]+")*)\]\.includes\(process\.platform\)/gu)??[];
 expect(gate.length).toBe(3);
 for(const row of gate)expect(JSON.parse("["+row.slice(2,row.indexOf("]"))+"]")).toEqual([...declared]);
});

test("every name the explicit public facade of the platform contract re-exports still exists in the contract",()=>{
 const facade=readFileSync(join(processRoot,"📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts"),"utf8"),contract=readFileSync(join(storageRoot,"🌐️platform/📋️contract/🟦️.ts"),"utf8"),statfs=readFileSync(join(storageRoot,"🌐️platform/📋️contract/📐️statfs/🟦️.ts"),"utf8");
 const exported=new Set([...(contract+statfs).matchAll(/export\s+(?:async\s+)?(?:function|const|interface|type)\s+([A-Za-z0-9_]+)/gu)].map(match=>match[1]!));
 for(const block of (contract+statfs).matchAll(/export\s+(?:type\s+)?\{([^}]*)\}/gu))for(const item of block[1]!.split(","))if(item.trim())exported.add(item.trim().replace(/^type\s+/u,""));
 const wanted=[...facade.matchAll(/export\s+(?:type\s+)?\{([^}]*)\}/gu)].flatMap(block=>block[1]!.split(",").map(item=>item.trim().replace(/^type\s+/u,"")).filter(Boolean));
 expect(wanted.length).toBeGreaterThan(30);
 expect(wanted.filter(name=>!exported.has(name))).toEqual([]);
});
