import {fileURLToPath} from "node:url";
import {packCatalogFile} from "./📁️generate.ts";
const standards=["🔖️2x3/🪆️subsets/🧱️base","4️⃣4/🪆️subsets/✳️any","🔖️4x3/🪆️subsets/✳️any"];
/** 🏭️ Regenerate every committed `catalog.ifcpack` from its neutral `🔣️.json` (`bun 🏭️pack-all.ts`); the output is deterministic. */
for(const standard of standards){const owner=fileURLToPath(new URL(`../../🏅️standards/${standard}/🧬️schema/📚️catalog/`,import.meta.url));console.log(standard,await packCatalogFile({json:owner+"🔣️.json",pack:owner+"catalog.ifcpack"}));}
