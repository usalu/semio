/** 🪟️ Scheduled UI access, a hidden author-preview argument and visible inert previews. */
import { useEffect, useState, type ReactNode } from "react";
import { courseAccess, courseRelease, type CourseView } from "@semio-tech/course";
import { Icon, SharedBorderRow, SharedBorderRowCell, useLevel, windowFrameChipLevel } from "@semio-tech/ui-react/chrome";
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

/** 🌫️ The tint every content that is not available yet wears, over the content it leaves loaded and visible beneath: the one look of a locked page, a locked task and a clip that is not hosted yet. */
export function UnavailableTint(props: { readonly children?: ReactNode }): ReactNode {
  return <div data-course-unavailable-tint="" className={`course-access-tint absolute inset-0 z-10 flex items-center justify-center p-double text-foreground ${props.children === undefined ? "pointer-events-none" : "pointer-events-auto"}`}>{props.children}</div>;
}

/** 🔒️ Keeps a locked preview rendered exactly as its unlocked self (the same card filling the same cell, with the same chips) behind a theme-aware tint with no operable descendants; the gate is its own stacking context, so the tint never covers the footer chips of a neighbouring card that share its seam. */
export function AccessGate(props: { readonly course: CourseView | undefined; readonly page: string; readonly task?: string; readonly at: number; readonly locale: CourseLocale; readonly text: CourseText; readonly children: ReactNode }): ReactNode {
  const level = windowFrameChipLevel(useLevel());
  if (clientAccess(props.course, props.page, props.at, props.task)) return props.children;
  const release = props.course === undefined ? undefined : courseRelease(props.course, props.page, props.task);
  const date = release == null || !Number.isFinite(new Date(release).getTime()) ? undefined : new Intl.DateTimeFormat(props.locale, { dateStyle: "medium", timeStyle: "short" }).format(release);
  return (
    <div data-course-locked={props.task === undefined ? props.page : `${props.page}/${props.task}`} className="course-access-gate relative flex h-full min-h-0 w-full flex-1 flex-col" style={{ isolation: "isolate" }}>
      <div inert aria-hidden="true" className="course-access-preview flex min-h-0 flex-1 flex-col pointer-events-none select-none" onClickCapture={(event) => { event.preventDefault(); event.stopPropagation(); }} onKeyDownCapture={(event) => { event.preventDefault(); event.stopPropagation(); }}>{props.children}</div>
      <UnavailableTint>
        <SharedBorderRow role="status" className="ui-glass" data-glass-chip="" data-level={level}>
          <SharedBorderRowCell className="flex items-center gap-single px-double py-single text-sm">
            <Icon icon="lock" size="small" />
            {date === undefined ? props.text("course.access.locked") : props.text("course.access.opens", { date })}
          </SharedBorderRowCell>
        </SharedBorderRow>
      </UnavailableTint>
    </div>
  );
}
