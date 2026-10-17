import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {existsSync,readFileSync} from "node:fs";
import schema from "../🧬️schema/🔣️.json";
import law from "./🧫️fixtures/🔣️.json";
test("original document viewer and composition records receive typed fields from sealed pages",()=>{
 expect(new Ajv({strict:false}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");
 try{for(const row of law.records){
  const source=Buffer.from(row.wireHex,"hex");let at=0;
  const scalar=()=>{let value=0;for(let shift=0;shift<70;shift+=7){const byte=source[at++];value+=(byte&127)*2**shift;if(!(byte&128))return value;}throw Error("scalar")};
  const id=()=>{expect(source[at++]).toBe(0);const size=scalar();const value=source.subarray(at,at+size).toString("utf8");at+=size;return value};
  const triple=()=>[id(),id(),id()];expect(source[at++]).toBe(1);
  let actual:unknown;
  if(row.kind==="document")actual={docId:id(),schema:id()};
  else{const presence=source[at++];actual=row.kind==="viewer"?{line:presence&1?id():null,checkpoint:presence&2?id():null}:{owner:presence&1?triple():null,dialect:presence&2?triple():null};}
  expect(actual).toEqual(row.expected);expect(at).toBe(source.length);
  expect(db.query("select json(?) as value").get(JSON.stringify(actual))).toEqual({value:JSON.stringify(row.expected)});
 }}finally{db.close()}
 const path=new URL("../🦀️.rs",import.meta.url);expect(existsSync(path)).toBe(true);if(!existsSync(path))return;
 const source=readFileSync(path,"utf8");
 for(const proof of["pub struct RetainedHistoryIdentityRecordDecode","HistoryReadLimits","OriginalWireSourceIdentity","RetainedHistoryIdDecode","pub fn take_ready_into","pub fn take_partial_into","control.charge"]){expect(source.includes(proof)).toBe(true)}
 for(const cold of["decode_doc(","decode_viewer(","decode_composition(","source.to_vec()","HistoryReadLimits::default"]){expect(source.includes(cold)).toBe(false)}
});
