/** 🎟️ The original invocation and SQLite agree on source custody through out-of-order page returns. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";

test("renderer invocation keeps its initiating source until producer and supplied pages close", async () => {
 const law=await Bun.file(new URL("../🔣️.json",import.meta.url)).json();
 const schema=await Bun.file(new URL("../🧬️schema/🔣️.json",import.meta.url)).json();
 const validate=new Ajv({strict:true}).compile(schema);
 expect(validate(law)).toBe(true);expect(validate({...law,source:law.foreignSource})).toBe(false);
 const db=new Database(":memory:");
 try {
  db.exec("CREATE TABLE owners(id INTEGER PRIMARY KEY,source INTEGER,state TEXT,bytes INTEGER)");
  let invocation=false,producer=false,acknowledged=false,released=0;
  const pages=new Map<number,string>();
  for(const row of law.steps){
   switch(row.event){
    case "admit":invocation=true;producer=true;db.query("INSERT INTO owners VALUES(-1,?,'live',0)").run(law.source);break;
    case "acknowledge":acknowledged=true;break;
    case "supply":expect(acknowledged).toBe(true);for(const ordinal of law.pages){pages.set(ordinal,"issued");db.query("INSERT INTO owners VALUES(?,?,'issued',?)").run(ordinal,law.source,law.pageBytes);}break;
    case "closeProducer":producer=false;break;
    case "returnPages":for(const ordinal of law.pages){pages.set(ordinal,"returned");db.query("UPDATE owners SET state='returned' WHERE id=?").run(ordinal);}break;
    case "closePages":for(const ordinal of law.pages){pages.delete(ordinal);released+=law.pageBytes;db.query("DELETE FROM owners WHERE id=?").run(ordinal);}break;
    case "returnInvocation":expect(producer).toBe(false);expect(pages.size).toBe(0);invocation=false;db.exec("DELETE FROM owners WHERE id=-1");break;
   }
   expect({loans:Number(invocation)+pages.size,released}).toEqual({loans:row.loans,released:row.released});
   expect(db.query("SELECT count(*) AS loans FROM owners WHERE source=?").get(law.source)).toEqual({loans:row.loans});
   for(const denial of law.denials){
    const grant={items:Number(denial!=="items"),copy:Number(denial!=="copy"),depth:Number(denial!=="depth"),source:denial==="foreignSource"?law.foreignSource:law.source};
    const before=JSON.stringify([...pages]),sqlBefore=db.query("SELECT * FROM owners ORDER BY id").all();
    const admitted=grant.items>0&&grant.copy>0&&grant.depth>0&&grant.source===law.source;
    if(admitted)pages.delete(law.pages[0]);
    db.query("UPDATE owners SET state='forbidden' WHERE source=? AND ?>0 AND ?>0 AND ?>0").run(grant.source,grant.items,grant.copy,grant.depth);
    expect(admitted).toBe(false);expect(JSON.stringify([...pages])).toBe(before);expect(db.query("SELECT * FROM owners ORDER BY id").all()).toEqual(sqlBefore);
   }
  }
  expect(law.streamedFileBytes).toBeGreaterThan(law.liveOperationBytes);
 }finally{db.close();}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");
 for(const text of ["OriginalCallerSourceLoan","admit_loan(cx)","validate_retirement_context(cx)","job.borrow_outcome(descriptor)","descriptor.close_step(grant)","supply_result_last_page","self.returns.retains_source(self.source_identity)","source.close_step(cx.original_source_parent()?,cx)"])expect(source).toContain(text);
 for(const text of ["StepContext::new","RetainedCloneGrant::default","path.clone()","to_vec()","take_result()","Box::new"] )expect(source).not.toContain(text);
});
