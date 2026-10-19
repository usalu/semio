import { learnerTag, type CourseView } from "@semio-tech/course";
import { titled, type CourseLocale, type CourseText } from "../../🌐️i18n/🟦️.ts";
import { learnerName } from "../../🪪️identity/🟦️.tsx";
import { HOME_PAGES, type CourseState } from "../../🧭️session/🧮️state/🟦️.ts";

/** 🏡️ The root overview's general pages, in reading order. */
export function homePages(): readonly string[] {
  return [HOME_PAGES.learner, HOME_PAGES.quizzes, HOME_PAGES.leaderboard, HOME_PAGES.badges, HOME_PAGES.preferences];
}

/** 🌳️ The cards immediately within a home overview or topic. */
export function pagesWithin(page: string | undefined, course: CourseView): readonly string[] {
  if (page === undefined) return homePages();
  if (page === HOME_PAGES.quizzes) return course.topics === undefined ? course.quizzes.map((quiz) => quiz.id) : course.topics.map((topic) => topic.id);
  return course.topics?.find((topic) => topic.id === page)?.quizzes ?? [];
}

/** 🔗️ Every home address, including containers and their leaves. */
export function addressPages(course: CourseView): readonly string[] {
  return [...new Set([...homePages(), ...(course.topics?.map((topic) => topic.id) ?? []), ...course.quizzes.map((quiz) => quiz.id)])];
}

/** 🪜️ The overview containing a home page; root cards have no parent page. */
export function parentPage(page: string, course: CourseView): string | undefined {
  if (homePages().includes(page)) return undefined;
  if (course.topics?.some((topic) => topic.id === page)) return HOME_PAGES.quizzes;
  if (course.quizzes.some((quiz) => quiz.id === page)) return course.topics?.find((topic) => topic.quizzes.includes(page))?.id ?? HOME_PAGES.quizzes;
  return undefined;
}

/** 🗂️ Whether a home page contains a layered card grid. */
export function isHomeGrid(page: string | undefined, course: CourseView): boolean {
  return page === undefined || page === HOME_PAGES.quizzes || pagesWithin(page, course).length > 0;
}

/** 🖱️ Where a pointer sits in an overview, from the top-left corner to the bottom-right. */
export interface PreviewCamera {
  readonly x: number;
  readonly y: number;
}

/** 🖱️ The camera a grid applies to its own strip. The active grid zooms a hovered card and follows its own pointer, so it
 * takes a camera only while a followed peer supplies one. Every grid behind that follows the inherited pointer and does not zoom.
 * @see ../🟦️.tsx — the overview that publishes the active pointer and paints inherited cameras */
export function previewCamera(active: boolean, followed: PreviewCamera | undefined, inherited: PreviewCamera | undefined): PreviewCamera | undefined {
  return active ? followed : inherited;
}

/** 🧭️ The direct child opened by a descendant address in an overview. */
export function openedChild(container: string | undefined, page: string | undefined, course: CourseView): string | null {
  if (page === undefined || page === container) return null;
  const children = pagesWithin(container, course);
  let candidate: string | undefined = page;
  while (candidate !== undefined) {
    if (children.includes(candidate)) return candidate;
    candidate = parentPage(candidate, course);
  }
  return null;
}

/** 🏷️ What a page of home is called: a quiz by its title, the learner's page by the learner's name, the others by what
 * they hold; nothing for a page home does not have. */
export function pageLabel(page: string, state: Pick<CourseState, "course" | "learner">, locale: CourseLocale, text: CourseText, compact = false): string | undefined {
  const topic = state.course?.topics?.find((candidate) => candidate.id === page);
  if (topic !== undefined) return titled(topic, locale).full;
  const quiz = state.course?.quizzes.find((candidate) => candidate.id === page);
  if (quiz !== undefined) {
    const names = titled(quiz, locale);
    return compact ? names.short : names.full;
  }
  switch (page) {
    case HOME_PAGES.quizzes:
      return text("course.home.quizzes");
    case HOME_PAGES.learner:
      return state.learner === undefined ? text("course.identity.signIn") : learnerName(state.learner.identity, learnerTag(state.learner.id), text);
    case HOME_PAGES.leaderboard:
      return compact ? text("course.leaderboard.titleShort") : text("course.leaderboard.title");
    case HOME_PAGES.badges:
      return text("course.home.badges");
    case HOME_PAGES.preferences:
      return compact ? text("course.preferences.titleShort") : text("course.preferences.title");
    default:
      return undefined;
  }
}
