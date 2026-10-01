import schema from "../../../🧬️schema/🚀️launch/🔣️.json";

const spec = schema.$defs.LaunchNamePrefixV1;
const pattern = new RegExp(spec.pattern, "u");

/** 🏷️Admits one bounded owner-authored launch name against its schema authority. */
export function parseLaunchNamePrefix(input: unknown): string {
  if (typeof input !== "string" || input.length < spec.minLength || input.length > spec.maxLength * 2 || [...input].length > spec.maxLength || !pattern.test(input)) throw new Error("Invalid owner-authored launch name");
  return input;
}

/** 📜️Reads one canonical double-quoted launch name declaration without accepting duplicates. */
export function declaredLaunchNamePrefix(block: string): string | undefined {
  const rows = block.split(/\r?\n/u).map((line) => line.trim()).filter((line) => /^launch-name-prefix\s*=/u.test(line));
  if (!rows.length) return undefined;
  if (rows.length !== 1) throw new Error("Repeated launch name declaration");
  const value = rows[0]!.slice(rows[0]!.indexOf("=") + 1).trim();
  return parseLaunchNamePrefix(JSON.parse(value));
}
