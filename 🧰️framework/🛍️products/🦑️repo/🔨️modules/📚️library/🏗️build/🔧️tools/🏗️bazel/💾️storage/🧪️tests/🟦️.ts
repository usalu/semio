import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {join,resolve} from "node:path";
import {readFileSync} from "node:fs";
import {parse} from "@iarna/toml";
import componentSchema from "../../🧬️schema/🔣️.json";const schema=componentSchema.definitions.StorageLayout;
import fixture from "./🧫️fixtures/🔣️.json";
import {bazelCacheLayoutV1} from "../🟦️.ts";
import {canonicalArchitectureEnvironment} from "../../../../../⚡️caching/🦀️cargo/🌿️environment/🟦️.ts";

test("every Bazel generated writer selects an authored finite partition against an independent SQL oracle",()=>{
 const roots={tools:resolve(fixture.roots.tools),dependencies:resolve(fixture.roots.dependencies),products:resolve(fixture.roots.products)},layout=bazelCacheLayoutV1(roots),validate=new Ajv({strict:true}).compile(schema),database=new Database(":memory:");
 expect(validate(layout),JSON.stringify(validate.errors)).toBe(true);
 try{database.exec("CREATE TABLE roots(partition TEXT PRIMARY KEY,path TEXT)");for(const[key,value]of Object.entries(roots))database.query("INSERT INTO roots VALUES(?1,?2)").run(key,value.replaceAll("\\","/"));for(const row of fixture.paths){const oracle=database.query("SELECT path||'/'||?2 AS path FROM roots WHERE partition=?1").get(row.partition,row.suffix) as {path:string};expect(layout[row.name as keyof typeof layout].toString().replaceAll("\\","/")).toBe(oracle.path);}}
 finally{database.close();}
 expect(()=>bazelCacheLayoutV1({...roots,tools:"relative"})).toThrow("Absolute");expect(()=>bazelCacheLayoutV1({...roots,products:"/"+"x".repeat(256)})).toThrow("256");
});

test("native grants select the authored Cargo target against an independent TOML oracle",()=>{
 const root=process.cwd(),configuration=parse(readFileSync(join(root,".cargo/config.toml"),"utf8")) as {build:{"target-dir":string;"build-dir":string}};
 for(const row of fixture.cargoAuthority){const expected=resolve(root,row.target==="configuration"?configuration.build["target-dir"]:row.target),actual=canonicalArchitectureEnvironment(root,row.environment);expect(actual.CARGO_TARGET_DIR).toBe(expected);expect(actual.CARGO_BUILD_BUILD_DIR).toBe(resolve(root,configuration.build["build-dir"]));}
});
