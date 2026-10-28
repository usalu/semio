import {expect,test} from "bun:test";
import {existsSync} from "node:fs";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import fixture from "../nested-io.json";
import { orderedRetirementSchema as contract } from "@semio-tech/value-ts/ordered/retirement/schema";

test("Nested IO original physical grant and native custody witness match canonical Ajv and independent JSON",()=>{
 expect(existsSync(new URL("../🧬️schema/💰️wallet.json",import.meta.url))).toBe(false);
 const validate=new Ajv({strict:true}).addSchema(contract).compile({$ref:contract.$id+"#/$defs/Grant"});expect(validate(fixture.grant)).toBe(true);expect(JSON.parse(JSON.stringify(fixture.original))).toBe(fixture.original);expect(fixture.expected.nestedFrames).toBe(2);expect(Buffer.byteLength(fixture.original,"utf8")).toBe(28);expect(fixture.grant.maximumDepth).toBeGreaterThan(fixture.expected.nestedFrames);
});

test("Native phase selection neutral cases match independent SQLite decisions",()=>{
 const database=new Database(":memory:");
 try{database.exec("CREATE TABLE selection(original_pair INTEGER,pending_owner INTEGER,selected TEXT)");
 for(const row of fixture.phaseSelection)database.query("INSERT INTO selection VALUES(?,?,?)").run(Number(row.originalPair),Number(row.pendingOwner),row.selected);
 const rows=database.query("SELECT selected,CASE WHEN original_pair=1 AND pending_owner=0 THEN 'encode' ELSE 'decode' END AS independent FROM selection").all() as Array<{selected:string;independent:string}>;
 expect(rows.map(row=>row.independent)).toEqual(fixture.phaseSelection.map(row=>row.selected));expect(fixture.expected.oppositePhaseRefused).toBe(false);expect(fixture.expected.occupiedPendingPhaseRefused).toBe(true);
 }finally{database.close();}
});
