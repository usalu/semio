/** 🌀 `rotate-target-volume` payload — mirrors Rust `RotateTargetVolume` (`../🦀️.rs:13`).
 * `new_orientation: Option<[f64; 4]>` carries no `skip_serializing_if`, so the key stays required
 * with a nullable value. */
export interface RotateTargetVolume {
  id: string;
  newOrientation: [number, number, number, number] | null;
}
