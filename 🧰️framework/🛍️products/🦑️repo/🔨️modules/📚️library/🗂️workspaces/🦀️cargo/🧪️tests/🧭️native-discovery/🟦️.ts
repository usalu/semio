import {expect,test} from "bun:test";
import {mkdtempSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import glob from "fast-glob";
import TOML from "@iarna/toml";
import {cargoWorkspaceForManifest,discoverCargoWorkspaces,prepareCargoOwners} from "../../🟦️.ts";
import fixture from "../../🧫️fixtures/🧭️native-discovery/🔣️.json";

test("current root patterns discover every authored native owner before preparation",()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("SEMIO_TEST_ARTIFACT_DIR is required");mkdirSync(output,{recursive:true});
 const root=mkdtempSync(join(output,"native-owner-discovery-")),patterns=(TOML.parse(readFileSync(join(process.cwd(),"Cargo.toml"),"utf8")) as any).workspace.metadata.semio.repository["owner-manifests"] as string[];
 const put=(path:string,body:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),body);};
 const workspace=(owners:readonly string[])=>'[workspace]\nresolver="2"\nmembers=["pkg"]\n[workspace.metadata.semio.repository]\nschema-version=1\nowner-manifests='+JSON.stringify(owners)+'\nmember-manifests=["pkg/Cargo.toml"]\nexclude-patterns=[]\n';
 const packageSource=(name:string)=>'[package]\nname="'+name+'"\nversion="0.1.0"\nedition="2021"\nworkspace=".."\n[lib]\npath="🦀️.rs"\n';
 put("Cargo.toml",workspace(patterns));put("pkg/Cargo.toml",packageSource("native-root"));put("pkg/🦀️.rs","pub fn observed() {}\n");
 for(const [index,row]of fixture.owners.entries()){const directory=dirname(row.manifest);put(row.manifest,workspace([]));put(join(directory,"pkg/Cargo.toml"),packageSource("native-owner-"+index));put(join(directory,"pkg/🦀️.rs"),"pub fn observed() {}\n");}
 const expected=["Cargo.toml",...fixture.owners.filter(row=>row.admitted).map(row=>row.manifest)].sort();
 const independent=["Cargo.toml",...glob.sync(patterns,{cwd:root,onlyFiles:true,followSymbolicLinks:false,dot:false,ignore:["**/target/**","**/🗑️generated/**"]}).filter(path=>path!=="Cargo.toml"&&(TOML.parse(readFileSync(join(root,path),"utf8")) as any).workspace?.metadata?.semio?.repository)].sort();
 expect(independent).toEqual(expected);expect(discoverCargoWorkspaces(root).map(owner=>owner.manifest).sort()).toEqual(expected);
 for(const owner of expected){const selected=cargoWorkspaceForManifest(root,join(dirname(owner),"pkg/Cargo.toml")),prepared=prepareCargoOwners(root,selected);expect(prepared.owners).toEqual([dirname(owner)]);
  const native=Bun.spawnSync(["cargo","metadata","--offline","--no-deps","--format-version","1","--manifest-path",join(root,owner)],{cwd:root,stdout:"pipe",stderr:"pipe"});if(native.exitCode!==0)throw Error(native.stderr.toString());const metadata=JSON.parse(native.stdout.toString());expect(metadata.workspace_members).toHaveLength(1);expect(metadata.packages[0].manifest_path.replaceAll("\\","/")).toBe(join(root,dirname(owner),"pkg/Cargo.toml").replaceAll("\\","/"));
 }
 console.log("[DEBUG] Root native owner discovery agrees with independent glob, TOML and Cargo preparation");
});
