/** 🧪️ Ordered scalar edits match independent SQLite transactions and original refusal output. */
import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import{readFileSync}from"node:fs";
import{applyOrderedDiffColdReference}from"../🟦️.ts";
const read=(path:string)=>JSON.parse(readFileSync(new URL(path,import.meta.url),"utf8"));
type Snapshot={a:number;b:number;text:string};
type Edit={setA?:number;setB?:number;requireA?:number;warning?:boolean};
test("ordered diff reference preserves intermediate originals and transactional refusal",()=>{
 const law=read("🧫️fixtures/🔣️.json");expect(new Ajv({strict:true}).compile(read("../🧬️schema/🔣️.json"))(law)).toBe(true);
 for(const wide of[false,true])for(const source of law.cases){
  const base={...law.base,text:wide?law.wideText.text.repeat(law.wideText.repeat):law.base.text},row={...source,expected:source.accepted?{...source.expected,text:base.text}:undefined};
  const db=new Database(":memory:");db.run("CREATE TABLE original_state(a INTEGER,b INTEGER,text TEXT)");db.run("INSERT INTO original_state VALUES(?,?,?)",[base.a,base.b,base.text]);db.run("BEGIN");let accepted=true,messageCount=0;
  try{for(const edit of row.edits as Edit[]){const state=db.query("SELECT a FROM original_state").get()as{a:number};if(edit.requireA!==undefined&&edit.requireA!==state.a){messageCount++;throw Error("original conditional refusal");}if(edit.warning)messageCount++;if(edit.setA!==undefined)db.run("UPDATE original_state SET a=?",[edit.setA]);if(edit.setB!==undefined)db.run("UPDATE original_state SET b=?",[edit.setB]);}db.run("COMMIT");}catch{accepted=false;db.run("ROLLBACK");}
  let actual:Snapshot|undefined;try{actual=applyOrderedDiffColdReference(base,row.edits as Edit[],(edit,base)=>{if(edit.requireA!==undefined&&edit.requireA!==base.a)throw Error("original conditional refusal");return{...base,a:edit.setA??base.a,b:edit.setB??base.b};});}catch{}
  expect(accepted,row.name).toBe(row.accepted);expect(actual!==undefined,row.name).toBe(row.accepted);expect(messageCount,row.name).toBe(row.messageCount);
  const output=db.query("SELECT a,b,text FROM original_state").get()as Snapshot;expect(output).toEqual(accepted?row.expected:base);if(actual)expect(actual).toEqual(output);db.close();
 }
});
