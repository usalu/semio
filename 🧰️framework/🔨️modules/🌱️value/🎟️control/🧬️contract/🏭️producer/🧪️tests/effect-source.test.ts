import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./effect-source.schema.json";
import cases from "./effect-source.json";
/** 🪪️ A Source-backed producer requires the same bound original Work Source. */
test("producer and payer Source combinations",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{for(const row of cases){const result=db.query("SELECT NOT ? OR (? AND ?) AS accepted").get(Number(row.producer),Number(row.payer),Number(row.same)) as {accepted:number};expect(Boolean(result.accepted)).toBe(row.accepted)}}finally{db.close()}});
