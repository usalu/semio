import {test,expect} from "bun:test";
import {existsSync,readFileSync} from "node:fs";
import {resolve} from "node:path";
import {parse} from "@iarna/toml";
import fixture from "./🧫️fixtures/🔣️.json";

const framework=resolve(import.meta.dir,"../.."),manifest=readFileSync(resolve(framework,"📦️packages/🦀️rust/Cargo.toml"),"utf8");
test("the General entrypoint owns only General package dependencies in every channel",()=>{
 const primary=Bun.TOML.parse(manifest),oracle=parse(manifest);expect(primary).toEqual(oracle);
 for(const parser of [primary,oracle])for(const section of ["dependencies","dev-dependencies","build-dependencies"]){const dependencies=parser[section] as Record<string,unknown>|undefined;for(const name of Object.keys(dependencies??{}))expect(fixture.absentPackagePrefixes.some(prefix=>name.startsWith(prefix))).toBe(false);}
 for(const provider of fixture.neutralProviders)expect(manifest.includes(provider)).toBe(true);
});
test("the General entrypoint exports no specific registry or compatibility thunk",()=>{
 const source=readFileSync(resolve(framework,"📦️packages/🦀️rust/🦀️.rs"),"utf8").replace(/\/\/[^\n]*/g,"");
 expect(source.includes("semio_framework_os_kernel")).toBe(false);
 for(const name of fixture.specificExports)expect(new RegExp("\\b"+name+"\\b").test(source)).toBe(false);
});
test("the General native workspace owns its complete membership and provider closure",()=>{
 const path=resolve(framework,fixture.workspace.manifest);expect(existsSync(path)).toBe(true);
 const source=readFileSync(path,"utf8"),primary=Bun.TOML.parse(source),oracle=parse(source);expect(primary).toEqual(oracle);
 const workspace=primary.workspace as {members:string[];dependencies:Record<string,{path?:string}>};expect(workspace.members).toContain(fixture.workspace.entrypoint);
 for(const member of workspace.members){expect(fixture.workspace.absentRoots.some(root=>member.startsWith(root))).toBe(false);const owner=resolve(framework,member),body=readFileSync(resolve(owner,"Cargo.toml"),"utf8"),definition=Bun.TOML.parse(body);expect(definition).toEqual(parse(body));const manifest=definition.package as {workspace?:string};expect(resolve(owner,manifest.workspace??".")).toBe(framework);}
 for(const dependency of Object.values(workspace.dependencies)){if(dependency.path===undefined)continue;expect(dependency.path.startsWith("../")).toBe(false);expect(fixture.workspace.absentRoots.some(root=>dependency.path!.startsWith(root))).toBe(false);expect(existsSync(resolve(framework,dependency.path,"Cargo.toml"))).toBe(true);}
});
