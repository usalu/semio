/** 🎓️ Course and overview guidance through the shared repository introduction mechanism. */
import { Suspense, lazy, useEffect, useLayoutEffect, useMemo, useState, type ReactElement } from "react";
import type { IntroductionDefinition, IntroductionStepDefinition } from "@semio-tech/framework";
import type { CourseView } from "@semio-tech/course";
import { localized, courseText, type CourseLocale } from "../🌐️i18n/🟦️.ts";

const Introduction = lazy(() => import("@semio-tech/ui-react/introduction").then(({ UIIntroduction }) => ({ default: UIIntroduction })));

/** 🗺️ Localized, declarative steps anchored to the root overview's cards. */
export function courseIntroduction(course: CourseView, locale: CourseLocale): IntroductionDefinition {
  const text = courseText(locale);
  const anchor = (page: string): string => `course.home.${page}`;
  const step = (id: string, title: string, body: string, introduce: string | null): IntroductionStepDefinition => ({ id: "course.introduction." + id, title, body, introduce, show: [], placement: introduce === null ? "center" : "bottom", interactions: [], ordered: false, logos: [], demonstrations: [] });
  return {
    title: localized(course.introduction.title, locale),
    steps: [
      step("welcome", localized(course.introduction.title, locale), course.introduction.paragraphs.map(paragraph => localized(paragraph, locale)).join("\n\n"), null),
      step("learner", text("course.identity.signIn"), text("course.introduction.learner"), anchor("learner")),
      step("quizzes", text("course.home.quizzes"), text("course.introduction.quizzes"), anchor("quizzes")),
      step("board", text("course.leaderboard.title"), text("course.introduction.leaderboard"), anchor("board")),
      step("badges", text("course.home.badges"), text("course.introduction.badges"), anchor("badges")),
      step("preferences", text("course.preferences.title"), text("course.introduction.preferences"), anchor("prefs")),
    ],
  };
}

/** 👋️ A dismissible walkthrough above the overview, using its shared veil, chrome and keyboard controls. */
export function CourseIntroduction(props: { readonly course: CourseView; readonly locale: CourseLocale; readonly onDismiss: () => void; readonly onPageChange?: (page: string | undefined) => void }): ReactElement {
  const introduction = useMemo(() => courseIntroduction(props.course, props.locale), [props.course, props.locale]);
  const [stepIndex, setStepIndex] = useState(0);
  useLayoutEffect(() => props.onPageChange?.(introduction.steps[stepIndex]?.introduce?.split(".").at(-1)), [introduction, stepIndex, props.onPageChange]);
  useEffect(() => () => props.onPageChange?.(undefined), [props.onPageChange]);
  return <Suspense fallback={null}><Introduction introduction={introduction} stepIndex={stepIndex} onStepIndexChange={setStepIndex} onDismiss={props.onDismiss} /></Suspense>;
}
