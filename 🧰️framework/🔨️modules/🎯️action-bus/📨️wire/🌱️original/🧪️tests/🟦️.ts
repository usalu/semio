import {test,expect} from "bun:test";
import {createHash} from "node:crypto";
import law from "./🔣️.json";

test("original command wire pages agree with independent byte extents and SHA256",()=>{
 for(const row of law.cases){
  const bytes=Buffer.from(Array.from({length:row.bytes},(_,i)=>(i*law.pattern.multiplier+law.pattern.offset)%law.pattern.modulus));
  const pages=[];for(let offset=0;offset<bytes.length;offset+=law.pageBytes)pages.push(bytes.subarray(offset,offset+law.pageBytes));
  expect(pages.length).toBe(row.pages);
  expect(createHash("sha256").update(Buffer.concat(pages)).digest("hex")).toBe(row.sha256);
  for(const cut of law.cancelAfterPages){expect(pages.slice(0,cut).reduce((bytes,page)=>bytes+page.length,0)).toBe(Math.min(row.bytes,cut*law.pageBytes));}
 }
});
