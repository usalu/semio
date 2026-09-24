// #region Header
/** 🛡️ TypeScript twin of the hub's declared access-policy authority (`🦀️.rs`): the same closed-by-default,
 * deny-overrides-allow evaluation over the same declared document (`🔣️.json`). */
// #endregion Header

export type HubAccessRoleV1 = "admin" | "owner" | "author" | "spectator" | "share" | "authenticated";
export type HubAccessActionV1 =
  | "space.create"
  | "space.rename"
  | "space.visibility"
  | "space.archive"
  | "space.delete"
  | "member.upsert"
  | "member.remove"
  | "invite.create"
  | "invite.revoke"
  | "document.announce"
  | "agent.delegate"
  | "document.read"
  | "document.write"
  | "document.check-in"
  | "artifact.create"
  | "blob.read"
  | "blob.write";
export type HubAccessGrantV1 = Readonly<{ effect: "allow" | "deny"; roles: readonly HubAccessRoleV1[]; actions: readonly HubAccessActionV1[]; spaceKinds?: readonly string[] }>;
export type HubAccessPolicyV1 = Readonly<{ schema: "semio.hub.access-policy/v1"; grants: readonly HubAccessGrantV1[] }>;

/** ⚖️ Whether any of `roles` may perform `action` in a space of `spaceKind` under `policy`. */
export function hubAccessPermits(policy: HubAccessPolicyV1, roles: readonly HubAccessRoleV1[], action: HubAccessActionV1, spaceKind?: string): boolean {
  let allowed = false;
  for (const grant of policy.grants) {
    if (!grant.actions.includes(action) || !grant.roles.some((role) => roles.includes(role))) continue;
    if (grant.spaceKinds !== undefined && (spaceKind === undefined || !grant.spaceKinds.includes(spaceKind))) continue;
    if (grant.effect === "deny") return false;
    allowed = true;
  }
  return allowed;
}
