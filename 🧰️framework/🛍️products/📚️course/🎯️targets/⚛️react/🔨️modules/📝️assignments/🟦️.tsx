import type { Assignment } from "@semio-tech/course";
import type { CourseLocale } from "../🌐️i18n/🟦️.ts";

/** 📄️ Localized printable exercises, downloaded from the site's own published artifact collection. */
export function AssignmentDownloads({ assignments, locale, topic }: { readonly assignments: readonly Assignment[]; readonly locale: CourseLocale; readonly topic?: string }) {
  const visible = assignments.filter((assignment) => topic === undefined || assignment.topic === topic);
  if (visible.length === 0) return null;
  return (
    <section aria-label={locale === "en" ? "Assignments" : "Übungen"} className="flex flex-wrap gap-double px-double py-double">
      {visible.map((assignment) => (
        <article key={assignment.id} className="flex flex-col gap-single">
          <a href={`/assignments/${assignment.documents[locale].file}`} download={assignment.documents[locale].file} className="text-sm font-semibold underline underline-offset-4 focus-visible:outline focus-visible:outline-2">
            <span aria-hidden="true">{assignment.emoji} </span>{assignment.title[locale]} · PDF
          </a>
          <p className="m-0 text-xs">{assignment.description[locale]}</p>
        </article>
      ))}
    </section>
  );
}
