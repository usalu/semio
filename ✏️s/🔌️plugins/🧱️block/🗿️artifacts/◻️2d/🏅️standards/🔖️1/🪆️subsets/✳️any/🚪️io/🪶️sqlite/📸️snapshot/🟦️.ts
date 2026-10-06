/** ◻️ Direct complete Block2d semantic projection and reconstruction. */
import type {Block2dSnapshot} from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import {sqliteOperation,type SqliteDatabase,type SqliteDatabaseOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {ArtifactSqliteProjection} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import * as p from "../../../../../../../../../🧬️schema/🧱️shared/🪶️sqlite/🟦️.ts";
import {BLOCK2D_SQLITE_SCHEMA as SQL} from "./🗄️schema/🟦️.ts";

function capture(value:unknown):Block2dSnapshot{
 const r=p.strict(value,["schema","nodeKind","presentation","handleKinds","handles","compatibility","attributes","authors","camera2d","meta"]),show=p.strict(r.presentation,["shape","radius","width","height","color","iconKind"]);
 return{schema:p.text(r.schema),nodeKind:p.kind(r.nodeKind),presentation:{shape:p.nullable(show.shape,p.text),radius:p.nullable(show.radius,p.word),width:p.nullable(show.width,p.word),height:p.nullable(show.height,p.word),color:p.nullable(show.color,p.text),iconKind:p.nullable(show.iconKind,p.text)},handleKinds:p.list(r.handleKinds,value=>{const q=p.strict(value,["id","name","label","color","defaultWireKind"]);return{id:p.text(q.id),name:p.text(q.name),label:p.text(q.label),color:p.text(q.color),defaultWireKind:p.text(q.defaultWireKind)}}),handles:p.list(r.handles,value=>{const q=p.strict(value,["id","handleKind","angle","radius"]);return{id:p.text(q.id),handleKind:p.text(q.handleKind),angle:p.word(q.angle),radius:p.word(q.radius)}}),compatibility:p.list(r.compatibility,p.compatibility),attributes:p.list(r.attributes,p.attribute),authors:p.list(r.authors,p.author),camera2d:p.camera2d(r.camera2d),meta:p.meta(r.meta)};
}
/** 📤️ Capture each owned field before callbacks then populate all ten authored entities. */
export async function block2dSnapshotToSqliteDatabase(value:Block2dSnapshot,options:SqliteDatabaseOptions={}):Promise<SqliteDatabase>{
 const operation=sqliteOperation(options),s=capture(value);p.checkedRows(5+s.handleKinds.length+s.handles.length+s.compatibility.length+s.attributes.length+s.authors.length,operation.maxRows??1000000);
 const out=await ArtifactSqliteProjection.create(SQL,operation),document=await out.insert("block2_document",[s.schema],1n),k=s.nodeKind,show=s.presentation;
 await out.insert("block2_kind",[document,k.id,k.name,k.label,k.variant,k.description,k.icon,k.unit]);
 await out.insert("block2_presentation",[document,show.shape,...p.wordCells(show.radius),...p.wordCells(show.width),...p.wordCells(show.height),show.color,show.iconKind]);
 for(const[index,row]of s.handleKinds.entries())await out.insert("block2_handle_kind",[document,BigInt(index),row.id,row.name,row.label,row.color,row.defaultWireKind]);
 for(const[index,row]of s.handles.entries())await out.insert("block2_handle",[document,BigInt(index),row.id,row.handleKind,...p.wordCells(row.angle),...p.wordCells(row.radius)]);
 for(const[index,row]of s.compatibility.entries())await out.insert("block2_compatibility",[document,BigInt(index),row.id,row.source,row.target,row.bidirectional?1n:0n]);
 for(const[index,row]of s.attributes.entries())await out.insert("block2_attribute",[document,BigInt(index),row.key,row.value,row.definition]);
 for(const[index,row]of s.authors.entries())await out.insert("block2_author",[document,BigInt(index),row.id,row.name,row.email]);
 await out.insert("block2_camera2d",[document,...p.wordCells(s.camera2d.x),...p.wordCells(s.camera2d.y),...p.wordCells(s.camera2d.zoom)]);await out.insert("block2_meta",[document,s.meta.description]);return out.finish();
}
/** 📥️ Restore full literal fields through the caller’s actual paid row and text frontiers. */
export async function block2dSnapshotFromSqliteDatabase(database:SqliteDatabase,options:SqliteDatabaseOptions={}):Promise<Block2dSnapshot>{
 const reader=await p.Reader.create(database,SQL,sqliteOperation(options)),doc=await reader.cursor(reader.document,1),schema=await doc.text();doc.done();
 const nodeKind=await p.readKind(await reader.singleton("block2_kind")),show=await reader.singleton("block2_presentation"),presentation={shape:await show.optionalText(),radius:show.word(true),width:show.word(true),height:show.word(true),color:await show.optionalText(),iconKind:await show.optionalText()};show.done();
 const handleKinds=[],handles=[],compatibility=[],attributes=[],authors=[];
 for(const row of await reader.group("block2_handle_kind")){const c=await reader.cursor(row,3);handleKinds.push({id:await c.text(),name:await c.text(),label:await c.text(),color:await c.text(),defaultWireKind:await c.text()});c.done()}
 for(const row of await reader.group("block2_handle")){const c=await reader.cursor(row,3);handles.push({id:await c.text(),handleKind:await c.text(),angle:c.requiredWord(),radius:c.requiredWord()});c.done()}
 for(const row of await reader.group("block2_compatibility"))compatibility.push(await p.readCompatibility(await reader.cursor(row,3)));
 for(const row of await reader.group("block2_attribute"))attributes.push(await p.readAttribute(await reader.cursor(row,3)));
 for(const row of await reader.group("block2_author"))authors.push(await p.readAuthor(await reader.cursor(row,3)));
 const camera=await reader.singleton("block2_camera2d"),camera2d={x:camera.requiredWord(),y:camera.requiredWord(),zoom:camera.requiredWord()};camera.done();const note=await reader.singleton("block2_meta"),meta={description:await note.text()};note.done();await reader.finish();return{schema,nodeKind,presentation,handleKinds,handles,compatibility,attributes,authors,camera2d,meta};
}
