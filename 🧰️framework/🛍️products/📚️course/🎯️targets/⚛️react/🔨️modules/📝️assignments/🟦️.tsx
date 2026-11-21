/** 📝️ Assignments are the course's ungraded exercises: a card per assignment on the assignments page and, behind it, the
 * PDF in the learner's language, with a download, a tab of its own and the way back. A wide, fine-pointer screen also shows an inline preview.
 * @see ../🏠️home/🟦️.tsx — the overview that hosts the cards and pages
 * @see ../../../../🧬️schema/🔣️.json — `Assignment`, the contract of an exercise */
import { useId, type ReactElement } from "react";
import type { Assignment } from "@semio-tech/course";
import { UI_MOBILE_MEDIA_QUERY, useMediaQuery } from "@semio-tech/ui-react/chrome";
import { localized, type CourseLocale, type CourseText } from "../🌐️i18n/🟦️.ts";
import { BodyButton, CELL, CardAction, CardIcon, CardLink, CardSection, CourseCard, GROUP, Glyph, PageFrame, RowLink } from "../🪟️chrome/🟦️.tsx";
import { PRESENCE_ANCHORS } from "../👥️presence/👁️view/🟦️.ts";
import { HOME_PAGES } from "../🧭️session/🧮️state/🟦️.ts";
import type { CourseSession } from "../🧭️session/🟦️.ts";

const VIEWER = "h-[75vh] min-h-[24rem] w-full border-0 bg-background";

/** 📍️ Where the site serves the PDF of an assignment in `locale`, from its own origin. */
export function assignmentUrl(assignment: Assignment, locale: CourseLocale): string {
  return `/assignments/${assignment.documents[locale].file}`;
}

/** 🃏️ The card of an assignment on the assignments page: its emoji, title and description, a direct download and the way to its page (`#<assignment>`). */
export function AssignmentCardView(props: { readonly assignment: Assignment; readonly locale: CourseLocale; readonly text: CourseText; readonly revealed?: boolean; readonly onOpen?: () => void }): ReactElement {
  const { assignment, locale, text, revealed, onOpen } = props;
  const id = useId();
  const file = assignment.documents[locale].file;
  return (
    <CourseCard
      id={id}
      card={`assignment:${assignment.id}`}
      anchor={PRESENCE_ANCHORS.home(assignment.id)}
      href={onOpen === undefined ? undefined : `#${assignment.id}`}
      onOpen={onOpen}
      revealed={revealed}
      icon={<Glyph emoji={assignment.emoji} className="text-sm" />}
      title={localized(assignment.title, locale)}
      footerLeft={<CardLink href={assignmentUrl(assignment, locale)} download={file}>{text("course.assignments.download")}</CardLink>}
      footerRight={onOpen === undefined ? undefined : <CardAction primary onClick={onOpen}>{text("course.assignments.open")}</CardAction>}
    >
      <p className="m-0 text-xs leading-normal text-foreground">{localized(assignment.description, locale)}</p>
    </CourseCard>
  );
}

/** 📄️ The page of an assignment: the PDF's inline preview only when opened on a wide, fine-pointer screen, after the controls a keyboard reaches first — back to the assignments, download and a tab of its own. */
export function AssignmentPage(props: { readonly assignment: Assignment; readonly session: CourseSession; readonly locale: CourseLocale; readonly text: CourseText; readonly opened: boolean }): ReactElement {
  const { assignment, session, locale, text, opened } = props;
  const openSeparately = useMediaQuery(`${UI_MOBILE_MEDIA_QUERY}, (pointer: coarse)`);
  const id = useId();
  const title = localized(assignment.title, locale);
  const url = assignmentUrl(assignment, locale);
  return (
    <PageFrame page={`assignment:${assignment.id}`} wide>
      <CourseCard flush id={id} card="assignment" anchor={PRESENCE_ANCHORS.home(assignment.id)} icon={<Glyph emoji={assignment.emoji} className="text-sm" />} title={title}>
        <CardSection>
          <p className="m-0 text-sm leading-normal">{localized(assignment.description, locale)}</p>
        </CardSection>
        <div role="group" aria-label={text("course.assignments.viewerTitle", { title })} {...GROUP} {...CELL} className="flex flex-col">
          <BodyButton row onClick={() => session.open({ screen: "home", page: HOME_PAGES.assignments })}>
            <CardIcon icon="arrow-left" />
            {text("course.assignments.back")}
          </BodyButton>
          <RowLink primary={!openSeparately} href={url} download={assignment.documents[locale].file}>
            <CardIcon icon="download" />
            {text("course.assignments.download")}
          </RowLink>
          <RowLink primary={openSeparately} href={url} external>
            <CardIcon icon="external-link" />
            {text("course.assignments.newTab")}
          </RowLink>
        </div>
        {openSeparately ? null : (
          <>
            {opened ? <iframe key={url} {...CELL} src={`${url}#view=FitH`} title={text("course.assignments.viewerTitle", { title })} lang={locale} className={VIEWER} /> : <div aria-hidden="true" {...CELL} data-assignment-viewer="waiting" className={VIEWER} />}
            <CardSection>
              <p className="m-0 text-xs text-muted-foreground">{text("course.assignments.fallback")}</p>
            </CardSection>
          </>
        )}
      </CourseCard>
    </PageFrame>
  );
}
