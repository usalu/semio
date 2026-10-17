import{test,expect}from"bun:test";
import Ajv from"ajv/dist/2020.js";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
import law from"./🧫️fixtures/🔣️.json";
import schema from"../🧬️schema/🔣️.json";
test("scoped original parent returns a Weak metadata alias without an allocation or context birth",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);expect(validate({...law,aliasOrContextBirth:true})).toBe(false);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE weak_return(rank INTEGER,name TEXT,returned INTEGER,items INTEGER,born INTEGER,freed INTEGER)");law.events.forEach((event,rank)=>db.query("INSERT INTO weak_return VALUES(?,?,?,?,?,?)").run(rank,event.name,Number(event.returned),event.items,event.born,event.freed));expect(db.query("SELECT sum(items) AS items,sum(born) AS born,sum(freed) AS freed FROM weak_return").get()).toEqual({items:1,born:0,freed:0});expect(db.query("SELECT name FROM weak_return WHERE returned=1").get()).toEqual({name:"originalParent"});}finally{db.close()}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");for(const symbol of["with_original_native_weak_parent","return_original_native_weak","OriginalNativeWeakLoan","OriginalRetainedLoanIdentity","TypeId::of::<T>()"])expect(source).toContain(symbol);for(const forbidden of["Arc::clone","Arc::downgrade","StepContext::new","CancelToken::root_now","default_now_us"])expect(source).not.toContain(forbidden);
});
