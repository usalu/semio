import{test,expect}from'bun:test';
import{Database}from'bun:sqlite';
import Ajv from'ajv/dist/2020';
import law from'./🔣️.json';
import schema from'./🧬️.json';
test('bounded projection fixture preserves its multilingual record and parent work ceilings',()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const db=new Database(':memory:');
 try{
  const record=JSON.stringify(law.record),json=db.query('SELECT json(?1) AS value').get(record)as{value:string};
  expect(JSON.parse(json.value)).toEqual(law.record);
  const query=db.query('SELECT CASE WHEN ?1 >= ?2 THEN ?1 - ?2 ELSE 0 END AS child');
  for(const maximum of law.limits){for(const parent of [0,1,2]){
   const child=(query.get(maximum,parent)as{child:number}).child;
   expect(child).toBe(Math.max(0,maximum-parent));
   expect(child+Math.min(parent,maximum)).toBe(maximum);
  }}
  expect(law.limits).toContain(0);
  expect(law.cancellationTurns).toContain(0);
 }finally{db.close()}
});
