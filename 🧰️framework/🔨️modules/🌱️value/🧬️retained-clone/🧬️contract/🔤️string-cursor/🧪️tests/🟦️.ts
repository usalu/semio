import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv2020 from "ajv";
import {stringCloneOutputReady} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

test("string output readiness requires completion and retained transfer custody",()=>{
 const validate=new Ajv2020({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,states:[...fixture.states,{id:"unknown",complete:false,closing:false,output:false,ready:false}]})).toBe(false);
 const database=new Database(":memory:");try{for(const row of fixture.closeGrants){const d=row.demand,g=row.grant;const actual=g.items>=1&&g.copyBytes>=d.copyBytes&&g.capacityBytes>=d.capacityBytes&&g.releaseBytes>=d.releaseBytes&&g.depth>=d.depth;const oracle=database.query("SELECT ? >= 1 AND ? >= ? AND ? >= ? AND ? >= ? AND ? >= ? AS accepted").get(g.items,g.copyBytes,d.copyBytes,g.capacityBytes,d.capacityBytes,g.releaseBytes,d.releaseBytes,g.depth,d.depth)as{accepted:number};expect(actual).toBe(row.accepted);expect(Boolean(oracle.accepted)).toBe(row.accepted);}expect(database.query("SELECT ? < ? AS separate_idle_bound").get(fixture.maximumIdleTurns,fixture.closeTurns)).toEqual({separate_idle_bound:1});for(const state of fixture.states){const oracle=database.query("SELECT CASE WHEN ?=1 AND ?=0 AND ?=1 THEN 1 ELSE 0 END AS ready").get(Number(state.complete),Number(state.closing),Number(state.output))as{ready:number};expect(stringCloneOutputReady(state)).toBe(state.ready);expect(Boolean(oracle.ready)).toBe(state.ready);}for(const row of fixture.texts){const text=row.text.repeat(row.repeat);expect(Buffer.from(text,"utf8").toString("utf8")).toBe(text);}}finally{database.close();}
});
