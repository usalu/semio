import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "../🧫️fixtures/🔣️.json" with {type:"json"};
import {namespaceContinuity} from "../🟦️.ts";

test("retained entry namespace continuity matches neutral cases and exact-text SQLite",()=>{
 const ajv=new Ajv({strict:false}),validFixture=ajv.compile(schema),validObservation=ajv.compile({$defs:schema.$defs,$ref:"#/$defs/observation"});expect(validFixture(fixture),JSON.stringify(validFixture.errors)).toBe(true);const db=new Database(":memory:");
 try{for(const row of fixture.cases){const valid=validObservation(row.input);expect(valid,row.id).toBe(row.schemaValid);const sql=db.query("SELECT ?1 AND json_extract(?2,'$.published.root.device')=json_extract(?2,'$.published.journal.device') AND json_extract(?2,'$.published.root.device')=json_extract(?2,'$.published.entry.device') AND json_extract(?2,'$.current.root.device')=json_extract(?2,'$.current.journal.device') AND json_extract(?2,'$.current.root.device')=json_extract(?2,'$.current.entry.device') AND json_extract(?2,'$.published.root.inode')=json_extract(?2,'$.current.root.inode') AND json_extract(?2,'$.published.journal.inode')=json_extract(?2,'$.current.journal.inode') AND json_extract(?2,'$.published.entry.inode')=json_extract(?2,'$.current.entry.inode') AS accepted").get(Number(valid),JSON.stringify(row.input))as{accepted:number|null};expect(namespaceContinuity(row.input),row.id).toBe(row.accepted);expect(Boolean(sql.accepted),row.id).toBe(row.accepted);}}
 finally{db.close();}
});
