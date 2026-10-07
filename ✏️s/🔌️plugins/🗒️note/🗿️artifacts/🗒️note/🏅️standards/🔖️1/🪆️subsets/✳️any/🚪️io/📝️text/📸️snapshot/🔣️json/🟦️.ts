/** 🔣️ Note native JSON scalar binding before intrinsic admission. */
import {binary64} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import {parseNoteArtifact,type NoteArtifact} from "../../../../🧬️schema/🟦️.ts";
import {parseNoteDiff,type NoteDiff} from "../../../../🧬️schema/🔺️diff/🟦️.ts";
import {parseNoteSnapshot,type NoteSnapshot} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
const scalarFields=new Set(["x","y","width","height","rotation","fontSize","strokeWidth","gridSpacing","gridSubdivisions","gridOpacity","snapGridSpacing","pencilWidth","eraserRadius"]);
function bind(value:unknown,field=""):unknown{
 if(typeof value==="number"&&(scalarFields.has(field)||field==="points"||field==="color")){if(!Number.isFinite(value))throw Error("Note native JSON requires finite scalar");return binary64(value);}
 if(Array.isArray(value))return value.map(member=>bind(member,field));
 if(value!==null&&typeof value==="object")return Object.fromEntries(Object.entries(value).map(([key,member])=>[key,bind(member,key)]));
 return value;
}
/** 📥️ Bind native Note document numeric roles and validate the intrinsic document. */
export function parseNoteArtifactJson(value:unknown):NoteArtifact{return parseNoteArtifact(bind(value));}
/** 📥️ Bind native Note delta numeric roles, including replacement blocks. */
export function parseNoteDiffJson(value:unknown):NoteDiff{return parseNoteDiff(bind(value));}

/** 📸️ Bind native Note snapshot numeric roles and validate the intrinsic snapshot. */
export function parseNoteSnapshotJson(value:unknown):NoteSnapshot{return parseNoteSnapshot(bind(value));}
