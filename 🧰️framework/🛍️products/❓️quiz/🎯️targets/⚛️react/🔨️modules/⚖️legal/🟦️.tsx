/** ⚖️ The footer every screen ends with and the notice behind it: what this client stores and where — on the proctor,
 * in the browser, live among the learners online — in plain words that state only what the code does, and the links to
 * the operator's imprint and privacy policy. The links belong to the site, not to the client: a site passes them in,
 * and each is shown only when it is an absolute http(s) address. While no language is known the footer speaks every
 * offered language at once.
 *
 * @see ../🧭️session/🟦️.ts — the persisted local-only slices the notice names
 * @see ../👥️presence/🟦️.tsx — what is shared live and never stored
 */

import { useId, useState, type ReactElement, type ReactNode } from "react";
import type { QuizLabelKey, QuizLocale, QuizText } from "../🌐️i18n/🟦️.ts";
import { CardAction, CardIcon, Dialog } from "../🪟️chrome/🟦️.tsx";
import { EveryLanguage, everyLanguage } from "../🎛️preferences/🟦️.tsx";

/** 📜️ The legal pages of the site that hosts the client: its imprint and its privacy policy, each an absolute URL. */
export interface QuizLegal {
  readonly imprint?: string;
  readonly privacy?: string;
}

/** 🔗️ `url` when it is an absolute http(s) address, else nothing — nothing else is ever rendered as a link. */
export function legalLink(url: string | undefined): string | undefined {
  if (url === undefined) return undefined;
  try {
    const parsed = new URL(url);
    return parsed.protocol === "https:" || parsed.protocol === "http:" ? parsed.href : undefined;
  } catch {
    return undefined;
  }
}

const LINK = "quiz-target inline-flex cursor-pointer items-center border-0 bg-transparent p-0 text-xs text-muted-foreground underline hover:text-foreground";

function ExternalLink(props: { readonly href: string; readonly children: ReactNode; readonly external: ReactNode }): ReactElement {
  return (
    <a href={props.href} target="_blank" rel="noopener noreferrer" className={LINK}>
      {props.children}
      <span className="sr-only"> ({props.external})</span>
    </a>
  );
}

/** 🔏️ What the client stores, as a dialog: the proctor's records and what of them is public, this browser's storage,
 * what is only shown live — and, when the site names them, where to ask and what else to read. */
export function PrivacyNotice(props: { readonly legal: QuizLegal | undefined; readonly text: QuizText; readonly onClose: () => void }): ReactElement {
  const { legal, text, onClose } = props;
  const id = useId();
  const imprint = legalLink(legal?.imprint);
  const policy = legalLink(legal?.privacy);
  return (
    <Dialog
      id={`${id}-title`}
      role="dialog"
      wide
      icon={<CardIcon icon="info" />}
      title={text("quiz.legal.privacyTitle")}
      describedBy={`${id}-body`}
      onEscape={onClose}
      footerRight={
        <CardAction primary data-autofocus="" onClick={onClose}>
          {text("quiz.app.close")}
        </CardAction>
      }
    >
      <div id={`${id}-body`} className="flex flex-col gap-double text-sm leading-normal">
        <p className="m-0">{text("quiz.legal.server")}</p>
        <p className="m-0">{text("quiz.legal.browser")}</p>
        <p className="m-0">{text("quiz.legal.presence")}</p>
        <p className="m-0">{text("quiz.identity.forFun")}</p>
        {imprint === undefined ? null : <p className="m-0">{text("quiz.legal.contact")}</p>}
      </div>
      {imprint === undefined && policy === undefined ? null : (
        <p className="m-0 flex flex-wrap gap-x-double">
          {imprint === undefined ? null : (
            <ExternalLink href={imprint} external={text("quiz.legal.external")}>
              {text("quiz.legal.imprint")}
            </ExternalLink>
          )}
          {policy === undefined ? null : (
            <ExternalLink href={policy} external={text("quiz.legal.external")}>
              {text("quiz.legal.policy")}
            </ExternalLink>
          )}
        </p>
      )}
    </Dialog>
  );
}

/** 🦶️ The footer of the client: "What is stored" (the {@link PrivacyNotice}) and the site's imprint and privacy
 * policy when it names them, at the end of its row; before them whatever else the client keeps within reach on every
 * screen (`children`: the pets switch). Without a `locale` — no language is known yet — the links are named in every
 * offered language and the notice, which needs one, waits. */
export function LegalFooter(props: { readonly legal: QuizLegal | undefined; readonly locale: QuizLocale | undefined; readonly text: QuizText | undefined; readonly children?: ReactNode }): ReactElement | null {
  const { legal, locale, text } = props;
  const [reading, setReading] = useState(false);
  const imprint = legalLink(legal?.imprint);
  const policy = legalLink(legal?.privacy);
  const label = (key: QuizLabelKey): ReactNode => (text === undefined ? <EveryLanguage label={key} /> : text(key));
  if (text === undefined && imprint === undefined && policy === undefined) return null;
  return (
    <footer className="quiz-footer flex shrink-0 flex-wrap items-center gap-x-double border-t border-normal px-double" lang={locale}>
      {props.children}
      <nav className="ms-auto" aria-label={text === undefined ? everyLanguage("quiz.legal.label") : text("quiz.legal.label")}>
        <ul role="list" className="m-0 flex list-none flex-wrap items-center justify-end gap-x-double p-0">
          {text === undefined ? null : (
            <li>
              <button type="button" className={LINK} onClick={() => setReading(true)}>
                {text("quiz.legal.privacy")}
              </button>
            </li>
          )}
          {imprint === undefined ? null : (
            <li>
              <ExternalLink href={imprint} external={label("quiz.legal.external")}>
                {label("quiz.legal.imprint")}
              </ExternalLink>
            </li>
          )}
          {policy === undefined ? null : (
            <li>
              <ExternalLink href={policy} external={label("quiz.legal.external")}>
                {label("quiz.legal.policy")}
              </ExternalLink>
            </li>
          )}
        </ul>
      </nav>
      {reading && text !== undefined ? <PrivacyNotice legal={legal} text={text} onClose={() => setReading(false)} /> : null}
    </footer>
  );
}
