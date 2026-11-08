import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./policy.schema.json";
import cases from "./policy.json";
/** 🛂️ Original Source actors admit fresh work only under a resident clock decision. */
test("Source native quote and settlement policy",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{const q=db.query("SELECT (? AND ? AND ? > 0) AS quote, ? AS settle");for(const c of cases){const r=q.get(+c.sameSource,+c.decision,c.fuel,+c.sameSource) as {quote:number,settle:number};expect(Boolean(r.quote)).toBe(c.quote);expect(Boolean(r.settle)).toBe(c.settle)}}finally{db.close()}});
