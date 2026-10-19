/// <reference path="./🗄️.d.ts" />
import sql from "./🗄️.sql" with {type:"text"};
import type {EnergyModelSnapshot} from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import type { EnergyModel } from "../../../🧬️schema/📸️snapshot/⚡️model/🟦️.ts";
import { parseArtifactLink, type ArtifactLink, type ArtifactRef } from "@semio-tech/framework/schema/artifact-reference";
import { ArtifactSqliteProjection, artifactSqliteCheckpoint } from "@semio-tech/framework/io/sqlite-snapshot/artifact";
import type { SqliteDatabase, SqliteValue } from "@semio-tech/framework/io/sqlite-snapshot";
import {projectEnvelope,reconstructEnvelope} from "./🏘️envelope/🟦️.ts";
import {projectSystems,reconstructSystems} from "./⚙️systems/🟦️.ts";
import {projectSchedules,reconstructSchedules} from "./🗓️schedules/🟦️.ts";
import { DOCUMENT, MONTH, uint, write, emit, Row, Reader } from "./🧾️row-codec/🟦️.ts";
import type { Options } from "./🧾️row-codec/🟦️.ts";
export const ENERGY_MODEL_SQLITE_SCHEMA:string=sql;
async function child(out:ArtifactSqliteProjection,table:string,value:EnergyModelSnapshot["structure"]):Promise<void>{const t=value.target;await out.insert(table,[value.childId,t.artifactId,t.dialect.artifactKind,t.dialect.standard,t.dialect.subset],1n)}
async function link(out:ArtifactSqliteProjection,table:string,value:ArtifactLink):Promise<void>{const v=parseArtifactLink(value),t=v.target,p=v.pin;const payload:SqliteValue[]=p.kind==="head"?[null,null,null,null,null]:p.kind==="checkpoint"?[p.id,null,null,null,null]:[null,p.blob.hash,p.blob.size>>32n,p.blob.size&0xffffffffn,p.blob.mediaType];await out.insert(table,[t.artifactId,t.dialect.artifactKind,t.dialect.standard,t.dialect.subset,v.role,p.kind,...payload],1n)}
/** 📤️ Projects the literal Energy model into authored domain entities. */
export async function energyModelToSqliteDatabase(s:EnergyModelSnapshot,options:Options={}):Promise<SqliteDatabase>{
 await artifactSqliteCheckpoint(options,"projectSnapshot",0,0);const m=s.model,out=await ArtifactSqliteProjection.create(sql,options);await write(out,"energy_document",[s.schema,m.name,m.version,m.site.latitude_deg,m.site.longitude_deg,m.site.elevation_m,m.site.time_zone_hours,m.site.north_axis_deg,m.ground_temperature.deep_c,uint(m.run_period.start_month,255),uint(m.run_period.start_day,255),uint(m.run_period.end_month,255),uint(m.run_period.end_day,255),uint(m.run_period.year,65535)],DOCUMENT,options,1n);await child(out,"energy_structure_child",s.structure);await child(out,"energy_zones_child",s.zones);if(s.referencedModel!==null)await link(out,"energy_referenced_model_link",s.referencedModel);if(s.weatherLink!==null)await link(out,"energy_weather_link",s.weatherLink);
 if(m.ground_temperature.building_surface_c.length!==12||m.ground_temperature.shallow_c.length!==12)throw Error("Energy ground temperatures have exactly12months");for(let n=0;n<12;n++)await emit(out,"energy_ground_month",n,[m.ground_temperature.building_surface_c[n]!,m.ground_temperature.shallow_c[n]!],MONTH,options);
 await projectEnvelope(m,out,options);await projectSystems(m,out,options);await projectSchedules(m.schedules,out,options);return out.finish();
}
function reference(row:Row,start:number):ArtifactRef{return{artifactId:row.text(start),dialect:{artifactKind:row.text(start+1),standard:row.text(start+2),subset:row.text(start+3)}}}
async function readChild(r:Reader,table:string):Promise<EnergyModelSnapshot["structure"]>{const row=(await r.one(table,6,[],true))!;return{childId:row.text(1),target:reference(row,2)}}
async function readLink(r:Reader,table:string):Promise<ArtifactLink|null>{const row=await r.one(table,12,[],false);if(row===null)return null;const kind=row.text(6);let pin:ArtifactLink["pin"];if(kind==="head"){if(row.row.values.slice(7).some(v=>v!==null))throw Error("Energy head pin has extra payload");pin={kind}}else if(kind==="checkpoint"){if(row.row.values.slice(8).some(v=>v!==null))throw Error("Energy checkpoint pin has extra payload");pin={kind,id:row.text(7)}}else if(kind==="snapshot"){if(row.row.values[7]!==null)throw Error("Energy snapshot pin has checkpoint payload");pin={kind,blob:{hash:row.text(8),size:(BigInt(row.u(9))<<32n)|BigInt(row.u(10)),mediaType:row.text(11)}}}else throw Error("Energy pin tag differs");return{target:reference(row,1),role:row.text(5),pin}}
/** 📥️ Reconstructs typed Energy state without engineering inference or native-wire carriers. */
export async function energyModelFromSqliteDatabase(database:SqliteDatabase,options:Options={}):Promise<EnergyModelSnapshot>{
 const r=await Reader.create(database,options),root=(await r.one("energy_document",15,DOCUMENT,true))!,months=await r.rows("energy_ground_month",1n,5,MONTH);if(months.length!==12)throw Error("Energy ground temperatures have exactly12months");const envelope=await reconstructEnvelope(r),systems=await reconstructSystems(r),schedules=await reconstructSchedules(r);
 const model:EnergyModel={name:root.text(2),version:root.text(3),site:{latitude_deg:root.float(4),longitude_deg:root.float(5),elevation_m:root.float(6),time_zone_hours:root.float(7),north_axis_deg:root.float(8)},ground_temperature:{building_surface_c:months.map(v=>v.float(3)),shallow_c:months.map(v=>v.float(4)),deep_c:root.float(9)},run_period:{start_month:root.u(10,255n),start_day:root.u(11,255n),end_month:root.u(12,255n),end_day:root.u(13,255n),year:root.u(14,65535n)},...envelope,...systems,schedules};const result:EnergyModelSnapshot={schema:root.text(1),model,structure:await readChild(r,"energy_structure_child"),zones:await readChild(r,"energy_zones_child"),referencedModel:await readLink(r,"energy_referenced_model_link"),weatherLink:await readLink(r,"energy_weather_link")};await r.finish();return result;
}
