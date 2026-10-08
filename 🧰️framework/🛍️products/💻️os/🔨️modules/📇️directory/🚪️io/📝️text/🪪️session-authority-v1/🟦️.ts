/** 🚪️ Canonical Directory JSON admission and physical receipt projection. */
import { type DirectorySessionAuthorityV1, DIRECTORY_SESSION_AUTHORITY_MAX_BYTES, DIRECTORY_SESSION_AUTHORITY_SCHEMA_V1 } from "../../../🧬️schema/🪪️session-authority-v1/🟦️.ts";

function boundedText(value: unknown, maximum: number): string | null {
  return typeof value === "string" && value.length > 0 && [...value].length <= maximum && !value.startsWith(" ") && !value.endsWith(" ") && !/[\u0000-\u001f\u007f-\u009f\ud800-\udfff]/u.test(value) ? value : null;
}
/** 🛡️ Parses only the canonical bounded response; session capabilities and ids cannot cross it. */
export function parseDirectorySessionAuthorityJsonV1(source: string): DirectorySessionAuthorityV1 {
  if (new TextEncoder().encode(source).byteLength > DIRECTORY_SESSION_AUTHORITY_MAX_BYTES) throw new Error("directory session authority: capacity");
  const value: unknown = JSON.parse(source);
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("directory session authority: record");
  const row = value as Readonly<Record<string, unknown>>;
  if (Object.keys(row).join(",") !== "schema,sessionBindingSha256,authorizationGeneration,userId,email,displayName,expiresAt,sessionKind") throw new Error("directory session authority: fields");
  const userId = typeof row.userId === "string" && /^[A-Za-z0-9][A-Za-z0-9._:/-]{0,255}$/u.test(row.userId) ? row.userId : null;
  const email = boundedText(row.email, 320);
  const displayName = boundedText(row.displayName, 128);
  if (row.schema !== DIRECTORY_SESSION_AUTHORITY_SCHEMA_V1 || typeof row.sessionBindingSha256 !== "string" || !/^(?!0{64}$)[0-9a-f]{64}$/u.test(row.sessionBindingSha256) || !Number.isSafeInteger(row.authorizationGeneration) || (row.authorizationGeneration as number) < 1 || userId === null || email === null || displayName === null || !Number.isSafeInteger(row.expiresAt) || (row.expiresAt as number) < 1 || (row.sessionKind !== "external" && row.sessionKind !== "development-local")) throw new Error("directory session authority: invalid");
  const result: DirectorySessionAuthorityV1 = { schema: DIRECTORY_SESSION_AUTHORITY_SCHEMA_V1, sessionBindingSha256: row.sessionBindingSha256, authorizationGeneration: row.authorizationGeneration as number, userId, email, displayName, expiresAt: row.expiresAt as number, sessionKind: row.sessionKind };
  if (JSON.stringify(result) !== source) throw new Error("directory session authority: noncanonical");
  return result;
}
