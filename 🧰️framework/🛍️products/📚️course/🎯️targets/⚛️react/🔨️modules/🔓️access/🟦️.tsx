/** 🪟️ Scheduled UI access, a hidden author-preview argument and visible inert previews. */
import { useEffect, useState, type ReactNode } from "react";
import { courseAccess, courseRelease, type CourseView } from "@semio-tech/course";
import { Icon } from "@semio-tech/ui-react/chrome";
import type { CourseLocale, CourseText } from "../🌐️i18n/🟦️.ts";

/** 🗝️ Author preview is available only through the explicit URL argument. */
export function previewAccess(search = typeof window === "undefined" ? "" : window.location.search): boolean {
  return new URLSearchParams(search).get("preview") === "1";
}

/** 🚧️ Every UI entry uses the same date and ancestor checks, including ordinary deep links. */
export function clientAccess(course: CourseView | undefined, page: string, at: number, task?: string): boolean {
  return previewAccess() || course === undefined || courseAccess(course, page, at, task);
}

/** ⏰️ Wakes at the next release and when the browser regains attention, without polling. */
export function useReleaseClock(course: CourseView | undefined, now: () => number = Date.now): number {
  const [at, setAt] = useState(now);
  useEffect(() => {
    let timer: ReturnType<typeof setTimeout> | undefined;
    const update = (): void => {
      clearTimeout(timer);
      const instant = now();
      setAt(instant);
      const next = Math.min(...Object.values(course?.unlocks ?? {}).filter((value): value is number => value !== null && value > instant));
      if (Number.isFinite(next)) timer = setTimeout(update, Math.min(next - instant, 2_147_483_647));
    };
    update();
    window.addEventListener("focus", update);
    document.addEventListener("visibilitychange", update);
    return () => {
      clearTimeout(timer);
      window.removeEventListener("focus", update);
      document.removeEventListener("visibilitychange", update);
    };
  }, [course?.unlocks, now]);
  return at;
}

/** 🔒️ Keeps a locked preview rendered behind a theme-aware tint with no operable descendants. */
export function AccessGate(props: { readonly course: CourseView | undefined; readonly page: string; readonly task?: string; readonly at: number; readonly locale: CourseLocale; readonly text: CourseText; readonly children: ReactNode }): ReactNode {
  if (clientAccess(props.course, props.page, props.at, props.task)) return props.children;
  const release = props.course === undefined ? undefined : courseRelease(props.course, props.page, props.task);
  const date = release == null || !Number.isFinite(new Date(release).getTime()) ? undefined : new Intl.DateTimeFormat(props.locale, { dateStyle: "medium", timeStyle: "short" }).format(release);
  return (
    <div data-course-locked={props.task === undefined ? props.page : `${props.page}/${props.task}`} className="course-access-gate relative h-full min-h-0 w-full">
      <div inert aria-hidden="true" className="course-access-preview h-full pointer-events-none select-none" onClickCapture={(event) => { event.preventDefault(); event.stopPropagation(); }} onKeyDownCapture={(event) => { event.preventDefault(); event.stopPropagation(); }}>{props.children}</div>
      <div className="course-access-tint absolute inset-0 z-10 flex items-center justify-center bg-background/80 p-double text-foreground">
        <p className="m-0 flex items-center gap-single border border-normal bg-background px-double py-single text-sm" role="status">
          <Icon icon="lock" size="small" />
          {date === undefined ? props.text("course.access.locked") : props.text("course.access.opens", { date })}
        </p>
      </div>
    </div>
  );
}
