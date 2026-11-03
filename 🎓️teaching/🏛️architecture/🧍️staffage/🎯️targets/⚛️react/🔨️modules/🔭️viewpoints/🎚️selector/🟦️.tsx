/** 🎚️ The viewpoint selector: side elevation, front elevation, plan (top view) or three views, as one radio group with arrow-key selection and one polite live region that announces a change.
 *
 * Names come in the reader's language. A viewpoint the asset cannot draw at this scale stays selectable and is marked, with the reason from the resolver in its accessible name and tooltip, so choosing it shows why instead of silently choosing another.
 *
 * @see ../🟦️.ts — the choices, names and resolutions
 * @see ../🪟️sheet/🟦️.tsx — the three-view sheet that the "all" choice opens
 */
import type { ReactElement } from "react";
import type { Language, Viewpoint } from "@teaching/staffage";
import { Segmented } from "../../🪟️chrome/🟦️.tsx";
import { VIEWPOINT_CHOICES, VIEWPOINT_NAMES, VIEWPOINT_PHRASES, viewpointWord, type ViewResolution, type ViewpointChoice } from "../🟦️.ts";

/** ⚙️ The props of {@link ViewpointSelector}. */
export type ViewpointSelectorProps = {
  readonly value: ViewpointChoice;
  readonly language: Language;
  readonly onChange: (choice: ViewpointChoice) => void;
  readonly focus?: Viewpoint;
  readonly resolutions?: Readonly<Record<Viewpoint, ViewResolution>>;
  readonly names?: Readonly<Partial<Record<Viewpoint, string>>>;
  readonly choices?: readonly ViewpointChoice[];
  readonly className?: string;
};

/** 📢️ What a screen reader hears when the choice changes. */
export function announcement(value: ViewpointChoice, language: Language, focus: Viewpoint = "elevation"): string {
  return value === "all" ? viewpointWord("viewpoints.announce.all", language, { view: VIEWPOINT_PHRASES[focus][language] }) : viewpointWord("viewpoints.announce.single", language, { view: VIEWPOINT_PHRASES[value][language] });
}

/** 🎚️ The selector. */
export function ViewpointSelector(props: ViewpointSelectorProps): ReactElement {
  const { value, language, onChange, resolutions, names, className, focus, choices = VIEWPOINT_CHOICES } = props;
  return (
    <div data-viewpoint-selector="" className={className}>
      <Segmented
        label={viewpointWord("viewpoints.label", language)}
        value={value}
        onChange={onChange}
        options={choices.map((choice) => {
          const resolution = choice === "all" ? undefined : resolutions?.[choice];
          const missing = resolution !== undefined && !resolution.available;
          const name = choice === "all" ? VIEWPOINT_NAMES.all[language] : names?.[choice] ?? VIEWPOINT_NAMES[choice][language];
          const reason = resolution !== undefined && !resolution.available ? resolution.message[language] : "";
          const spoken = missing ? viewpointWord("viewpoints.choice.unavailable", language, { name: VIEWPOINT_NAMES[choice][language], reason }) : VIEWPOINT_NAMES[choice][language];
          return { value: choice, label: missing ? <span className="line-through decoration-dotted">{name}</span> : name, name: spoken, title: missing ? spoken : viewpointWord("viewpoints.help", language), lang: language };
        })}
      />
      <span role="status" aria-live="polite" data-viewpoint-announcement="" className="sr-only">
        {announcement(value, language, focus)}
      </span>
    </div>
  );
}
