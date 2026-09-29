/** 👋️ The catalog's introduction: the screen of the first visit, the "How it works" card of the overview and the page
 * behind that card. */

import { useId, type ReactElement } from "react";
import { roomScope, type CatalogView } from "@semio-tech/quiz";
import { localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { CardAction, CardIcon, PageFrame, QuizCard, type PaneView } from "../🪟️chrome/🟦️.tsx";
import { PRESENCE_ANCHORS, PanePeers } from "../👥️presence/🟦️.tsx";
import { HOME_PAGES } from "../🧭️session/🟦️.ts";

function Paragraphs(props: { readonly catalog: CatalogView; readonly locale: QuizLocale }): ReactElement {
  return (
    <>
      {props.catalog.introduction.paragraphs.map((paragraph, index) => (
        <p key={index} className="m-0 text-sm leading-normal">
          {localized(paragraph, props.locale)}
        </p>
      ))}
    </>
  );
}

/** 👋️ The introduction screen of the first visit. */
export function IntroductionScreen(props: { readonly catalog: CatalogView; readonly text: QuizText; readonly locale: QuizLocale; readonly onContinue: () => void }): ReactElement {
  const { catalog, text, locale, onContinue } = props;
  const id = useId();
  return (
    <QuizCard
      id={id}
      card="introduction"
      anchor={PRESENCE_ANCHORS.introduction}
      headingLevel={1}
      focusableHeading
      icon={<CardIcon icon="info" />}
      title={localized(catalog.introduction.title, locale)}
      footerRight={
        <CardAction primary onClick={onContinue}>
          {text("quiz.introduction.continue")}
        </CardAction>
      }
    >
      <Paragraphs catalog={catalog} locale={locale} />
    </QuizCard>
  );
}

/** 🃏️ The "How it works" card of the overview: the first paragraph; its heading and "Read more" open the page
 * (`#intro`). */
export function IntroductionCard(props: { readonly catalog: CatalogView; readonly text: QuizText; readonly locale: QuizLocale; readonly revealed: boolean; readonly onOpen: () => void }): ReactElement {
  const { catalog, text, locale, revealed, onOpen } = props;
  const id = useId();
  const lead = catalog.introduction.paragraphs[0];
  return (
    <QuizCard
      id={id}
      card="introduction"
      anchor={PRESENCE_ANCHORS.home("introduction")}
      href={`#${HOME_PAGES.introduction}`}
      onOpen={onOpen}
      revealed={revealed}
      icon={<CardIcon icon="info" />}
      title={text("quiz.home.howItWorks")}
      footerRight={
        <CardAction primary onClick={onOpen}>
          {text("quiz.home.readMore")}
        </CardAction>
      }
    >
      {lead === undefined ? null : <p className="m-0 text-xs leading-normal text-muted-foreground">{localized(lead, locale)}</p>}
    </QuizCard>
  );
}

/** 👋️ The introduction page behind the "How it works" card. */
export function IntroductionPage(props: { readonly catalog: CatalogView; readonly locale: QuizLocale; readonly view: PaneView }): ReactElement {
  const { catalog, locale, view } = props;
  const id = useId();
  return (
    <PageFrame page={HOME_PAGES.introduction} overlay={<PanePeers scope={roomScope(catalog.id, { screen: "introduction" })} opened={view.opened} />}>
      <QuizCard id={id} card="introduction-page" anchor={PRESENCE_ANCHORS.introduction} icon={<CardIcon icon="info" />} title={localized(catalog.introduction.title, locale)}>
        <Paragraphs catalog={catalog} locale={locale} />
      </QuizCard>
    </PageFrame>
  );
}
