/** 🌱 Direct `create-artifact` payload. */
export interface SpaceArtifactRow {
  id: string;
  name: string;
  kindId: string;
  schema: string;
  dialect: { artifactKind: string; standard: string; subset: string };
  createdAtMs: number;
  createdBy: string;
  updatedAtMs: number;
  updatedBy: string;
}

export interface CreateArtifact {
  artifact: SpaceArtifactRow;
}
