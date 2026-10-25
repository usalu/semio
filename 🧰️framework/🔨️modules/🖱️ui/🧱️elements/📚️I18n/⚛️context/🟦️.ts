/** ⚛️ The React side of the owned translation engine: the context a shell provides its own instance through, the provider, and the
 * subscription hook that re-renders a view when the language or a bundle of its instance changes.
 *
 * @see ../../../🌐️i18n/🔡️instance/🟦️.ts — the engine whose `version` the hook subscribes to
 * @see ../../🏷️Label/🟦️.tsx — `useUiTranslation`, the typed hook every label call site uses
 */
import * as React from "react";
import { uiI18nInstance } from "../../../🌐️i18n/🟦️.ts";
import { type UiI18nInstance } from "../../../🌐️i18n/🔡️instance/🟦️.ts";

const UI_I18N_CONTEXT_KEY = Symbol.for("semio.ui.i18n.context");

function pageUiI18nContext(): React.Context<UiI18nInstance | null> {
  const registry = globalThis as unknown as Record<symbol, React.Context<UiI18nInstance | null> | undefined>;
  return (registry[UI_I18N_CONTEXT_KEY] ??= React.createContext<UiI18nInstance | null>(null));
}

/** 📚️ The page's one i18n context, held in the global symbol registry so every evaluation of this module shares it. */
export const UiI18nContext = pageUiI18nContext();

/** 🪝️ Provides `instance` to every translation hook below it; outside any provider the hooks stand on the page's shared instance. */
export function UiI18nProvider({ instance, children }: { readonly instance: UiI18nInstance; readonly children?: React.ReactNode }): React.ReactElement {
  return React.createElement(UiI18nContext.Provider, { value: instance }, children);
}

/** 📃️ The nearest provided instance, or the page's shared one; re-renders its caller whenever that instance changes language or bundles. */
export function useUiI18nInstance(): UiI18nInstance {
  const instance = React.useContext(UiI18nContext) ?? uiI18nInstance;
  React.useSyncExternalStore(instance.subscribe, () => instance.version, () => instance.version);
  return instance;
}
