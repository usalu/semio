/** 🧩️ Semantic engine selection owner. */




/** 🔗️ Validates browser factory declarations without scheduling their optional engines. */
export function linkedSessionEngines(declarations: unknown): readonly string[] {
  if (!Array.isArray(declarations)) throw new Error("Linked browser session factories must be an array");
  const engines = new Set<string>();
  for (const entry of declarations) {
    if (entry === null || typeof entry !== "object" || Array.isArray(entry) || Object.keys(entry).length !== 2) throw new Error("Invalid linked browser session factory");
    const module: unknown = Reflect.get(entry, "module");
    const engine: unknown = Reflect.get(entry, "engine");
    if (typeof module !== "string" || module.length === 0 || typeof engine !== "string" || !/^\.\/(?!.*(?:^|\/)\.\.(?:\/|$))[^\\]+$/.test(engine)) throw new Error("Invalid linked browser session factory path");
    engines.add(engine);
  }
  return [...engines];
}
