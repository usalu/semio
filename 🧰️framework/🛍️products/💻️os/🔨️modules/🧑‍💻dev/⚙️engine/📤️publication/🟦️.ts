import { existsSync } from "node:fs";

/** @emoji 🍎 Prefer Command Line Tools over an unlicensed Xcode.app so cargo/wasm-pack can link. */
function ensureAppleDeveloperDir(): void {
  if (process.platform !== "darwin" || process.env.FORCE_XCODE === "1") return;
  const clt = "/Library/Developer/CommandLineTools";
  if (!existsSync(clt)) return;
  // Prefer CLT over an installed-but-unlicensed Xcode.app (cargo/cc otherwise die with exit 69).
  process.env.DEVELOPER_DIR = clt;
  const sdk = `${clt}/SDKs/MacOSX.sdk`;
  if (existsSync(sdk)) process.env.SDKROOT = sdk;
}

export { ensureAppleDeveloperDir };
