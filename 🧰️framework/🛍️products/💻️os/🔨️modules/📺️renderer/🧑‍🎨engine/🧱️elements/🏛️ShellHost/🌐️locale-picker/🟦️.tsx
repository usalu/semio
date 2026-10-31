import * as React from "../../⚛️react-port/🟦️.ts";
import { SHELL_LOCALES, type ShellLocale } from "@semio-tech/framework";

const LANGUAGE_CHOICES: Readonly<Record<ShellLocale, Readonly<{ name: string; prompt: string }>>> = {
  en: { name: "English", prompt: "Choose your language" },
  de: { name: "Deutsch", prompt: "Sprache wählen" },
};

/** 🌐️ The explicit language choice shown while no lock, stored preference, host option or platform language names a supported locale: every supported language is offered in its own words, none is preselected. */
export function ShellLocalePicker(props: Readonly<{ onChoose(locale: ShellLocale): void }>): React.ReactElement {
  return (
    <main role="group" aria-labelledby="shell-locale-picker-title" data-slot="shell-locale-picker" style={{ display: "grid", placeItems: "center", height: "100%", width: "100%" }}>
      <div style={{ display: "grid", gap: "1rem", justifyItems: "center" }}>
        <h1 id="shell-locale-picker-title" style={{ fontSize: "1.25rem", margin: 0 }}>
          {SHELL_LOCALES.map((locale, index) => (
            <React.Fragment key={locale}>
              {index === 0 ? null : " / "}
              <span lang={locale}>{LANGUAGE_CHOICES[locale].prompt}</span>
            </React.Fragment>
          ))}
        </h1>
        <div style={{ display: "flex", gap: "0.75rem" }}>
          {SHELL_LOCALES.map((locale) => (
            <button key={locale} type="button" lang={locale} data-locale={locale} onClick={() => props.onChoose(locale)}>
              {LANGUAGE_CHOICES[locale].name}
            </button>
          ))}
        </div>
      </div>
    </main>
  );
}
