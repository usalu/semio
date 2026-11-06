/** 📤️ Independent original-source publication retains every paid stage and unchanged native source. */
import{test,expect}from"bun:test";import Ajv from"ajv";import{Database}from"bun:sqlite";import{existsSync}from"node:fs";
import { orderedRetirementSchema as domain } from "@semio-tech/value-ts/ordered/retirement/schema";
test("original job publication borrows the same source through bounded build loan and paid closure",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🔣️.json",import.meta.url)).json();const [maximumItems,maximumCopyBytes,maximumCapacityBytes,maximumReleaseBytes,maximumDepth]=law.originalGrant,grant={maximumItems,maximumCopyBytes,maximumCapacityBytes,maximumReleaseBytes,maximumDepth};const admit=new Ajv({strict:true}).addSchema(domain).compile({$ref:domain.$id+"#/$defs/Grant"});expect(admit(grant)).toBe(true);expect(admit({...grant,maximumCopyBytes:-1})).toBe(false);
 const db=new Database(":memory:");try{db.run("CREATE TABLE original_copy(source INTEGER,sequence INTEGER,bytes INTEGER)");for(const source of law.sourceBytes){let copied=0,sequence=0;while(copied<source){const bytes=Math.min(law.originalGrant[1],source-copied);db.query("INSERT INTO original_copy VALUES(?,?,?)").run(source,sequence++,bytes);copied+=bytes;}expect(db.query("SELECT coalesce(sum(bytes),0) AS bytes FROM original_copy WHERE source=?").get(source)).toEqual({bytes:source});}expect(db.query("SELECT count(*) AS n FROM original_copy WHERE bytes>?").get(law.originalGrant[1])).toEqual({n:0});expect(db.query("SELECT count(*) AS turns FROM original_copy WHERE source=321").get()).toEqual({turns:19});}finally{db.close();}
});

test("original publication close work has an independent finite-budget oracle",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🔣️.json",import.meta.url)).json(),schema=await Bun.file(new URL("🧬️schema/🔣️.json",import.meta.url)).json();
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");try{
  for(const row of law.closeBudgets){const result=db.query("SELECT ? >= ? AS admitted, CASE WHEN ? >= ? THEN ? ELSE 0 END AS spent").get(row.budget,row.demand,row.budget,row.demand,row.demand);expect(result).toEqual({admitted:Number(row.admitted),spent:row.admitted?row.demand:0});}
  expect(db.query("SELECT ? < ? AS denied").get(law.closeWork.deniedBudget,law.closeWork.live)).toEqual({denied:1});expect(law.closeWork.empty).toBe(0);
 }finally{db.close();}
});

test("original publication and scalar examples cannot own whole-trial contracts",()=>{expect(["../🧬️schema/🔣️.json","../../../../🌱️value/♻️retirement/🍃️leaf/🧬️schema/🔣️.json"].map(path=>existsSync(new URL(path,import.meta.url)))).toEqual([false,false]);});
