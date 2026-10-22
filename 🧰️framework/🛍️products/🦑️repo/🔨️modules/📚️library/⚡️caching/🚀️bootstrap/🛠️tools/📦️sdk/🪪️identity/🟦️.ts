import {createHash} from "node:crypto";
/** 🧮️ Keys installed dependency and patch bytes independently of installer implementation edits. */
export function nxToolingInstallIdentityV1(environment: { platform: string; architecture: string; libc: string }, files: ReadonlyMap<string, Uint8Array>): string {
  const hash = createHash("sha256").update("nx-tooling-install-v4\0"), field = (value: Uint8Array | string) => { const bytes = typeof value === "string" ? Buffer.from(value) : value; hash.update(`${bytes.byteLength}:`).update(bytes); };
  for (const value of [environment.platform, environment.architecture, environment.libc]) field(value);
  for (const [name, bytes] of files) { field(name); field(bytes); }
  return hash.digest("hex");
}

