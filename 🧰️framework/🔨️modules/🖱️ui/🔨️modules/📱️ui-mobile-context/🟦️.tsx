/** 🧭️ Owns the mobile layout context that tells chrome whether to adapt to a handheld surface. */
import * as React from "react";
import { UI_MOBILE_MEDIA_QUERY } from "../../📱️device/🟦️.ts";
import { reactHostPort } from "../../🧱️elements/🔌️Ports/⚛️react/🟦️.ts";
import { useMediaQuery } from "../../🌓️appearance/🟦️.ts";

export const UiMobileContext = reactHostPort.createContext<boolean | undefined>(undefined);

/** 📱️ Broadcasts the shell's authoritative mobile flag to descendants (e.g. {@link Pane}, {@link Window}) that have no `mobile` prop of their own. */
export const UiMobileProvider: React.FC<{
  readonly mobile: boolean;
  readonly children: React.ReactNode;
}> = ({ mobile, children }) => <UiMobileContext.Provider value={mobile}>{children}</UiMobileContext.Provider>;

/** 🪝️ Returns the nearest {@link UiMobileProvider} flag, falling back to {@link UI_MOBILE_MEDIA_QUERY} for standalone usage (Storybook, tests). */
export function useUiMobile(): boolean {
  const media = useMediaQuery(UI_MOBILE_MEDIA_QUERY);
  const ctx = reactHostPort.useContext(UiMobileContext);
  return ctx ?? media;
}
