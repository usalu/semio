/** ✉️ Explicit authored union branch inputs and independent SQLite laws. */
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import f from "../../../🧫️fixtures/🪶️sqlite/🔣️.json";
import type {SemioSubsetSnapshot,SemioSnapshot} from "../../../🟦️.ts";
import {semioSnapshotToSqliteDatabase,semioSnapshotFromSqliteDatabase} from "../../../🪶️sqlite/🟦️.ts";
import {binary64,exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework";
export const subsets:SemioSubsetSnapshot[]=[
  {subset:"brep",schema:"stdio.semio.brep",vertices:[],edges:[],loops:[],faces:[],shells:[],solids:[],coedges:[],nextLabel:0n},
  {subset:"mesh",schema:"stdio.semio.mesh",meshes:[],materials:[],textures:[]},
  {subset:"model",schema:"stdio.semio.model",spatial:[],elements:[],relations:[]},
  {subset:"value",schema:"stdio.semio.value",root:{kind:"null"},nodes:[]},
  {subset:"document",schema:"stdio.semio.document",styles:[],images:[],blocks:[]},
  {subset:"cad",schema:"stdio.semio.cad",layers:[],blocks:[],entities:[]},
  {subset:"drawing",schema:"stdio.semio.drawing",canvas:{width:binary64(0),height:binary64(0),background:undefined},styles:[],layers:[]},
  {subset:"image",schema:"stdio.semio.image",width:0,height:0,colorspace:"rgba",bitDepth:8,frames:[],icc:null,metadata:[]},
  {subset:"video",schema:"stdio.semio.video",streams:[]},
  {subset:"audio",schema:"stdio.semio.audio",sampleRate:0,format:"f32",channels:[],tags:[]},
  {subset:"animation",schema:"stdio.semio.animation",timelines:[]},
  {subset:"presentation",schema:"stdio.semio.presentation",masters:[],layouts:[],slides:[]},
  {subset:"flow",schema:"stdio.semio.flow",nodes:[],edges:[]},
  {subset:"text",schema:"stdio.semio.text",runs:[{language:"",content:"English Deutsch\u0000文",marks:[]}]},
  {subset:"table",schema:"stdio.semio.table",columns:[],rows:[]},
  {subset:"graph",schema:"stdio.semio.graph",nodes:[],edges:[]},
  {subset:"object",schema:"stdio.semio.object",transform:{translation:{x:binary64(0),y:binary64(0),z:binary64(0)},rotation:{x:binary64(0),y:binary64(0),z:binary64(0),w:binary64(1)},scale:{x:binary64(1),y:binary64(1),z:binary64(1)}}},
  {subset:"kit",schema:"stdio.semio.kit",types:[],designs:[],objects:[],models:[],representations:[]},
];
/** 🧪️ Registers only the named owner group's complete independent branch laws. */
export function registerBranches(names:readonly SemioSubsetSnapshot["subset"][]):void{
 for(const name of names){const subset=subsets.find(subset=>subset.subset===name);if(!subset)throw Error("Unknown authored Semio branch");
 test("Semio base independent selected branch "+subset.subset,async()=>{const input:SemioSnapshot={schema:f.schema,subset},db=Database.deserialize(await exportSqliteDatabase(await semioSnapshotToSqliteDatabase(input)));try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(db.query(f.query).all()).toEqual([{schema:f.schema,subset:subset.subset}]);expect(await semioSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(input);if(subset.subset==="text"){db.query("UPDATE semio_text_run SET content='edited independent SQLite'").run();const expected=structuredClone(input);if(expected.subset.subset!=="text")throw Error("fixture text");expected.subset.runs[0]!.content="edited independent SQLite";expect(await semioSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(expected);}}finally{db.close();}},20000);
 }
}

