/** 🧱 `change-object-mesh` payload — mirrors Rust `ChangeObjectMesh` (`../🦀️.rs:13`).
 * `new_mesh_url: Option<String>` carries no `skip_serializing_if`, so the key stays required with
 * a nullable value. */
export interface ChangeObjectMesh {
  id: string;
  newMeshUrl: string | null;
}
