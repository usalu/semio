/** 🔗️ One selected owner's module and its exact native browser engine producer. */
export type BrowserSessionFactoryContributionV1 = Readonly<{ module: string; engine: string }>;

/** 🧬️ Validates owned browser factory facts without importing their implementation into the framework. */
export function parseBrowserSessionFactoriesV1(value: unknown): readonly BrowserSessionFactoryContributionV1[] {
  if (!Array.isArray(value)) throw Error("Browser session contributions must be an array");
  const rows: BrowserSessionFactoryContributionV1[] = [];
  for (const entry of value) {
    if (!entry || typeof entry !== "object" || Array.isArray(entry) || Object.keys(entry).length !== 2) throw Error("Invalid browser session contribution");
    const module = Reflect.get(entry, "module"), engine = Reflect.get(entry, "engine");
    if (typeof module !== "string" || !/^(?:@[a-z0-9-]+\/)?[a-z0-9][a-z0-9._-]*$/.test(module) || typeof engine !== "string" || !/^\.\/(?!.*(?:^|\/)\.{1,2}(?:\/|$))(?!.*[\\:\x00-\x1f])[^/]+(?:\/[^/]+)+$/.test(engine)) throw Error("Invalid browser session contribution path");
    if (rows.some(row => row.module === module && row.engine === engine)) throw Error("Duplicate browser session contribution");
    rows.push({ module, engine });
  }
  return rows;
}

/** 🔗️ Projects only the engines explicitly declared by an owner's browser modules. */
export function linkedSessionEngines(declarations: unknown): readonly string[] {
  return [...new Set(parseBrowserSessionFactoriesV1(declarations).map(row => row.engine))];
}
