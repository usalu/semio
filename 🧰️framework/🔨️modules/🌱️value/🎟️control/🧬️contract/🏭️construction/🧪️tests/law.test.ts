import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import cases from "./cases.json";
import schema from "./schema.json";
test("construction replies belong to the same original action",()=>{expect(new Ajv({strict:true}).validate(schema,cases)).toBe(true);const db=new Database(":memory:");try{for(const c of cases){const r=db.query("select case when not ? or not ? or not ? then 'foreign' when ? = 'none' then 'pending' when ? then 'consumed' else ? end as verdict").get(Number(c.sameIssuer),Number(c.sameAction),Number(c.sameContext),c.recorded,Number(c.secondTake),c.recorded) as {verdict:string};expect(r.verdict).toBe(c.expected);expect(c.firstTake).toBe(c.sameIssuer&&c.sameAction&&c.sameContext&&c.recorded!=="none");if(c.secondTake)expect(c.firstTake).toBe(true)}}finally{db.close()}});

test("construction receipt and cancellation preserve the actual marker",()=>{const cases=require("./marker-cases.json");const schema=require("./marker-schema.json");expect(new Ajv({strict:true}).validate(schema,cases)).toBe(true);const db=new Database(":memory:");try{for(const c of cases){const row=db.query("select (not ? and ? = 0 and ? is null) as cancel, (case when ? is null then ? = 0 else ? = ? end) as post").get(Number(c.performed),c.capacity,c.actual,c.actual,c.capacity,c.actual,c.capacity) as {cancel:number,post:number};expect(Boolean(row.cancel)).toBe(c.canCancel);expect(Boolean(row.post)).toBe(c.canPost)}}finally{db.close()}});
