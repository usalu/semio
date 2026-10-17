import {test,expect} from "bun:test";
import Ajv from "ajv";
import TOML from "@iarna/toml";
import {mkdtempSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
const artifactRoot=process.env.SEMIO_TEST_ARTIFACT_DIR;
if(!artifactRoot)throw Error("Original bounded test artifact directory required");
const put=(root:string,path:string,source:string):void=>{mkdirSync(join(root,path,".."),{recursive:true});writeFileSync(join(root,path),source);};
const workspace=(members:string[],dependencies:string):string=>`[workspace]\nresolver="2"\nmembers=${JSON.stringify(members)}\n[workspace.package]\nedition="2021"\n${dependencies}`;
const pkg=(name:string,deps=""):string=>`[package]\nname="${name}"\nversion="0.1.0"\nedition.workspace=true\n[lib]\npath="🦀️.rs"\n${deps}`;


test("current physical workspace dependency identities match independent Cargo and TOML", () => {
 const corpus=JSON.parse(readFileSync(new URL("../🧫️fixtures/📦️identity/🔣️.json",import.meta.url),"utf8"));
 const validator=new Ajv({strict:true}).compile(JSON.parse(readFileSync(new URL("../../🧬️schema/📦️identity/🔣️.json",import.meta.url),"utf8")));
 expect(validator(corpus)).toBe(true);
 for(const row of corpus.cases){
  const root=mkdtempSync(join(artifactRoot!,"cargo-package-identity-"));
  put(root,"Cargo.toml",workspace(["provider","consumer"],`[workspace.dependencies]\n${row.dependency}={path="provider"}\n`));
  put(root,"provider/Cargo.toml",pkg(row.package));put(root,"provider/🦀️.rs","");
  put(root,"consumer/Cargo.toml",pkg("consumer",`[dependencies]\n${row.dependency}.workspace=true\n`));put(root,"consumer/🦀️.rs","");
  const native=Bun.spawnSync(["cargo","generate-lockfile","--offline","--manifest-path",join(root,"Cargo.toml")],{cwd:root,stdout:"pipe",stderr:"pipe"});expect(native.exitCode===0,row.id+": "+native.stderr.toString()).toBe(row.valid);
 }
 const directory=join(process.cwd(),"🧰️framework"),source=readFileSync(join(directory,"Cargo.toml"),"utf8"),manifest=Bun.TOML.parse(source)as any;
 expect(manifest).toEqual(TOML.parse(source));let checked=0;
 for(const[key,row]of Object.entries(manifest.workspace.dependencies)as[string,any][]){
  if(!row||typeof row!=="object"||typeof row.path!=="string")continue;
  const source=readFileSync(join(directory,row.path,"Cargo.toml"),"utf8"),physical=Bun.TOML.parse(source)as any;
  expect(physical).toEqual(TOML.parse(source));expect(physical.package.name,key).toBe(row.package??key);checked++;
 }
 expect(checked).toBeGreaterThan(0);
});
