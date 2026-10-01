import { existsSync, lstatSync, readFileSync, readdirSync } from "node:fs";
import { ownsPayload, NATIVE_DISCOVERY_OPERATIONS } from "../📦️payload/🟦️.ts";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
export type BunWorkspaceDeclarationV1 = Readonly<{ schemaVersion: 1; members: readonly string[]; owners: readonly string[] }>;
export type BunWorkspaceScope = Readonly<{ directory: string; manifest: string; lock: string; declaration: BunWorkspaceDeclarationV1 }>;
export type BunRepositoryMembership = Readonly<{ scopes: readonly BunWorkspaceScope[]; packages: readonly string[] }>;
const slash = (path: string): string => path.replaceAll("\\", "/");
const bounded = (root: string, path: string): string => { const full=resolve(root,path), local=relative(root,full);if((local===".." || local.startsWith("../") || local.startsWith("..\\")) || isAbsolute(local)) throw new Error(`Bun workspace escapes repository: ${path}`);return full; };
const regular = (root: string, path: string, verified = new Set<string>()): string => { const full=bounded(root,path);let current=root;for(const part of relative(root,full).split(/[\\/]/).filter(Boolean)){current=join(current,part);if(!verified.has(current)){if(lstatSync(current).isSymbolicLink())throw new Error(`Bun workspace symlink: ${path}`);verified.add(current);}}if(!lstatSync(full).isFile())throw new Error(`Bun workspace requires a regular manifest: ${path}`);return full; };
const parse = (root: string, path: string, verified?: Set<string>): any => JSON.parse(readFileSync(regular(root,path,verified),"utf8"));
/** 🧬️ Admits authored native Bun patterns independently of package identities or product roles. */
export function parseBunWorkspaceDeclaration(value: unknown): BunWorkspaceDeclarationV1 {
 if(!value || typeof value!=="object" || Array.isArray(value))throw new Error("Bun workspace declaration requires an object");const row=value as any;
 if(Object.keys(row).some(k=>!["schemaVersion","members","owners"].includes(k)) || row.schemaVersion!==1 || !Array.isArray(row.members) || !row.members.length || !Array.isArray(row.owners))throw new Error("Invalid Bun workspace declaration");
 for(const [key,values] of [["members",row.members],["owners",row.owners]] as const)if(values.some((v:any)=>typeof v!=="string" || (!v || v==="!") || /^!?\//.test(v) || /^!?[A-Za-z]:\//.test(v) || v.includes("\\") || key==="owners" && v!=="*/package.json") || new Set(values).size!==values.length)throw new Error("Invalid Bun workspace patterns");
 if(!row.members.some((p:string)=>!p.startsWith("!")))throw new Error("Bun workspace needs a positive source pattern");
 return {schemaVersion:1,members:[...row.members],owners:[...row.owners]};
}
const match = (pattern: string, path: string): boolean => {
 const parts=pattern.split("/"), source=parts.map((part,index)=>part==="**"?index===parts.length-1?".*":"(?:[^/]+/)*":part.split("").map(c=>c==="*"?"[^/]*":c==="?"?"[^/]":c.replace(/[.*+?^${}()|[\]\\]/g,"\\$&")).join("")+(index===parts.length-1?"":"/")).join("");return new RegExp("^"+source+"$").test(path);
};
const opaque=new Set(["🗑️generated","node_modules","target","dist","build","storybook-static","temp","coverage","🔌️plugin-modules","compose","🧫️fixtures"]);
/** 📁️ Expands source patterns through current physical files without reading cached package memberships. */
function bunWorkspaceCandidates(root: string, scope: BunWorkspaceScope, inventory = new Map<string, readonly string[]>()): readonly string[] {
 const positive=scope.declaration.members.filter(p=>!p.startsWith("!")), negative=scope.declaration.members.filter(p=>p.startsWith("!")).map(p=>p.slice(1)), found=new Set<string>();
 for(const pattern of positive){const prefix=pattern.split(/[*?]/)[0]!.replace(/\/$/,""), start=bounded(root,join(scope.directory,prefix));if(!existsSync(start))continue;if(lstatSync(start).isSymbolicLink())throw new Error(`Bun source directory is a symlink: ${start}`);
 const walk=(directory:string):readonly string[]=>{const known=inventory.get(directory);if(known)return known;const entries=readdirSync(directory,{withFileTypes:true}), paths:string[]=[];
 if(entries.some(entry=>entry.name==="package.json"))paths.push(directory);
 for(const entry of entries)if(entry.isDirectory() && !entry.name.startsWith(".") && !opaque.has(entry.name))paths.push(...walk(join(directory,entry.name)));inventory.set(directory,paths);return paths;};
 for(const directory of walk(start)){const local=slash(relative(resolve(root,scope.directory),directory));if(match(pattern,local) && !negative.some(p=>match(p,local+"/") || match(p,local))){found.add(slash(relative(root,directory)));}}
 }
 return [...found].sort((a,b)=>a.localeCompare(b));
}
/** 📦️ Resolves independent package roots using the same explicit physical export ownership contract. */
function workspacePackages(root: string, scope: BunWorkspaceScope, inventory: Map<string, readonly string[]>, documents = new Map<string, any>(), verified = new Set<string>()): readonly string[] {
 const document=(path:string):any=>{if(!documents.has(path))documents.set(path,parse(root,path,verified));return documents.get(path);};
 const paths=bunWorkspaceCandidates(root,scope,inventory), candidates=paths.map(relDir=>({relDir,absDir:resolve(root,relDir),...document(relDir+"/package.json")})), byDirectory=new Map(candidates.map(c=>[c.absDir,c]));
 return candidates.filter(candidate=>{let parent=dirname(candidate.absDir);while(parent!==root && parent!==dirname(parent)){const owner=byDirectory.get(parent);if(owner)return !ownsPayload(owner,candidate,NATIVE_DISCOVERY_OPERATIONS);parent=dirname(parent);}return true;}).map(row=>row.relDir);
}
/** 📦️ Reads independently owned packages from current physical source. */
export function bunWorkspacePackages(root: string, scope: BunWorkspaceScope): readonly string[] { return workspacePackages(root,scope,new Map()); }
/** 🧾️ Emits native wildcard membership and explicit exclusions for owner-bound exported payload packages. */
export function bunWorkspaceNativePatterns(root: string, scope: BunWorkspaceScope): readonly string[] {
 const inventory=new Map<string,readonly string[]>(),selected=new Set(workspacePackages(root,scope,inventory));return [...scope.declaration.members,...bunWorkspaceCandidates(root,scope,inventory).filter(path=>!selected.has(path)).map(path=>"!"+slash(relative(resolve(root,scope.directory),resolve(root,path))).replace(/[?*\[\]{}]/g,"\\$&"))];
}

/** 🗂️ Discovers present authored Bun workspaces while retaining independent selected installation scopes. */
export function bunRepositoryMembership(root: string): BunRepositoryMembership {
 const verified=new Set<string>(), document=parse(root,"package.json",verified), declaration=parseBunWorkspaceDeclaration(document.semio?.workspace), paths=new Set(["package.json"]);
 for(const pattern of declaration.owners){if(pattern!=="*/package.json")throw new Error(`Unsupported physical owner recipe: ${pattern}`);for(const entry of readdirSync(root,{withFileTypes:true}))if(entry.isDirectory() && !entry.name.startsWith(".") && existsSync(join(root,entry.name,"package.json")))paths.add(entry.name+"/package.json");}
 const scopes:BunWorkspaceScope[]=[],inventory=new Map<string,readonly string[]>(),documents=new Map<string,any>(),all=new Set<string>();for(const path of [...paths].sort()){const row=parse(root,path,verified);if(row.semio?.workspace===undefined)continue;const d=dirname(path), directory=d==="."?".":slash(d), admitted=parseBunWorkspaceDeclaration(row.semio.workspace);if(!Array.isArray(row.workspaces) || JSON.stringify(row.workspaces.slice(0,admitted.members.length))!==JSON.stringify(admitted.members) || row.workspaces.slice(admitted.members.length).some((p:unknown)=>typeof p!=="string" || !p.startsWith("!")))throw new Error(`Bun native source patterns drift: ${path}`);for(const p of admitted.members)bounded(root,join(directory,p.replace(/^!/,"").split(/[*?]/)[0]!));const scope={directory,manifest:path,lock:directory==="."?"bun.lock":directory+"/bun.lock",declaration:admitted};const packages=workspacePackages(root,scope,inventory,documents,verified);for(const exclusion of row.workspaces.slice(admitted.members.length))if(packages.some(p=>match(exclusion.slice(1),slash(relative(resolve(root,directory),resolve(root,p))))))throw new Error(`Bun native exclusion hides an independent owner: ${path} ${exclusion}`);packages.forEach(path=>all.add(path));scopes.push(scope);}return {scopes,packages:[...all].sort((a,b)=>a.localeCompare(b))};
}
/** 🗂️ Reads the current admitted native installation scopes. */
export function discoverBunWorkspaces(root: string): readonly BunWorkspaceScope[] { return bunRepositoryMembership(root).scopes; }
/** 📦️ Unions current source package documents across admitted scopes without fabricating absent memberships. */
export function bunRepositoryPackages(root: string): readonly string[] { return bunRepositoryMembership(root).packages; }
