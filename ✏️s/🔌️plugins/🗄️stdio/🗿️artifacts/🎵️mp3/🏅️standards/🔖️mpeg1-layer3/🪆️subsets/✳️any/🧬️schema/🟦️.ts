import type {Mp3Snapshot} from "./📸️snapshot/🟦️.ts";
import {parseMp3Snapshot} from "./📸️snapshot/🟦️.ts";
/** 🧬️ Artifact state shares the canonical complete snapshot domain. */
export interface Mp3Artifact extends Mp3Snapshot{}
/** 🚪️ Reads the canonical artifact state. */
export function parseMp3Artifact(value:unknown,at="$"):Mp3Artifact{return parseMp3Snapshot(value,at)}
