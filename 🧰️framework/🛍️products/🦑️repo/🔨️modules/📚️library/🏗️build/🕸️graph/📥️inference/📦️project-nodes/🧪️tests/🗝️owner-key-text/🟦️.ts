import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import {createHash} from "node:crypto";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const nodes=await import("../../🟨️.mjs"),present=await import("../../📋️present/🟨️.mjs"),observation=await import("../../../../../👁️observation/🧬️schema/🔣️.json");

test("the key text of an owner record equals the independent oracle and names nothing of the machine that wrote it",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 expect(observation.default.$defs.record.const.stamp).toBe(fixture.stamp);
 const run=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py"),join(import.meta.dir,"🔣️.json")],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const oracle=JSON.parse(run.stdout) as {stem:string;text:string;digest:string}[];
 const rows=fixture.owners.map(owner=>{const text=nodes.ownerRecordKeyTextV1(nodes.projectRecordKeyV1(owner.head,owner.root,owner.json,owner.selectedTargets,owner.selectedInputs));return{stem:present.ownerRecordKeyV1(owner.root),text,digest:createHash("sha256").update(text).digest("hex")};});
 expect(rows).toEqual(oracle);
 for(const row of rows)expect(row.text).not.toMatch(/\/Users\/|\/home\/|[A-Za-z]:\\\\|\/private\//u);
 expect(new Set(rows.map(row=>row.stem)).size).toBe(rows.length);
});
