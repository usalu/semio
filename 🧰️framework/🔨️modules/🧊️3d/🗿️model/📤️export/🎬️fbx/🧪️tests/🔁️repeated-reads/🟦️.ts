/** 🔁️ Regression: assimp reads the same animated FBX again and again, each read in a fresh instance, because the heap of one WebAssembly instance only grows and a long-lived instance aborts after many large files (`Aborted()`, then `import_error` for every later file).
 *
 * @see ../🔍️second-reader/🟦️.ts
 * @see staffage tests of the model catalog, feature file — scenario "A long run of FBX files never exhausts assimp"
 */
import { describe, expect, it } from "vitest";
import { encodeFbx } from "../../🟦️.ts";
import { NO_OPERATION, fixture } from "../🧰️support/🟦️.ts";
import type { FbxFixture } from "../🧰️support/🟦️.ts";
import { loadAssimp } from "../🧰️support/🧩️assimpjs/🟦️.ts";

const READS = 60;

describe("assimp reads many FBX files in a row", () => {
  it(`reads the animated arm ${READS} times, each in a fresh instance, with the same result`, async () => {
    const { document, cases } = fixture<FbxFixture>("rigged-arm");
    const wave = cases.find((item) => item.options.animation === "wave")!;
    const bytes = encodeFbx(document, wave.options, NO_OPERATION);
    const seen = new Set<string>();
    const instances = new Set<unknown>();
    for (let read = 0; read < READS; read++) {
      const assimp = await loadAssimp();
      instances.add(assimp);
      const list = new assimp.FileList();
      list.AddFile("model.fbx", bytes);
      const result = assimp.ConvertFileList(list, "assjson");
      expect(result.IsSuccess(), `read ${read}: ${result.GetErrorCode()}`).toBe(true);
      const scene = JSON.parse(new TextDecoder().decode(result.GetFile(0).GetContent())) as { animations?: { channels: unknown[] }[] };
      seen.add(`${scene.animations?.length}:${scene.animations?.[0]?.channels.length}`);
    }
    expect(instances.size).toBe(READS);
    expect(seen.size).toBe(1);
    expect([...seen][0]).not.toMatch(/^(undefined|0):/);
  });
});
