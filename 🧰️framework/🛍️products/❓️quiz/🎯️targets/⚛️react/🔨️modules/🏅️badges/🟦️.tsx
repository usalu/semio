/** 🏅️ Badges: the overview card with one tile per badge of the catalog (earned or still locked), the page behind it
 * listing every badge with its description and the day it was earned, and the badges earned in the runs of one quiz.
 * Earned and locked are told apart by an icon (a check or a lock) and a solid or dashed frame for the eye and in words
 * for assistive technology — the muted colour of a locked badge only repeats what they say.
 *
 * @see https://www.w3.org/WAI/WCAG22/Understanding/use-of-color.html
 */

import { useId, type ReactElement } from "react";
import { Icon } from "@semio-tech/ui-react/chrome";
import { roomScope, type BadgeAward, type CatalogBadgeView, type Slug } from "@semio-tech/quiz";
import { localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { formatDate } from "../📏️quantity/🟦️.ts";
import { CardAction, CardIcon, Glyph, PageFrame, QuizCard, cn, type PaneView } from "../🪟️chrome/🟦️.tsx";
import { HOME_PAGES, type QuizState } from "../🧭️session/🟦️.ts";
import { PRESENCE_ANCHORS, PanePeers } from "../👥️presence/🟦️.tsx";

/** 🎗️ The award of `badge` the learner holds, if any. */
export function awardOf(state: QuizState, badge: Slug): BadgeAward | undefined {
  return state.learnerView?.badges.find((held) => held.badge === badge);
}

/** 🎯️ The badges of the catalog the learner earned in runs of `quiz`, in catalog order. */
export function badgesEarnedIn(state: QuizState, quiz: Slug): readonly CatalogBadgeView[] {
  const runs = new Set((state.learnerView?.runs ?? []).filter((run) => run.quiz === quiz).map((run) => run.run));
  const earned = new Set((state.learnerView?.badges ?? []).filter((award) => runs.has(award.run)).map((award) => award.badge));
  return (state.catalog?.badges ?? []).filter((badge) => earned.has(badge.id));
}

/** ✅️ The mark of a badge's state for the eye: a check when earned, a lock when not. */
function BadgeMark(props: { readonly earned: boolean; readonly className?: string }): ReactElement {
  const icon = props.earned ? "check" : "lock";
  return (
    <span aria-hidden="true" data-badge-mark={icon} className={cn("inline-flex shrink-0", props.className)}>
      <Icon icon={icon} size="small" />
    </span>
  );
}

/** 🏅️ The badges card of the overview; its heading and "All badges" open the badges page (`#badges`). */
export function BadgesCard(props: { readonly state: QuizState; readonly text: QuizText; readonly locale: QuizLocale; readonly revealed: boolean; readonly onOpen: () => void }): ReactElement {
  const { state, text, locale, revealed, onOpen } = props;
  const id = useId();
  const badges = state.catalog?.badges ?? [];
  const earned = badges.filter((badge) => awardOf(state, badge.id) !== undefined).length;
  return (
    <QuizCard
      id={id}
      card="badges"
      anchor={PRESENCE_ANCHORS.home("badges")}
      href={`#${HOME_PAGES.badges}`}
      onOpen={onOpen}
      revealed={revealed}
      icon={<CardIcon icon="award" />}
      title={text("quiz.home.badges")}
      footerRight={
        <CardAction primary onClick={onOpen}>
          {text("quiz.home.allBadges")}
        </CardAction>
      }
    >
      <ul role="list" className="m-0 flex list-none flex-wrap gap-single p-0">
        {badges.map((badge) => {
          const held = awardOf(state, badge.id) !== undefined;
          const standing = held ? text("quiz.home.earned") : text("quiz.home.locked");
          return (
            <li
              key={badge.id}
              data-earned={held ? "" : undefined}
              data-state={held ? "earned" : "locked"}
              title={`${localized(badge.label, locale)}: ${standing}`}
              className={cn("relative flex size-workbench items-center justify-center border border-normal text-lg", held ? "border-solid text-foreground" : "border-dashed text-muted-foreground")}
            >
              <Glyph emoji={badge.emoji} />
              <BadgeMark earned={held} className="absolute -bottom-px -right-px bg-background text-foreground" />
              <span className="sr-only">
                {localized(badge.label, locale)}: {standing}
              </span>
            </li>
          );
        })}
      </ul>
      <p className="m-0 text-xs text-muted-foreground">{text("quiz.home.badgesEarned", { earned, total: badges.length })}</p>
    </QuizCard>
  );
}

/** 🏅️ The badges page behind the badges card: every badge with its description, earned (with the day) or not yet. */
export function BadgesPage(props: { readonly state: QuizState; readonly text: QuizText; readonly locale: QuizLocale; readonly view: PaneView }): ReactElement {
  const { state, text, locale, view } = props;
  const id = useId();
  const badges = state.catalog?.badges ?? [];
  return (
    <PageFrame page={HOME_PAGES.badges} wide overlay={<PanePeers scope={state.catalog === undefined ? undefined : roomScope(state.catalog.id, { screen: "badges" })} opened={view.opened} />}>
      <QuizCard id={id} card="all-badges" anchor={PRESENCE_ANCHORS.badges} icon={<CardIcon icon="award" />} title={text("quiz.home.badges")}>
        <ul role="list" className="m-0 grid list-none grid-cols-[repeat(auto-fill,minmax(min(100%,18rem),1fr))] gap-double p-0">
          {badges.map((badge) => {
            const award = awardOf(state, badge.id);
            return (
              <li
                key={badge.id}
                data-earned={award === undefined ? undefined : ""}
                data-state={award === undefined ? "locked" : "earned"}
                className={cn("flex gap-double border border-normal p-double", award === undefined ? "border-dashed text-muted-foreground" : "border-solid")}
              >
                <Glyph emoji={badge.emoji} className="text-2xl leading-none" />
                <div className="flex min-w-0 flex-col gap-single">
                  <h3 className="m-0 text-sm font-semibold text-foreground">{localized(badge.label, locale)}</h3>
                  <p className="m-0 flex items-center gap-single text-xs font-semibold">
                    <BadgeMark earned={award !== undefined} />
                    {award === undefined ? text("quiz.home.locked") : text("quiz.home.earnedAt", { date: formatDate(award.at, locale) })}
                  </p>
                  <p className="m-0 text-xs">{localized(badge.description, locale)}</p>
                </div>
              </li>
            );
          })}
        </ul>
      </QuizCard>
    </PageFrame>
  );
}
