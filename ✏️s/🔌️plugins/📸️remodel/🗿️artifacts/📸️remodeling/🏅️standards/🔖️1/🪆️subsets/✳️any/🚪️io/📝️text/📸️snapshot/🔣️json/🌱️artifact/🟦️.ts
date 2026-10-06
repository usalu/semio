/** 🔣️ Declared Remodeling artifact JSON admission. */
import {decodeRemodelingSnapshot} from "../🟦️.ts";
import type {RemodelingArtifact} from "../../../../../🧬️schema/🟦️.ts";

/** 🔣️ Admit only the explicitly declared file transport scalars. */
export const decodeRemodelingArtifact=(v:unknown):RemodelingArtifact=>decodeRemodelingSnapshot(v);
