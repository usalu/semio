import {expect,test} from "bun:test";
import Ajv from "ajv";
import * as TOML from "@iarna/toml";
import {readFileSync,existsSync,mkdirSync,writeFileSync} from "node:fs";
import {resolve,join} from "node:path";
import {spawnSync} from "node:child_process";
const root=resolve(import.meta.dir,"../../../../../..");
const owner=resolve(import.meta.dir,"../..");
const corpus=JSON.parse(readFileSync(join(owner,"🧫️fixtures/📍️ownership/🔣️.json"),"utf8")) as {source:string;unit:string;retainedUnitSha256:string;family:string[];laws:string[];cacheBehavior:Record<string,string>};
const validator=new Ajv({strict:true}).compile(JSON.parse(readFileSync(join(owner,"🧬️schema/📍️ownership/🔣️.json"),"utf8")));
const read=(path:string):string=>readFileSync(join(root,path),"utf8");
test("closed compute ownership corpus retains six actual law names",()=>{
 expect(validator(corpus),JSON.stringify(validator.errors)).toBe(true);
 expect(validator({...corpus,unknown:true})).toBe(false);
 expect(new Set(corpus.family).size).toBe(7);
 expect(new Set(corpus.laws).size).toBe(6);
});
test("physical neutral owner retains original native assertions with independent Node identity",()=>{
 expect(existsSync(join(root,corpus.source))).toBe(true);
 const unit=read(corpus.unit);
 for(const law of corpus.laws)expect(unit).toContain("fn "+law+"(");
 expect(new Bun.CryptoHasher("sha256").update(unit).digest("hex")).toBe(corpus.retainedUnitSha256);
 const child=spawnSync("node",["--eval","const fs=require('node:fs'),crypto=require('node:crypto');process.stdout.write(JSON.stringify({digest:crypto.createHash('sha256').update(fs.readFileSync(process.argv[1])).digest('hex')}));",join(root,corpus.unit)],{encoding:"utf8"});
 expect(child.status,child.stderr).toBe(0);
 expect(JSON.parse(child.stdout)).toEqual({digest:corpus.retainedUnitSha256});
 const source=read(corpus.source);
 for(const name of corpus.family)expect(source).toMatch(new RegExp("pub (?:struct|enum|trait) "+name+"\\b"));
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("caller-owned ticket output required");mkdirSync(output,{recursive:true});writeFileSync(join(output,"compute-ownership-node.json"),child.stdout);
});
test("independent TOML oracle proves generic compute has no product dependency or facade",()=>{
 const manifest=TOML.parse(read("🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust/Cargo.toml"));
 expect(Object.keys(manifest.dependencies??{})).not.toContain("semio-framework-os-kernel");
 expect(manifest.lib).toHaveProperty("name","semio_framework_2d");
 const binding=read("🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust/🦀️.rs");
 expect(binding).toContain('pub mod compute;');
 expect(binding).not.toMatch(/os_spr|os_engine|KernelEngineHandle/);
});
