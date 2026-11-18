import * as React from "react";
import { ToggleGroup, type ToggleGroupSingleProps } from "../🎛️ToggleGroup/🟦️.tsx";

/** 🈯️ One language of a {@link LanguageChoice}: its BCP 47 `value` and its name in its own words (`endonym`). */
export interface LanguageChoiceOption<L extends string = string> {
  readonly value: L;
  readonly endonym: string;
}

/** 🈯️ Props of {@link LanguageChoice}: `value` is `undefined` while no language is chosen, so no option is preselected. */
export interface LanguageChoiceProps<L extends string = string> extends Omit<ToggleGroupSingleProps, "kind" | "semantics" | "items" | "value" | "defaultValue" | "onValueChange" | "aria-label"> {
  readonly options: readonly LanguageChoiceOption<L>[];
  readonly value?: L;
  readonly onChoose: (language: L) => void;
  readonly label: string;
}

const noIcon = <span hidden />;

/**
 * 🈯️ An explicit, accessible language choice: one radio per language, each named in its own language (`lang` set per option), inside one framed
 * group that shares its dividers. No default language: nothing is chosen until `value` names one. Used by language gates and navbar switchers.
 *
 * @see ../🎛️ToggleGroup/🟦️.tsx
 */
export function LanguageChoice<L extends string>({ options, value, onChoose, label, ...rest }: LanguageChoiceProps<L>): React.ReactElement {
  const items = options.map((option) => ({ value: option.value, icon: noIcon, text: option.endonym, lang: option.value, "data-locale": option.value }));
  return <ToggleGroup {...rest} kind="single" semantics="radio" aria-label={label} data-language-choice="" items={items} value={value ?? ""} onValueChange={(next) => next && onChoose(next as L)} />;
}
