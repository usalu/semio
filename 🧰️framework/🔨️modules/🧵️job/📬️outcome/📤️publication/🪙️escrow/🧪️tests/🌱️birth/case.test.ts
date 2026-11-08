import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import data from "./case.json";
import schema from "./case.schema.json";
test("publisher backing is initialized in bounded paid chunks",()=>{
 expect(new Ajv().validate(schema,data)).toBe(true);
 const db=new Database(":memory:");
 const row=db.query("SELECT ?1 + ?2 * ?3 AS bytes, (?1 + ?2 * ?3 + ?4 - 1) / ?4 AS turns, ?4 <= ?5 AS bounded").get(data.headerBytes,data.rows,data.rowBytes,data.chunk,data.maximumCopy) as {bytes:number,turns:number,bounded:number};
 expect(row.bytes).toBe(5936);expect(row.turns).toBe(6);expect(row.bounded).toBe(1);db.close();
});
