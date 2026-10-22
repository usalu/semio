import {expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020";
import vectors from "./🔣️.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🎫️ Validates rooted lifecycle vectors with independent JSON Schema and SQLite transitions. */
export function ticketScopeVectorLawV1():void {
 const validate=new Ajv2020({strict:true,allErrors:true}).compile(schema);expect(validate(vectors),JSON.stringify(validate.errors)).toBe(true);
 const db=new Database(":memory:");
 try {
  const segment=db.query("WITH RECURSIVE chars(i,c) AS(SELECT 1,unicode(substr(?1,1,1)) UNION ALL SELECT i+1,unicode(substr(?1,i+1,1)) FROM chars WHERE i<length(?1)),base(v) AS(SELECT upper(substr(?1,1,CASE instr(?1,'.') WHEN 0 THEN length(?1) ELSE instr(?1,'.')-1 END))) SELECT CASE WHEN ?1<>'' AND ?1 NOT IN('.','..') AND rtrim(?1,' .')=?1 AND instr(?1,char(0))=0 AND NOT EXISTS(SELECT 1 FROM chars WHERE c<32 OR c=127 OR instr('/\\<>:\"|?*',char(c))>0) AND NOT EXISTS(SELECT 1 FROM base WHERE v IN('CON','PRN','AUX','NUL') OR v GLOB 'COM[1-9¹²³]' OR v GLOB 'LPT[1-9¹²³]') THEN 1 ELSE 0 END accepted");
  for(const value of vectors.invalidSegments)expect(segment.get(value)).toEqual({accepted:0});
  for(const value of vectors.validSegments)expect(segment.get(value)).toEqual({accepted:1});
  const normalize=db.query("WITH RECURSIVE raw(p) AS(SELECT replace(?1,char(92),'/')),collapsed(p) AS(SELECT p FROM raw UNION ALL SELECT replace(p,'//','/') FROM collapsed WHERE instr(p,'//')>0),plain(p) AS(SELECT p FROM collapsed WHERE instr(p,'//')=0),bounded(p) AS(SELECT CASE WHEN substr(raw.p,1,2)='//' AND substr(raw.p,3,1)<>'/' AND length(raw.p)>2 THEN '/' ELSE '' END||plain.p FROM raw,plain) SELECT CASE WHEN p='/' OR p GLOB '[A-Za-z]:/' THEN p ELSE rtrim(p,'/') END normalized FROM bounded"),parent=db.query("WITH RECURSIVE positions(i) AS(SELECT 1 UNION ALL SELECT i+1 FROM positions WHERE i<length(?1)),last(n) AS(SELECT coalesce(max(i),0) FROM positions WHERE substr(?1,i,1)='/') SELECT CASE WHEN ?1 GLOB '[A-Za-z]:/' OR (substr(?1,1,2)='//' AND length(substr(?1,3))-length(replace(substr(?1,3),'/',''))<=1) THEN ?1 WHEN n=0 THEN '.' WHEN n=1 THEN '/' WHEN n=3 AND substr(?1,2,1)=':' THEN substr(?1,1,3) ELSE substr(?1,1,n-1) END parent FROM last");
  for(const value of vectors.separatorCases){const actual=normalize.get(value.input) as {normalized:string};expect(actual).toEqual({normalized:value.normalized});expect(parent.get(actual.normalized)).toEqual({parent:value.parent});}
  db.exec("CREATE TABLE tickets(root TEXT PRIMARY KEY,slug TEXT,status TEXT,prompt TEXT,kinds TEXT)");
  for(const root of vectors.roots)db.query("INSERT INTO tickets VALUES(?,upper(replace(?,' ','-')),'open',?,json_array('ticket.open'))").run(root.name,vectors.open.title,vectors.open.prompt);
  db.query("UPDATE tickets SET slug=upper(replace(?,' ','-')),prompt=? WHERE root=?").run(vectors.change.title,vectors.change.description,vectors.roots[0].name);
  db.query("UPDATE tickets SET status='closed',kinds=json_insert(kinds,'$[#]','ticket.close') WHERE root=? AND status='open'").run(vectors.roots[0].name);
  db.query("UPDATE tickets SET status='open',kinds=json_insert(kinds,'$[#]','ticket.reopen') WHERE root=? AND status='closed'").run(vectors.roots[0].name);
  const first=db.query("SELECT slug,status,prompt,json(kinds) kinds FROM tickets WHERE root=?").get(vectors.roots[0].name) as {slug:string;status:string;prompt:string;kinds:string};
  expect({...first,kinds:JSON.parse(first.kinds)}).toEqual({slug:vectors.expected.slug,status:vectors.expected.status,prompt:vectors.expected.prompt,kinds:vectors.expected.kinds});
  const other=db.query("SELECT slug,status,prompt,json(kinds) kinds FROM tickets WHERE root=?").get(vectors.roots[1].name) as {slug:string;status:string;prompt:string;kinds:string};
  expect({...other,kinds:JSON.parse(other.kinds)}).toEqual({slug:vectors.expected.originalSlug,status:vectors.expected.otherStatus,prompt:vectors.expected.otherPrompt,kinds:vectors.expected.otherKinds});
  db.query("DELETE FROM tickets WHERE root=?").run(vectors.roots[0].name);expect(db.query("SELECT root FROM tickets ORDER BY root").all()).toEqual([{root:vectors.roots[1].name}]);
 }finally{db.close();}
}
