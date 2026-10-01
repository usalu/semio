/** ✉️ All eighteen owned union branches retain independently queryable typed entities. */
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import f from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import type {SemioSubsetSnapshot,SemioSnapshot} from "../../🟦️.ts";
import {SEMIO_SQLITE_SCHEMA,semioSnapshotToSqliteDatabase,semioSnapshotFromSqliteDatabase} from "../../🪶️sqlite/🟦️.ts";
import {binary64,exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework";
const subsets:SemioSubsetSnapshot[]=[
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
test("Semio base eighteen handwritten branch and schema authorities",async()=>{expect(subsets.map(s=>String(s.subset))).toEqual(f.subsets);expect(SEMIO_SQLITE_SCHEMA.startsWith(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql",import.meta.url)).text())).toBe(true);});
for(const subset of subsets)test("Semio base independent selected branch "+subset.subset,async()=>{const input:SemioSnapshot={schema:f.schema,subset},db=Database.deserialize(await exportSqliteDatabase(await semioSnapshotToSqliteDatabase(input)));try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(db.query(f.query).all()).toEqual([{schema:f.schema,subset:subset.subset}]);expect(await semioSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(input);if(subset.subset==="text"){db.query("UPDATE semio_text_run SET content='edited independent SQLite'").run();const expected=structuredClone(input);if(expected.subset.subset!=="text")throw Error("fixture text");expected.subset.runs[0]!.content="edited independent SQLite";expect(await semioSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(expected);}}finally{db.close();}},20000);
test("Semio base single selected owner and aggregate header budget",async()=>{const input:SemioSnapshot={schema:f.schema,subset:subsets[13]!},d=await semioSnapshotToSqliteDatabase(input);for(const mutate of[(x:any)=>x.tables[0].rows[0].values[16]=null,(x:any)=>x.tables[0].rows[0].values[3]=1n,(x:any)=>x.tables[0].rows[0].values[2]="unknown",(x:any)=>x.tables.find((t:any)=>t.name==="semio_audio_document").rows.push({rowid:1n,values:[1n,"stdio.semio.audio",0n,"f32"]})]){const x=structuredClone(d);mutate(x);expect(semioSnapshotFromSqliteDatabase(x)).rejects.toThrow();}expect(semioSnapshotToSqliteDatabase(input,{maxRows:2})).rejects.toThrow();expect(semioSnapshotFromSqliteDatabase(d,{maxValueBytes:1})).rejects.toThrow();const c=new AbortController();c.abort();expect(semioSnapshotToSqliteDatabase(input,{signal:c.signal})).rejects.toThrow();});

import type {SemioArtifact} from "../../../🟦️.ts";
import type {SemioBrepArtifact} from "../../../../../🧊️brep/🧬️schema/🟦️.ts";
import type {SemioMeshArtifact} from "../../../../../🔺️mesh/🧬️schema/🟦️.ts";
import type {SemioModelArtifact} from "../../../../../🏛️model/🧬️schema/🟦️.ts";
import type {SemioValueArtifact} from "../../../../../🔢️value/🧬️schema/🟦️.ts";
import type {SemioDocumentArtifact} from "../../../../../📑️document/🧬️schema/🟦️.ts";
import type {SemioCadArtifact} from "../../../../../📐️cad/🧬️schema/🟦️.ts";
import type {SemioDrawingArtifact} from "../../../../../🖊️drawing/🧬️schema/🟦️.ts";
import type {SemioImageArtifact} from "../../../../../🖼️image/🧬️schema/🟦️.ts";
import type {SemioVideoArtifact} from "../../../../../🎬️video/🧬️schema/🟦️.ts";
import type {SemioAudioArtifact} from "../../../../../🔊️audio/🧬️schema/🟦️.ts";
import type {SemioAnimationArtifact} from "../../../../../🎞️animation/🧬️schema/🟦️.ts";
import type {SemioPresentationArtifact} from "../../../../../📽️presentation/🧬️schema/🟦️.ts";
import type {SemioFlowArtifact} from "../../../../../🌊️flow/🧬️schema/🟦️.ts";
import type {SemioTextArtifact} from "../../../../../🔤️text/🧬️schema/🟦️.ts";
import type {SemioTableArtifact} from "../../../../../📊️table/🧬️schema/🟦️.ts";
import type {SemioGraphArtifact} from "../../../../../🕸️graph/🧬️schema/🟦️.ts";
import type {SemioObjectArtifact} from "../../../../../📦️object/🧬️schema/🟦️.ts";
import type {SemioKitArtifact} from "../../../../../🧰️kit/🧬️schema/🟦️.ts";
test("Semio all nineteen actual artifact consumers share persisted snapshot types",()=>{
  for(const subset of subsets){
    switch(subset.subset){
      case "brep":{const artifact:SemioBrepArtifact=subset;expect(artifact.nextLabel).toBe(0n);break;}
      case "mesh":{const artifact:SemioMeshArtifact=subset;expect(artifact.meshes).toEqual([]);break;}
      case "model":{const artifact:SemioModelArtifact=subset;expect(artifact.spatial).toEqual([]);break;}
      case "value":{const artifact:SemioValueArtifact=subset;expect(artifact.root).toEqual({kind:"null"});break;}
      case "document":{const artifact:SemioDocumentArtifact=subset;expect(artifact.blocks).toEqual([]);break;}
      case "cad":{const artifact:SemioCadArtifact=subset;expect(artifact.entities).toEqual([]);break;}
      case "drawing":{const artifact:SemioDrawingArtifact=subset;expect(artifact.canvas.width.bits).toBe(0n);break;}
      case "image":{const artifact:SemioImageArtifact=subset;expect(artifact.icc).toBe(null);break;}
      case "video":{const artifact:SemioVideoArtifact=subset;expect(artifact.streams).toEqual([]);break;}
      case "audio":{const artifact:SemioAudioArtifact=subset;expect(artifact.channels).toEqual([]);break;}
      case "animation":{const artifact:SemioAnimationArtifact=subset;expect(artifact.timelines).toEqual([]);break;}
      case "presentation":{const artifact:SemioPresentationArtifact=subset;expect(artifact.slides).toEqual([]);break;}
      case "flow":{const artifact:SemioFlowArtifact=subset;expect(artifact.nodes).toEqual([]);break;}
      case "text":{const artifact:SemioTextArtifact=subset;expect(artifact.runs[0]!.marks).toEqual([]);break;}
      case "table":{const artifact:SemioTableArtifact=subset;expect(artifact.columns).toEqual([]);break;}
      case "graph":{const artifact:SemioGraphArtifact=subset;expect(artifact.edges).toEqual([]);break;}
      case "object":{const artifact:SemioObjectArtifact=subset;expect(artifact.transform.rotation.w.bits).toBe(binary64(1).bits);break;}
      case "kit":{const artifact:SemioKitArtifact=subset;expect(artifact.types).toEqual([]);break;}
    }
    const artifact:SemioArtifact={schema:f.schema,subset};expect(artifact.subset).toBe(subset);
  }
});

import type {AnimKeyframeDiff} from "../../../../../🎞️animation/🧬️schema/🔺️diff/🟦️.ts";
import type {SemioAnimationMutation} from "../../../../../🎞️animation/🧬️schema/🧬️mutations/🟦️.ts";
import type {SlideFrameDiff} from "../../../../../📽️presentation/🧬️schema/🔺️diff/🟦️.ts";
import type {RunStyleDiff,DocBlockDiff} from "../../../../../📑️document/🧬️schema/🔺️diff/🟦️.ts";
import type {SemioDocumentMutation} from "../../../../../📑️document/🧬️schema/🧬️mutations/🟦️.ts";
import type {SemioVideoSampleDiff} from "../../../../../🎬️video/🧬️schema/🔺️diff/🟦️.ts";
import type {SemioVideoMutation} from "../../../../../🎬️video/🧬️schema/🧬️mutations/🟦️.ts";
import {parseSemioAudioDiff} from "../../../../../🔊️audio/🧬️schema/🔺️diff/🟦️.ts";
test("Semio owned diff and mutation consumers retain IEEE words and unsigned64",()=>{
  const word={bits:BigInt(f.ownedScalarWords.binary64)},pts=BigInt(f.ownedScalarWords.unsigned64);
  const key:AnimKeyframeDiff={t:word};
  const keyMutation:SemioAnimationMutation={mutation:"setKeyframeTime",timelineIndex:0,channelIndex:0,index:0,t:word};
  const frame:SlideFrameDiff={origin:{x:word,y:word},width:word,height:word};
  const run:RunStyleDiff={size:word};
  const block:DocBlockDiff={kind:"image",width:word,height:null};
  const image:SemioDocumentMutation={mutation:"setImageBlock",path:{segments:[],index:0},imageId:"i",alt:"",width:word,height:null};
  const sample:SemioVideoSampleDiff={pts};
  const sampleMutation:SemioVideoMutation={mutation:"setSampleFlags",streamIndex:0,index:0,pts,key:false};
  const audio=parseSemioAudioDiff({channels:{removed:[],modified:[{index:0,diff:{samples:[{bits:f.ownedScalarWords.binary32}]}}],added:[]}});
  expect(key.t!.bits).toBe(word.bits);expect(keyMutation.t.bits).toBe(word.bits);expect(frame.origin!.x.bits).toBe(word.bits);expect(frame.width!.bits).toBe(word.bits);expect(run.size!.bits).toBe(word.bits);expect(block.width!.bits).toBe(word.bits);expect(image.width!.bits).toBe(word.bits);expect(sample.pts).toBe(pts);expect(sampleMutation.pts).toBe(pts);expect(audio.channels!.modified[0]!.diff.samples![0]!.bits).toBe(f.ownedScalarWords.binary32);
});
