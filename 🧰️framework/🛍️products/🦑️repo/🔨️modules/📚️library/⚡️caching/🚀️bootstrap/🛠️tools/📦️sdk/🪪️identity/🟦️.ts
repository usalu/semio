import {createHash} from "node:crypto";
/** 🧮️ Keys installed dependency and patch bytes independently of installer implementation edits. */
export function nxToolingInstallIdentityV1(environment: { platform: string; architecture: string; libc: string }, files: ReadonlyMap<string, Uint8Array>): string {
  const hash = createHash("sha256").update(`nx-tooling-install-v3\0${environment.platform}\0${environment.architecture}\0${environment.libc}\0`);
  for (const [name, bytes] of files) hash.update(name + "\0").update(bytes).update("\0");
  return hash.digest("hex");
}

