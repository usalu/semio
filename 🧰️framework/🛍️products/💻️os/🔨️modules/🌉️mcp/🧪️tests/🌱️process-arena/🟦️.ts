import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";import{resolve,dirname,relative}from"node:path";
import Ajv from"ajv";import{parse as oracleParse}from"smol-toml";
import{cargoWorkspaceDeclaresMemberV1,cargoWorkspaceForManifest,cargoDeclaredPackageSelectionsV1}from"../../../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";
import law from"./input.json";import schema from"./input.schema.json";
test("the real ProcessArena dependency belongs to its authored repository workspace",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const root=process.cwd(),manifest=resolve(root,law.manifest),pkg=oracleParse(readFileSync(manifest,"utf8"))as any;
 expect(pkg.package.name).toBe(law.package);expect(pkg.package.workspace).toBe(law.workspaceRelative);expect(pkg.package.metadata.semio.role).toBe(law.role);
 expect(relative(root,resolve(dirname(manifest),pkg.package.workspace)).replaceAll("\\","/")||".").toBe(law.owner);
 const rootSource=readFileSync(resolve(root,"Cargo.toml"),"utf8"),rootOracle=oracleParse(rootSource)as any;
 expect(rootOracle.workspace.members.filter((member:string)=>member===law.rootMember).length).toBe(1);
 expect(cargoWorkspaceDeclaresMemberV1(rootSource,law.rootMember)).toBe(true);
 const nestedSource=readFileSync(resolve(root,"🧰️framework/Cargo.toml"),"utf8"),nestedOracle=oracleParse(nestedSource)as any;
 expect(nestedOracle.workspace.exclude.filter((member:string)=>member===law.frameworkExclude).length).toBe(1);
 expect(cargoWorkspaceDeclaresMemberV1(nestedSource,law.frameworkExclude)).toBe(false);
 const owner=cargoWorkspaceForManifest(root,law.manifest);expect(owner.directory).toBe(law.owner);
 const selected=cargoDeclaredPackageSelectionsV1(root,law.manifest,[law.package]);expect(selected?.map(row=>row.manifest)).toEqual([law.manifest]);
});

import membership from"./members.json";import membershipSchema from"./members.schema.json";
test("all new MCP host owners have exact source discovery and distinct package roles",()=>{
 expect(new Ajv({strict:true}).compile(membershipSchema)(membership)).toBe(true);
 const root=process.cwd();
 const rootSource=readFileSync(resolve(root,"Cargo.toml"),"utf8"),rootOracle=oracleParse(rootSource)as any;
 for(const row of membership.rows){
  const pkg=oracleParse(readFileSync(resolve(root,row.manifest),"utf8"))as any;
  expect(pkg.package.name).toBe(row.name);expect(pkg.package.metadata.semio.role).toBe(row.role);
  const owner=cargoWorkspaceForManifest(root,row.manifest);expect(owner.manifest).toBe(row.owner);
  if(row.explicitContribution){
   const source=oracleParse(readFileSync(resolve(root,row.owner),"utf8"))as any;
   const member=relative(resolve(root,owner.directory),resolve(root,dirname(row.manifest))).replaceAll("\\","/");
   expect(source.workspace.members.includes(member)).toBe(true);
   expect(source.workspace.metadata.semio.repository["member-manifests"].includes(member+"/Cargo.toml")).toBe(true);
   expect(rootOracle.workspace.exclude.includes(row.manifest.replace(/\/Cargo.toml$/,""))).toBe(true);
  }
  const selected=cargoDeclaredPackageSelectionsV1(root,row.manifest,[row.name]);expect(selected?.map(pkg=>pkg.manifest)).toEqual([row.manifest]);
 }
},15000);
