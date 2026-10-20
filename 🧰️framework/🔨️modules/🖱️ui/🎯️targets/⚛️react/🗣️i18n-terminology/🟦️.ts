import type { AssertUiRibbonParentKeysCovered, UiRibbonParentCategory, AssertUiSettingsLanguageKeysCovered, UiLocale, AssertUiSettingsTerminologyKeysCovered, UiChromeTerminologyId } from "@semio-tech/ui/elements/I18n";
import { uiChromeTranslationBundles } from "@semio-tech/ui/i18n";
import type { ShellLocale } from "@semio-tech/framework-manifest/generated/ui-axes";
import { UI_TERMINOLOGY_NATIVE } from "@semio-tech/ui/modules/ui-terminology";

const _assertUiRibbonParentKeys: AssertUiRibbonParentKeysCovered<UiRibbonParentCategory> = true;

const _assertUiSettingsLanguageKeys: AssertUiSettingsLanguageKeysCovered<UiLocale> = true;

const _assertUiSettingsTerminologyKeys: AssertUiSettingsTerminologyKeysCovered<UiChromeTerminologyId> = true;

/** 🏷️ Compile-time check that every brand-lockable {@link ShellLocale} has a complete chrome bundle — closes the loop from a brand's `locks.locale` through `ShellLocale` to an actual translated `uiChromeTranslationBundles` entry. */
type AssertShellBrandLocalesBundled<L extends string> = L extends keyof typeof uiChromeTranslationBundles ? true : false;
const _assertShellBrandLocalesBundled: AssertShellBrandLocalesBundled<ShellLocale> = true;

/** 🗣️ A `(locale) -> label-record` pair for one terminology id, mirroring the Rust `*_LABELS_{ID}_{LOCALE}` const pattern for TS-native products (e.g. compose, coda) that never cross the WASM plugin boundary and so have no `AppDefinition.terminologies`/`AppLabelsOverlay`. */
export type UiTerminologyLabelSet<Keys extends string> = Readonly<Record<UiLocale, Readonly<Record<Keys, string>>>>;

/** 🗣️ Builds a `(terminologyId, locale) -> labels` resolver from a set of terminology-keyed label tables, falling back to `native` for unknown/undeclared ids — the TS analog of the Rust `puzzle2d_labels`-style resolver. */
export function createTerminologyLabelResolver<Keys extends string>(sets: Readonly<Record<string, UiTerminologyLabelSet<Keys>>>): (terminologyId: string, locale: UiLocale) => Readonly<Record<Keys, string>> {
  return (terminologyId, locale) => (sets[terminologyId] ?? sets[UI_TERMINOLOGY_NATIVE])![locale];
}
