/** 🧬️ TIFF artifact owns exact logical pages. */
import type {TiffIfd} from "./📸️snapshot/🟦️.ts";
export interface TiffArtifact{schema:"stdio.tiff";ifds:TiffIfd[]}
