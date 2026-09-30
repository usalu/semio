import { isLocalSessionProfileIdV1 } from "../../../📇️directory/🎫️local-session/🗄️broker/🧬️schema/🟦️.ts";

/** 🗄️ The local-session broker data root supplied by the selected application. */
export const DEV_LOCAL_HUB_DATA_ENV = "SEMIO_DEV_LOCAL_HUB_DATA";
/** 👤️ The local session profile selected for one development serve. */
export const DEV_LOCAL_HUB_PROFILE_ENV = "SEMIO_DEV_LOCAL_HUB_PROFILE";

/** 🔌️ Application-owned development process contribution. */
export type DevLocalHubProviderV1 = Readonly<{
  owner: Readonly<{ program: string; args: readonly string[] }>;
  defaultProfileId: string;
}>;
export const DEV_LOCAL_HUB_PROVIDER_ENV = "SEMIO_DEV_LOCAL_HUB_PROVIDER";

/** 📥️ A process contribution names its own executable without product discovery or imports. */
export function parseDevLocalHubProviderV1(value: unknown): DevLocalHubProviderV1 {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("dev-local-hub.invalid-provider");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).sort().join(",") !== "defaultProfileId,owner" || !isLocalSessionProfileIdV1(row.defaultProfileId)
    || typeof row.owner !== "object" || row.owner === null || Array.isArray(row.owner)) throw new Error("dev-local-hub.invalid-provider");
  const owner = row.owner as Record<string, unknown>;
  if (Object.keys(owner).sort().join(",") !== "args,program" || typeof owner.program !== "string" || owner.program.length < 1 || owner.program.length > 256
    || !Array.isArray(owner.args) || owner.args.length < 1 || owner.args.length > 32 || owner.args.some((arg) => typeof arg !== "string" || arg.length < 1 || arg.length > 2048)) throw new Error("dev-local-hub.invalid-provider-command");
  return { owner: { program: owner.program, args: owner.args as string[] }, defaultProfileId: row.defaultProfileId };
}
