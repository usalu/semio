import {expect,test} from "bun:test";
import Ajv from "ajv";
import sumBy from "lodash/sumBy";
import zipWith from "lodash/zipWith";
import {Database} from "bun:sqlite";
import corpus from "./🧫️fixtures/🔣️.json";
import schema from "./🧫️fixtures/🧬️.json";
test("receiving outcome keeps attempted work separate from physical items and owns every cause",()=>{
 expect(new Ajv({strict:false}).compile(schema)(corpus)).toBe(true);const db=new Database(":memory:");
 try{expect(sumBy(corpus.turns,"work")).toBe(corpus.expectedWork);expect(db.query("select sum(json_extract(value,'$.work')) as work,sum(json_extract(value,'$.items')) as items from json_each(?)").get(JSON.stringify(corpus.turns))).toEqual({work:3,items:0});}finally{db.close();}
 let held:string[]=[];for(const turn of corpus.turns){if(turn.capture){expect(held).toEqual([]);held=[...corpus.causes];}expect(held).toEqual(turn.expected.causes);}expect(held).toEqual(corpus.causes);
});
test("actual typed receiving ports forward outcomes without selecting one owned refusal",async()=>{
 const receiving=await Bun.file(new URL("../../🦀️.rs",import.meta.url)).text();expect(receiving).not.toContain("fn flat(");expect(receiving).toContain("RetainedWorkOutcome<RetainedCloneProgress");
 const pair=await Bun.file(new URL("../🦀️.rs",import.meta.url)).text();expect(pair).toContain("Native(OriginalReceivingPrimary)");expect(pair).toContain("RetainedWorkOutcome<RetainedCloneProgress,OriginalArtifactPairFailure>");
});

test("actual Window capture prequotes a vacant slot and retires its prior failure before another producer",async()=>{
 const source=await Bun.file(new URL("../../../../../../🔌️plugin/🪟️window/🎚️config/📥️retained/📦️typed-pack-load/🛬️native/🦀️.rs",import.meta.url)).text();
 expect(source).toContain("fn pair_failure_demand()");expect(source).toContain("pair_child_grant(grant)");expect(source).toContain("slot.is_some()||!crate::registry::fits");expect(source).toContain("assert!(slot.is_none()");expect(source).toContain("if !self.close_pair_failure(cx)?{}");expect(source).not.toContain("cx.consume_fuel(p.copied_items as u64)");
 const capture=source.slice(source.indexOf("fn pair_posted_error("),source.indexOf("fn pair_demand("));expect(capture).not.toContain(".or(");expect(capture).toContain("failure.progress=combined;failure");
});

test("caller restrictions and genuine source authority intersect every physical currency",async()=>{
 const db=new Database(":memory:");try{for(const row of corpus.grantCases){const projected=row.caller.map((value,index)=>Math.min(value,row.source[index]));expect(projected).toEqual(row.expected);expect(zipWith(row.caller,row.source,Math.min)).toEqual(row.expected);const oracle=db.query("select min(a.value,b.value) as allowed from json_each(?) a join json_each(?) b on a.key=b.key order by a.key").all(JSON.stringify(row.caller),JSON.stringify(row.source))as{allowed:number}[];expect(oracle.map(value=>value.allowed)).toEqual(row.expected);}}finally{db.close();}
 const source=await Bun.file(new URL("../../../../../../../../../🔨️modules/🌱️value/🎟️control/🧬️contract/🦀️.rs",import.meta.url)).text();expect(source.includes("pub fn restricted_by")).toBe(true);
});
