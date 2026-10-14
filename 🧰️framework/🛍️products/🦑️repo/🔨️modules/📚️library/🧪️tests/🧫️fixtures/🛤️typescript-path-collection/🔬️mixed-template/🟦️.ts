import { readFileSync } from "node:fs";
import { join } from "node:path";
const repoRoot = process.cwd();
const library = "🧰️library";
const inputs = [
  `${library}/🔣️taxonomy.json`,
  "🗿️artifact/🧬️mutations/🦀️.rs",
  "🗿️artifact/🧬️schema/🦀️.rs",
];
for (const input of inputs) {
  const absolute = join(repoRoot, input);
  readFileSync(absolute, "utf8");
}
