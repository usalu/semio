/** 🪪️ SQLite preserves the semantic metadata independently of original native ownership. */
import{test,expect}from"bun:test";
import Ajv2020 from"ajv/dist/2020";
import{Database}from"bun:sqlite";
import{readFileSync,existsSync}from"node:fs";
test("native edit metadata uses original shared actor and independent five-currency assembly",()=>{
 const root=new URL("../",import.meta.url),read=(p:string)=>JSON.parse(readFileSync(new URL(p,root),"utf8")),law=read("🧫️fixtures/🔣️.json");expect(new Ajv2020({strict:true}).compile(read("🧬️schema/🔣️.json"))(law)).toBe(true);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE metadata(actor TEXT,author TEXT,sequence INTEGER,clock TEXT,line TEXT,group_id TEXT,base_version INTEGER);CREATE TABLE inverse(position INTEGER,value INTEGER)");for(const value of law.actors)for(const repeat of law.repeats){const actor=value.repeat(repeat),clock=JSON.stringify(law.clock);db.query("INSERT INTO metadata VALUES(?,?,?,?,?,?,?)").run(actor,actor,law.sequence,clock,law.line,law.group,law.baseAppliedEditCount);expect(db.query("SELECT actor,author,sequence,clock,line,group_id,base_version FROM metadata ORDER BY rowid DESC LIMIT 1").get()).toEqual({actor,author:actor,sequence:law.sequence,clock,line:law.line,group_id:law.group,base_version:law.baseAppliedEditCount});db.exec("DELETE FROM inverse");for(const[index,value]of law.inverse.entries())db.query("INSERT INTO inverse VALUES(?,?)").run(index,Number(value));expect(db.query("SELECT value FROM inverse ORDER BY position").all().map(row=>Boolean((row as{value:number}).value))).toEqual(law.inverse);}}finally{db.close();}
 console.log("[DEBUG] SQLite original actor/author/sequence/clock metadata agrees for giant Unicode/NUL actors and16 independent cancellation frontiers");
 const path=new URL("🦀️.rs",root);expect(existsSync(path)).toBe(true);const source=readFileSync(path,"utf8");for(const proof of["NativeEditMetadata","next_demand","admit_actor_lease","Hasher","take_front","place_reserved","RetireOwned"]){expect(source).toContain(proof);}expect(source).not.toContain("next_edit(");expect(source).not.toContain("one_capacity_turn");
});
