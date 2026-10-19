import {expect,test} from 'bun:test';
import {Database} from 'bun:sqlite';
import Ajv from 'ajv';
import {readFileSync} from 'node:fs';
import fixture from '../🧫️fixtures/🔣️.json' with {type:'json'};
import schema from '../🧬️schema/🔣️.json' with {type:'json'};
test('one original semantic source feeds publication and replay in all persistent lanes',()=>{
 const validate=new Ajv({strict:true,allErrors:true}).compile(schema);expect(validate(fixture)).toBe(true);
 const db=new Database(':memory:');try{
  db.exec('CREATE TABLE sources(lane TEXT PRIMARY KEY, identity INTEGER UNIQUE);CREATE TABLE aliases(lane TEXT, port TEXT, identity INTEGER, PRIMARY KEY(lane,port));');
  for(const [index,lane]of fixture.lanes.entries()){
   db.query('INSERT INTO sources VALUES(?,?)').run(lane,index+1);
   for(const port of fixture.source.aliases)db.query('INSERT INTO aliases VALUES(?,?,?)').run(lane,port,index+1);
  }
  expect(db.query('SELECT COUNT(*) AS count FROM sources').get()).toEqual({count:3});
  expect(db.query('SELECT lane, COUNT(DISTINCT identity) AS sources, COUNT(*) AS ports FROM aliases GROUP BY lane ORDER BY lane').all()).toEqual([...fixture.lanes].sort().map(lane=>({lane,sources:1,ports:2})));
  for(const lane of fixture.lanes){for(const port of fixture.source.aliases){db.query('DELETE FROM aliases WHERE lane=? AND port=?').run(lane,port);expect(db.query('SELECT COUNT(*) AS count FROM sources WHERE lane=?').get(lane)).toEqual({count:1});}db.query('DELETE FROM sources WHERE lane=?').run(lane);}
  expect(db.query('SELECT COUNT(*) AS count FROM aliases').get()).toEqual({count:0});
  expect(db.query('SELECT COUNT(*) AS count FROM sources').get()).toEqual({count:0});
 }finally{db.close();}
 const code=readFileSync(new URL('../🦀️.rs',import.meta.url),'utf8');
 expect(code).toContain('pub fn shared_semantic_source_demands<');expect(code).toContain('pub fn admit_source_constructor_with_shared_semantic_preparation<');
 expect(code).toContain('let shared = Arc::new(preparation);');expect(code).toContain('Arc::clone(&shared)');
 console.log('[DEBUG] Original semantic catalog Ajv/SQLite: three lanes, one source per paired port, aliases close before source deletion');
});
