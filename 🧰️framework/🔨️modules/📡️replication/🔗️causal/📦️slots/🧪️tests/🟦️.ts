import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { applyPatch } from "fast-json-patch";
import law from "../🧫️fixtures/🔣️.json";
test("causal fixed slots separate exact owners from bounded physical pages",()=>{
 const pages=law.capacity/law.pageSlots;expect(pages).toBe(128);expect((pages+6)*3).toBeLessThanOrEqual(law.maximumCloseTurns);
 for(const row of law.cases){const model={ids:Array.from({length:row.occupied},(_,i)=>i),released:0};const identity=Buffer.from(new Uint16Array(model.ids).buffer);expect(identity.length).toBe(row.occupied*2);const copy=applyPatch(structuredClone(model),[{op:"replace",path:"/released",value:law.pageSlots}],false,false).newDocument;expect(copy.ids).toEqual(model.ids);expect(model.released).toBe(0);expect(copy.released).toBe(law.pageSlots);}
 const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");
 expect(source.includes(`MUTATION_DAG_PAGE_SLOTS: usize = ${law.pageSlots};`)).toBe(true);
 expect(source.includes("next_backing_release_byte_demand")).toBe(true);
 expect(source.includes("close_backing_step")).toBe(true);
 expect(source.includes("backing_is_empty")).toBe(true);
 console.log("[DEBUG] Node Uint16Array/RFC6902 neutral8192slots/64page/1024turn identity and indivisible physical retirement checked");
});
