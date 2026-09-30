import schema from "./🔣️.json";

/** 🎭️ A principal's role for one directory request. */
export type DirectoryAccessRoleV1 = "admin" | "owner" | "author" | "spectator" | "share" | "authenticated" | "agent-reader" | "agent-editor";
/** 🎬️ A directory, document or artifact authorization decision. */
export type DirectoryAccessActionV1 = "space.create" | "space.rename" | "space.visibility" | "space.archive" | "space.delete" | "member.upsert" | "member.remove" | "invite.create" | "invite.revoke" | "document.announce" | "agent.delegate" | "document.read" | "document.write" | "document.check-in" | "artifact.create" | "blob.read" | "blob.write" | "preference.record" | "preference.read";
/** 🎫️ One grant evaluated with deny precedence. */
export type DirectoryAccessGrantV1 = Readonly<{ effect: "allow" | "deny"; roles: readonly DirectoryAccessRoleV1[]; actions: readonly DirectoryAccessActionV1[]; spaceKinds?: readonly string[] }>;
/** 📜️ The directory's shared policy document. */
export type DirectoryAccessPolicyV1 = Readonly<{ schema: "semio.os.directory.access-policy/v1"; grants: readonly DirectoryAccessGrantV1[] }>;

function exactRecord(value: unknown, required: readonly string[], optional: readonly string[] = []): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("access policy requires an object");
  const row = value as Record<string, unknown>;
  if (required.some((key) => !Object.hasOwn(row, key)) || Object.keys(row).some((key) => !required.includes(key) && !optional.includes(key))) throw new Error("access policy fields differ from its schema");
  return row;
}

function choices(value: unknown, allowed: readonly string[]): string[] {
  if (!Array.isArray(value) || value.length === 0 || value.some((item) => typeof item !== "string" || !allowed.includes(item)) || new Set(value).size !== value.length) throw new Error("access policy choices must be known, nonempty and unique");
  return [...value];
}

/** 📥️ Validates the shared schema without exporting a validator library. */
export function parseDirectoryAccessPolicyV1(value: unknown): DirectoryAccessPolicyV1 {
  const row = exactRecord(value, ["schema", "grants"]), contract = schema.$defs.DirectoryAccessPolicyV1.properties;
  if (row.schema !== contract.schema.const || !Array.isArray(row.grants) || row.grants.length < contract.grants.minItems || row.grants.length > contract.grants.maxItems) throw new Error("access policy schema or grant count is invalid");
  const grants = row.grants.map((value): DirectoryAccessGrantV1 => {
    const grant = exactRecord(value, ["effect", "roles", "actions"], ["spaceKinds"]);
    if (grant.effect !== "allow" && grant.effect !== "deny") throw new Error("access policy effect is invalid");
    const roles = choices(grant.roles, schema.$defs.DirectoryAccessRoleV1.enum) as DirectoryAccessRoleV1[];
    const actions = choices(grant.actions, schema.$defs.DirectoryAccessActionV1.enum) as DirectoryAccessActionV1[];
    return { effect: grant.effect, roles, actions, ...(Object.hasOwn(grant, "spaceKinds") ? { spaceKinds: choices(grant.spaceKinds, schema.$defs.DirectoryAccessGrantV1.properties.spaceKinds.items.enum) } : {}) };
  });
  return { schema: "semio.os.directory.access-policy/v1", grants };
}

/** ⚖️ Evaluates one request against the shared closed-by-default policy. */
export function directoryAccessPermits(policy: DirectoryAccessPolicyV1, roles: readonly DirectoryAccessRoleV1[], action: DirectoryAccessActionV1, spaceKind?: string): boolean {
  let allowed = false;
  for (const grant of policy.grants) {
    if (!grant.actions.includes(action) || !grant.roles.some((role) => roles.includes(role))) continue;
    if (grant.spaceKinds !== undefined && (spaceKind === undefined || !grant.spaceKinds.includes(spaceKind))) continue;
    if (grant.effect === "deny") return false;
    allowed = true;
  }
  return allowed;
}
