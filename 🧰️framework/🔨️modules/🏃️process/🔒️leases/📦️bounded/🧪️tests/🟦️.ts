import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import fixture from "./🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {resourceLeaseMaximumPagesV1} from "../../🟦️.ts";

test("closed bootstrap SQLite admission agrees with independent schema and SQL page division",()=>{
 const validate=new Ajv({strict:true}).compile(schema),database=new Database(":memory:");try{for(const row of fixture.cases){expect(validate({version:1,maximumDatabaseBytes:row.maximumDatabaseBytes})).toBe(row.accepted);if(row.accepted){const oracle=database.query("SELECT ?1/4096 AS pages").get(row.maximumDatabaseBytes) as {pages:number};expect(resourceLeaseMaximumPagesV1(row.maximumDatabaseBytes)).toBe(oracle.pages);expect(oracle.pages).toBe(row.maximumPages!);}else expect(()=>resourceLeaseMaximumPagesV1(row.maximumDatabaseBytes)).toThrow();}}finally{database.close();}
});
