/** 🔧️ Contract: an edit of a tooling file that the declared protocol covers (an inferrer's sources or infrastructure) moves no record: the classified rows of an implementation, and with them every record key, depend on the stamps alone; only an edit of a file nobody covers, or a bump of a stamp on purpose, does (200 seeded worlds over the declared protocol and the real classifier). */
import{test,expect}from"bun:test";
import schema from "../../🧬️schema/🔣️.json";
import fixture from "../🔣️.json";
import{classifyImplementationRowsV1}from"../../🟨️.mjs";

const protocol=schema.definitions.Protocol.const as never as{inferrers:Record<string,{stamp:string;scope:string;sources:string[]}>;derived:string[]};
const generator=(seed:number)=>()=>{seed|=0;seed=seed+0x6D2B79F5|0;let t=Math.imul(seed^seed>>>15,1|seed);t=t+Math.imul(t^t>>>7,61|t)^t;return((t^t>>>14)>>>0)/4294967296;};
const digest=(pick:()=>number)=>Array.from({length:64},()=>"0123456789abcdef"[Math.floor(pick()*16)]).join("");
const rowsOf=(pick:()=>number)=>fixture.rows.map(row=>({kind:"digest",path:row.path,fact:{sha256:digest(pick),size:1,mtimeMs:1}}));
const effective=new Map(protocol.derived.map((id,index)=>[id,protocol.inferrers[id]!.stamp+"+"+String(index+1).repeat(32)] as [string,string]));
const classify=(rows:unknown[],stamps=effective)=>JSON.stringify(classifyImplementationRowsV1(rows as never,protocol as never,stamps));

test("an edit of a covered tooling file moves no record, an edit of an uncovered file or a stamp bump does (200 seeded worlds)",()=>{
 const dropped=fixture.rows.map((row,index)=>[row,index] as const).filter(([row])=>row.cls==="drop").map(([,index])=>index),kept=fixture.rows.map((row,index)=>[row,index] as const).filter(([row])=>row.cls==="keep").map(([,index])=>index);
 expect(dropped.length).toBeGreaterThan(3);expect(kept.length).toBeGreaterThan(1);
 const ids=Object.keys(protocol.inferrers).filter(id=>!protocol.derived.includes(id));
 for(let seed=1;seed<=200;seed++){
  const pick=generator(seed),rows=rowsOf(pick),before=classify(rows);
  const edited=rows.map(row=>({...row,fact:{...row.fact}})),victim=dropped[Math.floor(pick()*dropped.length)]!;
  edited[victim]!.fact.sha256=digest(pick);
  expect(classify(edited),"world "+seed+" covered edit of "+fixture.rows[victim]!.path).toBe(before);
  const data=kept[Math.floor(pick()*kept.length)]!,changed=rows.map(row=>({...row,fact:{...row.fact}}));changed[data]!.fact.sha256=digest(pick);
  expect(classify(changed),"world "+seed+" uncovered edit of "+fixture.rows[data]!.path).not.toBe(before);
  const id=ids[Math.floor(pick()*ids.length)]!,bumped=new Map([...effective,[id,protocol.inferrers[id]!.stamp+"+"+seed]] as [string,string][]);
  const after=JSON.parse(classify(rows,bumped)) as{kind:string;path:string;fact:string}[],reference=JSON.parse(before) as typeof after;
  const differing=after.filter((row,index)=>JSON.stringify(row)!==JSON.stringify(reference[index]));
  expect(differing.map(row=>row.path),"world "+seed+" stamp bump").toEqual(["inferrer:"+id]);
 }
});
