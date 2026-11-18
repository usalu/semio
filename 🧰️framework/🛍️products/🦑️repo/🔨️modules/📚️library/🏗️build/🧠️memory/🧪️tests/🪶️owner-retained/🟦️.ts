/** 🪶️ Contract: what the parent of a read keeps for a finished owner is small and flat in the number of owners. A finished owner is its project record (about 80 KB of JSON, three fourths of it the `inputs` lists whose project names repeat in every target of every owner); the parent keeps the record with its long strings shared (`internJsonStringsV1`), so 200 finished owners hold a few kilobytes each beyond the distinct names instead of the 150 KB a freshly parsed copy takes. Measured on the 205 stored records of the real store (17 MB of JSON for 1001 targets, 3505 distinct strings).
 * @see ../../🟨️.mjs */
import {expect,test} from "bun:test";
import {heapStats} from "bun:jsc";
import {createStringInternerV1,internJsonStringsV1} from "../../🟨️.mjs";

const OWNERS=200,TARGETS=5,NAMES=400;
const names=Array.from({length:NAMES},(_,index)=>`@semio-tech/project-number-${String(index).padStart(4,"0")}-rs`);
const record=(owner:number)=>({name:`owner-${owner}`,root:`owner-${owner}`,targets:Object.fromEntries(Array.from({length:TARGETS},(_,target)=>[`target-${target}`,{executor:"nx:run-commands",inputs:["nativeSources",{input:"nativeSources",projects:names}],options:{command:`cargo test --manifest-path owner-${owner}/Cargo.toml`}}]))});
const text=(owner:number)=>JSON.stringify(record(owner));
const settled=()=>{Bun.gc(true);Bun.gc(true);return heapStats().heapSize;};
const retained=(share:boolean)=>{
 const interner=createStringInternerV1(),texts=Array.from({length:OWNERS},(_,owner)=>text(owner)),keep:unknown[]=[],before=settled();
 for(const each of texts){const value=JSON.parse(each);keep.push(share?internJsonStringsV1(value,interner):value);}
 const after=settled();
 return {perOwner:(after-before)/OWNERS,kept:keep.length};
};

test("200 finished owners keep their records with shared strings: under 48 KB each, at most a third of a freshly parsed copy",()=>{
 const raw=retained(false),shared=retained(true);
 expect(shared.kept).toBe(OWNERS);
 expect(shared.perOwner,"bytes per finished owner with shared strings").toBeLessThan(48*1024);
 expect(shared.perOwner*3,"against the freshly parsed copy").toBeLessThan(raw.perOwner);
});

test("the retained bytes per owner do not grow with the number of owners: the second hundred costs what the first hundred cost",()=>{
 const interner=createStringInternerV1(),keep:unknown[]=[],texts=Array.from({length:OWNERS},(_,owner)=>text(owner));
 const lap=(from:number,to:number)=>{const before=settled();for(let owner=from;owner<to;owner++)keep.push(internJsonStringsV1(JSON.parse(texts[owner]!),interner));return (settled()-before)/(to-from);};
 const first=lap(0,100),second=lap(100,200);
 expect(second,"second hundred against first hundred").toBeLessThan(first*1.25+1024);
});
