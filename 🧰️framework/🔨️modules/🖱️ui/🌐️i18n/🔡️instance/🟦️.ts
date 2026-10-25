/** 🔡️ The owned translation engine behind the UI i18n port: nested bundles per language, `{{name}}` interpolation, language
 * negotiation (`de-AT` -> `de` -> fallback) and change notification — the subset of i18next the chrome and its products use,
 * implemented without any runtime dependency. i18next stays the independent oracle of its tests.
 *
 * @see ../🟦️.ts — the port wiring, shared instance and per-shell instances
 * @see ./🧪️tests/🟦️.ts — the language-agnostic corpus checked against this engine and against i18next
 */

/** 🌳️ One nested bundle: strings, or groups of further bundles (a `{ label: { normal, beginner } }` row is a group). */
export type UiI18nTree = { readonly [key: string]: UiI18nTree | string | number | boolean | null };

/** 📞️ What one translate call may carry: reserved names steer the lookup (a group always resolves to a copy of its tree), every other entry is an interpolation variable. */
export type UiI18nCallOptions = Readonly<{ lng?: string; defaultValue?: unknown; returnObjects?: boolean; replace?: Readonly<Record<string, unknown>> } & Record<string, unknown>>;

/** ⚙️ Static choices of one instance: the language it stands at, the one it falls back to, and the languages it may negotiate to. */
export type UiI18nInstanceOptions = Readonly<{ language: string; fallbackLanguage: string; supportedLanguages: readonly string[] }>;

/** 🌐️ A live translation instance; `version` moves on every language or bundle change so views can subscribe by snapshot. */
export interface UiI18nInstance {
  readonly language: string;
  readonly resolvedLanguage: string | undefined;
  readonly version: number;
  t(key: string, options?: UiI18nCallOptions): unknown;
  getFixedT(language: string): (key: string, options?: UiI18nCallOptions) => unknown;
  exists(key: string, options?: UiI18nCallOptions): boolean;
  changeLanguage(language: string): Promise<void>;
  addBundle(language: string, tree: UiI18nTree): void;
  subscribe(listener: () => void): () => void;
}

const PLACEHOLDER = /\{\{(.+?)\}\}/gu;
const RESERVED = new Set(["lng", "defaultValue", "returnObjects", "replace"]);

function lookup(tree: UiI18nTree | undefined, key: string): unknown {
  let node: unknown = tree;
  for (const part of key.split(".")) {
    if (node === null || typeof node !== "object" || !Object.hasOwn(node, part)) return undefined;
    node = (node as Record<string, unknown>)[part];
  }
  return node;
}

function variableOf(variables: Readonly<Record<string, unknown>>, name: string): unknown {
  return lookup(variables as UiI18nTree, name);
}

function interpolate(text: string, variables: Readonly<Record<string, unknown>>): string {
  return text.replace(PLACEHOLDER, (placeholder, name: string) => {
    const value = variableOf(variables, name.trim());
    return value === undefined ? placeholder : value === null ? "" : String(value);
  });
}

function resolveValue(value: unknown, variables: Readonly<Record<string, unknown>>): unknown {
  if (typeof value === "string") return interpolate(value, variables);
  if (Array.isArray(value)) return value.map((item) => resolveValue(item, variables));
  if (value !== null && typeof value === "object") return Object.fromEntries(Object.entries(value).map(([name, item]) => [name, resolveValue(item, variables)]));
  return value;
}

function mergeTrees(base: UiI18nTree | undefined, extra: UiI18nTree): UiI18nTree {
  const merged: Record<string, UiI18nTree | string | number | boolean | null> = { ...base };
  for (const [name, value] of Object.entries(extra)) {
    const present = merged[name];
    merged[name] = value !== null && typeof value === "object" && present !== undefined && present !== null && typeof present === "object" ? mergeTrees(present, value) : value;
  }
  return merged;
}

/** 🆕️ Creates an instance standing at `options.language`, holding no bundle yet; bundles arrive through `addBundle`. */
export function createUiI18nInstance(options: UiI18nInstanceOptions): UiI18nInstance {
  const bundles = new Map<string, UiI18nTree>();
  const listeners = new Set<() => void>();
  let language = options.language;
  let version = 0;
  const supported = (code: string): boolean => options.supportedLanguages.includes(code) || options.supportedLanguages.includes(code.split("-")[0]!);
  const hierarchy = (code: string): readonly string[] => [...new Set([code, code.split("-")[0]!, options.fallbackLanguage])].filter((candidate) => supported(candidate));
  const notify = (): void => {
    version += 1;
    for (const listener of [...listeners]) listener();
  };
  const find = (key: string, code: string): unknown => {
    for (const candidate of hierarchy(code)) {
      const found = lookup(bundles.get(candidate), key);
      if (found !== undefined) return found;
    }
    return undefined;
  };
  const translate = (key: string, code: string, call: UiI18nCallOptions = {}): unknown => {
    const stand = call.lng ?? code;
    const found = find(key, stand);
    const variables = { ...call.replace, ...Object.fromEntries(Object.entries(call).filter(([name]) => !RESERVED.has(name))) };
    if (found === undefined) return call.defaultValue === undefined ? key : resolveValue(call.defaultValue, variables);
    return resolveValue(found, variables);
  };
  return {
    get language() {
      return language;
    },
    get resolvedLanguage() {
      return hierarchy(language).find((candidate) => bundles.has(candidate));
    },
    get version() {
      return version;
    },
    t: (key, call) => translate(key, language, call),
    getFixedT: (fixed) => (key, call) => translate(key, fixed, call),
    exists: (key, call) => find(key, call?.lng ?? language) !== undefined,
    changeLanguage(next) {
      language = next;
      notify();
      return Promise.resolve();
    },
    addBundle(code, tree) {
      bundles.set(code, mergeTrees(bundles.get(code), tree));
      notify();
    },
    subscribe(listener) {
      listeners.add(listener);
      return () => void listeners.delete(listener);
    },
  };
}
