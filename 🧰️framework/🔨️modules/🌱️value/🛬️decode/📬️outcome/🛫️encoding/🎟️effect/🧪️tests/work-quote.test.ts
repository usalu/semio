/** ⛽️ A finite Source admits the actual independent computational bound. */
import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import law from"./work-quote.json";
import schema from"./work-quote.schema.json";
test("encoder original Work quotes stay separate from physical items",()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE turns(id TEXT PRIMARY KEY,stock INTEGER,bound INTEGER,actual INTEGER,accepted INTEGER)");for(const row of law.cases){db.query("INSERT INTO turns VALUES(?,?,?,?,?)").run(row.id,law.sourceWork,row.maximum,row.actual,Number(row.accepted));expect(row.accepted).toBe(row.maximum<=law.sourceWork);expect(row.actual<=row.maximum).toBe(true)}expect(db.query("SELECT id FROM turns WHERE bound > stock ORDER BY id").all()).toEqual([{id:"beyondSource"}]);expect(db.query("SELECT SUM(actual) AS actual FROM turns WHERE accepted=1").get()).toEqual({actual:6})}finally{db.close()}});
