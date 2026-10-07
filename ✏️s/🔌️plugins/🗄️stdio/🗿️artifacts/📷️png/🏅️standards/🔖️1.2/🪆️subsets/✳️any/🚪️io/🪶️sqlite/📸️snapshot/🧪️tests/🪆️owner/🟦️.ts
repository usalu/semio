/** 🪆️ PNG owns decoded fields rather than native carrier recipes. */
import {test,expect} from "bun:test";
import {parsePngSnapshot,defaultPngSnapshot} from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {pngSnapshotToSqliteDatabase,pngSnapshotFromSqliteDatabase} from "../../🟦️.ts";
test("PNG decoded owner rejects source bytes and preserves arbitrary decoded metadata occurrences",async()=>{
 for(const value of [{schema:"stdio.png",bytes:[]},{...defaultPngSnapshot(),schema:"invented"},{...defaultPngSnapshot(),bytes:[1]}])expect(()=>parsePngSnapshot(value)).toThrow();
 const snapshot=defaultPngSnapshot();snapshot.image.ancillaryChunks=[{kind:[97,98,67,100],data:[0,255,0],afterRaster:false},{kind:[97,98,67,100],data:[17],afterRaster:true}];snapshot.image.textChunks=[{keyword:"Caption",value:"世界",kind:"iText",compressed:true,languageTag:"de",translatedKeyword:"Beschriftung"}];expect(await pngSnapshotFromSqliteDatabase(await pngSnapshotToSqliteDatabase(snapshot))).toEqual(snapshot);
});
