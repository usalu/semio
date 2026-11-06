import{test,expect}from'bun:test';
import{Database}from'bun:sqlite';
import Ajv from'ajv/dist/2020';
import law from'../view.json';
import schema from'../view.schema.json';
test('original work view agrees with independently retained SQL reservations',()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const db=new Database(':memory:');
 try{
  db.run('CREATE TABLE scope(stock INTEGER NOT NULL, actual INTEGER NOT NULL, reserved INTEGER)');
  db.run('INSERT INTO scope VALUES(?1, 0, NULL)',[law.stock]);
  const view=()=>Number((db.query('SELECT coalesce(reserved,stock-actual) AS work FROM scope').get()as{work:number}).work),views=[view()];
  db.run('UPDATE scope SET reserved=?1',[law.reserved]);views.push(view());
  db.run('UPDATE scope SET actual=actual+?1,reserved=NULL',[law.committed]);views.push(view());views.push(view());
  db.run('UPDATE scope SET reserved=?1',[law.cancelled]);db.run('UPDATE scope SET reserved=NULL');views.push(view());
  expect(views).toEqual(law.views);expect(view()).toBe(law.nextScope);expect(law.actual).toBe(law.committed);
 }finally{db.close()}
});
