
import { receiveScriptProcessInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {BundleScript,ScriptRouter} from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runScriptMain} from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnershipScript, runNativeScript, runRefusalScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
/** 🏛️ Checks the actual independent vocabulary owner and literal wire fields. */
class OwnershipScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    await runOwnershipScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 🚪️ Runs every native vocabulary law without selecting a test name. */
class NativeScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    await runNativeScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
/** ⚠️ Proves the owned IO cause projection and wire contract against independent schemas. */
class RefusalScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    await runRefusalScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

const router=new ScriptRouter(import.meta.dirname).register("test-ownership",OwnershipScript).register("test-native",NativeScript).register("test-refusal",RefusalScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({defaultCommand:"test-ownership"}) }));
