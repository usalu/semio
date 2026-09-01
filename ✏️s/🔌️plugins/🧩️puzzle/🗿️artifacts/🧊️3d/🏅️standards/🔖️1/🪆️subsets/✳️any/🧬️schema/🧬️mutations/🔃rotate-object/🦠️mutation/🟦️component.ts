/** 🔃 `rotate-object` payload — mirrors Rust `RotateObject` (`../🦀️.rs:13`). `new_orientation:
 * Option<[f64; 4]>` carries no `skip_serializing_if`, so the key stays required with a nullable
 * value. */
export interface RotateObject {
  id: string;
  newOrientation: [number, number, number, number] | null;
}
