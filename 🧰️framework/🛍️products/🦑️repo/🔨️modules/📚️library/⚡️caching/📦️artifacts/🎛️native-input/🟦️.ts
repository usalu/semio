/** 🎯️ Keeps caller arguments within the selected Nx leaf's package and production/test input contract. */
export function validateNativeCargoArguments(operation: "build" | "check" | "test", args: readonly string[]): void {
  const delimiter = args.indexOf("--"),
    cargo = delimiter < 0 ? args : args.slice(0, delimiter);
  for (const argument of cargo)
    if (/^(?:--(?:workspace|package|manifest-path|exclude|config)(?:=|$)|-p)/.test(argument) || (operation !== "test" && /^--(?:all-targets|tests?|examples?|benches|bench)(?:=|$)/.test(argument)))
      throw new Error(`Argument ${argument} changes the native input contract; use a dedicated Nx target for that selection`);
}

/** 🎯️ Normalizes an artifact router invocation before Cargo can observe caller-controlled selectors. */
export function artifactRustCargoArguments(operation: "build" | "check" | "test", segments: readonly string[]): { cargoArgs: string[]; testLevel?: string } {
  const levels = new Set(["fundamental", "quick", "long", "exhaustive"]),
    [first, ...rest] = segments;
  const testLevel = operation === "test" && first && levels.has(first) ? first : undefined;
  const cargoArgs = testLevel ? rest : [...segments];
  validateNativeCargoArguments(operation, cargoArgs);
  return { cargoArgs, testLevel };
}
