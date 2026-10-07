import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { stringify, parse } from "@iarna/toml";
import { getWorkspaceRoot } from "../../../🟦️.ts";

test("pinned Cargo launches a large static graph and its procedural macro without a compiler wrapper", async () => {
  const workspace=getWorkspaceRoot(), artifact=process.env.SEMIO_TEST_ARTIFACT_DIR;
  if(!artifact)throw Error("SEMIO_TEST_ARTIFACT_DIR is required");
  const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"../../🧫️fixtures/📚️library-search-path/🔣️.json"),"utf8"));
  expect(fixture.libraries*fixture.libraryValue+fixture.macroValue).toBe(fixture.expected);
  const root=mkdtempSync(join(artifact,"cargo-library-path-")), build=join(root,fixture.buildDirectory), members:string[]=[], dependencies:Record<string,{path:string}>={};
  const write=(path:string,content:string):void => {mkdirSync(resolve(path,".."),{recursive:true});writeFileSync(path,content);};
  for(let index=0;index<fixture.libraries;index++) {
    const name="library_"+String(index).padStart(3,"0");members.push(name);dependencies[name]={path:"../"+name};
    write(join(root,name,"Cargo.toml"),stringify({package:{name,version:"0.1.0",edition:"2021"}}));
    write(join(root,name,"src/lib.rs"),`pub fn answer() -> u32 { ${fixture.libraryValue} }\n`);
  }
  members.push("macro_value","application");dependencies.macro_value={path:"../macro_value"};
  write(join(root,"macro_value/Cargo.toml"),stringify({package:{name:"macro_value",version:"0.1.0",edition:"2021"},lib:{"proc-macro":true}}));
  write(join(root,"macro_value/src/lib.rs"),`#[proc_macro] pub fn answer(_: proc_macro::TokenStream) -> proc_macro::TokenStream { "${fixture.macroValue}".parse().unwrap() }\n`);
  write(join(root,"application/Cargo.toml"),stringify({package:{name:"application",version:"0.1.0",edition:"2021"},dependencies}));
  write(join(root,"application/build.rs"),'fn main() { println!("cargo:rerun-if-changed=build.rs"); }\n');
  write(join(root,"application/src/main.rs"),`fn main() { println!("{}", ${members.filter(name=>name.startsWith("library_")).map(name=>name+"::answer()").join(" + ")} + macro_value::answer!()); }\n`);
  const manifest=stringify({workspace:{members,resolver:"2"}});write(join(root,"Cargo.toml"),manifest);
  expect(Bun.TOML.parse(manifest)).toEqual(parse(manifest));
  expect(members.map(name=>join(build,"debug/build",name,"0123456789abcdef/out")).join(";").length).toBeGreaterThan(fixture.minimumUnfilteredCharacters);
  const toolchain=Bun.TOML.parse(readFileSync(join(workspace,"rust-toolchain.toml"),"utf8")) as {toolchain:{channel:string}};
  const {RUSTC_WRAPPER:_wrapper,RUSTC_WORKSPACE_WRAPPER:_workspaceWrapper,...ambient}=process.env;
  const child=Bun.spawn(["cargo","-Zbuild-dir-new-layout","run","--quiet","--offline","--manifest-path",join(root,"Cargo.toml"),"-p","application"],{cwd:root,env:{...ambient,RUSTUP_TOOLCHAIN:toolchain.toolchain.channel,CARGO_TARGET_DIR:join(root,"target"),CARGO_BUILD_BUILD_DIR:build,CARGO_INCREMENTAL:"0"},stdout:"pipe",stderr:"pipe"});
  const [output,errors,code]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
  console.log(`[DEBUG] Cargo library path contract exit=${code} libraries=${fixture.libraries}`);
  expect(code,errors).toBe(0);expect(output.trim()).toBe(String(fixture.expected));
}, {timeout:120000});


test("the pinned Cargo consumes the current metadata configuration without obsolete keys", async () => {
  const workspace=getWorkspaceRoot(),text=readFileSync(join(workspace,".cargo/config.toml"),"utf8"),config=Bun.TOML.parse(text) as {unstable:Record<string,unknown>};
  expect(config).toEqual(parse(text));expect(config.unstable["no-embed-metadata"]).toBeUndefined();expect(config.unstable["embed-metadata"]).toBe("DoNotEmbed");
  const child=Bun.spawn(["cargo","-Z","unstable-options","config","get","unstable.embed-metadata","--format","json"],{cwd:workspace,stdout:"pipe",stderr:"pipe"});
  const [output,errors,code]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
  expect(code,errors).toBe(0);expect(errors).not.toContain("unused config key");expect(JSON.parse(output)).toEqual({unstable:{"embed-metadata":"DoNotEmbed"}});
});
