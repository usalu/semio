/** 🧾️ Exact original JSON syntax-tree fields and ownership have independent schema and SQLite witnesses. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import Ajv2020 from "ajv/dist/2020";
import {visit} from "jsonc-parser";
import inputs from "./🧫️fixtures/🏛️inputs.json";
import snapshots from "./🧫️fixtures/📸️snapshots.json";
import malformed from "./🧫️fixtures/⚠️malformed.json";
import grants from "./🧫️fixtures/🎟️grants.json";
import { sqliteSnapshotRelationalSchema as relational } from "@semio-tech/framework/io/sqlite-snapshot/schema/relational";
import {parseJsonSnapshot} from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import snapshotSchema from "../../../../../🧬️schema/📸️snapshot/🔣️.json";
import { retainedCloneWireSchema as grantSchema } from "@semio-tech/framework/value/retained-clone/wire/schema";
import {semioSchemaAjvV1} from "@semio-tech/framework/schema/oracles/validator";
import {jsonSnapshotFromSqliteDatabase,jsonSnapshotToSqliteDatabase} from "../../🟦️.ts";
import type {SqliteDatabase,SqliteValue} from "@semio-tech/framework/io/sqlite-snapshot";
const scalar=(value:unknown):SqliteValue=>value!==null&&typeof value==="object"?BigInt((value as{integer:string}).integer):value as SqliteValue;
const database=(value:typeof inputs[number]):SqliteDatabase=>({tables:value.tables.map(table=>({...table,rows:table.rows.map(row=>({rowid:BigInt(row.rowid),values:row.values.map(scalar)}))}))});
function oracle(original:SqliteDatabase):Database{const db=new Database(":memory:",{safeIntegers:true});try{for(const table of original.tables)db.run(table.sql);for(const table of original.tables)for(const row of table.rows)db.run("INSERT INTO "+table.name+" VALUES("+row.values.map(()=>"?").join(",")+")",row.values as(string|number|bigint|null)[]);return db;}catch(error){db.close();throw error;}}
test("JSON original inputs and trees use separate canonical semantic and original grant admission",()=>{
 const admit=new Ajv2020({strict:true}).addKeyword("x-sqlite").compile(relational);for(const input of inputs)expect(admit(input)).toBe(true);expect(admit({...inputs[0],bodyGrant:grants.body})).toBe(false);
 const ajv=semioSchemaAjvV1({strict:true});const tree=ajv.compile(snapshotSchema);for(const input of snapshots)expect(tree(input)).toBe(true);const paid=ajv.compile(grantSchema);for(const grant of Object.values(grants))expect(paid(grant)).toBe(true);
});
test("JSON complete original trees preserve duplicate keys, exact lexemes and empty markers in independent SQLite",async()=>{
 for(let i=0;i<inputs.length;i++){const original=database(inputs[i]!),restored=await jsonSnapshotFromSqliteDatabase(original);expect(restored).toEqual(parseJsonSnapshot(snapshots[i]));expect(await jsonSnapshotFromSqliteDatabase(await jsonSnapshotToSqliteDatabase(restored))).toEqual(restored);const db=oracle(original);try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);const numbers=db.query("SELECT number_lexeme,number_value FROM json_value WHERE kind='number' ORDER BY id").all()as{number_lexeme:string;number_value:number|null}[];for(const row of numbers){let numeric:number|null=null;try{const value:unknown=JSON.parse(row.number_lexeme);if(typeof value==="number"&&Number.isFinite(value))numeric=value===0?0:value;}catch{}expect(row.number_value).toBe(numeric);}
  if(i===0){expect(db.query("SELECT key FROM json_object_member WHERE object_value_id=1 ORDER BY ordinal").all()).toEqual(snapshots[0]!.value.members!.map(member=>({key:member.key})));expect(db.query("SELECT kind FROM json_value WHERE id IN (4,8,9) ORDER BY id").all()).toEqual([{kind:"null"},{kind:"array"},{kind:"object"}]);expect(db.query("SELECT schema FROM json_document").get()).toEqual({schema:snapshots[0]!.schema});}
 }finally{db.close();}}
 const text='{"duplicate":900719925474099312345,"duplicate":-0,"x":1e+000,"a":[null,{},[]]}';const keys:string[]=[],numbers:string[]=[];visit(text,{onObjectProperty:key=>{keys.push(key)},onLiteralValue:(_value,offset,length)=>{const token=text.slice(offset,offset+length);if(/^-?\d/.test(token))numbers.push(token)}});expect(keys).toEqual(["duplicate","duplicate","x","a"]);expect(numbers).toEqual(["900719925474099312345","-0","1e+000"]);
});
test("JSON malformed original ownership, active payload, identity and derived cells independently refuse",async()=>{
 for(const law of malformed){const specimen=structuredClone(inputs[0]!);if("cycle"in law){specimen.tables[1]!.rows.push({rowid:"99",values:[{integer:"99"},"array",null,null,null,null]});specimen.tables[3]!.rows.push({rowid:"99",values:[{integer:"99"},{integer:"99"},{integer:"0"},{integer:"99"}]});}else{const table=specimen.tables.find(table=>table.name===law.table)!,index=table.rows.findIndex(row=>row.rowid===String(law.rowid));if("remove"in law)table.rows.splice(index,1);else if("duplicate"in law)table.rows.push(structuredClone(table.rows[index]!));else table.rows[index]!.values[law.column]=law.value;}
  const original=database(specimen);await expect(jsonSnapshotFromSqliteDatabase(original)).rejects.toThrow();let refused=false;let db:Database|undefined;try{db=oracle(original);refused=db.query("PRAGMA foreign_key_check").all().length>0;for(const table of original.tables)for(const row of table.rows)refused||=row.values[0]!==row.rowid;
   const bad=db.query("WITH links AS (SELECT object_value_id AS parent,ordinal,value_id AS child,'object' AS kind FROM json_object_member UNION ALL SELECT array_value_id,ordinal,value_id,'array' FROM json_array_element), reached(id) AS (SELECT root_value_id FROM json_document UNION SELECT child FROM links JOIN reached ON links.parent=reached.id) SELECT (SELECT count(*) FROM json_value WHERE id NOT IN reached)+(SELECT count(*) FROM links l JOIN json_value v ON v.id=l.parent WHERE v.kind<>l.kind)+(SELECT count(*) FROM links WHERE child IN (SELECT root_value_id FROM json_document))+(SELECT count(*) FROM (SELECT child FROM links GROUP BY child HAVING count(*)<>1))+(SELECT count(*) FROM (SELECT parent FROM links GROUP BY parent HAVING min(ordinal)<>0 OR max(ordinal)<>count(*)-1 OR count(DISTINCT ordinal)<>count(*))) AS bad").get()as{bad:bigint};refused||=bad.bad!==0n;for(const row of db.query("SELECT number_lexeme,number_value FROM json_value WHERE kind='number'").all()as{number_lexeme:string;number_value:number|null}[]){let n:number|null=null;try{const v:unknown=JSON.parse(row.number_lexeme);if(typeof v==="number"&&Number.isFinite(v))n=v===0?0:v;}catch{}refused||=row.number_value!==n;}
  }catch{refused=true;}finally{db?.close();}expect(refused,law.id).toBe(true);
 }
});
test("JSON actual owner explicitly mounts both receiving directions and borrowed named validators",()=>{const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");for(const hook of ["to_sqlite_database_receiving","from_sqlite_database_receiving","validate_sqlite_snapshot_subset_decoding","validate_sqlite_snapshot_subset_encoding"])expect(source).toContain("fn "+hook);});
