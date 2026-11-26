#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
/** 🏙️ `@semio-tech/bim-plugin` router: `repo ./📜️script.ts test`. */
import { runRepositoryCargoTests } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { verifyBimGuest } from "../../🧪️tests/🌐️guest/🟦️.ts";
import { withPlaygroundSessionV1 } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session/👓️reading/🟦️.ts";
import { browserProductRootsV1 } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/📂️paths/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests(["semio-hub-bim", "semio-hub-bim-component"], this.repoRoot, this.invocation, rest);
  }
}

class GuestInstanceCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments, "quick");
    if (rest.length) throw Error("Unexpected BIM guest check arguments");
    await withPlaygroundSessionV1(this.repoRoot, this.invocation, "bim", async (session, _catalog, control) => {
      const plugin = session.plugins.find((entry) => entry.pluginId === session.registryPluginId);
      if (!plugin) throw Error("BIM playground has no guest module");
      await verifyBimGuest(browserProductRootsV1(this.repoRoot, "prototype").modules, plugin, control.signal);
      console.log("BIM editor and viewer opened, returned their configuration, and closed in Chromium");
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("guest-instance-check", GuestInstanceCheckScript);


await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
