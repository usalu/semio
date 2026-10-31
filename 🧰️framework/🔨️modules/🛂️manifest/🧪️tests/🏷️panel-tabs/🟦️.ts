import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {panelTabKindId,type PanelTabKind} from "../../📋️app-definition-refresh-and-context-menu/🟦️.ts";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

type Corpus=Readonly<{cases:readonly Readonly<{panel:PanelTabKind;native:string;expected:string}>[]}>;

/** 🏷️ Executes the neutral panel identities against TypeScript, the authored Rust match and SQLite. */
test("every panel identity preserves the native manifest key",()=>{
 const validate=new Ajv({strict:false}).compile<Corpus>(schema),input:unknown=fixture;
 if(!validate(input))throw Error(JSON.stringify(validate.errors));
 const native=readFileSync(new URL("../../🧱️app-surface/🪟️window-kinds-and-panels/🦀️.rs",import.meta.url),"utf8");
 const matches=[...native.matchAll(/PanelTabKind::(\w+) => "([^"]+)"/gu)].map(row=>[row[1]!,row[2]!] as const),db=new Database(":memory:");
 try{
  db.run("CREATE TABLE tabs (kind TEXT PRIMARY KEY,id TEXT NOT NULL)");
  for(const[kind,id]of matches)db.run("INSERT INTO tabs VALUES (?,?)",[kind,id]);
  expect(input.cases.length).toBe(matches.length+1);
  expect(new Set(input.cases.map(row=>row.native)).size).toBe(input.cases.length);
  expect(native).toMatch(/PanelTabKind::App\(id\) => id\.as_str\(\)/u);
  for(const row of input.cases){
   const oracle=db.query("SELECT CASE WHEN ?1='App' THEN ?2 ELSE (SELECT id FROM tabs WHERE kind=?1) END id").get(row.native,row.panel.kind==="app"?row.panel.id:null) as {id:string};
   expect(oracle.id).toBe(row.expected);
   expect(panelTabKindId(row.panel)).toBe(oracle.id);
  }
 }finally{db.close();}
});
