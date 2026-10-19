
import { receiveScriptProcessInvocation } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { PLAY_HOST } from "../🧩️runtime/🟦️.ts";
import { publishPlayPageCohortV1 } from "./📄pages/📦️publication/🟦️.ts";
import { runBuildScript, runMapTilesScript, runBundleSiteScript, runVerifyScript } from "./🟦️.ts";

/** 🚀️ `build`: the whole play release as ONE build driver — the components of every pane's closure in one Cargo pass per workspace, each streamed through
 * describe, transpile, `wasm-opt` and its module probe as it links; the descriptor emitter, fonts, browser support, map tiles and the wasm-bindgen engines as
 * jobs under the same jobserver (the engines in the `wasm-release` lane, before the component passes); the Vite bundle as soon as its engines exist; and the
 * pages assembly once every module is published. The pipeline is declared in this project's `build` target metadata.
 * @see ../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts
 * @see ../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts */
class BuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runBuildScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}

/** 🗺️ `map-tiles`: prefetches the map tiles the bundle embeds (network-bound). */
class MapTilesScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runMapTilesScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}

/** 🌐️ `bundle`: the Vite site bundle into `dist/site`. It reads the generated catalog and the wasm-bindgen engines but no compiled component, so it runs
 * while the components still compile; the runtime modules are bound at pages assembly. */
class BundleSiteScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runBundleSiteScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}

/** 📄️ `pages`: binds the runtime modules, browser support and fonts into the bundled site with the module catalog, then splits it into the CDN pages. */
class PagesScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("pages accepts no arguments");
    await publishPlayPageCohortV1(this.repoRoot,this.invocation,PLAY_HOST);
  }
}

/** ⚖️ `verify`: the release gate over the published pages — the generated catalog equals a refusing render of the committed descriptors, and every module
 * page file is exactly the one the module catalog binds. */
class VerifyScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runVerifyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("map-tiles", MapTilesScript).register("bundle", BundleSiteScript).register("pages", PagesScript).register("verify", VerifyScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (router).run(process.argv.slice(2), original));
