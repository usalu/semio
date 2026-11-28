/** 🪪️ Hashes the exact component used by the native smoke law. */
import { createHash } from "node:crypto";
import { createReadStream, readFileSync } from "node:fs";
import { lstat } from "node:fs/promises";
import { isAbsolute, join, resolve } from "node:path";
import { readBuildMode } from "../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
import { repositoryCachePartitionRootDataV1 } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🟦️.ts";
import { componentModelGenerationPathV1, componentModelLayoutV1, readComponentModelHeadV1 } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/📋️contract/🟦️.ts";
import componentModel from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/🧬️schema/🔣️.json";
import registration from "../../🧪️tests/🧫️fixtures/🛂️committed-descriptor/🔣️.json";

/** 📦️ Selects the managed component model or an explicit diagnostic component. */
export function bimGuestFiles(root: string): readonly [wasm: string, receipt?: string] {
  const selected = process.env.SEMIO_BIM_GUEST_COMPONENT;
  if (selected !== undefined) {
    if (!isAbsolute(selected)) throw Error("SEMIO_BIM_GUEST_COMPONENT must be absolute");
    return [selected];
  }
  const mode = readBuildMode(process.env.SEMIO_BUILD_MODE ?? "prototype");
  const products = repositoryCachePartitionRootDataV1(root, "products");
  const layout = componentModelLayoutV1(products, { version: 1, mode, pluginId: registration.pluginId, cratePath: registration.componentCratePath });
  const generation = (() => {
    try { return readComponentModelHeadV1(JSON.parse(readFileSync(layout.head, "utf8"))).generation; }
    catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return "0".repeat(componentModel.definitions.Layout.const.generationLength); throw error; }
  })();
  const path = componentModelGenerationPathV1(layout, generation);
  const { component, receipt } = componentModel.definitions.Protocol.const;
  return [join(path, component), join(path, receipt)];
}

/** 🔎️ Requires a real guest input before starting the mounted native suite. */
export async function requireBimGuest(root: string): Promise<string> {
  const [wasm, receipt] = bimGuestFiles(root);
  for (const path of receipt ? [wasm, receipt] : [wasm]) {
    if (!(await lstat(path)).isFile()) throw Error(`BIM guest input is not a regular file at ${path}`);
  }
  return wasm;
}

if (import.meta.main) {
  const [wasm, receipt] = bimGuestFiles(resolve(import.meta.dir, "../../../../.."));
  const digests: string[] = [];
  for (const file of receipt ? [wasm, receipt] : [wasm]) {
    const metadata = await lstat(file).catch((error: NodeJS.ErrnoException) => {
      if (error.code === "ENOENT") return undefined;
      throw error;
    });
    if (!metadata?.isFile()) break;
    const digest = createHash("sha256");
    for await (const bytes of createReadStream(file)) digest.update(bytes);
    digests.push(digest.digest("hex"));
  }
  console.log(digests.length === (receipt ? 2 : 1) ? digests.join(":") : "no-component");
}
