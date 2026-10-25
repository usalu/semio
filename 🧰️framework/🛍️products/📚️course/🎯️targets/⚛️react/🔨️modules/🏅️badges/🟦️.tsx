/** 🏅️ Badges: the overview card with one tile per badge of the course (earned or still locked), the page behind it
 * listing every badge with its description and the day it was earned, and the badges earned in the runs of one quiz.
 * Earned and locked are told apart by an icon (a check or a lock) and a solid or dashed frame for the eye and in words
 * for assistive technology — the muted colour of a locked badge only repeats what they say.
 *
 * @see https://www.w3.org/WAI/WCAG22/Understanding/use-of-color.html
 */

import { useId, type ReactElement } from "react";
import { Icon } from "@semio-tech/ui-react/chrome";
import { roomScope, type BadgeAward, type CourseBadgeView, type Slug } from "@semio-tech/course";
import { localized, type CourseLocale, type CourseText } from "../🌐️i18n/🟦️.ts";
import { formatDate } from "../📏️quantity/🟦️.ts";
import { CardAction, CardIcon, Glyph, PageFrame, CourseCard, cn, type PaneView } from "../🪟️chrome/🟦️.tsx";
import { HOME_PAGES, type CourseState } from "../🧭️session/🧮️state/🟦️.ts";
import { PanePeers } from "../👥️presence/🟦️.tsx";
import { PRESENCE_ANCHORS } from "../👥️presence/👁️view/🟦️.ts";

/** 🎗️ The award of `badge` the learner holds, if any. */
export function awardOf(state: CourseState, badge: Slug): BadgeAward | undefined {
  return state.learnerView?.badges.find((held) => held.badge === badge);
}

/** 🎯️ The badges of the course the learner earned in runs of `quiz`, in course order. */
export function badgesEarnedIn(state: CourseState, quiz: Slug): readonly CourseBadgeView[] {
  const runs = new Set((state.learnerView?.runs ?? []).filter((run) => run.quiz === quiz).map((run) => run.run));
  const earned = new Set((state.learnerView?.badges ?? []).filter((award) => runs.has(award.run)).map((award) => award.badge));
  return (state.course?.badges ?? []).filter((badge) => earned.has(badge.id));
}

/** ✅️ The mark of a badge's state for the eye: a check when earned, a lock when not. The glyph is a stroke on a
 * transparent ground — a filled plate would paint the page colour over the badge. */
function BadgeMark(props: { readonly earned: boolean; readonly size?: "tiny" | "small"; readonly className?: string }): ReactElement {
  const icon = props.earned ? "check" : "lock";
  const size = props.size ?? "small";
  return (
    <span aria-hidden="true" data-badge-mark={icon} className={cn("inline-flex shrink-0 bg-transparent leading-none [&_svg]:size-full [&_svg]:bg-transparent [&_svg]:fill-none", size === "tiny" ? "size-tiny" : "size-small", props.className)}>
      <Icon icon={icon} size={size} />
    </span>
  );
}

/** 🎖️ The badges card of the overview; its heading and "All badges" open the badges page (`#badges`). */
export function BadgesCard(props: { readonly state: CourseState; readonly text: CourseText; readonly locale: CourseLocale; readonly revealed: boolean; readonly onOpen: () => void }): ReactElement {
  const { state, text, locale, revealed, onOpen } = props;
  const id = useId();
  const badges = state.course?.badges ?? [];
  const earned = badges.filter((badge) => awardOf(state, badge.id) !== undefined).length;
  return (
    <CourseCard
      id={id}
      card="badges"
      anchor={PRESENCE_ANCHORS.home("badges")}
      href={`#${HOME_PAGES.badges}`}
      onOpen={onOpen}
      revealed={revealed}
      icon={<CardIcon icon="award" />}
      title={text("course.home.badges")}
      footerRight={
        <CardAction primary onClick={onOpen}>
          {text("course.home.allBadges")}
        </CardAction>
      }
    >
      <ul role="list" className="m-0 flex list-none flex-wrap gap-single p-0">
        {badges.map((badge) => {
          const held = awardOf(state, badge.id) !== undefined;
          const standing = held ? text("course.home.earned") : text("course.home.locked");
          return (
            <li
              key={badge.id}
              data-earned={held ? "" : undefined}
              data-state={held ? "earned" : "locked"}
              title={`${localized(badge.label, locale)}: ${standing}`}
              className={cn("relative flex size-workbench items-center justify-center border border-normal text-lg", held ? "border-solid text-foreground" : "border-dashed text-muted-foreground")}
            >
              <Glyph emoji={badge.emoji} />
              <BadgeMark earned={held} size="tiny" className="pointer-events-none absolute bottom-0 end-0" />
              <span className="sr-only">
                {localized(badge.label, locale)}: {standing}
              </span>
            </li>
          );
        })}
      </ul>
      <p className="m-0 text-xs text-muted-foreground">{text("course.home.badgesEarned", { earned, total: badges.length })}</p>
    </CourseCard>
  );
}

/** 🥇️ The badges page behind the badges card: every badge with its description, earned (with the day) or not yet. */
export function BadgesPage(props: { readonly state: CourseState; readonly text: CourseText; readonly locale: CourseLocale; readonly view: PaneView }): ReactElement {
  const { state, text, locale, view } = props;
  const id = useId();
  const badges = state.course?.badges ?? [];
  return (
    <PageFrame page={HOME_PAGES.badges} wide overlay={<PanePeers scope={state.course === undefined ? undefined : roomScope(state.course.id, { screen: "badges" })} opened={view.opened} />}>
      <CourseCard id={id} card="all-badges" anchor={PRESENCE_ANCHORS.badges} icon={<CardIcon icon="award" />} title={text("course.home.badges")}>
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
                    {award === undefined ? text("course.home.locked") : text("course.home.earnedAt", { date: formatDate(award.at, locale) })}
                  </p>
                  <p className="m-0 text-xs">{localized(badge.description, locale)}</p>
                </div>
              </li>
            );
          })}
        </ul>
      </CourseCard>
    </PageFrame>
  );
}
