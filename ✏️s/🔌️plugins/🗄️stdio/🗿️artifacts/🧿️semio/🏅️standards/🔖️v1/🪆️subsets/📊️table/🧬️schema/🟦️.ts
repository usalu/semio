/** 📊️ The table artifact owns actual typed SemioValue cells rather than a separate value algebra. */
import type {SemioTableSnapshot} from "./📸️snapshot/🟦️.ts";
export type {SemioTableCellKind,SemioTableColumn,SemioTableRow,SemioValue} from "./📸️snapshot/🟦️.ts";
export interface SemioTableArtifact extends SemioTableSnapshot {}
