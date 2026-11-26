/** 🪪️ Hashes the exact component used by the native smoke law. */
import { createHash } from "node:crypto";
import { createReadStream } from "node:fs";
import { stat } from "node:fs/promises";
import { isAbsolute, resolve } from "node:path";
import registration from "../../🧪️tests/🧫️fixtures/🛂️committed-descriptor/🔣️.json";

const root = resolve(import.meta.dir, "../../../../..");
const selected = process.env.SEMIO_BIM_GUEST_COMPONENT;
if (selected !== undefined && !isAbsolute(selected)) throw new Error("SEMIO_BIM_GUEST_COMPONENT must be absolute");
const mode = process.env.SEMIO_BUILD_MODE ?? "prototype";
if (selected === undefined && mode !== "prototype" && mode !== "release") throw new Error("BIM smoke requires a valid component build mode");
const staged = resolve(root, registration.componentDirectory, registration.deliverables[mode === "release" ? "release" : "prototype"], registration.componentName);
const file = selected ?? staged;
const metadata = await stat(file).catch((error: NodeJS.ErrnoException) => {
  if (error.code === "ENOENT") return undefined;
  throw error;
});
if (metadata?.isFile()) {
  const digest = createHash("sha256");
  for await (const bytes of createReadStream(file)) digest.update(bytes);
  console.log(digest.digest("hex"));
} else {
  console.log("no-component");
}
