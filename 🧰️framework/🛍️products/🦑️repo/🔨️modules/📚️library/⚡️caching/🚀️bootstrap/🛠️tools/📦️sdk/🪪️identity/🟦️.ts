import {createHash} from "node:crypto";
/** 🧷️ Exact device and inode evidence retained from a bigint stat. */
export interface NxSdkPhysicalFileIdentityV1{readonly device:bigint;readonly inode:bigint}
/** 🧮️ Keys installed dependency and patch bytes independently of installer implementation edits and of the authorizer bundle that runs beside the installation. */
export function nxToolingInstallIdentityV1(environment: { platform: string; architecture: string; libc: string }, files: ReadonlyMap<string, Uint8Array>): string {
  const hash = createHash("sha256").update("nx-tooling-install-v5\0"), field = (value: Uint8Array | string) => { const bytes = typeof value === "string" ? Buffer.from(value) : value; hash.update(`${bytes.byteLength}:`).update(bytes); };
  for (const value of [environment.platform, environment.architecture, environment.libc]) field(value);
  for (const [name, bytes] of files) { field(name); field(bytes); }
  return hash.digest("hex");
}
/** 🔖️ Keys one compiled authorizer bundle by the dependency generation it runs beside and by the exact digest manifests of its source and compiler closures, so a library edit changes this key and never the installation key. */
export function nxAuthorizerBundleIdentityV1(dependencies: string, files: ReadonlyMap<string, Uint8Array>): string {
  if (!/^[a-f0-9]{64}$/u.test(dependencies)) throw new Error("Exact dependency generation identity required");
  const hash = createHash("sha256").update("nx-authorizer-bundle-v1\0"), field = (value: Uint8Array | string) => { const bytes = typeof value === "string" ? Buffer.from(value) : value; hash.update(`${bytes.byteLength}:`).update(bytes); };
  field(dependencies);
  for (const [name, bytes] of files) { field(name); field(bytes); }
  return hash.digest("hex");
}
