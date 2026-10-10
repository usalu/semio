import {expect,test,spyOn} from "bun:test";
import {mkdtempSync,mkdirSync,readFileSync,writeFileSync,renameSync,symlinkSync} from "node:fs";
import {dirname,join} from "node:path";
import glob from "fast-glob";
import TOML from "@iarna/toml";
import Ajv from "ajv";
import {cargoWorkspaceForManifest,cargoWorkspaceMembers,discoverCargoWorkspaces} from "../🟦️.ts";
import fixture from "../🧫️fixtures/🧭️native-discovery/🔣️.json";

test("Cargo production ownership never reads testing collections and preserves genuine module owners",()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Explicit artifact storage required");
 const root=mkdtempSync(join(output,"cargo-production-")),law=fixture.testingCollections;
 const put=(path:string,source:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),source);};
 const workspace=(owners:readonly string[],members:readonly string[])=>'[workspace]\nresolver="2"\nmembers='+JSON.stringify(members)+'\n[workspace.metadata.semio.repository]\nschema-version=1\nowner-manifests='+JSON.stringify(owners)+'\nmember-manifests='+JSON.stringify(members.map(member=>member+"/Cargo.toml"))+'\nexclude-patterns=[]\n';
 put("Cargo.toml",workspace(["ordinary/**/Cargo.toml"],["framework/**"]));
 for(const [index,row] of law.members.entries())put(row.manifest,'[package]\nname="production-'+index+'"\nversion="0.1.0"\nedition="2021"\n');
 for(const row of law.owners)put(row.manifest,workspace([],["pkg"])+'[workspace.metadata.testing]\nidentity='+JSON.stringify(row.manifest)+"\n");
 const expectedMembers=law.members.filter(row=>row.admitted).map(row=>row.manifest).sort(),expectedOwners=["Cargo.toml",...law.owners.filter(row=>row.admitted).map(row=>row.manifest)].sort();
 const physical=new Set(glob.sync("**/Cargo.toml",{cwd:root,onlyFiles:true,followSymbolicLinks:false}));
 for(const row of [...law.members,...law.owners])expect(physical.has(row.manifest)).toBe(true);
 const forbidden=new Set([...law.members,...law.owners].filter(row=>!row.admitted).map(row=>readFileSync(join(root,row.manifest),"utf8"))),original=Bun.TOML.parse,parser=spyOn(Bun.TOML,"parse").mockImplementation(source=>{if(typeof source==="string" && forbidden.has(source))throw Error("Denied fixture document was parsed");return original(source);});
 try{
  expect(cargoWorkspaceMembers(root,cargoWorkspaceForManifest(root,"Cargo.toml")).map(row=>row.manifest)).toEqual(expectedMembers);
  expect(discoverCargoWorkspaces(root).map(row=>row.manifest).sort()).toEqual(expectedOwners);
 }finally{parser.mockRestore();}
 for(const manifest of [...expectedMembers,...expectedOwners]){const source=readFileSync(join(root,manifest),"utf8");expect(Bun.TOML.parse(source)).toEqual(TOML.parse(source));}
 const isolated="isolated/🧪️tests/Cargo.toml";put(isolated,'[package]\nname="isolated-test"\nversion="0.1.0"\n[workspace]\nmembers=["."]\n');expect(cargoWorkspaceForManifest(root,isolated).manifest).toBe(isolated);expect(discoverCargoWorkspaces(root).map(row=>row.manifest)).not.toContain(isolated);
 console.error("[DEBUG] Original Cargo production admission agrees with physical fast-glob and independent TOML; four collections excluded before parse, immediate module names and explicit isolated tests preserved");
});

