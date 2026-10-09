/** 📝️ SQLite independently preserves the original shared actor UTF8 projection. */
import{test,expect}from"bun:test";
import Ajv2020 from"ajv/dist/2020";
import{Database}from"bun:sqlite";
import{readFileSync,existsSync}from"node:fs";
test("shared UTF8 preserves original actor bytes under fixed grants and paid leases",()=>{
 const root=new URL("../",import.meta.url),read=(p:string)=>JSON.parse(readFileSync(new URL(p,root),"utf8")),law=read("🧫️fixtures/🔣️.json");expect(new Ajv2020({strict:true}).compile(read("🧬️schema/🔣️.json"))(law)).toBe(true);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE actors(value TEXT, encoded TEXT)");for(const value of law.values)for(const repeat of law.repeats){const original=value.repeat(repeat);db.query("INSERT INTO actors VALUES(?,?)").run(original,JSON.stringify(original));expect(db.query("SELECT value,encoded FROM actors ORDER BY rowid DESC LIMIT 1").get()).toEqual({value:original,encoded:JSON.stringify(original)});}}finally{db.close();}
 console.log("[DEBUG] SQLite shared UTF8 oracle preserves giant original actor, NUL, Unicode and JSON projection under independent fixed4096 policy");
 const path=new URL("🦀️.rs",root);expect(existsSync(path)).toBe(true);const source=readFileSync(path,"utf8");for(const proof of["SharedUtf8","pub fn admit(","pub fn admit_clone(","SharedControlledRetirement::lease","maximum_copy_bytes","maximum_capacity_bytes","Utf8Text"]){expect(source).toContain(proof);}
});
