import { createHash } from "node:crypto";
import { readdirSync } from "node:fs";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

const POLICY = schema.definitions.Policy.const, TEXTS = schema.definitions.Texts.const;

/** 🗝️ The file stem a stored record of one owner root carries: the first hex digits of the sha256 of the root, so the key names the owner and nothing of the process, session, machine or generation that wrote it. */
export function ownerRecordKeyV1(root) { return createHash("sha256").update(root).digest("hex").slice(0, POLICY.keyHexDigits); }

/** 📋️ Which of the owner roots already have a stored record in `directory`, from one listing: the roots with a record, and how many roots were asked. A directory that cannot be listed holds no record. */
export function ownerRecordsPresentV1(directory, roots) {
  let names;
  try { names = new Set(readdirSync(directory)); } catch { names = new Set(); }
  const present = new Set(roots.filter(root => names.has(ownerRecordKeyV1(root) + POLICY.recordSuffix)));
  return Object.freeze({ present, total: roots.length });
}

/** 💬️ The one line that tells a reader how many owner records are present, in the given language (English or German). */
export function ownerRecordsPresentTextV1(language, census) {
  const row = TEXTS[language === "de" ? "de" : "en"];
  return row.present.replace("{present}", String(census.present.size)).replace("{total}", String(census.total)).replace("{missing}", String(census.total - census.present.size));
}
