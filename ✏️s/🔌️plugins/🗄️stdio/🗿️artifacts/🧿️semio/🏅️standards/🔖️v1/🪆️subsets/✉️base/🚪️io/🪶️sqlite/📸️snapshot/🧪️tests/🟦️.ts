/** ✉️ All eighteen owned union branches retain independently queryable typed entities. */
import {expect,test} from "bun:test";
import f from "../🧫️fixtures/🔣️.json";
import type {SemioSnapshot} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {SEMIO_SQLITE_SCHEMA,semioSnapshotToSqliteDatabase,semioSnapshotFromSqliteDatabase} from "../🟦️.ts";
import {binary64} from "@semio-tech/framework";
import {subsets} from "./🧫️branches/🟦️.ts";
test("Semio base eighteen handwritten branch and schema authorities",async()=>{expect(subsets.map(s=>String(s.subset))).toEqual(f.subsets);expect(SEMIO_SQLITE_SCHEMA.startsWith(await Bun.file(new URL("../🗄️.sql",import.meta.url)).text())).toBe(true);});
test("quick::Semio base single selected owner and aggregate header budget",async()=>{const input:SemioSnapshot={schema:f.schema,subset:subsets[13]!},d=await semioSnapshotToSqliteDatabase(input);for(const mutate of[(x:any)=>x.tables[0].rows[0].values[16]=null,(x:any)=>x.tables[0].rows[0].values[3]=1n,(x:any)=>x.tables[0].rows[0].values[2]="unknown",(x:any)=>x.tables.find((t:any)=>t.name==="semio_audio_document").rows.push({rowid:1n,values:[1n,"stdio.semio.audio",0n,"f32"]})]){const x=structuredClone(d);mutate(x);await expect(semioSnapshotFromSqliteDatabase(x)).rejects.toThrow();}await expect(semioSnapshotToSqliteDatabase(input,{maxRows:2})).rejects.toThrow();await expect(semioSnapshotFromSqliteDatabase(d,{maxValueBytes:1})).rejects.toThrow();const c=new AbortController();c.abort();await expect(semioSnapshotToSqliteDatabase(input,{signal:c.signal})).rejects.toThrow();},30000);

import type {SemioArtifact} from "../../../../🧬️schema/🟦️.ts";
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


import {Database} from "bun:sqlite";
import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework";
test("quick::Semio native admission corpus preserves every full union schema through an independent engine",async()=>{
 for(const subset of subsets){
  const expected:SemioSnapshot={schema:f.nativeAdmission.schema,subset};
  const logical=await semioSnapshotToSqliteDatabase(expected);
  const bytes=await exportSqliteDatabase(logical);
  const independent=Database.deserialize(bytes);
  expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
  expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(independent.query("SELECT schema,subset FROM semio_base_document").get()).toEqual({schema:f.nativeAdmission.schema,subset:subset.subset});
  const actual=await semioSnapshotFromSqliteDatabase(await importSqliteDatabase(independent.serialize()));
  independent.close();expect(actual).toEqual(expected);
 }
 console.log("[DEBUG] Semio admission corpus independently retained all eighteen complete union owners");
},30000);
