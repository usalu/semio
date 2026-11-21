/** 🌱️ A scoped infrastructure crossing cannot consume its unified semantic total. */
import coreWireBirth from "./core-wire-birth.json";
import uiFunding from "./ui-funded.json";
import uiFundingSchema from "./ui-funded.schema.json";
import{test,expect}from"bun:test";import{Database}from"bun:sqlite";import Ajv from"ajv";import fixture from"./scoped-budget.json";import schema from"./scoped-budget.schema.json";
test("scoped actor admission keeps total currency and gates bootstrap",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{for(const row of fixture.cases){const result=db.query("SELECT (?<=? AND ?<=? AND NOT ?) AS accepted,(?>0 AND ?>? AND ?<=? AND ?<=? AND NOT ?) AS mayAdvance").get(row.fuel,fixture.totalFuel,row.deadline,fixture.totalDeadline,Number(row.cancelled),row.fuel,row.deadline,fixture.now,row.fuel,fixture.totalFuel,row.deadline,fixture.totalDeadline,Number(row.cancelled))as{accepted:number,mayAdvance:number};expect(result).toEqual({accepted:Number(row.accepted),mayAdvance:Number(row.mayAdvance)});}}finally{db.close()}});
test("first Core-wire borrower has funded publisher and receipt capacity",()=>{
 const {now,deadline,fuel,grant,expected}=coreWireBirth;
 const db=new Database(":memory:");
 try{
  const actual=db.query("SELECT (?>? AND ?>0 AND ?>0 AND ?>0 AND ?>0) AS publisherReadyOnFirstBorrow, (?>0 AND ?>0 AND ?>0) AS receiptHasPhysicalEffects").get(deadline,now,fuel,grant.maximumItems,grant.maximumCopyBytes,grant.maximumDepth,grant.maximumCapacityBytes,grant.maximumReleaseBytes,fuel) as Record<string,number>;
  expect(Object.fromEntries(Object.entries(actual).map(([key,value])=>[key,Boolean(value)]))).toEqual(expected);
  const deltas=db.query<{bytes:number},[string]>(`
    WITH totals AS (SELECT key, sum(value) OVER (ORDER BY key) AS bytes FROM json_each(?1))
    SELECT bytes-lag(bytes,1,0) OVER (ORDER BY key) AS bytes FROM totals
  `).all(JSON.stringify(coreWireBirth.dispatchCopyBytes));
  expect(deltas.map(row=>row.bytes)).toEqual(coreWireBirth.dispatchCopyBytes);
 }finally{db.close()}
});

test("funded UI admission retains the exact first unaffordable or stopped row",()=>{
 expect(new Ajv({strict:true}).compile(uiFundingSchema)(uiFunding)).toBe(true);
 const db=new Database(":memory:");
 try{
  db.run("CREATE TABLE costs (ordinal INTEGER PRIMARY KEY, work INTEGER NOT NULL)");
  uiFunding.rows.forEach((work,ordinal)=>db.run("INSERT INTO costs VALUES (?, ?)",[ordinal,work]));
  const prefix=db.query<{next:number,spent:number},[number,number,number]>(`
   WITH scoped AS (
    SELECT ordinal, work, sum(work) OVER (ORDER BY ordinal) AS spent
    FROM costs WHERE ordinal >= ?1
   ), boundary AS (
    SELECT ordinal FROM scoped WHERE work = 0 OR spent > ?2 OR ordinal >= ?3 ORDER BY ordinal LIMIT 1
   )
   SELECT coalesce((SELECT ordinal FROM boundary), (SELECT count(*) FROM costs)) AS next,
          coalesce((SELECT sum(work) FROM scoped WHERE ordinal < (SELECT ordinal FROM boundary)),
                   (SELECT sum(work) FROM scoped WHERE NOT EXISTS (SELECT 1 FROM boundary)), 0) AS spent
  `);
  for(const row of uiFunding.cases)expect(prefix.get(row.start,row.work,row.stopAt)).toEqual({next:row.next,spent:row.spent});
 }finally{db.close();}
});
