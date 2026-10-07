/** 💰️ Whole physical TIFF operations retain prior debt and exact cumulative grants. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import native from "../../../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json";
import {parseTiffSnapshot} from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {SqliteOperation} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {tiffSnapshotToSqliteFile,tiffSnapshotFromSqliteFile} from "../../🟦️.ts";
test("actual TIFF owned projection and physical file share one complete operation grant",async()=>{const value=parseTiffSnapshot(native.cases[1]!.snapshot),measured=new SqliteOperation(),bytes=await tiffSnapshotToSqliteFile(value,measured),oracle=Database.deserialize(bytes);try{expect(oracle.query("SELECT lo,hi FROM tiff_sample ORDER BY ordinal").all()).toEqual(native.cases[1]!.expectedSamples);expect(oracle.query("SELECT COUNT(*) AS tables FROM sqlite_schema WHERE type='table'").get()).toEqual({tables:17});}finally{oracle.close();}const debt=7,grant=measured.ownedBytes,exact=new SqliteOperation({maxAllocationBytes:grant+debt});exact.allocateBytes(debt);expect(await tiffSnapshotToSqliteFile(value,exact)).toEqual(bytes);expect(exact.remainingBytes()).toBe(0);const short=new SqliteOperation({maxAllocationBytes:grant+debt-1});short.allocateBytes(debt);await expect(tiffSnapshotToSqliteFile(value,short)).rejects.toHaveProperty("kind","ownershipLimit");expect(await tiffSnapshotFromSqliteFile(bytes)).toEqual(value);});
