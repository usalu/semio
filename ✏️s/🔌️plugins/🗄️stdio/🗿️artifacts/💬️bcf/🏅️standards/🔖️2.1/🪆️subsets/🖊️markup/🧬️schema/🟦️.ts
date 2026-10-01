/** 🧬️ BCF artifact state shares its canonical persisted domain. */
import {parseBcfSnapshot,type BcfSnapshot} from "./📸️snapshot/🟦️.ts";
export interface BcfArtifact extends BcfSnapshot{}
/** 🛂️ Validates artifact state through the same owned snapshot domain. */
export function parseBcfArtifact(value:unknown):BcfArtifact{return parseBcfSnapshot(value)}
