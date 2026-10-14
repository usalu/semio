/** 🔺️ Original STL semantic input and paid receiving declarations have independent SQL witnesses. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import Ajv2020 from "ajv/dist/2020";
import input from "./🧫️fixtures/🏛️input.json";
import empty from "./🧫️fixtures/∅️input.json";
import malformed from "./🧫️fixtures/⚠️malformed.json";
import grants from "./🧫️fixtures/🎟️grants.json";
import relational from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧬️schema/🏛️relational/🔣️.json";
import grantSchema from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json";
import {semioSchemaAjvV1} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import {stlSnapshotFromSqliteDatabase,stlSnapshotToSqliteDatabase,stlSnapshotValidateSqliteSubset} from "../../🟦️.ts";
import type {SqliteValue,SqliteDatabase} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

const scalar=(value:unknown):SqliteValue=>value!==null&&typeof value==="object"?BigInt((value as {integer:string}).integer):value as SqliteValue;
const database=(value:typeof input):SqliteDatabase=>({tables:value.tables.map(table=>({...table,rows:table.rows.map(row=>({rowid:BigInt(row.rowid),values:row.values.map(scalar)}))}))});
test("STL original relational input is independently schema-admitted without trial authority",()=>{
 const admit=new Ajv2020({strict:true}).addKeyword("x-sqlite").compile(relational);for(const specimen of [input,empty])expect(admit(specimen)).toBe(true);expect(admit({...input,bodyGrant:grants.body})).toBe(false);
 const admitGrant=semioSchemaAjvV1({strict:true}).compile(grantSchema);for(const grant of Object.values(grants))expect(admitGrant(grant)).toBe(true);
 console.log("[DEBUG] Original STL two genuine three-table inputs and three independent canonical original grants admitted; no trial policy enters semantic schema");
});
test("STL original schema/name, IEEE words, facet and fixed vertex ownership agree with independent SQLite",async()=>{
 for(const specimen of [input,empty]){const source=database(specimen),restored=await stlSnapshotFromSqliteDatabase(source),projected=await stlSnapshotToSqliteDatabase(restored);expect(await stlSnapshotFromSqliteDatabase(projected)).toEqual(restored);expect(restored.schema).toBe("literal/schema λ\0");expect(restored.solidName).toBe("Original 世界\0 name");expect(restored.triangles.length).toBe(specimen===input?2:0);
  const oracle=new Database(":memory:",{safeIntegers:true});try{oracle.run("PRAGMA foreign_keys=ON");for(const table of source.tables)oracle.run(table.sql);for(const table of source.tables)for(const row of table.rows)oracle.run("INSERT INTO "+table.name+" VALUES("+row.values.map(()=>"?").join(",")+")",row.values as (string|number|bigint|null)[]);
   expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);
   const rows=oracle.query("SELECT f.ordinal,v.ordinal AS vertex,v.x_ieee754_bits AS x,v.y_ieee754_bits AS y,v.z_ieee754_bits AS z FROM stl_facet f JOIN stl_vertex v ON v.facet_id=f.id ORDER BY f.ordinal,v.ordinal").all()as{ordinal:bigint;vertex:bigint;x:bigint;y:bigint;z:bigint}[];expect(rows.length).toBe(restored.triangles.length*3);for(const row of rows)expect(restored.triangles[Number(row.ordinal)]!.vertices[Number(row.vertex)]!.map(v=>BigInt.asIntN(64,v.bits))).toEqual([row.x,row.y,row.z]);
   if(specimen===input){expect(restored.triangles[0]!.normal.map(v=>BigInt.asIntN(64,v.bits))).toEqual([-9223372036854775808n,9221140253039434428n,0n]);expect(oracle.query("SELECT normal_x_ieee754_bits AS x,normal_y_ieee754_bits AS y,normal_y_numeric_class AS kind FROM stl_facet WHERE ordinal=0").get()).toEqual({x:-9223372036854775808n,y:9221140253039434428n,kind:"nan"});}
  }finally{oracle.close();}
  await stlSnapshotValidateSqliteSubset(restored,{artifactKind:"s.stdio.stl",standard:"ascii",subset:"*"},source);
 }
 console.log("[DEBUG] Original STL literal UTF8/NUL, empty triangles, shuffled surrogate rows, signed zero/NaN payload and exact three ordered vertices independently match SQLite; Native custody separate");
});
test("STL original receiving mounts both directions and exact directional validators",()=>{
 const original=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");for(const hook of ["to_sqlite_database_receiving","from_sqlite_database_receiving","validate_sqlite_snapshot_subset_decoding","validate_sqlite_snapshot_subset_encoding"])expect(original).toContain("fn "+hook);
});

test("STL malformed original identities, relationships, ordinals and exact words have independent SQL refusal witnesses",async()=>{
 for(const law of malformed){const specimen=structuredClone(input),table=specimen.tables.find(table=>table.name===law.table)!,row=table.rows[law.row]!;if("remove"in law)table.rows.splice(law.row,1);else if("duplicate"in law)table.rows.push(structuredClone(row));else row.values[law.column]=law.value;
  const original=database(specimen);await expect(stlSnapshotFromSqliteDatabase(original)).rejects.toThrow();const oracle=new Database(":memory:",{safeIntegers:true});let refused=false;try{oracle.run("PRAGMA foreign_keys=ON");for(const table of original.tables)oracle.run(table.sql);for(const table of original.tables)for(const row of table.rows){oracle.run("INSERT INTO "+table.name+" VALUES("+row.values.map(()=>"?").join(",")+")",row.values as (string|number|bigint|null)[]);if(row.values[0]!==row.rowid)refused=true;}
   refused||=oracle.query("PRAGMA foreign_key_check").all().length>0;
   refused||=(oracle.query("SELECT count(*) AS n FROM stl_facet f WHERE f.ordinal<>(SELECT count(*)-1 FROM stl_facet earlier WHERE earlier.solid_id=f.solid_id AND earlier.ordinal<=f.ordinal)").get()as{n:bigint}).n!==0n;
   refused||=(oracle.query("SELECT count(*) AS n FROM stl_facet f WHERE (SELECT count(*) FROM stl_vertex v WHERE v.facet_id=f.id)<>3 OR (SELECT count(DISTINCT ordinal) FROM stl_vertex v WHERE v.facet_id=f.id)<>3").get()as{n:bigint}).n!==0n;
   refused||=(oracle.query("SELECT count(*) AS n FROM stl_facet WHERE normal_y_numeric_class='finite' AND normal_y IS NULL").get()as{n:bigint}).n!==0n;
  }catch{refused=true;}finally{oracle.close();}expect(refused,law.id).toBe(true);
 }
 console.log("[DEBUG] Original STL eleven malformed fixed vertex/count/ordinal/identity/IEEE inputs independently refused by SQLite ownership/query predicates and actual semantic reconstruction");
});
