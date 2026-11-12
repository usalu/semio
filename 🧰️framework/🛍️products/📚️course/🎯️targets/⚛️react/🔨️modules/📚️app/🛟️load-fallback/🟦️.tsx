/** 🛟️ What the client shows while the texts of its own language have not arrived, or never will: a waiting text, and after the
 * load gave up an explanation with a way to try again and a reload. No language is known — the one that failed to load is the
 * one that would have said it — so every offered language speaks, English first, each part marking its own `lang`, from texts
 * that live here and need no bundle.
 *
 * @see ../../🌐️i18n/🟦️.ts — the bundles whose chunk may fail
 */

import type { ReactElement } from "react";
import { LANGUAGES } from "@semio-tech/course";

type CourseLocale = (typeof LANGUAGES)[number];

interface FallbackText {
  readonly loading: string;
  readonly failed: string;
  readonly retry: string;
  readonly reload: string;
}

/** 🗣️ The fallback's own texts in every offered language. */
export const LOAD_FALLBACK_TEXTS: { readonly [L in CourseLocale]: FallbackText } = {
  en: { loading: "Loading…", failed: "The course could not be loaded. A new version may be available – reload the page.", retry: "Try again now", reload: "Reload page" },
  de: { loading: "Wird geladen …", failed: "Der Kurs konnte nicht geladen werden. Möglicherweise gibt es eine neue Version – lade die Seite neu.", retry: "Jetzt erneut versuchen", reload: "Seite neu laden" },
};

/** 🩺️ The waiting text, or — once `failed` — the explanation with its two buttons, each in every offered language. */
export function LoadFallback(props: { readonly failed: boolean; readonly onRetry: () => void; readonly onReload: () => void }): ReactElement {
  const { failed, onRetry, onReload } = props;
  return (
    <main id="course-main" data-load-fallback={failed ? "failed" : "waiting"} aria-busy={!failed} className="flex min-h-dvh flex-col items-center justify-center gap-double bg-background p-double text-foreground">
      {LANGUAGES.map((locale) => (
        <section key={locale} lang={locale} className="flex max-w-xl flex-col items-center gap-single text-center">
          <p role={failed ? "alert" : "status"} className="m-0 text-sm">
            {failed ? LOAD_FALLBACK_TEXTS[locale].failed : LOAD_FALLBACK_TEXTS[locale].loading}
          </p>
          {failed ? (
            <span className="flex flex-wrap justify-center gap-single">
              <button type="button" onClick={onRetry} className="cursor-pointer rounded-md border border-border bg-transparent px-double py-single text-sm text-foreground">
                {LOAD_FALLBACK_TEXTS[locale].retry}
              </button>
              <button type="button" onClick={onReload} className="cursor-pointer rounded-md border border-border bg-transparent px-double py-single text-sm text-foreground">
                {LOAD_FALLBACK_TEXTS[locale].reload}
              </button>
            </span>
          ) : null}
        </section>
      ))}
    </main>
  );
}
