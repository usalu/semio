import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import fixture from"./🧫️fixtures/narrowing.json"with{type:"json"};
import schema from"../🧬️schema/narrowing.json"with{type:"json"};
test("VM refinement never changes Source identity or increases any issued axis",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");
 try{for(const row of fixture.cases){const a=row.actual,b=fixture.issued;let identity=true;for(const key of["operation","generation","epoch"]as const)identity&&=a.input[key]===b.input[key];identity&&=a.input.work.clockBinding===b.input.work.clockBinding&&a.input.work.absoluteDeadlineUs===b.input.work.absoluteDeadlineUs;
  let bounded=true;for(const key of Object.keys(a.input.grant)as(keyof typeof a.input.grant)[])bounded&&=Boolean((db.query("SELECT (?<=?) AS bounded").get(a.input.grant[key],b.input.grant[key])as{bounded:number}).bounded);
  const oracle=(db.query("SELECT (? AND ? AND ?<=? AND ?<=? AND ?<=?) AS valid").get(Number(identity),Number(bounded),a.input.work.fuel,b.input.work.fuel,a.fuel,b.fuel,a.deadlineUs,b.deadlineUs)as{valid:number}).valid;expect(Boolean(oracle),row.id).toBe(row.valid);
 }}finally{db.close();}
});
