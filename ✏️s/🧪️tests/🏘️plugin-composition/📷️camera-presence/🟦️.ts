/** 📷️ The canonical viewport presence wire, the renderer presence ledgers and every presence owner agree; the law needs the OS camera contract and the plugin presence schemas, so it lives where plugins may meet. */
import { expect, test } from "bun:test";
import { resolve } from "node:path";
import { testCameraPresenceContract } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👥️presence/📷️camera/🧪️tests/🟦️.ts";

test("camera presence wire, schemas and document/window isolation agree with the independent oracles", async () => {
  expect(await testCameraPresenceContract(resolve(import.meta.dir, "../../../.."))).toBeGreaterThan(0);
});
