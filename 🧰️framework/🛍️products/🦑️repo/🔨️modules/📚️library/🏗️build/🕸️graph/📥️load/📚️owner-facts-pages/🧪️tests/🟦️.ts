/** 📚️ Contract: owner facts paged to a file come back equal to what was admitted, the resident facts stay within their budget, and a memo written from paged entries is byte-identical to one written from resident entries.
 * @see ../🟦️.ts */
import{expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{existsSync,mkdtempSync,readdirSync,readFileSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createOwnerFactsPagesV1,ownerFactsResidentCharactersV1,type PagedOwnerEntryV1}from"../🟦️.ts";
import{packAdmissionMemoV1,writeAdmissionMemoV1}from"../../🧠️admission-memo/🟦️.ts";

const entry=(index:number):PagedOwnerEntryV1=>({facts:{project:{name:"owner-"+index},text:"é😀\"\\".repeat(200+index),list:Array.from({length:50},(_,at)=>({at,index}))},rows:[{kind:"file",path:"/w/"+index,fact:{version:index}}]});
const directory=()=>mkdtempSync(join(tmpdir(),"owner-facts-pages-"));
const sha=(path:string)=>createHash("sha256").update(readFileSync(path)).digest("hex");

test("the resident character budget follows the heap budget: a share of it over the heap bytes one character costs",()=>{
 expect(ownerFactsResidentCharactersV1(2048,16,8)).toBe(16777216);expect(ownerFactsResidentCharactersV1(0,16,8)).toBe(1);
});

test("paged facts come back equal, the resident characters stay within the budget and the least recently used leave first",()=>{
 const root=directory(),pages=createOwnerFactsPagesV1({residentCharacters:30000,maximumPageBytes:1<<30,directory:root});
 try{
  const entries=Array.from({length:40},(_,at)=>entry(at)),expected=entries.map(each=>JSON.parse(JSON.stringify(each.facts)));
  for(const each of entries)pages.admit(each);
  expect(pages.stats().residentCharacters).toBeLessThanOrEqual(30000);expect(pages.stats().paged).toBe(40);
  expect(entries.slice(0,10).every(each=>each.facts===undefined)).toBe(true);expect(entries[39]!.facts).toBeDefined();
  entries.forEach((each,at)=>expect(pages.facts(each)).toEqual(expected[at]));
  expect(pages.stats().residentCharacters).toBeLessThanOrEqual(30000);
  const last=entries[39]!;expect(pages.facts(last)).toBe(last.facts);
 }finally{pages.close();rmSync(root,{recursive:true,force:true});}
});

test("the page file exists only while open and nothing is left behind after close",()=>{
 const root=directory(),pages=createOwnerFactsPagesV1({residentCharacters:1,maximumPageBytes:1<<30,directory:root});
 try{
  pages.admit(entry(1));pages.admit(entry(2));
  if(process.platform!=="win32")expect(readdirSync(root)).toEqual([]);
  pages.close();expect(readdirSync(root)).toEqual([]);
 }finally{pages.close();rmSync(root,{recursive:true,force:true});}
});

test("a page file that cannot be written or is full keeps the facts resident and loses nothing",()=>{
 const root=directory(),missing=createOwnerFactsPagesV1({residentCharacters:1,maximumPageBytes:1<<30,directory:join(root,"absent")}),small=createOwnerFactsPagesV1({residentCharacters:1,maximumPageBytes:10,directory:root});
 try{
  for(const pages of [missing,small]){const first=entry(1),second=entry(2),facts=JSON.parse(JSON.stringify(first.facts));pages.admit(first);pages.admit(second);expect(pages.facts(first)).toEqual(facts);expect(first.facts).toBeDefined();expect(second.facts).toBeDefined();}
  expect(existsSync(join(root,"absent"))).toBe(false);
 }finally{missing.close();small.close();rmSync(root,{recursive:true,force:true});}
});

test("a released entry is forgotten and its facts are not read again",()=>{
 const root=directory(),pages=createOwnerFactsPagesV1({residentCharacters:1,maximumPageBytes:1<<30,directory:root});
 try{const first=entry(1),second=entry(2);pages.admit(first);pages.admit(second);expect(first.facts).toBeUndefined();pages.release(first);expect(pages.facts(first)).toBeUndefined();}finally{pages.close();rmSync(root,{recursive:true,force:true});}
});

test("a memo written from paged entries is byte-identical to the memo written from resident entries",async()=>{
 const root=directory(),pages=createOwnerFactsPagesV1({residentCharacters:5000,maximumPageBytes:1<<30,directory:root});
 try{
  const resident=Array.from({length:30},(_,at)=>entry(at)),paged=Array.from({length:30},(_,at)=>entry(at));
  for(const each of paged)pages.admit(each);
  expect(paged.filter(each=>each.facts===undefined).length).toBeGreaterThan(20);
  const owners=(from:PagedOwnerEntryV1[],lazy:boolean)=>from.map((each,at)=>({suffix:"owner-"+at+"\0[[],[]]",...lazy?{get facts(){return pages.facts(each);}}:{facts:each.facts},rows:each.rows}));
  const admission={owners:[],files:[],targets:{},inputs:{},catalogProviders:[]},format="a".repeat(64),scope="s".repeat(8);
  const left=join(root,"resident.json"),right=join(root,"paged.json");
  await writeAdmissionMemoV1(left,packAdmissionMemoV1({format,scope,admission,observations:[],owners:owners(resident,false)}),1<<30);
  await writeAdmissionMemoV1(right,packAdmissionMemoV1({format,scope,admission,observations:[],owners:owners(paged,true)}),1<<30);
  expect(sha(right)).toBe(sha(left));
 }finally{pages.close();rmSync(root,{recursive:true,force:true});}
});
