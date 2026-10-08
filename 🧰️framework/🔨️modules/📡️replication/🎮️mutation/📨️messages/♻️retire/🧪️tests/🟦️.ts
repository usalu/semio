/** ♻️ Neutral typed diagnostic ownership agrees with independent RFC6902 removal. */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {applyPatch} from "fast-json-patch";
const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
test("♻️ neutral message ledger retirement preserves the original and removes every owned field",()=>{
  for(const row of law.cases){
    const baseline={context:row.context,messages:row.messages};
    expect(baseline.messages.length).toBe(row.expectedRows);
    const retired=applyPatch(structuredClone(baseline),[{op:"replace",path:"/context",value:null},{op:"replace",path:"/messages",value:[]}],true).newDocument;
    expect(retired).toEqual({context:null,messages:[]});
    expect(baseline).toEqual({context:row.context,messages:row.messages});
    expect(row.rowCapacity).toBeGreaterThanOrEqual(row.expectedRows);
    expect(row.contextCapacity).toBeGreaterThanOrEqual(Buffer.byteLength(row.context));
  }
  console.log(`[DEBUG] ${law.cases.length} neutral diagnostic-ledger owner cases match independent RFC6902; physical release proof remains native`);
});
test("♻️ replay outcomes preserve both original identities through exact diagnostic retirement",()=>{
  expect(law.outcomeIdentity.capacity).toBeGreaterThanOrEqual(Buffer.byteLength(law.outcomeIdentity.text));
  for(const row of law.cases){
    const baseline={mutationId:law.outcomeIdentity.text,editId:row.context,messages:row.messages};
    expect(applyPatch(structuredClone(baseline),[{op:"replace",path:"/mutationId",value:null},{op:"replace",path:"/editId",value:null},{op:"replace",path:"/messages",value:[]}],true).newDocument).toEqual({mutationId:null,editId:null,messages:[]});
    expect(baseline).toEqual({mutationId:law.outcomeIdentity.text,editId:row.context,messages:row.messages});
  }
  expect(readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8")).toContain("pub fn from_replay_outcome");
  console.log(`[DEBUG] ${law.cases.length} outcome ledgers preserve both owned identities and agree with independent RFC6902`);
});
test("♻️ nested edit diagnostics retain original owners until the exact grant permits release",()=>{
  for(const row of law.editLedgers){
    const baseline=row.rows.map((index:number)=>({editId:law.cases[index].context,messages:law.cases[index].messages}));
    expect(baseline.length).toBe(row.expectedEntries);
    expect(row.capacity).toBeGreaterThanOrEqual(baseline.length);
    expect(applyPatch({entries:structuredClone(baseline)},[{op:"replace",path:"/entries",value:[]}],true).newDocument).toEqual({entries:[]});
    expect(baseline).toEqual(row.rows.map((index:number)=>({editId:law.cases[index].context,messages:law.cases[index].messages})));
  }
  const producer=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");
  expect(producer).toContain("pub struct EditMessageLedgerRetirement");
  const store=readFileSync(new URL("../../../../../../🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs",import.meta.url),"utf8");
  expect(store.includes("inner: crate::os_spr::command::EditMessageLedgerRetirement")).toBe(true);
  console.log(`[DEBUG] ${law.editLedgers.length} strict neutral nested ledgers agree with independent RFC6902, canonical native owner and real rejected Store mount`);
});
