/** 🖋️ `edit-object-label` payload — mirrors Rust `EditObjectLabel` (`../🦀️.rs:13`). `new_label:
 * Option<String>` carries no `skip_serializing_if`, so the key stays required with a nullable
 * value. */
export interface EditObjectLabel {
  id: string;
  newLabel: string | null;
}
