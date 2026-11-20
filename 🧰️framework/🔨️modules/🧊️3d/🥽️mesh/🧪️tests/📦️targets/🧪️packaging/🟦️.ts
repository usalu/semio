/** 🧪️ Packaging: 2 tests covering mesh packaging schema fixes actual owners, direct codec imports and independent manifest parity; all live mesh locks contain exact handcrafted production edges and workspace development owners. */
import{test,expect}from"bun:test";
import{Glob,TOML}from"bun";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
import{resolve,dirname}from"node:path";
import{parse}from"smol-toml";
import Ajv from"ajv";
import schema from"./🔣️.json";
import corpus from"./🧪️.json";
const root=resolve(import.meta.dir,"../../../../../../.."),domain=resolve(root,"🧰️framework/🔨️modules/🧊️3d"),workspace=resolve(root,"🧰️framework/Cargo.toml");
const read=(path:string)=>readFileSync(path,"utf8");
function table(value:unknown):Record<string,unknown>{if(!value||typeof value!=="object"||Array.isArray(value))throw Error("TOML table required");return Object.fromEntries(Object.entries(value));}
function strings(value:unknown):string[]{if(!Array.isArray(value)||!value.every(item=>typeof item==="string"))throw Error("String array required");return value;}
function parsed(path:string){const source=read(path),actual=TOML.parse(source);expect(actual).toEqual(parse(source));return table(actual);}
function sourceClosure(path:string,seen=new Set<string>()){if(seen.has(path))return seen;seen.add(path);for(const match of read(path).matchAll(/#\[path\s*=\s*"([^"]+)"\]/g)){if(!match[1])throw Error("Literal mount required");sourceClosure(resolve(dirname(path),match[1]),seen);}return seen;}
test("mesh packaging schema fixes actual owners, direct codec imports and independent manifest parity",()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const ws=parsed(workspace),members=strings(table(ws.workspace).members),shared=table(table(ws.workspace).dependencies);
 for(const owner of corpus.packages){const directory=resolve(domain,owner.directory),manifest=parsed(resolve(directory,"Cargo.toml")),pkg=table(manifest.package);expect(pkg.name).toBe(owner.name);expect(resolve(directory,String(pkg.workspace),"Cargo.toml")).toBe(workspace);expect(members.filter(member=>resolve(dirname(workspace),member)===directory)).toHaveLength(1);
 for(const[kind,expected]of[["dependencies",owner.production],["dev-dependencies",owner.development]]as const){const dependencies=manifest[kind]===undefined?{}:table(manifest[kind]);expect(Object.keys(dependencies).sort()).toEqual([...expected].sort());for(const[name,value]of Object.entries(dependencies)){const edge=typeof value==="string"?{}:table(value),inherited=edge.workspace===true?shared[name]:value,definition=typeof inherited==="string"?{}:table(inherited);if(typeof definition.path==="string"){const base=edge.workspace===true?dirname(workspace):directory;expect(table(parsed(resolve(base,definition.path,"Cargo.toml")).package).name).toBe(name);}}}
 const lib=table(manifest.lib),entry=resolve(directory,String(lib.path)),files=sourceClosure(entry);let bytes=0,lines=0;for(const file of files){const source=read(file);bytes+=Buffer.byteLength(source);lines+=source.split("\n").filter(line=>line.trim()).length;if(source.includes("value_derive::")){expect(owner.production).toContain("semio-framework-value-derive");expect(read(entry)).toContain("extern crate semio_framework_value_derive as value_derive;");}if(source.includes('::semio_framework_value'))expect(owner.production).toContain("semio-framework-value");}
 expect(bytes).toBeLessThanOrEqual(262144);expect(files.size).toBeLessThanOrEqual(32);expect(lines).toBeLessThanOrEqual(4000);if(owner.name!=="semio-framework-3d"){const bound=table(table(table(pkg.metadata).semio)["rust-source-boundary"]);expect(bound).toEqual({maxSourceBytes:262144,maxSourceFiles:32,maxActiveLines:4000});}
 const allFiles=new Set(files);if(manifest.test!==undefined){if(!Array.isArray(manifest.test))throw Error("Explicit test targets required");for(const target of manifest.test){const testPath=table(target).path;if(typeof testPath!=="string")throw Error("Test Source path required");for(const file of sourceClosure(resolve(directory,testPath)))allFiles.add(file);}}
 const declared=[...owner.production,...owner.development];for(const file of allFiles){const source=read(file).replace(/^\s*\/\/.*$/gm,"");for(const match of source.matchAll(/\b(semio_framework_[a-z0-9_]+)::|\b(?:use|extern crate) (semio_framework_[a-z0-9_]+)\b/g)){const name=match[1]??match[2];if(!name)throw Error("Literal dependency name required");expect(declared).toContain(name.replaceAll("_","-"));}for(const match of source.matchAll(/\b(serde_json|serde|parry3d)::/g)){if(!match[1])throw Error("Literal oracle required");expect(declared).toContain(match[1]);}}
 }
 expect(read(resolve(domain,"🌀️rigid/🦀️.rs"))).not.toContain("same crate, different file");
});
function lockedMeshOwners(manifest:string,readManifest:(path:string)=>Record<string,unknown>):Set<string>{
 const workspaceManifest=readManifest(manifest),directory=dirname(manifest);
 const workspace=workspaceManifest.workspace===undefined?{}:table(workspaceManifest.workspace);
 const members=new Set(workspace.members===undefined?[]:strings(workspace.members).flatMap(member=>
  member.includes("*")?[...new Glob(member+"/Cargo.toml").scanSync(directory)].map(path=>resolve(directory,path)):[resolve(directory,member,"Cargo.toml")]));
 if(workspaceManifest.package!==undefined)members.add(manifest);
 const owners=new Set<string>(),visited=new Set<string>();
 const visit=(path:string)=>{
  if(visited.has(path))return;visited.add(path);
  const manifest=readManifest(path),pkg=table(manifest.package),directory=dirname(path);
  if(typeof pkg.name!=="string")throw Error("Cargo package name required");
  owners.add(pkg.name);
  const workspacePath=typeof pkg.workspace==="string"?resolve(directory,pkg.workspace,"Cargo.toml"):path;
  const owner=readManifest(workspacePath).workspace,shared=owner===undefined?{}:table(owner).dependencies;
  const dependencies=shared===undefined?{}:table(shared);
  const scopes=[manifest,...Object.values(manifest.target===undefined?{}:table(manifest.target)).map(table)];
  const kinds=members.has(path)?["dependencies","build-dependencies","dev-dependencies"]:["dependencies","build-dependencies"];
  for(const scope of scopes)for(const kind of kinds)for(const [name,value] of Object.entries(scope[kind]===undefined?{}:table(scope[kind]))){
   if(typeof value==="string")continue;
   const edge=table(value),definition=edge.workspace===true?dependencies[name]:edge;
   if(definition===undefined)throw Error("Missing workspace dependency "+name+" in "+workspacePath);
   if(typeof definition==="string")continue;
   const {path:dependency}=table(definition);
   if(typeof dependency==="string")visit(resolve(edge.workspace===true?dirname(workspacePath):directory,dependency,"Cargo.toml"));
  }
 };
 for(const member of members)visit(member);
 return owners;
}

test("every workspace locks its reachable mesh owners and exact dependency edges",()=>{
 const db=new Database(":memory:");db.exec("create table edges(owner text,dependency text);create table expected(owner text,dependency text)");
 const manifests=new Map<string,Record<string,unknown>>();
 const manifest=(path:string)=>{const cached=manifests.get(path);if(cached)return cached;const value=table(TOML.parse(read(path)));manifests.set(path,value);return value;};
 try{for(const path of corpus.locks){
 const lock=parsed(resolve(root,path));if(!Array.isArray(lock.package))throw Error("Lock packages required");
 const packages=lock.package.map(table),framework=path==="🧰️framework/Cargo.lock";
 const reachable=lockedMeshOwners(resolve(root,dirname(path),"Cargo.toml"),manifest);
 const selected=corpus.packages.filter(owner=>reachable.has(owner.name)),names=new Set(corpus.packages.map(owner=>owner.name));
 expect(packages.filter(owner=>typeof owner.name==="string"&&names.has(owner.name)).map(owner=>owner.name).sort()).toEqual(selected.map(owner=>owner.name).sort());

 db.exec("delete from edges;delete from expected");for(const owner of selected){const entries=packages.filter(pkg=>pkg.name===owner.name);expect(entries).toHaveLength(1);const entry=entries[0];if(!entry)throw Error("Actual locked owner required");expect(entry.version).toBe("0.1.0");expect(entry.source).toBeUndefined();expect(entry.checksum).toBeUndefined();const dependencies=entry.dependencies===undefined?[]:strings(entry.dependencies),expected=[...owner.production,...framework?owner.development:[]].sort();expect([...dependencies].sort()).toEqual(expected);for(const dependency of dependencies){expect(packages.some(pkg=>dependency===pkg.name||dependency===pkg.name+" "+pkg.version)).toBe(true);db.query("insert into edges values (?,?)").run(owner.name,dependency);}for(const dependency of expected)db.query("insert into expected values (?,?)").run(owner.name,dependency);}
 expect(db.query("select * from edges except select * from expected").all()).toEqual([]);expect(db.query("select * from expected except select * from edges").all()).toEqual([]);
 const production=corpus.packages.flatMap(owner=>owner.production.map(dependency=>({owner:owner.name,dependency})));const visit=(name:string,active=new Set<string>()):boolean=>{if(active.has(name))return true;const next=new Set(active).add(name);return production.filter(edge=>edge.owner===name&&names.has(edge.dependency)).some(edge=>visit(edge.dependency,next));};expect(corpus.packages.some(owner=>visit(owner.name))).toBe(false);
 db.exec("delete from edges");for(const edge of production)db.query("insert into edges values (?,?)").run(edge.owner,edge.dependency);expect(db.query("with recursive reach(a,b) as (select owner,dependency from edges union select reach.a,edges.dependency from reach join edges on reach.b=edges.owner) select a from reach where a=b").all()).toEqual([]);
 }}finally{db.close();}
}, 30_000);
