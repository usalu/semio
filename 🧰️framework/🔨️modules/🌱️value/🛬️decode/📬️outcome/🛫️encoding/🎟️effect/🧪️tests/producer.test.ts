/** 🧾️ Producer success facts stay independent from the actual owned failure. */
import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import law from"./producer.json";
import schema from"./producer.schema.json";
test("encoder producer facts preserve both successful and refused physical effects",()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE replies(kind TEXT PRIMARY KEY,performed INTEGER,items INTEGER,bytes INTEGER,cause TEXT,body BLOB)");for(const[kind,row]of Object.entries({success:law.success,failure:law.failure})){db.query("INSERT INTO replies VALUES(?,?,?,?,?,?)").run(kind,Number(row.performed),row.copiedItems,row.copiedBytes,"cause"in row?row.cause:null,new TextEncoder().encode(law.input))}expect(db.query("SELECT kind FROM replies WHERE length(body)<>bytes OR performed<>1 OR items<>1").all()).toEqual([]);expect(db.query("SELECT kind,cause FROM replies WHERE cause IS NOT NULL").all()).toEqual([{kind:"failure",cause:law.failure.cause}]);expect(db.query("SELECT COUNT(*) AS replies,SUM(bytes) AS bytes FROM replies").get()).toEqual({replies:2,bytes:16})}finally{db.close()}});
