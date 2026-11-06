import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import law from"./🧫️fixtures/🔣️.json"with{type:"json"};
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
test("funded source JSON matches independent SQLite",()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{expect(db.query("SELECT json(?) AS value").get(JSON.stringify(law.value))).toEqual({value:law.expected});}finally{db.close();}});
