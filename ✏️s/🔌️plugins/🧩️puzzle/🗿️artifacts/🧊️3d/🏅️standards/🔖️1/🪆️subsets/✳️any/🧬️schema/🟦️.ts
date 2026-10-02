/** 🧊️ Artifact facade for the complete persisted Puzzle3d model. */
export * from "./📸️snapshot/🟦️.ts";
export type {Puzzle3dSnapshot as Puzzle3dArtifact} from "./📸️snapshot/🟦️.ts";
export {parsePuzzle3dSnapshot as parsePuzzle3dArtifact} from "./📸️snapshot/🟦️.ts";
export {puzzle3dSnapshotToSqliteDatabase,puzzle3dSnapshotFromSqliteDatabase} from "./📸️snapshot/🪶️sqlite/🟦️.ts";
