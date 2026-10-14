/** 🏗 `change-object-kind` payload — mirrors Rust `ChangeObjectKind` (`../🦀️.rs:13`).
 * `new_object_kind: Option<String>` carries no `skip_serializing_if`, so the key stays required
 * with a nullable value. */
export interface ChangeObjectKind {
  id: string;
  newObjectKind: string | null;
}
