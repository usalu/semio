import * as bun from "bun:test";

bun.test("the sleeping fixture waits to be cancelled", async () => {
  await Bun.sleep(120000);
});
