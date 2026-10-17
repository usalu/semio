import {readRustSourceClosureV1} from "@semio-tech/process/artifacts/native-build/source";
import {readFile} from "node:fs/promises";
import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {readModuleTreeSource} from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import Ajv from "ajv";
import {parse} from "smol-toml";
import {Database} from "bun:sqlite";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
test("IO codec owns bounded domain-neutral contracts and original laws",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const owner=join(import.meta.dir,".."),manifest=parse(readFileSync(join(owner,"📦️packages/🦀️rust/Cargo.toml"),"utf8")) as {package:{name:string;metadata:{semio:Record<string,unknown>}};dependencies:Record<string,unknown>},source=readFileSync(join(owner,"🦀️.rs"),"utf8"),laws=readFileSync(join(owner,"🧪️tests/🦀️.rs"),"utf8"),original=readModuleTreeSource(join(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🦀️.rs"));
 expect(manifest.package.name).toBe(fixture.owner);expect(Object.keys(manifest.dependencies).sort()).toEqual(fixture.productionDependencies);expect(manifest.package.metadata.semio["rust-source-boundary"]).toEqual(fixture.sourceBoundary);
 for(const name of fixture.definitions){const pattern=new RegExp("pub (?:struct|enum|trait|type) "+name+"\\b");expect(pattern.test(source),name).toBe(true);expect(pattern.test(original),name).toBe(false);}
 for(const name of fixture.originalNativeLaws)expect(laws).toContain("fn "+name+"(");
 const oracle=new Database(":memory:");try{for(const row of fixture.budgets){const actual=oracle.query("SELECT CASE WHEN ?+?<=? THEN ?+? ELSE ? END AS result, ?+?<=? AS admitted").get(row.used,row.increment,row.limit,row.used,row.increment,row.used,row.used,row.increment,row.limit) as {result:number;admitted:number};expect(actual).toEqual({result:row.result,admitted:Number(row.admitted)});}}finally{oracle.close();}
 for(const cfg of [["target_arch=aarch64","target_os=macos","target_env=","target_pointer_width=64","unix"],["target_arch=x86_64","target_os=linux","target_env=gnu","target_pointer_width=64","unix"],["target_arch=x86_64","target_os=windows","target_env=msvc","target_pointer_width=64","windows"]])for(const native of [false,true]){
  const rows=await readRustSourceClosureV1([join(owner,"📦️packages/🦀️rust/🦀️.rs")],process.cwd(),[...cfg,...(native?["test"]:[])],path=>readFile(path).catch((error:NodeJS.ErrnoException)=>{if(error.code==="ENOENT")return null;throw error;})),stats={files:rows.length,lines:rows.reduce((sum,row)=>sum+row.activeLines,0),bytes:rows.reduce((sum,row)=>sum+row.bytes,0)};
  expect(rows.find(row=>row.path.endsWith("/🚪️io/📦️codec/🦀️.rs"))!.kind).toBe("source");expect(stats.files).toBeLessThanOrEqual(fixture.sourceBoundary.maxSourceFiles);expect(stats.lines).toBeLessThanOrEqual(fixture.sourceBoundary.maxActiveLines);expect(stats.bytes).toBeLessThanOrEqual(fixture.sourceBoundary.maxSourceBytes);
 }
});
