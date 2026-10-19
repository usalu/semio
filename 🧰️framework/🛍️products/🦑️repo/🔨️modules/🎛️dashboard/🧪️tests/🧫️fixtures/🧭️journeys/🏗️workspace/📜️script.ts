#!/usr/bin/env bun
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptProgram } from "./🟦️.ts";

/** 🚪️ `words` — runs the program of this script. */
class WordsScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🧭️ `serve` — runs the program of this script. */
class ServeScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🏭️ `ticker` — runs the program of this script. */
class TickerScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🧪️ `burst` — runs the program of this script. */
class BurstScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 📦️ `exit` — runs the program of this script. */
class ExitScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🔧️ `env` — runs the program of this script. */
class EnvScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🗂️ `probe` — runs the program of this script. */
class ProbeScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🧰️ `title` — runs the program of this script. */
class TitleScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🔬️ `resize` — runs the program of this script. */
class ResizeScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
const router = new ScriptRouter(import.meta.dir).register("words", WordsScript).register("serve", ServeScript).register("ticker", TickerScript).register("burst", BurstScript).register("exit", ExitScript).register("env", EnvScript).register("probe", ProbeScript).register("title", TitleScript).register("resize", ResizeScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
