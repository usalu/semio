/** 🚫️ Contract: no generation input lies under a cache root. The workspace walk of the roster never enters a directory the declared policy lists as generated, the sealed copies of the unit cache are listed, and a member that vanishes is noticed by exactly one advance and by none after it. An independent Python walker answers the same member list.
 * @see ../../🟨️.mjs */
import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {lstat,readdir} from "node:fs/promises";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import cachePolicy from "../../../../../⚡️caching/🔣️policy.json" with {type: "json"};
import {UNIT_CACHE_POLICY_V1} from "../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts";
import {advanceFrontierV1,walkFrontierV1} from "../../../🕰️generations/🌿️frontier-walk/🟨️.mjs";
import {diffFrontiersV1} from "../../../🕰️generations/🧭️frontier/🟨️.mjs";
import {ROSTER_RULES_V1} from "../../🟨️.mjs";

const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1(),schema=join(here,"../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🧬️schema/🔣️.json"),policyFile=join(here,"../../../../../⚡️caching/🔣️policy.json");
const ports=(root:string)=>({
 list:async(directory:string)=>{try{return(await readdir(join(root,directory),{withFileTypes:true})).map(entry=>({name:entry.name,kind:entry.isSymbolicLink()?"other" as const:entry.isDirectory()?"directory" as const:entry.isFile()?"file" as const:"other" as const}));}catch{return undefined;}},
 stat:async(path:string)=>{try{const status=await lstat(join(root,path));return{directory:status.isDirectory(),file:status.isFile(),mtimeMs:status.mtimeMs,ino:status.ino,mode:status.mode};}catch{return undefined;}},
 descend:(name:string,path:string,parent:string)=>ROSTER_RULES_V1.descends(name,path,parent),
 member:()=>true,
});
const put=(root:string,path:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),"x");};

test("every cache store directory of the unit cache is a declared generated directory, so the roster never enters it",()=>{
 expect(cachePolicy.generatedDirectories).toContain(UNIT_CACHE_POLICY_V1.seal.directory);
 for(const name of cachePolicy.generatedDirectories)expect(ROSTER_RULES_V1.descends(name,"units/"+name,"units")).toBe(false);
});

test("a walk finds no member below a sealed copy or a cache root and equals the Python walker; a vanished member is noticed once",async()=>{
 if(!python)return;
 const root=mkdtempSync(join(realpathSync(tmpdir()),"non-inputs-"));
 try{
  for(const path of["real/project.json","real/src/a.ts","units/sealed/0123456789abcdef/📦️packages/x.ts","units/sealed/pointer-1.json","units/bin/rustc-wrapper-1","launch-kit",".cache/b/x","node_modules/y/z.js"])put(root,path);
  const walk=await walkFrontierV1(ports(root)),paths=walk.members.map(row=>row[0]).sort();
  const expected=JSON.parse(Bun.spawnSync([python,"-I",join(here,"🐍️.py"),policyFile,schema,root]).stdout.toString());
  expect(expected.cacheDirectories).toEqual([]);
  expect(paths.filter(path=>path.startsWith("units/sealed")||path.startsWith("units/bin")||path.startsWith(".cache")||path.startsWith("node_modules"))).toEqual([]);
  expect(paths).toEqual(expected.members.filter((path:string)=>!path.startsWith("node_modules")&&!path.startsWith(".cache")));
  await new Promise(done=>setTimeout(done,2100));
  const captured={version:1,capturedAt:Date.now(),directories:walk.directories,members:walk.members};
  rmSync(join(root,"launch-kit"));
  const first=await advanceFrontierV1({frontier:captured,...ports(root)});
  expect(first&&!("unchanged"in first)).toBe(true);
  if(!first||"unchanged"in first)return;
  expect(diffFrontiersV1(captured,{...captured,directories:first.directories,members:first.members}).filter(change=>!change.directory)).toEqual([{path:"launch-kit",kind:"removed",directory:false}]);
  await new Promise(done=>setTimeout(done,2100));
  const settled={version:1,capturedAt:Date.now(),directories:first.directories,members:first.members};
  expect(await advanceFrontierV1({frontier:settled,...ports(root)})).toEqual({unchanged:true});
 }finally{rmSync(root,{recursive:true,force:true});}
});
