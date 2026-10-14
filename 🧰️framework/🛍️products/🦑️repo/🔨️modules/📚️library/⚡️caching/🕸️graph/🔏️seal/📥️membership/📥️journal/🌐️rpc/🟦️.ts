import { createHash } from "node:crypto";
import { lstatSync, readFileSync, realpathSync } from "node:fs";
import { dirname, isAbsolute, join } from "node:path";
import {nxAuthorizerReceiptHoldsV1} from "../../../../../🚀️bootstrap/🛠️tools/🔏️authorizers/📤️receipt/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import neutralSchema from "../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📥️journal/🧬️schema/🔣️.json" with { type: "json" };
import { validateJsonSchemaSubset } from "../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import type { FilesystemJournalScopeV1, FilesystemJournalWitnessV1 } from "../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📥️journal/🟦️.ts";

/** 📥️ A closed journal request remains independent of Nx transport envelopes. */
export type FilesystemJournalRpcRequestV1 = Readonly<{ type: "GET_SEMIO_FILESYSTEM_JOURNAL"; version: 1; scope: FilesystemJournalScopeV1; deadlineUnixMilliseconds:number|null }>;
/** 🔏️ The actual loaded immutable toolkit binds a live witness to its implementation. */
export type FilesystemJournalRpcResponseV1 = Readonly<{ version: 1; toolingIdentity: string; witness: FilesystemJournalWitnessV1 }> | null;
const contract = { ...schema, $defs: { ...schema.$defs, ...neutralSchema.$defs } };

/** 🧬️ Validates repository requests with the neutral scope schema. */
export function readFilesystemJournalRpcRequestV1(value: unknown): FilesystemJournalRpcRequestV1 {
  if (validateJsonSchemaSubset({ ...contract, $ref: "#/$defs/Request" }, value).length) throw Error("Closed filesystem journal request refused");
  return value as FilesystemJournalRpcRequestV1;
}

/** 🕰️ The transport consumes the exact original wall-clock deadline without renewing it. */
export function filesystemJournalRpcRemainingMillisecondsV1(request:FilesystemJournalRpcRequestV1,now:number):number|null{
 if(!Number.isFinite(now))throw Error("Original journal wall clock unavailable");
 return request.deadlineUnixMilliseconds===null?null:request.deadlineUnixMilliseconds-now;
}

/** 📤️ Unwraps only witnesses from the caller's actual immutable toolkit. */
export function readFilesystemJournalRpcResponseV1(value: unknown, toolingIdentity: string): FilesystemJournalWitnessV1 | null {
  if (value === null) return null;
  if (validateJsonSchemaSubset({ ...contract, $ref: "#/$defs/Response" }, value).length) throw Error("Filesystem journal response refused");
  const response = value as Exclude<FilesystemJournalRpcResponseV1, null>;
  return response.toolingIdentity === toolingIdentity ? response.witness : null;
}

/** 🧷️ Reads the immutable marker above the SDK's real loaded location and verifies its actual patch receipt. */
export function loadedFilesystemJournalToolingIdentityV1(implementationDirectory: string): string | null {
  let directory = realpathSync(implementationDirectory), sdk: string | undefined;
  for (;;) {
    const packageFile = join(directory, "package.json");
    try { if (JSON.parse(readFileSync(packageFile, "utf8")).name === "nx") sdk = directory; } catch {}
    const markerFile = join(directory, ".semio-nx-tooling.json");
    try {
      const status = lstatSync(markerFile, { bigint: true });
      if (!status.isFile() || status.isSymbolicLink() || !sdk) return null;
      const marker = JSON.parse(readFileSync(markerFile, "utf8")), version = JSON.parse(readFileSync(join(sdk, "package.json"), "utf8")).version;
      if (marker.version !== 1 || !/^[a-f0-9]{64}$/.test(marker.digest) || directory.split(/[\\/]/).at(-1) !== marker.digest) return null;
      if(!nxAuthorizerReceiptHoldsV1(directory,marker.authorizers))return null;
      const receipt = marker.patches?.[`nx@${version}`];
      if (receipt?.version !== 1 || !receipt.files || typeof receipt.files !== "object" || !Object.keys(receipt.files).length) return null;
      for (const [path, expected] of Object.entries(receipt.files)) {
        if (isAbsolute(path) || /[\\\0]/u.test(path) || path.split("/").some(part => !part || part === "." || part === "..") || typeof expected !== "string" || !/^[a-f0-9]{64}$/.test(expected)) return null;
        const file = join(sdk, path), before = lstatSync(file, { bigint: true });
        if (!before.isFile() || before.isSymbolicLink()) return null;
        const bytes = readFileSync(file), after = lstatSync(file, { bigint: true });
        if (before.dev !== after.dev || before.ino !== after.ino || before.size !== after.size || before.mtimeNs !== after.mtimeNs || before.ctimeNs !== after.ctimeNs || createHash("sha256").update(bytes).digest("hex") !== expected) return null;
      }
      const after = lstatSync(markerFile, { bigint: true });
      if (status.dev !== after.dev || status.ino !== after.ino || status.size !== after.size || status.mtimeNs !== after.mtimeNs || status.ctimeNs !== after.ctimeNs) return null;
      return marker.digest;
    } catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") return null; }
    const parent = dirname(directory); if (parent === directory) return null; directory = parent;
  }
}
