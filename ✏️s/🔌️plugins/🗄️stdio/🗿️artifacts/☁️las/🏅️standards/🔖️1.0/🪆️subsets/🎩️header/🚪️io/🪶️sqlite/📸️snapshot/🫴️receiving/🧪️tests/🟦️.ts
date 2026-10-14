import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020.js";
import {readFileSync} from "node:fs";
import contract from "./🧫️fixtures/🔣️.json";
import schema from "./🧫️fixtures/🔣️schema.json";
import {lasSnapshotFixture} from "../../🧪️tests/🧰️support/🟦️.ts";
import {lasSnapshotToSqliteDatabase,lasSnapshotFromSqliteDatabase,lasSnapshotValidateSqliteSubset} from "../../🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
const read=(name:string)=>readFileSync(new URL(name,import.meta.url),"utf8");
const dialect={artifactKind:"s.stdio.las",standard:"1.0",subset:"*"};
test("LAS original receiving neutral policies and complete independent signed relations",async()=>{
 const ajv=new Ajv({strict:true});expect(ajv.compile(schema)(contract)).toBe(true);
 for(const law of contract.cases){const snapshot=structuredClone(lasSnapshotFixture);if(law.id==="empty"){snapshot.vlrs=[];snapshot.points=[];}if(law.id==="long"){const literal=contract.literal.repeat(257);snapshot.schema=literal;snapshot.header.systemIdentifier=literal;snapshot.header.generatingSoftware=literal;for(const vlr of snapshot.vlrs){vlr.userId=literal;vlr.description=literal;}}const database=Database.deserialize(await exportSqliteDatabase(await lasSnapshotToSqliteDatabase(snapshot)));try{if(law.signed){database.run("PRAGMA foreign_keys=OFF");database.run(contract.signedSql)};expect(database.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(database.query("PRAGMA foreign_key_check").all()).toEqual([]);let rows=0;for(const[name,width]of contract.tables){expect(database.query("PRAGMA table_info("+name+")").all()).toHaveLength(Number(width));rows+=(database.query("SELECT COUNT(*) AS n FROM "+name).get()as{n:number}).n;}expect(rows).toBe(law.rows);expect(await lasSnapshotFromSqliteDatabase(await importSqliteDatabase(database.serialize()))).toEqual(snapshot);const words=Database.deserialize(database.serialize(),{safeIntegers:true});try{const bits=words.query("SELECT x_scale_ieee754_bits AS word FROM las_header").get()as{word:bigint};expect(BigInt.asUintN(64,bits.word)).toBe(snapshot.header.xScale.bits);}finally{words.close();}}finally{database.close();}}
 console.log("[DEBUG] LAS independently readable eight-table complete and empty state preserves signed relation identity and raw64bit header words");
});
test("LAS full directional validation observes changed fields and malformed relation edits",async()=>{
 for(const law of contract.changes){const database=Database.deserialize(await exportSqliteDatabase(await lasSnapshotToSqliteDatabase(lasSnapshotFixture)));try{database.run("PRAGMA foreign_keys=OFF");database.run(law.sql);const value=await importSqliteDatabase(database.serialize());if(law.malformed)await expect(lasSnapshotFromSqliteDatabase(value)).rejects.toThrow();else{await lasSnapshotFromSqliteDatabase(value);await expect(lasSnapshotValidateSqliteSubset(lasSnapshotFixture,dialect,value)).rejects.toThrow();}}finally{database.close();}}
 console.log("[DEBUG] LAS independent authored SQL edits qualify every semantic entity and zero-sign mismatch policy");
});
test("LAS actual original four receiving hooks and physical native laws are mounted",()=>{
 const source=read("../../🦀️.rs");for(const name of ["to_sqlite_database_receiving","from_sqlite_database_receiving","validate_sqlite_snapshot_subset_decoding","validate_sqlite_snapshot_subset_encoding"])expect(source).toContain(name);
 const receiving=read("../🦀️.rs");for(const name of ["Frame","Projection","NativeSnapshotBodyWallet","validate_schema_sql","NativeSqliteReceivingFrame","receive_sqlite_snapshot","to_bits","parent_range","identity_position"])expect(receiving).toContain(name);expect(receiving).not.toContain("insert_ieee754");
 const laws=read("./🦀️.rs");for(const name of ["sqlite_snapshot_las_original_receiving_complete","sqlite_snapshot_las_original_receiving_refusals","sqlite_snapshot_las_original_receiving_cancellation","sqlite_snapshot_las_original_receiving_cumulative","sqlite_snapshot_las_original_receiving_registered_full_file"])expect(laws).toContain(name);
 console.log("[DEBUG] LAS original hook declarations and physical law mounts remain separate from Native qualification");
});

