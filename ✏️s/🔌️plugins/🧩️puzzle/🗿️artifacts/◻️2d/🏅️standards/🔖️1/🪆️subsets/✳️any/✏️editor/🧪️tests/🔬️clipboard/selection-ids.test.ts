import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import law from"../🧫️fixtures/selection-ids.json";
import schema from"../🧬️schema/selection-ids.json";
test("selection identity corpus is closed",()=>{expect(new Ajv().compile(schema)(law)).toBe(true)});
for(const row of law.cases)test(row.name,()=>{
 const selected=row.groups.flat(),ids=new Set(selected);
 expect(law.nodes.filter(node=>ids.has(node.id)||node.handles.some(handle=>ids.has(handle.id))).map(node=>node.id)).toEqual(row.expected);
 const db=new Database(":memory:");
 try{
  const rows=db.query("select json_extract(n.value,'$.id') as id from json_each(?) n where json_extract(n.value,'$.id') in (select value from json_each(?)) or exists(select 1 from json_each(json_extract(n.value,'$.handles')) h where json_extract(h.value,'$.id') in (select value from json_each(?))) order by cast(n.key as integer)").all(JSON.stringify(law.nodes),JSON.stringify(selected),JSON.stringify(selected)) as {id:string}[];
  expect(rows.map(row=>row.id)).toEqual(row.expected);
 }finally{db.close()}
});
