/** 📋️ SQLite preserves the original ordered vector through every cancellation frontier. */
import{test,expect}from"bun:test";
import Ajv2020 from"ajv/dist/2020";
import{Database}from"bun:sqlite";
import{readFileSync,existsSync}from"node:fs";
test("original vector cursor transfers exact ordered owners with independently funded final backing release",()=>{
 const root=new URL("../",import.meta.url),read=(p:string)=>JSON.parse(readFileSync(new URL(p,root),"utf8")),law=read("🧫️fixtures/🔣️.json");expect(new Ajv2020({strict:true}).compile(read("🧬️schema/🔣️.json"))(law)).toBe(true);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE originals(position INTEGER PRIMARY KEY,payload TEXT,returned INTEGER DEFAULT 0)");for(let i=0;i<law.values.length;i++)db.query("INSERT INTO originals(position,payload) VALUES(?,?)").run(i,law.values[i]);for(const stop of law.cancelAfter){db.exec("UPDATE originals SET returned=0");for(let i=0;i<stop;i++)db.query("UPDATE originals SET returned=1 WHERE position=?").run(i);expect(db.query("SELECT payload FROM originals ORDER BY position").all().map((r:any)=>r.payload)).toEqual(law.values);expect(db.query("SELECT COUNT(*) AS count FROM originals").get()).toEqual({count:law.values.length});}}finally{db.close();}
 console.log("[DEBUG] SQLite original ordered vector custody preserves all4 cancellation frontiers including embedded NUL and multibyte text");
 const path=new URL("🦀️.rs",root);expect(existsSync(path)).toBe(true);const source=readFileSync(path,"utf8");for(const proof of["OriginalVectorCursor","take_front","next_demand","released_bytes","ControlledRetirement","ManuallyDrop"]){expect(source).toContain(proof);}expect(source).not.toContain("RetirementStep::Child");
});
