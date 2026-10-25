/** 🔡️ Contract: the owned translation engine answers the language-agnostic corpus exactly as i18next, the independent oracle, configured the way the UI chrome configures it, answers it. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import i18next, { type i18n } from "i18next";
import { createUiI18nInstance, type UiI18nCallOptions, type UiI18nTree } from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

type Step = Readonly<Record<string, unknown>> & { readonly call: string };
type Row = (typeof corpus.instances)[number];

interface Subject {
  t(key: string, options?: UiI18nCallOptions): unknown;
  exists(key: string, options?: UiI18nCallOptions): boolean;
  fixed(language: string, key: string): unknown;
  language(): string | undefined;
  resolvedLanguage(): string | undefined;
  changeLanguage(language: string): Promise<unknown>;
  addBundle(language: string, tree: UiI18nTree): void;
}

const owned = (row: Row): Subject => {
  const instance = createUiI18nInstance(row.options);
  for (const [language, tree] of Object.entries(row.bundles)) instance.addBundle(language, tree as UiI18nTree);
  return { t: instance.t, exists: instance.exists, fixed: (language, key) => instance.getFixedT(language)(key), language: () => instance.language, resolvedLanguage: () => instance.resolvedLanguage, changeLanguage: instance.changeLanguage, addBundle: instance.addBundle };
};

const oracle = (row: Row): Subject => {
  const instance: i18n = i18next.createInstance();
  void instance.init({ resources: {}, fallbackLng: row.options.fallbackLanguage, supportedLngs: [...row.options.supportedLanguages], nonExplicitSupportedLngs: true, lng: row.options.language, showSupportNotice: false, returnObjects: true, initImmediate: false, interpolation: { escapeValue: false } });
  for (const [language, tree] of Object.entries(row.bundles)) instance.addResourceBundle(language, "translation", tree, true, true);
  return { t: (key, options) => instance.t(key as never, options as never), exists: (key, options) => instance.exists(key, options as never), fixed: (language, key) => instance.getFixedT(language)(key as never), language: () => instance.language, resolvedLanguage: () => instance.resolvedLanguage, changeLanguage: (language) => instance.changeLanguage(language), addBundle: (language, tree) => void instance.addResourceBundle(language, "translation", tree, true, true) };
};

async function run(subject: Subject, steps: readonly Step[]): Promise<unknown[]> {
  const seen: unknown[] = [];
  for (const step of steps) {
    if (step.call === "t") seen.push(subject.t(step.key as string, step.options as UiI18nCallOptions | undefined));
    else if (step.call === "exists") seen.push(subject.exists(step.key as string, step.options as UiI18nCallOptions | undefined));
    else if (step.call === "fixed") seen.push(subject.fixed(step.language as string, step.key as string));
    else if (step.call === "language") seen.push(subject.language());
    else if (step.call === "resolvedLanguage") seen.push(subject.resolvedLanguage());
    else if (step.call === "changeLanguage") await subject.changeLanguage(step.language as string);
    else if (step.call === "addBundle") subject.addBundle(step.language as string, step.tree as UiI18nTree);
    else throw new Error(`unknown step ${step.call}`);
  }
  return seen;
}

const expectedOf = (steps: readonly Step[]): unknown[] => steps.filter((step) => "expected" in step).map((step) => step.expected);
const answered = (steps: readonly Step[], seen: unknown[]): unknown[] => seen.filter((_, index) => "expected" in steps.filter((step) => !["changeLanguage", "addBundle"].includes(step.call))[index]!);

test("the corpus is closed under its schema", () => {
  const ajv = new Ajv({ strict: true });
  expect(ajv.validate(schema, corpus), JSON.stringify(ajv.errors)).toBe(true);
});

for (const row of corpus.instances) {
  const steps = row.steps as readonly Step[];
  test(`owned engine: ${row.name}`, async () => {
    expect(answered(steps, await run(owned(row), steps))).toEqual(expectedOf(steps));
  });
  test(`i18next oracle agrees: ${row.name}`, async () => {
    expect(answered(steps, await run(oracle(row), steps))).toEqual(expectedOf(steps));
  });
}
