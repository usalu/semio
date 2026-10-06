/** 📸️ Explicit semantic Remodeling entities under the owning handwritten schema. */
import * as model from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import { ArtifactSqliteProjection, artifactSqliteTables, artifactSqliteCheckpoint, artifactSqliteOrderedRowsControlled, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import { type SqliteDatabase, type SqliteRow, type SqliteValue } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {binary32Value,binary64Value,parseBinary32,parseBinary64,type Binary32,type Binary64} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import { NativeDecodeControl } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
import { REMODELING_SQLITE_SCHEMA } from "./🗄️schema/🟦️.ts";

type Cells=readonly SqliteValue[];
const fail=(message:string):never=>{throw new Error("Remodeling SQLite "+message)};
const text=(value:string):string=>typeof value==="string"?value:fail("requires TEXT");
const uint=(value:number):bigint=>Number.isInteger(value)&&value>=0&&value<=4294967295?BigInt(value):fail("requires unsigned32");
const bool=(value:boolean):bigint=>typeof value==="boolean"?value?1n:0n:fail("requires Boolean");
const signed=(value:bigint):bigint=>typeof value==="bigint"&&value>=-9223372036854775808n&&value<=9223372036854775807n?value:fail("requires signed64");
const unsigned=(value:bigint):Cells=>typeof value==="bigint"&&value>=0n&&value<=18446744073709551615n?[value.toString(),BigInt.asIntN(64,value)]:fail("requires unsigned64");
function float(value:Binary64|Binary32,width:32|64):Cells{const word=width===64?parseBinary64(value).bits:BigInt(parseBinary32(value).bits),query=width===64?binary64Value(value as Binary64):binary32Value(value as Binary32),kind=Number.isNaN(query)?"nan":query===Infinity?"positiveInfinity":query===-Infinity?"negativeInfinity":"finite";return[Number.isNaN(query)?null:query,width===64?BigInt.asIntN(64,word):word,kind]}
const f32=(value:Binary32):Cells=>float(value,32);
const f64=(value:Binary64):Cells=>float(value,64);
const optionalFloat=<T>(value:T|null,encode:(value:T)=>Cells):Cells=>value===null?[null,null,null]:encode(value);
const optionalText=(value:string|null):SqliteValue=>value===null?null:text(value);
const child=(value:model.ArtifactChild):Cells=>[text(value.childId),text(value.target.artifactId),text(value.target.dialect.artifactKind),text(value.target.dialect.standard),text(value.target.dialect.subset)];

async function forecast(value:model.RemodelingSnapshot,options:ArtifactSqliteOptions):Promise<number>{
 let rows=13,work=0;const add=async(count:number)=>{rows+=count;if(!Number.isSafeInteger(rows)||rows>(options.maxRows??1_000_000))fail("row limit");if(++work%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",work,0)};
 await artifactSqliteCheckpoint(options,"projectSnapshot",0,0);
 await add(Object.keys(value.assets).length);
 for(const artifact of Object.values(value.durableArtifacts))await add(1+artifact.chunks.length);
 for(const stream of value.streams)await add(1+stream.frames.length+(stream.source===null?0:1));
 for(const camera of value.calibration.cameras)await add(1+camera.distortion.length);
 await add(value.calibration.rig.length);
 for(const point of value.gcps)await add(1+point.observations.length);
 const buffer=async(value:model.Float32Buffer)=>add(1+(value.kind==="inline"?value.values.length:0));
 if(value.results.sparse!==null){await add(1+(value.results.sparse.colors===null?0:1));await buffer(value.results.sparse.points)}
 if(value.results.dense!==null){await add(1+(value.results.dense.colors===null?0:1)+(value.results.dense.classification===null?0:1));await buffer(value.results.dense.positions);if(value.results.dense.confidence!==null)await buffer(value.results.dense.confidence)}
 if(value.results.trajectory!==null)await add(1+value.results.trajectory.poses.length);
 await add(value.results.tracks.length+(value.results.geo===null?0:1)+(value.results.mesh.watertight===null?0:1));
 if(value.results.qc!==null)await add(1+value.results.qc.warnings.length+(value.results.qc.watertight===null?0:1));
 await artifactSqliteCheckpoint(options,"projectSnapshot",work,work);return rows;
}

/** 📤️ Project every typed persisted field into its literal semantic entity. */
export async function remodelingSnapshotToSqliteDatabase(snapshot:model.RemodelingSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const rows=await forecast(snapshot,options),p=await ArtifactSqliteProjection.create(REMODELING_SQLITE_SCHEMA,options);p.checkRowsAdditional(rows);
 const doc=await p.insert("remodel_document",[text(snapshot.schema),text(snapshot.id)]),calibration=await p.insert("remodel_calibration",[doc]),parameters=await p.insert("remodel_parameters",[doc]),results=await p.insert("remodel_results",[doc]);
 let ordinal=0;for(const [key,value]of Object.entries(snapshot.assets).sort(([a],[b])=>a<b?-1:a>b?1:0))await p.insert("remodel_asset",[doc,BigInt(ordinal++),text(key),...child(value)]);
 ordinal=0;for(const [key,value]of Object.entries(snapshot.durableArtifacts).sort(([a],[b])=>a<b?-1:a>b?1:0)){const id=await p.insert("remodel_durable_artifact",[doc,BigInt(ordinal++),text(key),text(value.kind),optionalText(value.mime),uint(value.width),uint(value.height)]);for(let i=0;i<value.chunks.length;i++)await p.insert("remodel_durable_chunk",[id,BigInt(i),value.chunks[i]!])}
 for(let i=0;i<snapshot.streams.length;i++){const v=snapshot.streams[i]!,id=await p.insert("remodel_stream",[doc,BigInt(i),text(v.id),text(v.name),text(v.kind),optionalText(v.cameraId),...f64(v.syncOffsetMs),...f64(v.fpsHint)]);for(let j=0;j<v.frames.length;j++){const f=v.frames[j]!;await p.insert("remodel_frame",[id,BigInt(j),uint(f.index),...f64(f.timestampMs),text(f.assetId)])}if(v.source!==null){const s=v.source;await p.insert("remodel_video_source",[id,text(s.name),text(s.container),text(s.codec),...f64(s.durationMs),uint(s.frameCount),uint(s.width),uint(s.height)])}}
 for(let i=0;i<snapshot.calibration.cameras.length;i++){const v=snapshot.calibration.cameras[i]!,id=await p.insert("remodel_camera",[calibration,BigInt(i),text(v.id),text(v.label),text(v.model),...f64(v.fx),...f64(v.fy),...f64(v.cx),...f64(v.cy),...f64(v.skew),...optionalFloat(v.rmsReprojectionPx,f32),bool(v.locked)]);if(v.distortion.length!==5)fail("distortion width");for(let j=0;j<5;j++)await p.insert("remodel_camera_distortion",[id,BigInt(j),...f32(v.distortion[j]!)])}
 for(let i=0;i<snapshot.calibration.rig.length;i++){const v=snapshot.calibration.rig[i]!;if(v.rotationWxyz.length!==4||v.translationM.length!==3)fail("rig vector width");await p.insert("remodel_rig_extrinsic",[calibration,BigInt(i),text(v.cameraId),...f32(v.rotationWxyz[0]),...f32(v.rotationWxyz[1]),...f32(v.rotationWxyz[2]),...f32(v.rotationWxyz[3]),...f32(v.translationM[0]),...f32(v.translationM[1]),...f32(v.translationM[2])])}
 for(let i=0;i<snapshot.gcps.length;i++){const v=snapshot.gcps[i]!;if(v.worldPosition.length!==3)fail("world vector width");const id=await p.insert("remodel_ground_control_point",[doc,BigInt(i),text(v.id),text(v.name),...f64(v.worldPosition[0]),...f64(v.worldPosition[1]),...f64(v.worldPosition[2])]);for(let j=0;j<v.observations.length;j++){const o=v.observations[j]!;if(o.pixel.length!==2)fail("pixel vector width");await p.insert("remodel_ground_control_observation",[id,BigInt(j),text(o.streamId),uint(o.frameIndex),...f32(o.pixel[0]),...f32(o.pixel[1])])}}
 const v=snapshot.params;
 await p.insert("remodel_ingest_parameters",[parameters,uint(v.ingest.frameSampleStride),uint(v.ingest.maxFrames),uint(v.ingest.downscaleLongEdgePx),...f32(v.ingest.minSharpness)]);
 await p.insert("remodel_feature_parameters",[parameters,text(v.feature.detector),uint(v.feature.targetCount),uint(v.feature.octaves),...f32(v.feature.edgeThreshold)]);
 await p.insert("remodel_match_parameters",[parameters,text(v.matching.matcher),...f32(v.matching.ratioTest),bool(v.matching.crossCheck),uint(v.matching.sequentialWindow),uint(v.matching.maxPairsPerFrame),bool(v.matching.loopClosure)]);
 await p.insert("remodel_sfm_parameters",[parameters,uint(v.sfm.ransacIterations),...f32(v.sfm.ransacThresholdPx),uint(v.sfm.minTrackLength),uint(v.sfm.baMaxIterations),text(v.sfm.robustLoss),...f32(v.sfm.huberDeltaPx)]);
 await p.insert("remodel_dense_parameters",[parameters,text(v.dense.resolution),uint(v.dense.windowRadiusPx),uint(v.dense.minViewConsistency),...f32(v.dense.confidenceThreshold),uint(v.dense.maxPoints)]);
 await p.insert("remodel_mesh_parameters",[parameters,...f32(v.mesh.tsdfVoxelSizeMm),...f32(v.mesh.tsdfTruncationMm),uint(v.mesh.decimateTargetTriangles),uint(v.mesh.smoothingIterations),bool(v.mesh.textureEnabled),uint(v.mesh.textureSize),bool(v.mesh.guaranteeWatertight),uint(v.mesh.holeFillMaxBoundaryVerts),bool(v.mesh.selfIntersectionCheck)]);
 await p.insert("remodel_motion_parameters",[parameters,bool(v.motion.enabled),uint(v.motion.maxTracks),uint(v.motion.trackWindowPx),...f32(v.motion.minTrackQuality),uint(v.motion.minTrackLengthFrames)]);
 await p.insert("remodel_geo_parameters",[parameters,bool(v.geo.enabled),...optionalFloat(v.geo.originLon,f64),...optionalFloat(v.geo.originLat,f64),...optionalFloat(v.geo.originAlt,f64),...f32(v.geo.gsdM),...f32(v.geo.dsmCellM),...f32(v.geo.dtmFilterRadiusM),uint(v.geo.orthoMaxPx)]);
 const r=snapshot.results,mesh=await p.insert("remodel_mesh_result",[results,...child(r.mesh.mesh),text(r.mesh.source),optionalText(r.mesh.textureAssetId)]);
 const report=async(v:model.WatertightReportSnapshot,mesh:bigint|null,qc:bigint|null)=>p.insert("remodel_watertight_report",[mesh,qc,uint(v.vertexCount),uint(v.triangleCount),uint(v.boundaryEdgeCount),uint(v.boundaryLoopCount),uint(v.nonManifoldEdgeCount),uint(v.nonManifoldVertexCount),uint(v.connectedComponents),bool(v.consistentlyOriented),signed(v.eulerCharacteristic),v.genus===null?null:signed(v.genus),...f64(v.signedVolume),v.selfIntersectionPairs===null?null:uint(v.selfIntersectionPairs),bool(v.closedFallbackUsed),bool(v.isClosed),bool(v.isTwoManifold),bool(v.isWatertight)]);
 if(r.mesh.watertight!==null)await report(r.mesh.watertight,mesh,null);
 const buffer=async(v:model.Float32Buffer,cloud:bigint,slot:string)=>{const id=await p.insert("remodel_float_buffer",[cloud,slot,text(v.kind),v.kind==="content"?text(v.contentId):null,...(v.kind==="content"?unsigned(v.chunkCount):[null,null])]);if(v.kind==="inline")for(let i=0;i<v.values.length;i++)await p.insert("remodel_float_sample",[id,BigInt(i),...f32(v.values[i]!)])};
 if(r.sparse!==null){const id=await p.insert("remodel_cloud",[results,"sparse"]);await buffer(r.sparse.points,id,"points");if(r.sparse.colors!==null)await p.insert("remodel_byte_buffer",[id,"colors",r.sparse.colors])}
 if(r.dense!==null){const id=await p.insert("remodel_cloud",[results,"dense"]);await buffer(r.dense.positions,id,"positions");if(r.dense.confidence!==null)await buffer(r.dense.confidence,id,"confidence");if(r.dense.colors!==null)await p.insert("remodel_byte_buffer",[id,"colors",r.dense.colors]);if(r.dense.classification!==null)await p.insert("remodel_byte_buffer",[id,"classification",r.dense.classification])}
 if(r.trajectory!==null){const id=await p.insert("remodel_trajectory",[results]);for(let i=0;i<r.trajectory.poses.length;i++){const v=r.trajectory.poses[i]!;if(v.rotationWxyz.length!==4||v.translation.length!==3)fail("pose vector width");await p.insert("remodel_camera_pose",[id,BigInt(i),text(v.cameraId),...f32(v.rotationWxyz[0]),...f32(v.rotationWxyz[1]),...f32(v.rotationWxyz[2]),...f32(v.rotationWxyz[3]),...f32(v.translation[0]),...f32(v.translation[1]),...f32(v.translation[2])])}}
 for(let i=0;i<r.tracks.length;i++){const v=r.tracks[i]!;await p.insert("remodel_motion_track",[results,BigInt(i),text(v.id),uint(v.length),text(v.class),...f32(v.meanSpeedMS)])}
 if(r.geo!==null)await p.insert("remodel_geo_products",[results,optionalText(r.geo.dsmAssetId),optionalText(r.geo.dtmAssetId),optionalText(r.geo.orthoAssetId)]);
 if(r.qc!==null){const v=r.qc,id=await p.insert("remodel_qc_report",[results,...f64(v.reprojectionRmsPx),...optionalFloat(v.gcpCheckpointRmse,f64),...f32(v.meanTrackLength),...f32(v.registeredFrameRatio),...f32(v.denseCoverageRatio)]);for(let i=0;i<v.warnings.length;i++)await p.insert("remodel_qc_warning",[id,BigInt(i),text(v.warnings[i]!)]);if(v.watertight!==null)await report(v.watertight,null,id)}
 return p.finish();
}

class Cursor{
 private index:number;
 constructor(readonly row:SqliteRow,start=1){this.index=start;if(row.values[0]!==row.rowid)fail("rowid alias differs")}
 next():SqliteValue{if(this.index>=this.row.values.length)return fail("missing scalar");return this.row.values[this.index++]!}
 text():string{const v=this.next();return typeof v==="string"?v:fail("requires TEXT")}
 integer():bigint{const v=this.next();return typeof v==="bigint"?v:fail("requires INTEGER")}
 uint():number{const v=this.integer();return v>=0n&&v<=4294967295n?Number(v):fail("unsigned32 width")}
 boolean():boolean{const v=this.integer();return v===0n?false:v===1n?true:fail("Boolean width")}
 optionalText():string|null{return this.peek()===null?(this.next(),null):this.text()}
 optionalInteger():bigint|null{return this.peek()===null?(this.next(),null):this.integer()}
 optionalUint():number|null{return this.peek()===null?(this.next(),null):this.uint()}
 peek():SqliteValue{return this.row.values[this.index]??null}
 expect(v:SqliteValue):void{if(this.next()!==v)fail("ownership or unused scalar differs")}
 float(width:32|64):Binary64|Binary32{const query=this.next(),integer=this.integer(),kind=this.text();if(width===32&&(integer<0n||integer>4294967295n))fail("binary32 word width");const v=width===32?{bits:Number(integer)}:{bits:BigInt.asUintN(64,integer)},expected=width===32?binary32Value(v as Binary32):binary64Value(v as Binary64),classification=Number.isNaN(expected)?"nan":expected===Infinity?"positiveInfinity":expected===-Infinity?"negativeInfinity":"finite";if(kind!==classification)fail("IEEE class differs");if(Number.isNaN(expected)){if(query!==null)fail("NaN query must be NULL")}else if(typeof query==="bigint"){if(!Number.isFinite(expected)||!Number.isInteger(expected)||BigInt(expected)!==query)fail("IEEE INTEGER query differs")}else if(typeof query!=="number"||query!==expected)fail("IEEE REAL query differs");return v}
 f32():Binary32{return this.float(32) as Binary32}
 f64():Binary64{return this.float(64) as Binary64}
 optionalFloat(width:32|64):Binary32|Binary64|null{if(this.row.values[this.index+1]===null){this.expect(null);this.expect(null);this.expect(null);return null}return this.float(width)}
 child():model.ArtifactChild{return{childId:this.text(),target:{artifactId:this.text(),dialect:{artifactKind:this.text(),standard:this.text(),subset:this.text()}}}}
 symbol<T extends string>(members:readonly T[]):T{const v=this.text();return members.includes(v as T)?v as T:fail("unknown enum")}
 done():void{if(this.index!==this.row.values.length)fail("extra scalar")}
}
class Reader{
 readonly used=new Set<SqliteRow>();
 readonly indexes=new Map<string,Map<bigint,SqliteRow[]>>();
 private constructor(readonly tables:Map<string,readonly SqliteRow[]>,readonly control:NativeDecodeControl,readonly options:ArtifactSqliteOptions){}
 static async create(database:SqliteDatabase,options:ArtifactSqliteOptions):Promise<Reader>{const rows=await artifactSqliteTables(database,REMODELING_SQLITE_SCHEMA,options),control=new NativeDecodeControl(options.maxValueBytes??268435456,p=>{options.onProgress?.({phase:"reconstructSnapshot",completed:p.completed,total:p.total});return!options.signal?.aborted},options.signal);await control.admitSlots(database.tables.length,64);await control.beginStage(database.tables.reduce((n,t)=>n+t.rows.length,0));return new Reader(new Map(database.tables.map((v,i)=>[v.name,rows[i]!])),control,options)}
 all(name:string):readonly SqliteRow[]{return this.tables.get(name)??fail("missing table "+name)}
 async take(row:SqliteRow,start=1):Promise<Cursor>{if(this.used.has(row))fail("entity has duplicate ownership");await this.control.charge(192);await this.control.step();this.used.add(row);return new Cursor(row,start)}
 async group(name:string,parent:bigint,column=1,ordinal:number|null=2):Promise<SqliteRow[]>{
  const key=name+":"+column;let index=this.indexes.get(key);
  if(index===undefined){const source=this.all(name);await this.control.admitSlots(source.length,48);index=new Map();await this.control.scopedStage(async control=>{await control.beginStage(source.length);for(const row of source){const owner=row.values[column];if(owner!==null){if(typeof owner!=="bigint")return fail("relationship parent is not INTEGER");let group=index!.get(owner);if(group===undefined){group=[];index!.set(owner,group)}group.push(row)}await control.step()}});this.indexes.set(key,index)}
  const rows=index.get(parent)??[];if(ordinal===null)return rows;await this.control.admitSlots(rows.length,8);return artifactSqliteOrderedRowsControlled(rows,ordinal,this.options);
 }
 async one(name:string,parent:bigint,column=1,required=true):Promise<SqliteRow|null>{const rows=await this.group(name,parent,column,null);if(rows.length>1||required&&rows.length!==1)fail("one-to-one cardinality "+name);return rows[0]??null}
 async finish():Promise<void>{await this.control.scopedStage(async control=>{let total=0;for(const rows of this.tables.values())total+=rows.length;await control.beginStage(total);for(const rows of this.tables.values())for(const row of rows){if(!this.used.has(row))fail("unowned entity");await control.step()}await control.checkpoint()})}
}

/** 📥️ Restore every owned field while checking complete relation ownership. */
export async function remodelingSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<model.RemodelingSnapshot>{
 const r=await Reader.create(database,options),docs=r.all("remodel_document");if(docs.length!==1)fail("one document required");const doc=docs[0]!,d=await r.take(doc),schema=d.text(),id=d.text();d.done();
 const cal=await r.one("remodel_calibration",doc.rowid),params=await r.one("remodel_parameters",doc.rowid),results=await r.one("remodel_results",doc.rowid);for(const row of[cal!,params!,results!]){const c=await r.take(row);c.expect(doc.rowid);c.done()}
 const assets:Record<string,model.ArtifactChild>={},durableArtifacts:model.RemodelingDurableArtifactStore={};
 let prior:string|null=null;
 for(const row of await r.group("remodel_asset",doc.rowid)){const c=await r.take(row,3),key=c.text();if(prior!==null&&prior>=key)fail("map keys not unique and ordered");prior=key;Object.defineProperty(assets,key,{value:c.child(),enumerable:true,writable:true,configurable:true});c.done()}
 prior=null;
 for(const row of await r.group("remodel_durable_artifact",doc.rowid)){const c=await r.take(row,3),key=c.text();if(prior!==null&&prior>=key)fail("content keys not unique and ordered");prior=key;const kind=c.text(),mime=c.optionalText(),width=c.uint(),height=c.uint();c.done();const chunks:Uint8Array[]=[];for(const leaf of await r.group("remodel_durable_chunk",row.rowid)){const c=await r.take(leaf,3),bytes=c.next();if(!(bytes instanceof Uint8Array))fail("literal octets required");chunks.push(await r.control.copyBytes(bytes as Uint8Array));c.done()}Object.defineProperty(durableArtifacts,key,{value:{kind,mime,width,height,chunks},enumerable:true,writable:true,configurable:true})}
 const streams:model.MediaStream[]=[];
 for(const row of await r.group("remodel_stream",doc.rowid)){const c=await r.take(row,3),id=c.text(),name=c.text(),kind=c.symbol(model.MEDIA_KINDS),cameraId=c.optionalText(),syncOffsetMs=c.f64(),fpsHint=c.f64();c.done();const frames:model.FrameRef[]=[];for(const frame of await r.group("remodel_frame",row.rowid)){const c=await r.take(frame,3);frames.push({index:c.uint(),timestampMs:c.f64(),assetId:c.text()});c.done()}const sourceRow=await r.one("remodel_video_source",row.rowid,1,false);let source:model.VideoSource|null=null;if(sourceRow!==null){const c=await r.take(sourceRow,2);source={name:c.text(),container:c.text(),codec:c.symbol(model.VIDEO_CODECS),durationMs:c.f64(),frameCount:c.uint(),width:c.uint(),height:c.uint()};c.done()}streams.push({id,name,kind,cameraId,syncOffsetMs,fpsHint,frames,source})}
 const cameras:model.CameraCalibration[]=[],rig:model.RigExtrinsic[]=[];
 for(const row of await r.group("remodel_camera",cal!.rowid)){const c=await r.take(row,3),id=c.text(),label=c.text(),cameraModel=c.text(),fx=c.f64(),fy=c.f64(),cx=c.f64(),cy=c.f64(),skew=c.f64(),rmsReprojectionPx=c.optionalFloat(32) as Binary32|null,locked=c.boolean();c.done();const distortion:Binary32[]=[];for(const value of await r.group("remodel_camera_distortion",row.rowid)){const c=await r.take(value,3);distortion.push(c.f32());c.done()}if(distortion.length!==5)fail("distortion width");cameras.push({id,label,model:cameraModel,fx,fy,cx,cy,skew,distortion:distortion as model.Vec5,rmsReprojectionPx,locked})}
 for(const row of await r.group("remodel_rig_extrinsic",cal!.rowid)){const c=await r.take(row,3);rig.push({cameraId:c.text(),rotationWxyz:[c.f32(),c.f32(),c.f32(),c.f32()],translationM:[c.f32(),c.f32(),c.f32()]});c.done()}
 const gcps:model.GroundControlPoint[]=[];
 for(const row of await r.group("remodel_ground_control_point",doc.rowid)){const c=await r.take(row,3),id=c.text(),name=c.text(),worldPosition:[Binary64,Binary64,Binary64]=[c.f64(),c.f64(),c.f64()];c.done();const observations:model.GcpObservation[]=[];for(const observation of await r.group("remodel_ground_control_observation",row.rowid)){const c=await r.take(observation,3);observations.push({streamId:c.text(),frameIndex:c.uint(),pixel:[c.f32(),c.f32()]});c.done()}gcps.push({id,name,worldPosition,observations})}
 const parameter=async(name:string)=>r.take((await r.one(name,params!.rowid))!,2);
 let c=await parameter("remodel_ingest_parameters");const ingest:model.IngestParams={frameSampleStride:c.uint(),maxFrames:c.uint(),downscaleLongEdgePx:c.uint(),minSharpness:c.f32()};c.done();
 c=await parameter("remodel_feature_parameters");const feature:model.FeatureParams={detector:c.symbol(model.FEATURE_DETECTORS),targetCount:c.uint(),octaves:c.uint(),edgeThreshold:c.f32()};c.done();
 c=await parameter("remodel_match_parameters");const matching:model.MatchParams={matcher:c.symbol(model.MATCHER_KINDS),ratioTest:c.f32(),crossCheck:c.boolean(),sequentialWindow:c.uint(),maxPairsPerFrame:c.uint(),loopClosure:c.boolean()};c.done();
 c=await parameter("remodel_sfm_parameters");const sfm:model.SfmParams={ransacIterations:c.uint(),ransacThresholdPx:c.f32(),minTrackLength:c.uint(),baMaxIterations:c.uint(),robustLoss:c.symbol(model.ROBUST_LOSS_KINDS),huberDeltaPx:c.f32()};c.done();
 c=await parameter("remodel_dense_parameters");const denseParams:model.DenseParams={resolution:c.symbol(model.DENSE_RESOLUTIONS),windowRadiusPx:c.uint(),minViewConsistency:c.uint(),confidenceThreshold:c.f32(),maxPoints:c.uint()};c.done();
 c=await parameter("remodel_mesh_parameters");const meshParams:model.MeshParams={tsdfVoxelSizeMm:c.f32(),tsdfTruncationMm:c.f32(),decimateTargetTriangles:c.uint(),smoothingIterations:c.uint(),textureEnabled:c.boolean(),textureSize:c.uint(),guaranteeWatertight:c.boolean(),holeFillMaxBoundaryVerts:c.uint(),selfIntersectionCheck:c.boolean()};c.done();
 c=await parameter("remodel_motion_parameters");const motion:model.MotionParams={enabled:c.boolean(),maxTracks:c.uint(),trackWindowPx:c.uint(),minTrackQuality:c.f32(),minTrackLengthFrames:c.uint()};c.done();
 c=await parameter("remodel_geo_parameters");const geoParams:model.GeoParams={enabled:c.boolean(),originLon:c.optionalFloat(64) as Binary64|null,originLat:c.optionalFloat(64) as Binary64|null,originAlt:c.optionalFloat(64) as Binary64|null,gsdM:c.f32(),dsmCellM:c.f32(),dtmFilterRadiusM:c.f32(),orthoMaxPx:c.uint()};c.done();
 const readReport=async(parent:bigint,column:1|2):Promise<model.WatertightReportSnapshot|null>=>{const row=await r.one("remodel_watertight_report",parent,column,false);if(row===null)return null;const c=await r.take(row);c.expect(column===1?parent:null);c.expect(column===2?parent:null);const value:model.WatertightReportSnapshot={vertexCount:c.uint(),triangleCount:c.uint(),boundaryEdgeCount:c.uint(),boundaryLoopCount:c.uint(),nonManifoldEdgeCount:c.uint(),nonManifoldVertexCount:c.uint(),connectedComponents:c.uint(),consistentlyOriented:c.boolean(),eulerCharacteristic:c.integer(),genus:c.optionalInteger(),signedVolume:c.f64(),selfIntersectionPairs:c.optionalUint(),closedFallbackUsed:c.boolean(),isClosed:c.boolean(),isTwoManifold:c.boolean(),isWatertight:c.boolean()};c.done();return value};
 const meshRow=(await r.one("remodel_mesh_result",results!.rowid))!;c=await r.take(meshRow,2);const mesh:model.RemodelingMesh={mesh:c.child(),source:c.symbol(model.MESH_SOURCES),textureAssetId:c.optionalText(),watertight:await readReport(meshRow.rowid,1)};c.done();
 const readBuffer=async(cloud:bigint,slot:string,required:boolean):Promise<model.Float32Buffer|null>=>{const rows=await r.group("remodel_float_buffer",cloud,1,null),selected=rows.filter(row=>row.values[2]===slot);if(selected.length>1||required&&selected.length!==1)fail("buffer cardinality");const row=selected[0];if(!row)return null;const c=await r.take(row,3),kind=c.text(),contentId=c.next(),decimal=c.next(),word=c.next();c.done();const samples=await r.group("remodel_float_sample",row.rowid);if(kind==="inline"){if(contentId!==null||decimal!==null||word!==null)fail("inline content fields");const values:Binary32[]=[];for(const row of samples){const c=await r.take(row,3);values.push(c.f32());c.done()}return{kind,values}}if(kind!=="content"||samples.length||typeof contentId!=="string"||typeof decimal!=="string"||!/^(0|[1-9][0-9]{0,19})$/.test(decimal)||typeof word!=="bigint")fail("content buffer fields");const chunkCount=BigInt(decimal as string);if(chunkCount>18446744073709551615n||BigInt.asIntN(64,chunkCount)!==word)fail("content count word differs");return{kind:"content",contentId:contentId as string,chunkCount}};
 const readBytes=async(cloud:bigint,slot:string):Promise<Uint8Array|null>=>{const selected=(await r.group("remodel_byte_buffer",cloud,1,null)).filter(row=>row.values[2]===slot);if(selected.length>1)fail("byte buffer cardinality");const row=selected[0];if(!row)return null;const c=await r.take(row,3),bytes=c.next();c.done();if(!(bytes instanceof Uint8Array))fail("literal octets required");return r.control.copyBytes(bytes as Uint8Array)};
 let sparse:model.SparseCloud|null=null,dense:model.DenseCloud|null=null;
 for(const row of await r.group("remodel_cloud",results!.rowid,1,null)){const c=await r.take(row,2),slot=c.text();c.done();if(slot==="sparse"){if(sparse!==null)fail("duplicate sparse cloud");sparse={points:(await readBuffer(row.rowid,"points",true))!,colors:await readBytes(row.rowid,"colors")}}else if(slot==="dense"){if(dense!==null)fail("duplicate dense cloud");dense={positions:(await readBuffer(row.rowid,"positions",true))!,confidence:await readBuffer(row.rowid,"confidence",false),colors:await readBytes(row.rowid,"colors"),classification:await readBytes(row.rowid,"classification")}}else fail("cloud slot")}
 const trajectoryRow=await r.one("remodel_trajectory",results!.rowid,1,false);let trajectory:model.CameraTrajectory|null=null;
 if(trajectoryRow!==null){c=await r.take(trajectoryRow,2);c.done();const poses:model.CameraPosePreview[]=[];for(const row of await r.group("remodel_camera_pose",trajectoryRow.rowid)){const c=await r.take(row,3);poses.push({cameraId:c.text(),rotationWxyz:[c.f32(),c.f32(),c.f32(),c.f32()],translation:[c.f32(),c.f32(),c.f32()]});c.done()}trajectory={poses}}
 const tracks:model.MotionTrackSummary[]=[];for(const row of await r.group("remodel_motion_track",results!.rowid)){const c=await r.take(row,3);tracks.push({id:c.text(),length:c.uint(),class:c.symbol(model.TRACK_CLASSES),meanSpeedMS:c.f32()});c.done()}
 const geoRow=await r.one("remodel_geo_products",results!.rowid,1,false);let geo:model.GeoProducts|null=null;if(geoRow!==null){c=await r.take(geoRow,2);geo={dsmAssetId:c.optionalText(),dtmAssetId:c.optionalText(),orthoAssetId:c.optionalText()};c.done()}
 const qcRow=await r.one("remodel_qc_report",results!.rowid,1,false);let qc:model.QcReportSnapshot|null=null;
 if(qcRow!==null){c=await r.take(qcRow,2);qc={reprojectionRmsPx:c.f64(),gcpCheckpointRmse:c.optionalFloat(64) as Binary64|null,meanTrackLength:c.f32(),registeredFrameRatio:c.f32(),denseCoverageRatio:c.f32(),warnings:[],watertight:await readReport(qcRow.rowid,2)};c.done();for(const row of await r.group("remodel_qc_warning",qcRow.rowid)){const c=await r.take(row,3);qc.warnings.push(c.text());c.done()}}
 await r.finish();return{schema,id,streams,assets,durableArtifacts,calibration:{cameras,rig},params:{ingest,feature,matching,sfm,dense:denseParams,mesh:meshParams,motion,geo:geoParams},gcps,results:{sparse,dense,mesh,trajectory,tracks,geo,qc}};
}
