import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
/** 🪢️ The independent relation preserves byte-call ownership until acknowledgement or instance retirement. */
test("original byte unit call requires acknowledgement and preserves cancellation custody",async()=>{
 const fixture=await Bun.file(new URL("🧫️fixtures/🔣️.json",import.meta.url)).json(),schema=await Bun.file(new URL("../🧬️schema/🔣️.json",import.meta.url)).json();expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const edges=new Map([["allocate:allocated","copy"],["copy:copied","call"],["call:returned","borrow"],["borrow:acknowledged","release"],["release:released","complete"]]);const db=new Database(":memory:");try{db.exec("CREATE TABLE edge(phase TEXT,event TEXT,next TEXT,PRIMARY KEY(phase,event))");for(const[key,next]of edges){const[phase,event]=key.split(":");db.run("INSERT INTO edge VALUES (?,?,?)",[phase,event,next])}for(const phase of["allocate","copy","call","borrow","release"])db.run("INSERT INTO edge VALUES (?,?,?)",[phase,"cancelled","abort"]);for(const row of fixture.cases){const expected=row.event==="cancelled"&&!["complete","abort"].includes(row.phase)?"abort":edges.get(row.phase+":"+row.event);expect(expected===row.next).toBe(row.valid);expect(Boolean(db.query("SELECT 1 FROM edge WHERE phase=? AND event=? AND next=?").get(row.phase,row.event,row.next))).toBe(row.valid);}}finally{db.close()}
});
