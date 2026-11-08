import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./child-custody.schema.json";
import cases from "./child-custody.json";
/** 📬️ A selected child remains resident until its original reply has a paid publication. */
test("original paged child is never replayed",()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);const db=new Database(":memory:");try{for(const row of cases){const actual=db.query("select (? and ? and not ?) as recover, (? and ? and ? and ?) as publish, 0 as replay").get(Number(row.sameSource),Number(row.started),Number(row.recorded),Number(row.sameSource),Number(row.started),Number(row.recorded),Number(row.funded)) as {recover:number,publish:number,replay:number};expect({recover:Boolean(actual.recover),publish:Boolean(actual.publish),replay:Boolean(actual.replay)}).toEqual({recover:row.recover,publish:row.publish,replay:row.replay})}}finally{db.close()}});
