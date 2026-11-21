import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/declaration-collections.json";
import schema from "./🧫️fixtures/declaration-collections.schema.json";

test("declaration membership preserves authored sequence and exact replacement",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");
 try{
  const roster=db.query<{id:string},[string,string]>(`
   SELECT authored.value AS id FROM json_each(?1) AS authored
   WHERE NOT EXISTS (SELECT 1 FROM json_each(?2) WHERE value=authored.value)
   ORDER BY authored.key
  `).all(JSON.stringify(fixture.actions),JSON.stringify(fixture.windowActions));
  expect(roster.map(({id})=>id)).toEqual(fixture.expectedAppActions);
  const duplicate=db.query<{id:string},[string]>(`
   SELECT action.value AS id FROM json_each(?1) AS action
   WHERE EXISTS (SELECT 1 FROM json_each(?1) AS earlier WHERE earlier.value=action.value AND earlier.key<action.key)
   ORDER BY action.key LIMIT 1
  `).get(JSON.stringify(fixture.duplicateActions));
  expect(duplicate?.id).toBe(fixture.firstDuplicate);
  const terms=db.query<{id:string},[string]>("SELECT value AS id FROM json_each(?) ORDER BY key").all(JSON.stringify(fixture.terminologyIds));
  expect(terms.map(({id})=>id)).toEqual(fixture.expectedTerminologies);
  const latest=db.query<{id:string,breadcrumb:string},[string]>(`
   WITH declarations AS (
    SELECT json_extract(value,'$.id') AS id, json_extract(value,'$.breadcrumb') AS breadcrumb, key
    FROM json_each(?)
   )
   SELECT id,breadcrumb FROM declarations AS row
   WHERE row.key=(SELECT max(later.key) FROM declarations AS later WHERE row.id=later.id)
   ORDER BY id COLLATE BINARY
  `).all(JSON.stringify(fixture.breadcrumbs));
  expect(latest).toEqual(fixture.expectedBreadcrumbs);
  const commands=db.query<{id:string},[string]>("SELECT value AS id FROM json_each(?) ORDER BY key").all(JSON.stringify(fixture.commands));
  expect(commands.map(({id})=>id)).toEqual(fixture.expectedCommands);
 }finally{db.close();}
});
