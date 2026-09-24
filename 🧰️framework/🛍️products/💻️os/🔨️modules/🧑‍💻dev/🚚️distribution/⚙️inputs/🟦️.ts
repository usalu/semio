import { frameworkOsLockedPrefsEnv } from "../../../../../🦑️repo/🔨️modules/📚️library/🎮️playground/🔒️preferences/🟦️.ts";
import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import { join } from "node:path";
import policy from "./🔣️.json";

/** 🔏️ Hashes Vite's exposed environment and ignored production dotenv files without logging their values. */
export async function productionBrowserInputHash(root: string, values: Readonly<Record<string, string | undefined>> = process.env): Promise<string> {
  const effective = { ...values, ...frameworkOsLockedPrefsEnv({ ...values, SEMIO_BRAND: undefined }) };
  const environment = Object.keys(effective).filter(key => effective[key] !== undefined && (policy.environmentVariables.includes(key) || policy.environmentPrefixes.some(prefix => key.startsWith(prefix)))).sort().map(key => [key, effective[key]]);
  const files = await Promise.all(policy.files.map(async file => {
    try { return [file, (await readFile(join(root, file))).toString("base64")]; }
    catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return [file, null]; throw error; }
  }));
  return createHash("sha256").update(JSON.stringify({ environment, files })).digest("hex");
}
