/** 📸️ Home's persisted snapshot has the same literal owned fields as its artifact. */
import {parseSHomeArtifact,type SHomeArtifact} from "../🟦️.ts";
export interface SHomeSnapshot extends SHomeArtifact{}
/** 🚪️ Retain every native unsigned64 generation without numeric narrowing. */
export function parseSHomeSnapshot(value:unknown,at="$"):SHomeSnapshot{return parseSHomeArtifact(value,at);}
