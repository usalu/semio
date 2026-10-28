/** 🔓️ Authored release schedules shared by every client; all ancestor dates must have arrived. */
import type { CourseView, Timestamp } from "../../🧬️schema/🟦️.ts";

/** 📆️ The latest applicable release; null means awaiting scheduling and undefined means unrestricted. */
export function courseRelease(course: Pick<CourseView, "topics" | "unlocks">, page: string, task?: string): Timestamp | null | undefined {
  const topic = course.topics?.find((candidate) => candidate.quizzes.includes(page))?.id;
  const keys = [topic, page, task === undefined ? undefined : `${page}/${task}`];
  let latest: Timestamp | undefined;
  for (const key of keys) {
    if (key === undefined || !Object.hasOwn(course.unlocks ?? {}, key)) continue;
    const at = course.unlocks![key];
    if (at === null) return null;
    if (at !== undefined) latest = Math.max(latest ?? 0, at);
  }
  return latest;
}

/** 🚪️ Whether a topic, quiz or question is available at the supplied Unix millisecond instant. */
export function courseAccess(course: Pick<CourseView, "topics" | "unlocks">, page: string, now: Timestamp, task?: string): boolean {
  const release = courseRelease(course, page, task);
  return release === undefined || (release !== null && now >= release);
}
