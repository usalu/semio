import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv/dist/2020.js";
import schema from"../🧬️schema/🔣️.json";
import fixture from"./🧫️fixtures/🔣️.json";
import{PersistentListBuilder}from"../🟦️.ts";
test("owned persistent construction preserves original rows across every fanout carry boundary",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");
 for(const row of fixture.cases){const source=Array.from({length:row.length},(_,i)=>({rank:i*fixture.stride,label:fixture.prefix+i}));const builder=new PersistentListBuilder<(typeof source)[number]>(source.length);
 for(const original of source){builder.adopt(original);while(!builder.acceptsRow)builder.advance();}
 builder.finish();while(!builder.ready)builder.advance();const root=builder.take();
 expect(root.length).toBe(row.length);expect(root.height).toBe(row.height);expect(Array.from(root)).toEqual(source);for(let i=0;i<source.length;i++)expect(root.get(i)).toBe(source[i]);
 const oracle=db.query("WITH RECURSIVE rows(i) AS(SELECT 0 WHERE ?>0 UNION ALL SELECT i+1 FROM rows WHERE i+1<?) SELECT json_group_array(json_object('rank',i*?,'label',?||i)) AS json FROM rows").get(row.length,row.length,fixture.stride,fixture.prefix)as{json:string};expect(JSON.parse(oracle.json)).toEqual(Array.from(root));builder.close();expect(builder.empty).toBe(true);root.close();
 }
 db.close();console.log("[DEBUG] nine persistent construction cardinalities and exact heights match SQLite rows; original model identities remain intact, native physical grants/receipts are separate");
});
test("owned persistent builder refuses missing and excess original rows",()=>{
 const missing=new PersistentListBuilder<number>(1);expect(()=>missing.finish()).toThrow();missing.close();expect(missing.empty).toBe(true);
 const excess=new PersistentListBuilder<number>(0);expect(()=>excess.adopt(1)).toThrow();excess.close();expect(excess.empty).toBe(true);
});
test("original partial persistent pages close at every authored receiving frontier",()=>{
 for(const row of fixture.cases)for(const cut of fixture.cuts){const builder=new PersistentListBuilder<object>(row.length);let adopted=0;while(adopted<row.length&&adopted<cut){builder.adopt({rank:adopted++});while(!builder.acceptsRow)builder.advance();}if(adopted===row.length){builder.finish();if(cut>row.length)while(!builder.ready)builder.advance();}builder.close();expect(builder.empty).toBe(true);}
 console.log("[DEBUG] all45 neutral partial construction frontiers close; physical retirement is independently native");
});
test("every leaf birth, page carry, partial branch and ready-root frontier cancels",()=>{
 for(const cut of fixture.preparation.cuts){const builder=new PersistentListBuilder<object>(fixture.preparation.length);let supplied=0,finished=false;for(let turn=0;turn<cut&&!builder.ready;turn++){if(builder.acceptsRow&&supplied<fixture.preparation.length)builder.adopt({rank:supplied++});else if(builder.acceptsRow&&!finished){builder.finish();finished=true;}else builder.advance();}builder.close();expect(builder.empty).toBe(true);}
 console.log("[DEBUG] all"+fixture.preparation.cuts.length+" authored construction action frontiers cancel across leaf/carry/branch/finish/ready phases");
});
