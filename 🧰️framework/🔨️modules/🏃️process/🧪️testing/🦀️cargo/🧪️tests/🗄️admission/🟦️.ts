/** 🗄️ Admits one Cargo test law the way the repository caller does: the original process invocation, a compiler-workspace reservation on a fresh fixed image bound to that invocation's control, and the configured Source boundary.
 * @see ../../../../../📦️artifacts/🏗️native-build/🧪️tests/🦀️cargo/🟦️.ts
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { receiveScriptProcessInvocation } from "../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { lawSourceBoundaryV1, withIssuedLawStorageV1, type LawStorageV1 } from "../../../../📦️artifacts/🏗️native-build/🧪️tests/🦀️cargo/🟦️.ts";
import type { CargoTestAdmissionV1 } from "../../🟦️.ts";

/** 🎟️ Retains the issued storage, control and Source boundary every Cargo test run requires until the run settles. */
export function withCargoAdmissionV1<T>(body: (admission: CargoTestAdmissionV1, law: LawStorageV1) => Promise<T>): Promise<T> {
  return receiveScriptProcessInvocation(process.env, (original) => withIssuedLawStorageV1(original, new AbortController().signal, (law, control) => body({ workspaceRoot: process.cwd(), control, storage: law.storage, sourceBoundary: lawSourceBoundaryV1 }, law)));
}

/** 🦀️ Writes the one-crate dependency-free workspace the compiler planner reads before any selected command runs, and returns its manifest. */
export function writeOwnerPackageV1(root: string, name = "owner-package"): string {
  mkdirSync(root, { recursive: true });
  const manifest = join(root, "Cargo.toml");
  writeFileSync(manifest, `[workspace]\n[package]\nname = "${name}"\nversion = "0.0.0"\nedition = "2021"\n[lib]\npath = "lib.rs"\n`);
  writeFileSync(join(root, "lib.rs"), "");
  writeFileSync(join(root, "Cargo.lock"), `version = 4\n[[package]]\nname = "${name}"\nversion = "0.0.0"\n`);
  return manifest;
}
