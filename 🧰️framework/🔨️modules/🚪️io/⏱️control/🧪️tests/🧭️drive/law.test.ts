import{test,expect}from"bun:test";
import Ajv from"ajv/dist/2020.js";
import{Database}from"bun:sqlite";
import fixture from"./fixture.json";
import schema from"./schema.json";
test("native snapshot drive preserves independent producer work and every failure component",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE turns(work INTEGER,items INTEGER,result TEXT,errors TEXT)");
  for(const row of fixture.cases){
   if(row.admitted)for(const step of row.steps)db.query("INSERT INTO turns VALUES(?,?,?,?)").run(step.work,step.items,step.result,JSON.stringify(step.errors));
   const counts=db.query("SELECT COALESCE(SUM(work),0) AS producerWork,COALESCE(SUM(items),0) AS items,COUNT(*) AS calls FROM turns").get()as{producerWork:number,items:number,calls:number};
   expect(counts).toEqual({producerWork:row.expected.producerWork,items:row.expected.items,calls:row.expected.calls});
   const terminal=db.query("SELECT result,errors FROM turns ORDER BY rowid DESC LIMIT 1").get()as{result:string,errors:string}|null;
   expect(terminal?JSON.parse(terminal.errors):[]).toEqual(row.expected.errors);
   expect(!terminal||terminal.result!=="ready").toBe(row.expected.cursorRetained);
   db.exec("DELETE FROM turns");
  }
 }finally{db.close()}
});
