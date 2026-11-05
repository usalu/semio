import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import corpus from"./🔣️.json";
import schema from"./🧬️.json";
test("full UI receiving corpus covers every component and retains nonempty patch descendants",()=>{expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const db=new Database(":memory:");const kinds=new Set<string>();for(const row of corpus.cases){expect(db.query("select json_valid(?) valid").get(row.wire)).toEqual({valid:1});if(row.family==="component"&&row.accepted)kinds.add((JSON.parse(row.wire)as{type:string}).type);if(row.family==="patch"&&row.accepted){const n=db.query("select json_array_length(?, '$.ops') n").get(row.wire)as{n:number};expect(n.n).toBeGreaterThan(0)}if(row.accepted){const duplicates=db.query("select count(*)-count(distinct parent||char(0)||key) n from json_tree(?) where parent is not null and typeof(key)='text'").get(row.wire)as{n:number};expect(duplicates.n).toBe(0)}}expect([...kinds].sort()).toEqual([...corpus.components].sort());db.close();});
