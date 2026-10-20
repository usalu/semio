import { runGenuineBunTestV1 } from "./🟦️.ts";

/** 🚪️ Runs `--file=<test> [--birth=<id>] [--pattern=<regex>] [--root=<dir>]` inside a genuine Original process birth; exits 1 on any failure. */
async function main(argv: readonly string[]): Promise<void> {
  const option = (name: string): string | undefined => argv.find(argument => argument.startsWith(name + "="))?.slice(name.length + 1);
  const file = option("--file");
  if (file === undefined) throw Error("--file=<test file> required");
  await runGenuineBunTestV1({ root: option("--root") ?? process.env.SEMIO_WORKSPACE_ROOT ?? process.cwd(), file, birth: option("--birth"), pattern: option("--pattern") });
}

main(process.argv.slice(2)).then(() => process.exit(0), error => { console.error(error instanceof Error ? error.message : String(error)); process.exit(1); });
