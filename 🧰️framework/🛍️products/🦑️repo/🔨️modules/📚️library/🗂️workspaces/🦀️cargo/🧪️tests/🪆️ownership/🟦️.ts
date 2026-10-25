import {expect,test} from "bun:test";
import Ajv from "ajv";
import TOML from "@iarna/toml";
import glob from "fast-glob";
import {Database} from "bun:sqlite";
import {mkdirSync,mkdtempSync,readFileSync,writeFileSync,rmSync,existsSync} from "node:fs";
import {dirname,join,relative,resolve} from "node:path";
import {discoverCargoWorkspaces,cargoWorkspaceForManifest,cargoWorkspaceMembers,publishCargoWorkspaceMembership,publishSelectedCargoWorkspaceMembershipV1} from "../../🟦️.ts";
import corpus from "../🧫️fixtures/🪆️ownership/🔣️.json";
import schema from "../../🧬️schema/🪆️ownership/🔣️.json";

/** 🪆️ Keeps ancestor-owned packages outside a nested owner's automatic dependency membership. */
test("Cargo membership publication excludes physical foreign packages without changing their owners",()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original ticket artifacts required");mkdirSync(artifacts,{recursive:true});const database=new Database(":memory:");
 const put=(root:string,path:string,text:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);},workspace=(members:string[],admission:string[],owners:string[]=[])=>'[workspace]\nresolver="2"\nmembers='+JSON.stringify(members)+'\n[workspace.metadata.semio.repository]\nschema-version=1\nmember-manifests='+JSON.stringify(admission)+'\nowner-manifests='+JSON.stringify(owners)+'\nexclude-patterns=[]\n',pkg=(name:string,owner:string)=>'[package]\nname="'+name+'"\nversion="0.1.0"\nedition="2021"\nworkspace="'+owner+'"\n[lib]\npath="lib.rs"\n';
 try{for(const row of corpus.cases){const root=mkdtempSync(join(artifacts,"foreign-owner-"));try{
  put(root,"Cargo.toml",workspace(["framework/product"],["framework/*/Cargo.toml"],["framework/Cargo.toml","framework/nested/Cargo.toml"]));put(root,"framework/Cargo.toml",workspace(["general","products/plugin"],["*/Cargo.toml","products/plugin/Cargo.toml"])+(row.retained.length?'\n[unused]\n':""));if(row.retained.length){const path=join(root,"framework/Cargo.toml");writeFileSync(path,readFileSync(path,"utf8").replace('[workspace]\n','[workspace]\nexclude='+JSON.stringify(row.retained)+'\n'));}
  put(root,"framework/product/Cargo.toml",pkg("root-product","../.."));put(root,"framework/product/lib.rs","");put(root,"framework/general/Cargo.toml",pkg("nested-general","..")+'[dependencies]\nroot-product={path="../product"}\n');put(root,"framework/general/lib.rs","");put(root,"framework/nested/Cargo.toml",workspace(["kernel"],["kernel/Cargo.toml"]));put(root,"framework/nested/kernel/Cargo.toml",pkg("inner-kernel",".."));put(root,"framework/nested/kernel/lib.rs","");
  put(root,"framework/products/plugin/Cargo.toml",pkg("nested-plugin","../.."));put(root,"framework/products/plugin/lib.rs","");
  const packagePaths=["framework/products/plugin/Cargo.toml","framework/product/Cargo.toml","framework/general/Cargo.toml","framework/nested/kernel/Cargo.toml"],before=packagePaths.map(path=>readFileSync(join(root,path),"utf8")),owners=discoverCargoWorkspaces(root),selected=cargoWorkspaceForManifest(root,"framework/Cargo.toml"),known=row.selectedOnly?owners.filter(owner=>owner.directory.startsWith("framework")):owners;
  if(row.selectedOnly)expect(publishSelectedCargoWorkspaceMembershipV1(root,selected,"write",known)).toBe(true);else expect(publishCargoWorkspaceMembership(root,selected,"write")).toBe(true);
  const source=readFileSync(join(root,"framework/Cargo.toml"),"utf8"),document=TOML.parse(source) as {workspace:{members:string[];exclude:string[]}};expect(Bun.TOML.parse(source)).toEqual(document);expect(document.workspace.members).toEqual(row.expectedMembers);const memberOracle=database.query("select json_group_array(value) value from (select 'general' value union select 'products/plugin' order by value)").get()as{value:string};expect(document.workspace.members).toEqual(JSON.parse(memberOracle.value));expect(document.workspace.exclude).toEqual(row.expected);expect(packagePaths.map(path=>readFileSync(join(root,path),"utf8"))).toEqual(before);
  const oracle=database.query("select json_group_array(value) value from (select 'nested' value union select substr(?1,length('framework/')+1) where ?2<>?3 and ?1 like 'framework/%' union select value from json_each(?4) order by value)").get("framework/product",".","framework",JSON.stringify(row.selectedOnly?row.retained:[]))as{value:string};expect(document.workspace.exclude).toEqual(JSON.parse(oracle.value));
  for(const path of packagePaths){const data=TOML.parse(readFileSync(join(root,path),"utf8"))as{package:{workspace:string}},expected=relative(root,resolve(dirname(join(root,path)),data.package.workspace)).replaceAll("\\","/")||".";expect(cargoWorkspaceForManifest(root,path).directory).toBe(expected);}expect(cargoWorkspaceMembers(root,selected).map(row=>row.name)).toEqual(["nested-general","nested-plugin"]);
  expect(row.selectedOnly?publishSelectedCargoWorkspaceMembershipV1(root,selected,"check",known):publishCargoWorkspaceMembership(root,selected,"check")).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}}}finally{database.close();}
});

/** 🧾️ Verifies that every authored framework member is admitted by the same owner's Source inputs. */
test("current framework member manifest inputs preserve every explicit owned package",()=>{
 let root=import.meta.dir;while(!existsSync(join(root,"🧰️framework/Cargo.toml"))){const parent=dirname(root);if(parent===root)throw Error("Repository owner required");root=parent;}
 const framework=join(root,"🧰️framework"),source=readFileSync(join(framework,"Cargo.toml"),"utf8"),document=TOML.parse(source)as{workspace:{members:string[];metadata:{semio:{repository:{"member-manifests":string[];"exclude-patterns":string[]}}}}};expect(Bun.TOML.parse(source)).toEqual(document);
 const admission=document.workspace.metadata.semio.repository,independent=new Set(glob.sync(admission["member-manifests"],{cwd:framework,onlyFiles:true,followSymbolicLinks:false,ignore:[...admission["exclude-patterns"],...["node_modules","target","dist","build","🗑️generated"].map(name=>"**/"+name+"/**")]})),scope=cargoWorkspaceForManifest(root,"🧰️framework/Cargo.toml"),actual=cargoWorkspaceMembers(root,scope).map(row=>relative(framework,join(root,row.manifest)).replaceAll("\\","/")).sort(),authored=document.workspace.members.map(member=>member+"/Cargo.toml").sort();expect(actual).toEqual(authored);expect(authored.filter(path=>!independent.has(path))).toEqual([]);
 for(const path of authored){const data=TOML.parse(readFileSync(join(framework,path),"utf8"))as{package:{workspace:string}};expect(resolve(dirname(join(framework,path)),data.package.workspace)).toBe(framework);}
});
