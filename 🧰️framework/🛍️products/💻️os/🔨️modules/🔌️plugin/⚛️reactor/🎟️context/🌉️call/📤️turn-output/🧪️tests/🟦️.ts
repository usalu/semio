import{test,expect}from "bun:test";
import Ajv from "ajv";
import{Database}from "bun:sqlite";
/** 🪢️ The independent event projection retains a published result through denial until funded closure. */
test("resident turn output requires explicit funded retirement after publication",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🔣️.json",import.meta.url)).json(),schema=await Bun.file(new URL("../🧬️schema/🔣️.json",import.meta.url)).json();
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE events (ordinal INTEGER,event TEXT)");for(const [index,row]of law.states.entries()){db.query("INSERT INTO events VALUES(?,?)").run(index,row.event);expect((db.query("SELECT CASE event WHEN 'fundedClose' THEN 0 ELSE 1 END AS held FROM events ORDER BY ordinal DESC LIMIT 1").get()as{held:number}).held===1).toBe(row.held);}expect(JSON.parse(JSON.stringify(law.result))).toEqual(law.result);}finally{db.close()}
});
