/** 🧰️ Shared test support of the web renderer: the reference walker fixture. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { Asset } from "@teaching/staffage";

const staffage = resolve(process.cwd(), "../../../..");

/** 🚶️ The reference walker, loaded from the language-agnostic fixture. */
export const walker: Asset = JSON.parse(readFileSync(resolve(staffage, "🧫️fixtures/🚶️reference-walker/🔣️.json"), "utf8"));
