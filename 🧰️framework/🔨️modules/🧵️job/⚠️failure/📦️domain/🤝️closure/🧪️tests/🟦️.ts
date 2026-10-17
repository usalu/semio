import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{ValueError,type ValueRefusalKind}from"../../../../../🌱️value/📦️packages/🟦️typescript/⚠️refusal/🟦️.ts";
import type{OriginalClosureFailure,OriginalTurnFailure}from"../../../🟦️.ts";
import schema from"../🧬️schema/🔣️.json";
import law from"./🧫️fixtures/🔣️.json";
test("original source closure retains both complete turn owners without a second codec",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const make=(row:typeof law.wire.initial):OriginalTurnFailure=>({primary:{kind:"value",value:new ValueError(row.primary.kind as ValueRefusalKind,row.primary.message)},receiptRefusal:new ValueError(row.receiptRefusal.kind as ValueRefusalKind,row.receiptRefusal.message),contextRefusal:new ValueError(row.contextRefusal.kind as ValueRefusalKind,row.contextRefusal.message)}),initial=make(law.wire.initial),closure=make(law.wire.closure),compound:OriginalClosureFailure={initial,closure};expect(compound.initial).toBe(initial);expect(compound.closure).toBe(closure);expect(compound.initial.receiptRefusal).toBe(initial.receiptRefusal);expect(compound.closure.contextRefusal).toBe(closure.contextRefusal);
 const db=new Database(":memory:");try{expect(db.query("SELECT json_extract(?, '$.initial.receiptRefusal.message') AS initial, json_extract(?, '$.closure.contextRefusal.message') AS closure").get(JSON.stringify(law.wire),JSON.stringify(law.wire))).toEqual({initial:initial.receiptRefusal?.message,closure:closure.contextRefusal?.message})}finally{db.close()}
 const native=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");expect(native.includes("project_original_turn_failure(&self.initial,rest)")).toBe(true);expect(native.includes("project_original_turn_failure(&self.closure,rest)")).toBe(true);expect(native.includes('"original.source-closure"')).toBe(true);expect(native.includes("fn domain_retained_progress(&self)->RetainedCloneProgress{Default::default()}")).toBe(true);
});
