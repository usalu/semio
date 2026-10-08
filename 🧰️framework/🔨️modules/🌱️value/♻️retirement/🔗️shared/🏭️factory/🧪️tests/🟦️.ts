/** 🔐️ Original alias and cancellation custody agree with independent JSON ownership transitions. */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {applyPatch} from "fast-json-patch";
import "./🔐️custody/🟦️.ts";
const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
test("🔐️ shared factory metadata never enlarges a narrow payload copy grant",()=>{
  expect(law.ownershipMoves).toEqual({items:1,copyBytes:0});
  for(const text of law.texts)for(const aliases of law.aliases)for(const copy of law.copyBytes)for(const cancel of law.cancelAt){
    const baseline={snapshot:text,aliases,factory:"original",copy,cancel},pending=structuredClone(baseline);expect(pending).toEqual(baseline);
    for(let index=0;index<aliases;index++)applyPatch(pending,[{op:"replace",path:"/aliases",value:aliases-index-1}],true);
    expect(pending.snapshot).toBe(baseline.snapshot);expect(pending.factory).toBe("original");expect(applyPatch(pending,[{op:"replace",path:"/snapshot",value:null},{op:"replace",path:"/factory",value:null}],true).newDocument).toEqual({...baseline,snapshot:null,aliases:0,factory:null});expect(baseline.snapshot).toBe(text);
  }
  const shared=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8"),store=readFileSync(new URL("../../../../../../🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs",import.meta.url),"utf8");
  expect(shared.includes('pub use factory::FactorySharedRetirement;')).toBe(true);
  expect(store.includes('use semio_framework_value::retirement::shared::FactorySharedRetirement as ReturnedSnapshotReadRetirement;')).toBe(true);
  console.log(`[DEBUG] neutral aliases ${law.aliases}/copy ${law.copyBytes}/cancel ${law.cancelAt}, independent RFC6902 and actual Store shared factory mount agree`);
});
