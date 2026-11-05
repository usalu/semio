import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🥾️ Selects one authored native bootstrap operation before any host side effect. */
export function nativeBootstrapCommandV1(args: readonly string[]): "setup" | "rust" {
  const command = args[0] ?? "setup";
  if (args.length > 1 || !schema.definitions.BootstrapCommand.enum.some(value => value === command)) throw Error("Native bootstrap requires setup or rust and accepts no additional arguments");
  return command as "setup" | "rust";
}
