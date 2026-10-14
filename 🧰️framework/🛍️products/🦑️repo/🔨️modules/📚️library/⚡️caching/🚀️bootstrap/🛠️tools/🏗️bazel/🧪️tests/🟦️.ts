import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import manifest from "../🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

test("pinned Bazel selects six portable distributions against an independent SQL oracle",async()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(manifest),JSON.stringify(validate.errors)).toBe(true);
 const {bazelDistribution}=await import("../📜️script.ts"),database=new Database(":memory:");
 try{database.exec("CREATE TABLE distributions(platform TEXT,arch TEXT,file TEXT,PRIMARY KEY(platform,arch))");for(const row of manifest.distributions)database.query("INSERT INTO distributions VALUES (?1,?2,?3)").run(row.platform,row.arch,row.file);for(const row of fixture.cases){const oracle=database.query("SELECT file FROM distributions WHERE platform=?1 AND arch=?2").get(row.platform,row.arch) as {file:string};expect(oracle.file).toBe(row.file);expect(bazelDistribution(row.platform,row.arch).file).toBe(oracle.file);}for(const row of fixture.unsupported)expect(()=>bazelDistribution(row.platform,row.arch)).toThrow("Unsupported pinned Bazel");}
 finally{database.close();}
});
