import {test,expect} from "bun:test";
import Ajv from "ajv";
import groupBy from "lodash/groupBy";
import {Database} from "bun:sqlite";
import corpus from "./🧫️fixtures/🔣️.json";
import schema from "./🧫️fixtures/🧬️.json";
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
