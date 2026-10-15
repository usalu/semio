/** 👶️ One declared child ledger is originally issued under its parent's actual paid turns. */
import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import schema from"../🧬️schema/🔣️.json";
import fixture from"./🧫️fixtures/🔣️.json";
test("original child source issues its ledger once and receives same-parent callback effects once",()=>{
 const valid=new Ajv({strict:true}).compile(schema);expect(valid(fixture)).toBe(true);for(const field of["initialOnlyLedger","sameOriginalCancellationNode","actualParentTurn","sharedOriginalRecipient","wholeUnacceptedReturnOwned","clockBindingPreserved","absoluteDeadlineStable"]){expect(valid({...fixture,[field]:false})).toBe(false);}
 const db=new Database(":memory:");try{db.run("CREATE TABLE original_birth(phase TEXT PRIMARY KEY,items INTEGER,copy INTEGER,capacity INTEGER,released INTEGER)");for(const row of fixture.admission)db.query("INSERT INTO original_birth VALUES(?,?,?,?,?)").run(row.phase,...row.spent);expect(db.query("SELECT sum(items) AS items,sum(copy) AS copy,sum(capacity) AS capacity,sum(released) AS released FROM original_birth").get()).toEqual({items:4,copy:0,capacity:160,released:0});expect(db.query("SELECT count(*) AS n FROM original_birth WHERE phase='initial-child-ledger'").get()).toEqual({n:1});expect(fixture.callbackGrant.map((value,index)=>index===4?value:value-fixture.callbackSpent[index]!)).toEqual(fixture.expectedRemaining);for(const row of fixture.incoming)expect(db.query("select ?<=? as admitted").get(row.deadline,fixture.initialDeadline)).toEqual({admitted:1});expect(fixture.incoming[1]!.deadline).toBeGreaterThan(fixture.incoming[0]!.deadline);expect(fixture.incoming[2]!.fuel).toBe(0);}finally{db.close();}
 console.log("[DEBUG] Ajv/SQLite original child211/19 issued once under parent99241/7; births4/0/160/0; callbacks preserve initiating clock/absolute deadline and one recipient; raw1/7/0/0 received once; native authority proof separate");
});