test("Cargo controlled physical discovery preserves original cancellation and reports admitted operations",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Explicit artifact storage required");
 const root=mkdtempSync(join(output,"cargo-controlled-"));mkdirSync(join(root,"framework/live"),{recursive:true});
 const source='[workspace]\nmembers=["framework/*"]\n[workspace.metadata.semio.repository]\nschema-version=1\nowner-manifests=[]\nmember-manifests=["framework/*/Cargo.toml"]\nexclude-patterns=[]\n';
 writeFileSync(join(root,"Cargo.toml"),source);writeFileSync(join(root,"framework/live/Cargo.toml"),'[package]\nname="controlled-live"\nversion="0.1.0"\n');
 const api=await import("../🟦️.ts");expect(typeof api.discoverCargoWorkspacesControlledV1).toBe("function");expect(typeof api.cargoWorkspaceMembersControlledV1).toBe("function");
 const controller=new AbortController(),seen:unknown[]=[];await expect(api.discoverCargoWorkspacesControlledV1(root,{signal:controller.signal,advance:async progress=>{seen.push(progress);if(seen.length===3)controller.abort("original cancellation");await new Promise<void>(resolve=>setImmediate(resolve));}})).rejects.toBe("original cancellation");expect(seen).toHaveLength(3);expect(readFileSync(join(root,"Cargo.toml"),"utf8")).toBe(source);
 const domain=JSON.parse(readFileSync(new URL("../🧬️schema/📇️discovery/🔣️.json",import.meta.url),"utf8")),validate=new Ajv({strict:true}).compile({$defs:domain.$defs,$ref:"#/$defs/CargoWorkspaceProgressV1"}),progress:import("../🟦️.ts").CargoWorkspaceProgressV1[]=[],signal=new AbortController().signal,control={signal,advance:async(row:import("../🟦️.ts").CargoWorkspaceProgressV1)=>{expect(validate(row)).toBe(true);progress.push(row);await new Promise<void>(resolve=>setImmediate(resolve));}};
 const owners=await api.discoverCargoWorkspacesControlledV1(root,control);expect(owners).toEqual(discoverCargoWorkspaces(root));expect(await api.cargoWorkspaceMembersControlledV1(root,owners[0]!,control)).toEqual(cargoWorkspaceMembers(root,owners[0]!));expect(progress.some(row=>row.operation==="list")).toBe(true);expect(progress.some(row=>row.operation==="text")).toBe(true);
 expect(glob.sync("framework/*/Cargo.toml",{cwd:root,onlyFiles:true})).toEqual(["framework/live/Cargo.toml"]);console.error("[DEBUG] Controlled Cargo original signal cancelled exactly after three physical operations; success matches synchronous ownership and independent glob/TOML");
});

test("Cargo controlled membership refuses changed directory ancestry before listing it",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Explicit artifact storage required");
 const root=mkdtempSync(join(output,"cargo-ancestry-")),foreign=mkdtempSync(join(output,"cargo-peer-"));mkdirSync(join(root,"framework/live"),{recursive:true});mkdirSync(join(foreign,"live"),{recursive:true});
 writeFileSync(join(root,"Cargo.toml"),'[workspace]\nmembers=["framework/*"]\n[workspace.metadata.semio.repository]\nschema-version=1\nowner-manifests=[]\nmember-manifests=["framework/*/Cargo.toml"]\nexclude-patterns=[]\n');writeFileSync(join(root,"framework/live/Cargo.toml"),'[package]\nname="original-live"\nversion="0.1.0"\n');writeFileSync(join(foreign,"live/Cargo.toml"),'[package]\nname="peer-live"\nversion="0.1.0"\n');
 const api=await import("../🟦️.ts"),owner=cargoWorkspaceForManifest(root,"Cargo.toml"),seen:string[]=[];
 await expect(api.cargoWorkspaceMembersControlledV1(root,owner,{signal:new AbortController().signal,advance:async progress=>{if(progress.operation==="list"){seen.push(progress.path);if(progress.path===root){renameSync(join(root,"framework"),join(root,"retained-framework"));symlinkSync(foreign,join(root,"framework"),process.platform==="win32"?"junction":"dir");}}}})).rejects.toThrow("symlink");expect(seen).toEqual([root]);expect(TOML.parse(readFileSync(join(root,"retained-framework/live/Cargo.toml"),"utf8"))).toEqual({package:{name:"original-live",version:"0.1.0"}});console.error("[DEBUG] Original controlled Cargo refused peer directory symlink before listing descendant; retained source bytes remain physical");
});
