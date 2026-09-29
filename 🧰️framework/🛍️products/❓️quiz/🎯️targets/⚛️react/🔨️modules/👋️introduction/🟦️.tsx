/** 👋️ The catalog's introduction, shown on the first visit and reachable from the navigation afterwards. */

import type { ReactElement } from "react";
import type { CatalogView } from "@semio-tech/quiz";
import { localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";

/** 👋️ The introduction screen. */
export function IntroductionScreen(props: { readonly catalog: CatalogView; readonly text: QuizText; readonly locale: QuizLocale; readonly onContinue: () => void }): ReactElement {
  const { catalog, text, locale, onContinue } = props;
  return (
    <section className="quiz-introduction quiz-panel">
      <h1 tabIndex={-1}>{localized(catalog.introduction.title, locale)}</h1>
      {catalog.introduction.paragraphs.map((paragraph, index) => (
        <p key={index}>{localized(paragraph, locale)}</p>
      ))}
      <div className="quiz-actions">
        <button type="button" className="quiz-button quiz-button-primary" onClick={onContinue}>
          {text("quiz.introduction.continue")}
        </button>
      </div>
    </section>
  );
}
