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
 const law=read("🧫️fixtures/🔣️.json");expect(law.postCustody).toEqual(["original-read","mutable-handback","original-intermediate"]);expect(new Ajv({strict:true}).compile(read("../🧬️schema/🔣️.json"))(law)).toBe(true);
 for(const wide of[false,true])for(const source of law.cases){
  const base={...law.base,text:wide?law.wideText.text.repeat(law.wideText.repeat):law.base.text},row={...source,expected:source.accepted?{...source.expected,text:base.text}:undefined};
  const db=new Database(":memory:");db.run("CREATE TABLE original_state(a INTEGER,b INTEGER,text TEXT)");db.run("INSERT INTO original_state VALUES(?,?,?)",[base.a,base.b,base.text]);db.run("BEGIN");let accepted=true,messageCount=0;
  try{for(const edit of row.edits as Edit[]){const state=db.query("SELECT a FROM original_state").get()as{a:number};if(edit.requireA!==undefined&&edit.requireA!==state.a){messageCount++;throw Error("original conditional refusal");}if(edit.warning)messageCount++;if(edit.setA!==undefined)db.run("UPDATE original_state SET a=?",[edit.setA]);if(edit.setB!==undefined)db.run("UPDATE original_state SET b=?",[edit.setB]);}db.run("COMMIT");}catch{accepted=false;db.run("ROLLBACK");}
  let actual:Snapshot|undefined;try{actual=applyOrderedDiffColdReference(base,row.edits as Edit[],(edit,base)=>{if(edit.requireA!==undefined&&edit.requireA!==base.a)throw Error("original conditional refusal");return{...base,a:edit.setA??base.a,b:edit.setB??base.b};});}catch{}
  expect(accepted,row.name).toBe(row.accepted);expect(actual!==undefined,row.name).toBe(row.accepted);expect(messageCount,row.name).toBe(row.messageCount);
  const output=db.query("SELECT a,b,text FROM original_state").get()as Snapshot;expect(output).toEqual(accepted?row.expected:base);if(actual)expect(actual).toEqual(output);db.close();
 }
});

test("ordered work preserves the child and parent currencies under the original budget",()=>{const law=read("🧫️fixtures/🔣️.json");expect(new Ajv({strict:true}).compile(read("../🧬️schema/🔣️.json"))(law)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE turns(name TEXT,budget INTEGER,child INTEGER,parent INTEGER,actual INTEGER,admitted INTEGER)");for(const row of law.workCases){db.query("INSERT INTO turns VALUES(?,?,?,?,?,?)").run(row.name,row.budget,row.child,row.parent,row.actual,Number(row.admitted));}const rows=db.query("SELECT name,child+parent AS actual,(child+parent)<=budget AS fits FROM turns ORDER BY rowid").all()as {name:string,actual:number,fits:number}[];for(let i=0;i<rows.length;i++){expect(rows[i].actual,rows[i].name).toBe(law.workCases[i].actual);expect(Boolean(rows[i].fits)).toBe(true)}}finally{db.close()}});

test("diagnostic text chunks preserve UTF-8 bytes against SQLite",()=>{const law=read("🧫️fixtures/🔣️.json").diagnosticText;const text=law.pattern.repeat(law.repetitions);const bytes=new TextEncoder().encode(text);const db=new Database(":memory:");try{const oracle=db.query("SELECT length(CAST(? AS BLOB)) AS bytes,hex(CAST(? AS BLOB)) AS hex").get(text,text)as{bytes:number,hex:string};expect(oracle.bytes).toBe(law.bytes);let offset=0;let rebuilt="";while(offset<bytes.length){let end=Math.min(offset+law.chunkBytes,bytes.length);while(end>offset&&end<bytes.length&&(bytes[end]&0xc0)===0x80)end--;expect(end).toBeGreaterThan(offset);rebuilt+=new TextDecoder("utf-8",{fatal:true}).decode(bytes.subarray(offset,end));offset=end;}expect(rebuilt).toBe(text);expect(Buffer.from(rebuilt).toString("hex").toUpperCase()).toBe(oracle.hex);}finally{db.close()}});

test("diagnostic roster settles one funded slot per turn",()=>{const law=read("🧫️fixtures/🔣️.json").diagnosticSlots;const db=new Database(":memory:");try{const result=db.query("WITH RECURSIVE slots(i) AS (SELECT 1 UNION ALL SELECT i+1 FROM slots WHERE i<?) SELECT count(*) AS turns,sum(1) AS work FROM slots").get(law.targets*3)as{turns:number,work:number};expect(result.turns).toBe(law.turns);expect(result.work).toBe(law.turns*law.workPerTurn);}finally{db.close()}});
