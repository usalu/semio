/** 📖️ The stories gallery of `@semio-tech/pets-react`: a dev page where the owner and the artists judge pets in motion, never part of a release.
 *
 * Two views of one menagerie. *Species* is a page per species, every scene on a light and on a dark ground: at rest
 * and with every clip looping; every state in its tint, with the overlay clip of its look over the breathing idle loop
 * and the particles of its emitter; every trick (its clip replayed, the particles it throws, in the tint of the state
 * it starts from) and the purr; and every clip of getting around and being handled with what goes with it — hanging
 * in the hand with the drawing swinging about the scruff, tumbling, gliding under the species' canopy, aiming the
 * grappling gun, reeling on a rope to its hook, climbing and sliding against a wall line, mantling over a corner,
 * climbing a standing ladder, carrying a ladder, pushing a block, being dizzy, shrugging and scooting. Poses come from
 * the core (`sampleClip`, `solveRig`, `faceOf`, `particlesOf`), the drawing from the product (`depict`, `paint` and the
 * layer's scenery for tools, ladders and particles); mood, lids, mirror, speed and size apply to every specimen and the
 * pupils follow the pointer. Only specimens in view are built and painted.
 *
 * *Sandbox* is a page of mock cards — side walls with gutters between them and a page gutter around them, a task card
 * whose rows a pet may play with (`data-pet-prop`, keyed with the grounds of the cast) — on which the cast of a scene
 * lives through the real pet layer, with the host's switches (scene, mode, quiet, play, mischief, capacity, scale, seed,
 * tempo), the learner's hand, the deeds of the keyboard and an inspector. Through the director the dev server puts
 * between the layer and the core (`../🏗️builder/🌐️vite/🟦️.ts`) the inspector reads every frame — poofs, overlapping
 * bodies (there must never be one), particles, rate — and folds forced states, tricks, moods and activities into the
 * stage by the stage's own rules; footings are reached the way the learner reaches them, by pressing, carrying and
 * letting go of a pet. The last scene of every species page is the poof, the dust it leaves where it vanishes.
 *
 * The document hands over whatever the menagerie file exports; the first valid menagerie found in it is shown. Every
 * label exists in English and German, and the page starts in the browser's language.
 *
 * @see ./🌐️.html — the document
 * @see ./🔤️language-and-menagerie/🟦️.ts — the labels and the menagerie source
 * @see ./🎬️studio/🟦️.tsx — the studio of specimens
 * @see ./🎚️controls/🟦️.tsx — the small controls
 * @see ./🐾️species-gallery/🟦️.tsx — the species pages
 * @see ./🎛️director-and-inspector/🟦️.ts — the director and the inspector
 * @see ./🏟️sandbox/🟦️.tsx — the sandbox
 * @see ../🏗️builder/🌐️vite/🟦️.ts — the dev server, the menagerie module and the director
 * @see ../🔨️modules/🖌️depiction/🟦️.ts — the renderer under judgement
 * @see ../🔨️modules/🫧️layer/🟦️.tsx — the layer and its scenery
 */

import { StrictMode, useEffect, useMemo, useState, type ReactElement } from "react";
import { createRoot } from "react-dom/client";
import "./🎨️.css";
import { type Labels, LABELS, type Language, LANGUAGES, menagerieOf, preferredLanguage } from "./🔤️language-and-menagerie/🟦️.ts";
import { Action, Choice, Toggle } from "./🎚️controls/🟦️.tsx";
import { Gallery } from "./🐾️species-gallery/🟦️.tsx";
import type { Director } from "./🎛️director-and-inspector/🟦️.ts";
import { Sandbox } from "./🏟️sandbox/🟦️.tsx";

//#region 🔖️Page
/** 🏛️ The gallery: its header with the view, language and theme switches, then the chosen view of the menagerie found in `source`. */
function Stories({ source, origin, director }: { readonly source: unknown; readonly origin: string; readonly director: Director | null }): ReactElement {
  const [language, setLanguage] = useState<Language>(preferredLanguage);
  const [dark, setDark] = useState(() => window.matchMedia("(prefers-color-scheme: dark)").matches);
  const [view, setView] = useState<"species" | "sandbox">("species");
  const { menagerie, issues } = useMemo(() => menagerieOf(source), [source]);
  const text: Labels = LABELS[language];
  useEffect(() => {
    document.documentElement.lang = language;
    document.title = text.title;
  }, [language, text]);
  useEffect(() => {
    document.documentElement.classList.toggle("dark", dark);
  }, [dark]);
  return (
    <main className="stories">
      <header className="masthead">
        <h1>
          {text.title}
          {menagerie ? ` · ${menagerie.title[language]}` : ""}
        </h1>
        <code>{origin}</code>
        <Action label={text.species} pressed={view === "species"} onPress={() => setView("species")} />
        <Action label={text.sandbox} pressed={view === "sandbox"} onPress={() => setView("sandbox")} />
        <Choice label={text.language} value={language} options={LANGUAGES.map((name) => [name, name] as const)} onChange={setLanguage} />
        <Toggle label={text.dark} checked={dark} onChange={setDark} />
      </header>
      {menagerie ? null : (
        <div className="notice">
          <p>
            {source === null ? text.missing : text.empty} <code>{origin}</code>
          </p>
          {issues.length === 0 ? null : (
            <>
              <p>{text.issues}</p>
              <ul>
                {issues.map((issue, index) => (
                  <li key={index}>
                    <code>{issue.path}</code> · {issue.code}
                  </li>
                ))}
              </ul>
            </>
          )}
        </div>
      )}
      {menagerie && view === "species" ? <Gallery menagerie={menagerie} language={language} text={text} /> : null}
      {menagerie && view === "sandbox" ? <Sandbox menagerie={menagerie} text={text} director={director} /> : null}
    </main>
  );
}

/** 🚀️ Mounts the gallery into `root` for whatever the menagerie file at `origin` exports (`null` when there is no such file), with the director of the dev server between the pet layer and the core (none: the sandbox shows no reading and forces nothing). */
export function mountStories(root: HTMLElement | null, source: unknown, origin: string, director: Director | null = null): void {
  if (!root) return;
  createRoot(root).render(
    <StrictMode>
      <Stories source={source} origin={origin} director={director} />
    </StrictMode>,
  );
}
//#endregion 🔖️Page
