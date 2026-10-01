import type {Id3v1Tag,Id3v2Tag,Mp3Frame} from "../📸️snapshot/🟦️.ts";
import {parseId3v1Tag,parseId3v2Tag,parseMp3Frame,Mp3SnapshotRefusal} from "../📸️snapshot/🟦️.ts";
/** 🔺️ Sparse tag changes retain absent, cleared and set states. */
export interface Mp3Diff{id3v2?:Id3v2Tag|null;frames?:Mp3Frame[];id3v1?:Id3v1Tag|null}
/** 🪆️ Reads owned sparse changes without collapsing tag clearance into absence. */
export function parseMp3Diff(value:unknown,at="$"):Mp3Diff{if(value===null||typeof value!=="object"||Array.isArray(value))throw new Mp3SnapshotRefusal(at,"expected an object");const row=value as Record<string,unknown>;const result:Mp3Diff={};if(Object.hasOwn(row,"id3v2"))result.id3v2=row.id3v2===null?null:parseId3v2Tag(row.id3v2,`${at}.id3v2`);if(Object.hasOwn(row,"id3v1"))result.id3v1=row.id3v1===null?null:parseId3v1Tag(row.id3v1,`${at}.id3v1`);if(Object.hasOwn(row,"frames")){if(!Array.isArray(row.frames))throw new Mp3SnapshotRefusal(`${at}.frames`,"expected an array");result.frames=row.frames.map((frame,index)=>parseMp3Frame(frame,`${at}.frames[${index}]`))}return result}
