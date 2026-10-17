import Ajv from "ajv";
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {deflateSync,inflateSync} from "node:zlib";
import schema from "../🧬️schema/🔣️.json";
import contract from "./🧫️fixtures/🔣️.json";
import {deflateSnapshotToSqliteDatabase,deflateSnapshotFromSqliteDatabase} from "../../🟦️.ts";
import type {DeflateSnapshot} from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework/io/sqlite-snapshot";
const hints=["fastest","fast","default","maximum"] as const;
const source=():DeflateSnapshot=>({schema:contract.literal,compressionMethod:8,windowBits:7,compressionLevelHint:"default",dictId:undefined,payload:contract.octets.slice()});
test("Deflate neutral all six fields, reserved nibbles and ordered octets have independent SQLite witnesses",async()=>{
 const validate=new Ajv({strict:true,allErrors:true}).compile(schema);expect(validate(contract)).toBe(true);expect(contract.octets).toEqual(Array.from({length:256},(_,i)=>i));expect(new Set(contract.cases.map(value=>value.compressionMethod)).size).toBe(16);expect(new Set(contract.cases.map(value=>value.windowBits)).size).toBe(16);expect(new Set(contract.cases.map(value=>value.compressionLevelHint)).size).toBe(4);
 for(const value of contract.cases){const snapshot:DeflateSnapshot={...source(),...value,compressionLevelHint:value.compressionLevelHint as DeflateSnapshot["compressionLevelHint"],dictId:value.dictId??undefined};for(const payload of [snapshot.payload,[]]){const original={...snapshot,payload};const file=await exportSqliteDatabase(await deflateSnapshotToSqliteDatabase(original));for(const offset of contract.signedOffsets){const db=Database.deserialize(file);try{db.exec("PRAGMA foreign_keys=ON");expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);for(const [name,width]of Object.entries(contract.tables))expect(db.query(`PRAGMA table_info(${name})`).all().length).toBe(width);expect(db.query("SELECT schema,compression_method,window_bits,compression_level_hint,dictionary_adler32 FROM deflate_document").get()).toEqual({schema:contract.literal,compression_method:value.compressionMethod,window_bits:value.windowBits,compression_level_hint:hints.indexOf(value.compressionLevelHint as typeof hints[number]),dictionary_adler32:value.dictId});db.run("UPDATE deflate_payload_byte SET id=id+?",[offset]);expect(db.query("SELECT value FROM deflate_payload_byte ORDER BY ordinal").all()).toEqual(payload.map(value=>({value})));expect(await deflateSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(original);}finally{db.close();}}}}
 const input=Buffer.from(contract.octets),wire=deflateSync(input);expect(inflateSync(wire)).toEqual(input);expect(wire[0]!&15).toBe(8);
});
test("Deflate neutral malformed and changed field edits are actual SQLite mutations",async()=>{
 const original=source();const file=await exportSqliteDatabase(await deflateSnapshotToSqliteDatabase(original));for(const edit of contract.edits){const db=Database.deserialize(file);try{db.exec("PRAGMA foreign_keys=OFF; PRAGMA ignore_check_constraints=ON");db.exec(edit.sql);const input=await importSqliteDatabase(db.serialize());if(edit.reconstruct==="invalid")await expect(deflateSnapshotFromSqliteDatabase(input)).rejects.toThrow();else expect(await deflateSnapshotFromSqliteDatabase(input)).not.toEqual(original);}finally{db.close();}}
});
test("Deflate original four hooks have canonical supplied receiving declarations",()=>{
 const owner=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");for(const method of ["fn to_sqlite_database_receiving","fn from_sqlite_database_receiving","fn validate_sqlite_snapshot_subset_decoding","fn validate_sqlite_snapshot_subset_encoding"])expect(owner).toContain(method);const receiving=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");for(const token of ["RetireOwned","Port","ids","ordered","chunks(256)","settle","receiving_scope"])expect(receiving).toContain(token);for(const forbidden of [".clone()","Self::from_sqlite_database","DecodedFieldOwner","NativeDecodeControl::new","NativeEncodeControl::new"])expect(receiving).not.toContain(forbidden);
});
