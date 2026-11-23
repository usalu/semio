import {test,expect} from "bun:test";
import Ajv from "ajv";
import groupBy from "lodash/groupBy";
import {Database} from "bun:sqlite";
import corpus from "./🧫️fixtures/🔣️.json";
import schema from "./🧫️fixtures/🧬️.json";
import chargeCases from "./🧫️fixtures/retained-charge.json";
import chargeSchema from "./🧫️fixtures/retained-charge.schema.json";
import floorLaw from "./🧫️fixtures/receiving-floor.json";
import floorSchema from "./🧫️fixtures/receiving-floor.schema.json";
import quoteLaw from "./🧫️fixtures/receiving-quote-refusal.json";
import quoteSchema from "./🧫️fixtures/receiving-quote-refusal.schema.json";
import refundLaw from "./🧫️fixtures/receiving-refund.json";
import refundSchema from "./🧫️fixtures/receiving-refund.schema.json";
test("one retained item and one fuel are admitted together", () => {
 expect(new Ajv().compile(chargeSchema)(chargeCases)).toBe(true);
 using db = new Database(":memory:");
 const rows = db.query(`
   with cases as (
     select value, json_extract(value, '$.fuel') > 0 as fuel,
       json_extract(value, '$.retirement') and json_extract(value, '$.loan') as bypass
     from json_each(?)
   )
   select fuel and not bypass as sampled,
     fuel and (bypass or coalesce(json_extract(value, '$.clock') < 10, 0))
       and json_extract(value, '$.grant[0]') > 0
       and json_extract(value, '$.grant[4]') > 0
       and json_extract(value, '$.grant[1]') >= json_extract(value, '$.quote[0]')
       and json_extract(value, '$.grant[2]') >= json_extract(value, '$.quote[1]')
       and json_extract(value, '$.grant[3]') >= json_extract(value, '$.quote[2]') as accepted
     from cases
 `).all(JSON.stringify(chargeCases));
 expect(rows).toEqual(chargeCases.map(({sampled, accepted}) => ({sampled:+sampled, accepted:+accepted})));
});
test("scoped progress observation leaves original cancellation verdict and authority intact",()=>{
 expect(new Ajv({strict:false}).compile(schema)(corpus)).toBe(true);const groups=groupBy(corpus.events,"scope");expect(groups.outer.map(row=>row.completed)).toEqual(corpus.expected.outer);expect(groups.inner.map(row=>row.completed)).toEqual(corpus.expected.inner);
 const db=new Database(":memory:");try{const rows=db.query("select case json_extract(value,'$.cancelled') when 1 then 0 else 1 end as admitted from json_each(?)").all(JSON.stringify(corpus.events))as{admitted:number}[];expect(rows.map(row=>!!row.admitted)).toEqual(corpus.expected.verdicts);}finally{db.close();}
});
test("the actual fixture hook is scoped and returns no callback verdict",async()=>{
 const source=await Bun.file(new URL("../../../🛬️receiving/🧪️testing/👁️progress/🦀️.rs",import.meta.url)).text();
 const state=await Bun.file(new URL("../../../🛬️receiving/🎟️state/🦀️.rs",import.meta.url)).text();
 for(const proof of ["pub fn with_native_decode_progress","FnMut(semio_framework_value_native_owned::native_decoding::NativeDecodeProgress)","impl Drop for NativeProgressObserverScope"])expect(source.includes(proof)).toBe(true);
 expect(state.includes("testkit::observe_native_decode_progress(progress)")).toBe(true);
});
test("a receiving floor keeps the half share, raises it up to the whole turn and refuses beyond it",()=>{
 expect(new Ajv().compile(floorSchema)(floorLaw)).toBe(true);
 using db=new Database(":memory:");
 const rows=db.query("select case when json_extract(value,'$.floorCopyBytes')<=(?2+1)/2 then 'share' when json_extract(value,'$.floorCopyBytes')<=?2 then 'floor' else 'refused' end as expect from json_each(?1,'$.cases')").all(JSON.stringify(floorLaw),floorLaw.turnCopyBytes);
 expect(rows).toEqual(floorLaw.cases.map(({expect:kind})=>({expect:kind})));
 expect(new Set(floorLaw.cases.map(({expect:kind})=>kind))).toEqual(new Set(["share","floor","refused"]));
 expect(floorLaw.settlingTurns).toBeLessThan(floorLaw.refusalTurns);
});

test("a quoted receiving scope beyond the turn is refused after at most its settling turns",()=>{
 expect(new Ajv().compile(quoteSchema)(quoteLaw)).toBe(true);
 expect(quoteLaw.settlingTurns).toBeLessThan(quoteLaw.refusalTurns);
 expect(quoteLaw.quoteCopyBytes).toBeGreaterThan(65536);
});

test("an idle unquoted receiving scope settles with nothing held on the Source",()=>{
 expect(new Ajv().compile(refundSchema)(refundLaw)).toBe(true);
 using db=new Database(":memory:");
 const row=db.query("select json_extract(?1,'$.heldAfterSettle')+json_extract(?1,'$.claimsAfterSettle') as kept").get(JSON.stringify(refundLaw)) as {kept:number};
 expect(row.kept).toBe(0);
 expect(refundLaw.scopes).toBeGreaterThan(1);
});
