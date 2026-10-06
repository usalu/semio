/** 🔣️ Declared Block2d JSON boundary over its canonical complete parent. */
import * as m from "../../🧬️schema/📸️snapshot/🟦️.ts";
import * as p from "../../../../../../../../🧬️schema/🧱️shared/🚪️io/🔣️json/🟦️.ts";
/** 🔵️ Construct every native optional presentation field. */
function presentation(v:unknown):m.Block2dPresentation{const r=v===undefined?{}:p.row(v);return{shape:p.optionalText(r.shape),radius:p.optionalWord(r.radius),width:p.optionalWord(r.width),height:p.optionalWord(r.height),color:p.optionalText(r.color),iconKind:p.optionalText(r.iconKind)}}
/** 🔘️ Construct the five required literal kind fields. */
function handleKind(v:unknown):m.Block2dHandleKind{const r=p.row(v);return{id:p.text(r.id),name:p.text(r.name),label:p.text(r.label),color:p.text(r.color),defaultWireKind:p.text(r.defaultWireKind)}}
/** 🌱️ Construct exact handle words and unresolved semantic references. */
function handle(v:unknown):m.Block2dHandleTemplate{const r=p.row(v);return{id:p.text(r.id),handleKind:p.text(r.handleKind),angle:p.word(r.angle),radius:p.word(r.radius)}}
/** 📥️ Admit literal transport fields and only their declared file defaults. */
export function block2dFromJsonValue(v:unknown):m.Block2dSnapshot{const r=p.row(v),meta=r.meta===undefined?{}:p.row(r.meta);return m.parseBlock2dSnapshot({schema:p.text(r.schema),nodeKind:p.kind(r.nodeKind),presentation:presentation(r.presentation),handleKinds:p.list(r.handleKinds,handleKind),handles:p.list(r.handles,handle),compatibility:p.list(r.compatibility,p.compatible),attributes:p.list(r.attributes,p.attribute),authors:p.list(r.authors,p.author),camera2d:p.camera2d(r.camera2d),meta:{description:p.defaultText(meta.description)}})}
/** 📥️ Decode the actual declared JSON document. */
export function block2dFromJsonText(text:string):m.Block2dSnapshot{return block2dFromJsonValue(JSON.parse(text))}
/** 📤️ Emit all literal canonical words without numeric normalization. */
export function block2dToJsonText(value:m.Block2dSnapshot):string{const v=m.parseBlock2dSnapshot(value);return JSON.stringify({schema:v.schema,nodeKind:v.nodeKind,presentation:{shape:v.presentation.shape,radius:p.optionalOut(v.presentation.radius),width:p.optionalOut(v.presentation.width),height:p.optionalOut(v.presentation.height),color:v.presentation.color,iconKind:v.presentation.iconKind},handleKinds:v.handleKinds,handles:v.handles.map(h=>({id:h.id,handleKind:h.handleKind,angle:p.out(h.angle),radius:p.out(h.radius)})),compatibility:v.compatibility,attributes:v.attributes,authors:v.authors,camera2d:{x:p.out(v.camera2d.x),y:p.out(v.camera2d.y),zoom:p.out(v.camera2d.zoom)},meta:v.meta})}
/** 🔁️ Fixed point of the literal declared JSON boundary. */
export const block2dCanonicalJsonText=(text:string):string=>block2dToJsonText(block2dFromJsonText(text));

