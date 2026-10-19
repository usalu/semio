import { reactHostPort } from "../../../🧱️elements/🔌️Ports/⚛️react/🟦️.ts";
import { type ElementsSurfaceDevice, elementsSurfaceDeviceIsMobile, UI_MOBILE_MEDIA_QUERY, UI_TABLET_MEDIA_QUERY, elementsSurfaceDeviceForMatches } from "../../../📱️device/🟦️.ts";
import * as React from "react";
import { UiMobileProvider, UiMobileContext } from "../../../🔨️modules/📱️ui-mobile-context/🟦️.tsx";
import { useMediaQuery } from "../🌓️appearance/🟦️.ts";
import { type InteractionCommands, InteractionContext, ActiveInteractionContext } from "../../../🧱️elements/👻️PanelGhost/🟦️.tsx";

// #region 📱️UiMobile Context

const UiDeviceContext = reactHostPort.createContext<ElementsSurfaceDevice | undefined>(undefined);

/** 📱️ Broadcasts the shell's authoritative device to descendants — the three-way twin of
 * {@link UiMobileProvider}, used when a shell pins a device from settings instead of measuring one. */
export const UiDeviceProvider: React.FC<{
  readonly device: ElementsSurfaceDevice;
  readonly children: React.ReactNode;
}> = ({ device, children }) => (
  <UiDeviceContext.Provider value={device}>
    <UiMobileProvider mobile={elementsSurfaceDeviceIsMobile(device)}>{children}</UiMobileProvider>
  </UiDeviceContext.Provider>
);

/**
 * 📱️ The device this subtree paints for: the nearest {@link UiDeviceProvider}, else the viewport
 * measured through the ONE policy `📱️device/🟦️.ts` shares with the wgpu dock.
 *
 * `AGENTS.md` orders the devices desktop → mobile → tablet and the settings surface has always offered
 * all three; before this hook the automatic detection was binary ({@link useUiMobile}), so `tablet` was
 * a value a user could only reach by hand. An ancestor {@link UiMobileProvider} that says "mobile" still
 * wins, so a shell that already pinned the binary flag cannot be widened out from under itself.
 */
export function useUiDevice(): ElementsSurfaceDevice {
  const mobileMedia = useMediaQuery(UI_MOBILE_MEDIA_QUERY);
  const tabletMedia = useMediaQuery(UI_TABLET_MEDIA_QUERY);
  const deviceCtx = reactHostPort.useContext(UiDeviceContext);
  const mobileCtx = reactHostPort.useContext(UiMobileContext);
  if (deviceCtx !== undefined) return deviceCtx;
  if (mobileCtx === true) return "mobile";
  if (mobileCtx === false) return tabletMedia ? "tablet" : "desktop";
  return elementsSurfaceDeviceForMatches({ mobile: mobileMedia, tablet: tabletMedia });
}

// #endregion 📱️UiMobile Context

// #region 🔊️Section Specificity
// Enum defining priority levels for section content ownership.
// Consumers MUST use these constants for section precedence.

/**
 * ✂️ Priority enum for section content ownership across apps.
 **/
export enum SectionSpecificity {
  SKETCHPAD = 0,
  KIT = 10,
  QUALITY = 20,
  TYPE = 20,
  DESIGN = 20,
  DOCS = 20,
  SELECTION = 30,
}

// #endregion 🔊️Section Specificity

/** 🔤️ External provider for interaction commands; prefer {@link GhostProvider} at layout root. */
export const InteractionProvider: React.FC<{
  commands?: InteractionCommands;
  activeInteraction?: string;
  children: React.ReactNode;
}> = ({ commands, activeInteraction, children }) => (
  <InteractionContext.Provider value={commands}>
    <ActiveInteractionContext.Provider value={activeInteraction}>{children}</ActiveInteractionContext.Provider>
  </InteractionContext.Provider>
);

const useActiveInteraction = () => reactHostPort.useContext(ActiveInteractionContext);
