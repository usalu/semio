import callers from "./📋️callers.json" with {type:"json"};
import {readRustSourceClosureV1} from "@semio-tech/process/artifacts/native-build/source";
import {readFile} from "node:fs/promises";
import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
import {parse} from "smol-toml";
import {Database} from "bun:sqlite";
import fixture from "./🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";

test("composition admission owns original bounded projection below Store",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const root=join(import.meta.dir,".."),manifest=parse(readFileSync(join(root,"📦️packages/🦀️rust/Cargo.toml"),"utf8"))as any,source=readFileSync(join(root,"🦀️.rs"),"utf8"),store=readModuleTreeSource(join(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"));
 expect(manifest.package.name).toBe(fixture.owner);expect(manifest.package.metadata.semio["rust-source-boundary"]).toEqual(fixture.sourceBoundary);expect(Object.keys(manifest.dependencies).sort()).toEqual(fixture.productionDependencies);
 for(const name of fixture.definitions){expect(source).toContain("pub "+(name.endsWith("Error")?"enum ":"struct ")+name);expect(store).not.toContain("pub "+(name.endsWith("Error")?"enum ":"struct ")+name);}
 expect(source).not.toContain("crate::os_");expect(source).not.toContain("semio_framework_os_kernel");const db=new Database(":memory:");try{for(const row of fixture.cases){const result=db.query("select not exists(select 1 from json_each(?1) c where json_extract(c.value,'$.slot')<>'members' or json_extract(c.value,'$.kind')<>'s.test.member' or exists(select 1 from json_each(c.value) f where f.key in('childId','artifactId','standard','subset') and (length(cast(f.value as blob))=0 or length(cast(f.value as blob))>256))) and not exists(select 1 from json_each(?1) a join json_each(?1) b on a.key<b.key where json_extract(a.value,'$.childId')=json_extract(b.value,'$.childId') or json_extract(a.value,'$.artifactId')=json_extract(b.value,'$.artifactId')) accepted").get(JSON.stringify(row.children))as{accepted:number};expect(Boolean(result.accepted),row.id).toBe(row.accepted);}}finally{db.close();}
});

test("composition admission keeps bounded original production and native Source on every host",async()=>{
 const root=join(import.meta.dir,".."),entry=join(root,"🦀️.rs"),native=join(root,"🧪️tests/🦀️.rs");for(const host of ["macos","linux","windows"])for(const test of [false,true]){const rows=await readRustSourceClosureV1([entry,...(test?[native]:[])],process.cwd(),["target_arch=x86_64","target_os="+host,"target_env="+(host==="windows"?"msvc":""),"target_pointer_width=64",host==="windows"?"windows":"unix",...(test?["test"]:[])],path=>readFile(path).catch((error:NodeJS.ErrnoException)=>{if(error.code==="ENOENT")return null;throw error;})),bytes=rows.reduce((sum,row)=>sum+row.bytes,0),lines=rows.reduce((sum,row)=>sum+row.activeLines,0);expect(bytes).toBeLessThanOrEqual(fixture.sourceBoundary.maxSourceBytes);expect(lines).toBeLessThanOrEqual(fixture.sourceBoundary.maxActiveLines);expect(rows.length).toBeLessThanOrEqual(fixture.sourceBoundary.maxSourceFiles);}
});

test("Store and Plugin consume canonical projection owner without legacy type facades",()=>{
 const kernel=readFileSync(join(process.cwd(),"🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml"),"utf8"),plugin=readFileSync(join(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml"),"utf8"),store=readModuleTreeSource(join(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs")),child=readModuleTreeSource(join(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪆️child/🦀️.rs"));for(const manifest of[kernel,plugin])expect(parse(manifest).dependencies).toHaveProperty("semio-framework-composition-admission");expect(store).toContain("use semio_framework_composition_admission::");expect(store).not.toContain("pub use semio_framework_composition_admission::{");expect(store).not.toContain("$crate::os_store::ChildRestoreProjection");expect(child).toContain("semio_framework_composition_admission::ChildRestoreProjection");
});

test("all declared canonical projection callers have a direct physical Cargo owner",()=>{
 expect(new Ajv({strict:true}).compile({$schema:schema.$schema,definitions:schema.definitions,$ref:"#/definitions/CallerCorpus"})(callers)).toBe(true);const seen=new Set<string>();for(const row of callers.callers){expect(seen.has(row.source),row.source).toBe(false);seen.add(row.source);const source=readModuleTreeSource(join(process.cwd(),row.source)),manifest=parse(readFileSync(join(process.cwd(),row.manifest),"utf8"))as any;expect(source,row.source).toContain("semio_framework_composition_admission::");expect(source,row.source).not.toMatch(/(?:os_store|store)::ChildRestoreProjection/u);expect(manifest.dependencies,row.manifest).toHaveProperty("semio-framework-composition-admission");}
});
