import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import law from"./🧫️fixtures/🔣️.json"with{type:"json"};
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
test("resumed Source JSON keeps paid storage while matching independent canonical JSON",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);expect(JSON.stringify(law.value)).toBe(law.expected);const db=new Database(":memory:");try{expect(db.query("SELECT json(?) AS value").get(JSON.stringify(law.value))).toEqual({value:law.expected});}finally{db.close();}
});

test("actor budget JSON preserves total and scoped authority as separate objects",()=>{const db=new Database(":memory:");try{expect(db.query("SELECT json(?) AS budget").get(JSON.stringify(law.budget))).toEqual({budget:law.expectedBudget});expect(law.budget.retainedPolicy.maximumCapacityBytes).toBeLessThan(law.budget.input.grant.maximumCapacityBytes);}finally{db.close();}});
