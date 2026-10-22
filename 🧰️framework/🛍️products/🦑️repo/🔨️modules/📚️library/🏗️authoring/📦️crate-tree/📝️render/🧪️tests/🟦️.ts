/** 📝️ Contract: the crate render module reads dependency rows of a Cargo manifest as authored text, rebases relative paths, renders the manifest of a carved crate in byte order, finds the exclusion pattern that hides a workspace member, widens workspace members and the lockfile, extends the package router and rebases `include!`/fixture paths of moved Rust text; the corpus is language-agnostic, `@iarna/toml` is the independent third-party oracle for every manifest and lockfile and minimatch for the hiding patterns. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {parse} from "@iarna/toml";
import {minimatch} from "minimatch";
import {resolve} from "node:path";
import {addWorkspaceMembers,compareBytes,hidingExcludePattern,manifestRowKey,patchCargoLock,patchRouterPackages,readManifestDependencyRows,rebaseManifestDirPaths,rebaseManifestRow,rebaseRustIncludes,relativePosix,renderCrateManifest,workspaceAdmissionProblems,type ManifestSpec} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const platform=(path:string):string=>process.platform==="win32"?"C:"+path.split("/").join("\\"):path;
const posix=(path:string):string=>path.split("\\").join("/");
const pathsIn=(text:string):string=>text.replace(/\/w\//gu,platform("/w/")).replace(/\/w(?![\w/])/gu,platform("/w"));
for(const row of corpus.relative)test("relative path "+row.from+" -> "+row.to,()=>{expect(relativePosix(resolve(platform(row.from)),resolve(platform(row.to)))).toBe(row.expected);});
for(const row of corpus.compare)test("byte order of "+JSON.stringify(row.left)+" and "+JSON.stringify(row.right),()=>{expect(Math.sign(compareBytes(row.left,row.right))).toBe(row.sign);});
test("the manifest dependency rows, target tables and unsupported sections are read from the authored text",()=>{
 const rows=readManifestDependencyRows(corpus.manifest.text);expect([...rows.dependencies]).toEqual(corpus.manifest.dependencies);expect([...rows.devDependencies]).toEqual(corpus.manifest.devDependencies);expect(rows.targetSections.map(section=>section.trim()+"\n")).toEqual(corpus.manifest.targetSections);expect([...rows.unsupported]).toEqual(corpus.manifest.unsupported);
 const document=parse(corpus.manifest.text) as any;expect(Object.keys(document.dependencies).sort()).toEqual(rows.dependencies.map(manifestRowKey).sort());expect(Object.keys(document["dev-dependencies"])).toEqual(rows.devDependencies.map(manifestRowKey));expect(Object.keys(document.target).length).toBe(rows.targetSections.length);
});
test.failing("a table-form dependency is flagged unsupported instead of silently dropped",()=>{expect(readManifestDependencyRows(corpus.manifestDefect.text).unsupported).toContain(corpus.manifestDefect.expectedUnsupportedContains);});
for(const row of corpus.keys)test("manifest row key of "+JSON.stringify(row.row),()=>{expect(manifestRowKey(row.row)).toBe(row.expected);});
for(const row of corpus.rebase)test("rebase row "+JSON.stringify(row.row),()=>{expect(posix(rebaseManifestRow(row.row,resolve(platform(row.from)),resolve(platform(row.to))))).toBe(row.expected);});
for(const row of corpus.render)test("crate manifest: "+row.name,()=>{
 const text=renderCrateManifest(row.spec as ManifestSpec);expect(text).toBe(row.expected);const document=parse(text) as any;
 expect(document.package.name).toBe(row.spec.name);expect(document.package.description).toBe(row.spec.description);expect(document.lib.path).toBe(row.spec.libPath);expect(document.package["version"]).toEqual({workspace:true});expect(Object.keys(document.dependencies)).toEqual(Object.keys(document.dependencies).sort((a,b)=>compareBytes(a,b)));
});
for(const row of corpus.hiding)test("hiding exclude pattern of "+row.member+" under "+JSON.stringify(row.patterns),()=>{expect(hidingExcludePattern(row.member,row.patterns)).toBe(row.expected??undefined);});
test("minimatch agrees that a hiding pattern matches the member directory or an ancestor",()=>{
 for(const row of corpus.hiding){const segments=row.member.split("/"),hit=row.patterns.find(pattern=>segments.some((_,index)=>{const directory=segments.slice(0,index+1).join("/");return minimatch(directory,pattern)||minimatch(directory+"/",pattern)||(pattern.endsWith("/**")&&minimatch(directory,pattern.slice(0,-3)));}));expect(hit,row.member).toBe(row.expected??undefined);}
});
for(const row of corpus.admission)test("workspace admission: "+row.name,()=>{expect(workspaceAdmissionProblems(row.manifest)).toEqual(row.problems);});
for(const row of corpus.addMembers)test("add workspace members: "+row.name,()=>{const text=addWorkspaceMembers(row.manifest,row.additions);expect(text).toBe(row.expected);const workspace=(parse(text) as any).workspace;expect(workspace.members).toEqual([...workspace.members].sort(compareBytes));for(const member of row.additions)expect(workspace.members).toContain(member);});
test("a manifest without a members array is refused",()=>{expect(()=>addWorkspaceMembers('[package]\nname = "x"\n',["a"])).toThrow("Workspace manifest has no members array");});
test("the lockfile gains the carved crates, drops stale members and keeps every other block",()=>{
 const lock=corpus.lock,text=patchCargoLock(lock.text,lock.artifact,lock.carved,new Set(lock.members));expect(text).toBe(lock.expected);
 const before=parse(lock.text) as any,after=parse(text) as any,names=(document:any):string[]=>document.package.map((row:any)=>row.name);
 expect(names(after)).toEqual(["art","art-new","other","serde"]);expect(after.package.find((row:any)=>row.name==="other")).toEqual({name:"other",version:"1.0.0",source:before.package.find((row:any)=>row.name==="other").source});expect(after.package.find((row:any)=>row.name==="serde")).toEqual(before.package.find((row:any)=>row.name==="serde"));expect(after.package.find((row:any)=>row.name==="art").dependencies).toEqual(["art-new","serde","zzz"]);
 expect(()=>patchCargoLock(lock.text,"missing",[],undefined)).toThrow("Lockfile has no package missing");
});
for(const row of corpus.router)test("router packages: "+row.name,()=>{expect(patchRouterPackages(row.script,row.artifact,row.additions)).toBe(row.expected);});
test("a router call that is not found is refused",()=>{expect(()=>patchRouterPackages("nothing here","art",["x"])).toThrow("Router call for art not found");});
for(const[index,row]of corpus.includes.entries())test("rust includes "+index,()=>{expect(rebaseRustIncludes(row.text,resolve(platform(row.from)),resolve(platform(row.to)))).toBe(row.expected);});
for(const[index,row]of corpus.manifestPaths.entries())test("manifest directory paths "+index,()=>{expect(rebaseManifestDirPaths(row.text,resolve(platform(row.from)),resolve(platform(row.to)))).toBe(row.expected);});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
