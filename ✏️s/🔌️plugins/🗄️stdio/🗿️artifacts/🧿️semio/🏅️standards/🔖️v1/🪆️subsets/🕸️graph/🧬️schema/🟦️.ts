/** 🕸️ The graph artifact owns the same typed nodes, ports, properties and edges as its snapshot. */
import type {SemioGraphSnapshot} from "./📸️snapshot/🟦️.ts";
export type {SemioGraphNode,SemioGraphEdge,SemioGraphPort,SemioGraphPortKind} from "./📸️snapshot/🟦️.ts";
export interface SemioGraphArtifact extends SemioGraphSnapshot {}
