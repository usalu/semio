/** 🖐️ Complete Block5d persisted parent with the actual shared Block record authority. */
import * as shared from "../../../../../../../◻️2d/🧬️schema/🧱️shared/🟦️.ts";
import * as p from "../../../../../../../◻️2d/🧬️schema/🧱️shared/📐️scalar/🟦️.ts";
import type {Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
export * from "../../../../../../../◻️2d/🧬️schema/🧱️shared/🟦️.ts";
export interface Block5dPart2d{shape:string|null;radius:Binary64|null;width:Binary64|null;height:Binary64|null;color:string|null;iconKind:string|null}
export interface Block5dPart3d{orientation:p.BlockVector4|null;scale:p.BlockVector3|null}
export interface Block5dGripKind{id:string;name:string;label:string;color:string;defaultRopeKind:string}
export interface Block5dGripTemplate{id:string;gripKind:string;angle:Binary64;radius2d:Binary64;position:p.BlockVector3;direction:p.BlockVector3;radius3d:Binary64}
export interface Block5dSnapshot{
 /** @state artifact */ schema:string;
 /** @state artifact */ partKind:shared.BlockKindIdentity;
 /** @state artifact */ part2d:Block5dPart2d;
 /** @state artifact */ part3d:Block5dPart3d;
 /** @state artifact */ representations:shared.BlockRepresentation[];
 /** @state artifact */ gripKinds:Block5dGripKind[];
 /** @state artifact */ grips:Block5dGripTemplate[];
 /** @state artifact */ compatibility:shared.BlockCompatibilityRule[];
 /** @state artifact */ attributes:shared.BlockAttribute[];
 /** @state artifact */ authors:shared.BlockAuthor[];
 /** @state artifact */ camera2d:shared.BlockCamera2d;
 /** @state artifact */ camera3d:shared.BlockCamera3d;
 /** @state artifact */ meta:shared.BlockMeta;
}
/** 🔵️ Admit each optional native two-dimensional presentation field. */
export function parseBlock5dPart2d(v:unknown):Block5dPart2d{const r=p.row(v);return{shape:p.optional(r.shape,p.text),radius:p.optional(r.radius,p.word),width:p.optional(r.width,p.word),height:p.optional(r.height,p.word),color:p.optional(r.color,p.text),iconKind:p.optional(r.iconKind,p.text)}}
/** 🧭️ Admit exact optional pose words with one native absence representation. */
export function parseBlock5dPart3d(v:unknown):Block5dPart3d{const r=p.row(v);return{orientation:p.optional(r.orientation,p.vector4),scale:p.optional(r.scale,p.vector3)}}
/** 🔘️ Admit the required literal grip-kind fields. */
export function parseBlock5dGripKind(v:unknown):Block5dGripKind{const r=p.row(v);return{id:p.text(r.id),name:p.text(r.name),label:p.text(r.label),color:p.text(r.color),defaultRopeKind:p.text(r.defaultRopeKind)}}
/** 🌱️ Admit all actual flat template words and literal grip ownership. */
export function parseBlock5dGripTemplate(v:unknown):Block5dGripTemplate{const r=p.row(v);return{id:p.text(r.id),gripKind:p.text(r.gripKind),angle:p.word(r.angle),radius2d:p.word(r.radius2d),position:p.vector3(r.position),direction:p.vector3(r.direction),radius3d:p.word(r.radius3d)}}
/** 📸️ Admit every actual persisted field without file defaults. */
export function parseBlock5dSnapshot(v:unknown):Block5dSnapshot{const r=p.row(v);return{schema:p.text(r.schema),partKind:shared.parseBlockKindIdentity(r.partKind),part2d:parseBlock5dPart2d(r.part2d),part3d:parseBlock5dPart3d(r.part3d),representations:p.list(r.representations,shared.parseBlockRepresentation),gripKinds:p.list(r.gripKinds,parseBlock5dGripKind),grips:p.list(r.grips,parseBlock5dGripTemplate),compatibility:p.list(r.compatibility,shared.parseBlockCompatibilityRule),attributes:p.list(r.attributes,shared.parseBlockAttribute),authors:p.list(r.authors,shared.parseBlockAuthor),camera2d:shared.parseBlockCamera2d(r.camera2d),camera3d:shared.parseBlockCamera3d(r.camera3d),meta:shared.parseBlockMeta(r.meta)}}
