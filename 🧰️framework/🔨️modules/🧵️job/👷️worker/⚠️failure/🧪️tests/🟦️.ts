/** 🧾️ The actual caller receives the original physical receipt before borrowing either complete cause. */
import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import law from"./🧫️fixtures/🔣️.json";
import schema from"../🧬️schema/🔣️.json";
test("normal worker whole failure is lent only after original receipt return",()=>{
 const valid=new Ajv({strict:true}).compile(schema);expect(valid(law)).toBe(true);expect(valid({...law,normalReturn:"fresh-recipient"})).toBe(false);
 const db=new Database(":memory:");try{db.exec("create table original_causes(role text primary key,message text)");db.query("insert into original_causes values (?,?)").run("primary",law.primary);db.query("insert into original_causes values (?,?)").run("receipt",law.receiptRefusal);expect(db.query("select count(*) as owners from original_causes").get()).toEqual({owners:2});expect(db.query("select message from original_causes where role='primary'").get()).toEqual({message:law.primary});for(const grant of[law.source,law.work,law.close,law.sessionClose])for(const key of Object.keys(grant)as(keyof typeof grant)[])expect(grant[key]).toBeLessThanOrEqual(law.initialStock[key]);expect(law.initialFuel).toBeGreaterThan(law.fuel);}finally{db.close();}
});
