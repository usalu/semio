import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import {createHash} from "node:crypto";
import fixture from "./🔣️.json";
import {canonicalRowOrderV1,packRecordV1,unpackRecordV1} from "../../🟦️.ts";

test("a record packs to the same bytes whatever the order its rows were observed in, and the order is the one of an independent implementation",()=>{
 const run=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py"),join(import.meta.dir,"🔣️.json")],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const rows=fixture.rows as never[];
 expect(canonicalRowOrderV1(rows)).toEqual(JSON.parse(run.stdout));
 const digests=fixture.orders.map(order=>createHash("sha256").update(JSON.stringify(packRecordV1({digest:"d",authority:[rows[1]!,rows[4]!].sort(()=>order[0]!-3) as never,observations:order.map(at=>rows[at]!) as never,image:"{}"}))).digest("hex"));
 expect(new Set(digests).size).toBe(1);
 const back=unpackRecordV1(packRecordV1({digest:"d",authority:[],observations:fixture.orders[2]!.map(at=>rows[at]!) as never,image:"{}"}));
 expect([...back.observations].map(row=>JSON.stringify(row)).sort()).toEqual(rows.map(row=>JSON.stringify(row)).sort());
});
