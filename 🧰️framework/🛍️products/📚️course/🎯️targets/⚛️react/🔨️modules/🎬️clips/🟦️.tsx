/** 📽️ Learning clips are short videos hosted on other sites: a card per clip on the clips page with its poster and,
 * behind it, a page with the poster larger. A clip whose address for the learner's language is not known yet shows its poster
 * and says it is coming soon; one that has it links to that site in a tab of its own.
 * @see ../🏠️home/🟦️.tsx — the overview that hosts the cards and pages
 * @see ../../../../🧬️schema/🔣️.json — `Clip`, the contract of a clip and where its address goes */
import { createContext, useContext, useId, type ReactElement, type ReactNode } from "react";
import type { Clip } from "@semio-tech/course";
import { localized, type CourseLocale, type CourseText } from "../🌐️i18n/🟦️.ts";
import { BodyButton, CELL, CardIcon, CardLink, CardSection, CourseCard, Glyph, PageFrame, RowLink } from "../🪟️chrome/🟦️.tsx";
import { PRESENCE_ANCHORS } from "../👥️presence/👁️view/🟦️.ts";
import { HOME_PAGES } from "../🧭️session/🧮️state/🟦️.ts";
import type { CourseSession } from "../🧭️session/🟦️.ts";

/** 🖼️ The built URL of every poster the site ships, by the file name a clip names. */
export type ClipPosters = Readonly<Record<string, string>>;

const ClipPostersContext = createContext<ClipPosters>({});

/** 🎁️ Hands the posters of the site's build to every clip below. */
export function ClipPostersProvider(props: { readonly posters: ClipPosters | undefined; readonly children: ReactNode }): ReactElement {
  return <ClipPostersContext.Provider value={props.posters ?? {}}>{props.children}</ClipPostersContext.Provider>;
}

/** 🔗️ Where a clip is hosted for `locale`; `undefined` while it is not hosted in that language. */
export function clipUrl(clip: Clip, locale: CourseLocale): string | undefined {
  return clip.urls?.[locale];
}

function Poster(props: { readonly clip: Clip }): ReactElement {
  const url = useContext(ClipPostersContext)[props.clip.poster.file];
  return (
    <div data-clip-poster="" {...CELL} className="relative flex aspect-video w-full items-center justify-center overflow-hidden bg-background">
      {url === undefined ? <Glyph emoji={props.clip.emoji} className="text-2xl" /> : <img src={url} alt="" width={640} height={360} loading="lazy" decoding="async" className="size-full object-cover" />}
    </div>
  );
}

function WatchRow(props: { readonly clip: Clip; readonly locale: CourseLocale; readonly text: CourseText }): ReactElement {
  const { clip, locale, text } = props;
  const url = clipUrl(clip, locale);
  if (url === undefined) return <CardSection><span data-clip-state="soon" className="inline-flex items-center gap-single text-xs font-semibold text-muted-foreground"><CardIcon icon="clock" />{text("course.clips.soon")}</span></CardSection>;
  return <RowLink primary external href={url} label={text("course.clips.watchTitle", { title: localized(clip.title, locale) })}>{text("course.clips.watch")}</RowLink>;
}

function Watch(props: { readonly clip: Clip; readonly locale: CourseLocale; readonly text: CourseText }): ReactElement {
  const { clip, locale, text } = props;
  const url = clipUrl(clip, locale);
  if (url === undefined) return <span data-clip-state="soon" className="inline-flex items-center gap-single text-xs font-semibold text-muted-foreground"><CardIcon icon="clock" />{text("course.clips.soon")}</span>;
  return <CardLink primary external href={url} label={text("course.clips.watchTitle", { title: localized(clip.title, locale) })}>{text("course.clips.watch")}</CardLink>;
}

/** 🃏️ The card of a clip on the clips page: its poster, description and either the link to the hosted clip or that it is coming soon; its heading opens the clip's page (`#<clip>`). */
export function ClipCardView(props: { readonly clip: Clip; readonly locale: CourseLocale; readonly text: CourseText; readonly revealed?: boolean; readonly onOpen?: () => void }): ReactElement {
  const { clip, locale, text, revealed, onOpen } = props;
  const id = useId();
  return (
    <CourseCard
      flush
      id={id}
      card={`clip:${clip.id}`}
      anchor={PRESENCE_ANCHORS.home(clip.id)}
      href={onOpen === undefined ? undefined : `#${clip.id}`}
      onOpen={onOpen}
      revealed={revealed}
      icon={<Glyph emoji={clip.emoji} className="text-sm" />}
      title={localized(clip.title, locale)}
      footerRight={<Watch clip={clip} locale={locale} text={text} />}
    >
      <Poster clip={clip} />
      <CardSection>
        <p className="m-0 text-xs leading-normal text-foreground">{localized(clip.description, locale)}</p>
      </CardSection>
    </CourseCard>
  );
}

/** 📄️ The page of a clip: the way back to the clips, the link to the hosted clip or that it is coming soon, its description and its poster. */
export function ClipPage(props: { readonly clip: Clip; readonly session: CourseSession; readonly locale: CourseLocale; readonly text: CourseText }): ReactElement {
  const { clip, session, locale, text } = props;
  const id = useId();
  return (
    <PageFrame page={`clip:${clip.id}`}>
      <CourseCard flush id={id} card="clip" anchor={PRESENCE_ANCHORS.home(clip.id)} icon={<Glyph emoji={clip.emoji} className="text-sm" />} title={localized(clip.title, locale)}>
        <CardSection>
          <p className="m-0 text-sm leading-normal">{localized(clip.description, locale)}</p>
        </CardSection>
        <BodyButton row onClick={() => session.open({ screen: "home", page: HOME_PAGES.clips })}>
          <CardIcon icon="arrow-left" />
          {text("course.clips.back")}
        </BodyButton>
        <WatchRow clip={clip} locale={locale} text={text} />
        <Poster clip={clip} />
      </CourseCard>
    </PageFrame>
  );
}
