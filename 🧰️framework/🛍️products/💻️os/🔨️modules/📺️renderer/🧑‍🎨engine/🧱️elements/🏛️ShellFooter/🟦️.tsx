import { Fragment } from "../⚛️react-port/🟦️.ts";
import type { ShellFooterItem, ShellLocale } from "@semio-tech/framework";
import { Button, Popover, PopoverContent, PopoverTrigger, type NavbarItem } from "@semio-tech/ui-react";

/** 📱️ Whether the credits fold into one footer button: a phone and a tablet have no room for the logos next to the shell's own footer items (the full row is wider than a tablet).
 *
 * @see ../🏛️ShellHost/🖼️canvas-footer/🟠️footer-items/🟦️.tsx */
export const shellFooterCreditsCompact = (mobile: boolean, tablet: boolean): boolean => mobile || tablet;

/** 🏛️ Renders an owner's localized credits with accessible links on every device. */
export function shellFooterNavbarItem(item: ShellFooterItem, locale: ShellLocale, compact: boolean): NavbarItem {
  return { key: item.id, content: <div className="relative z-40 flex items-center gap-double px-single whitespace-nowrap">
    {!compact && <span>{item.caption[locale]}</span>}
    {item.logos.map((logo, index) => <Fragment key={logo.href}>
      {!compact && index > 0 && item.separator && <span>{item.separator[locale]}</span>}
      <a href={logo.href} target="_blank" rel="noopener noreferrer" className="hover:text-foreground" aria-label={logo.alt}>
        <img src={logo.src} alt={logo.alt} className={logo.darkSrc ? "h-workbench w-auto dark:hidden" : "h-workbench w-auto"} />
        {logo.darkSrc && <img src={logo.darkSrc} alt={logo.alt} className="hidden h-workbench w-auto dark:block" />}
      </a>
    </Fragment>)}
  </div> };
}

/** 🗂️ One footer cell that holds every credit of a narrow footer behind a single accessible button, so the footer fits at any width and the credits stay one tap away.
 *
 * @see shellFooterNavbarItem */
export function shellFooterCreditsMenuItem(items: readonly ShellFooterItem[], locale: ShellLocale): NavbarItem {
  const label = items.map((item) => item.caption[locale]).join(" · ");
  return { key: "footerCredits", content: <Popover>
    <PopoverTrigger asChild><Button id="ui.footer.credits" type="button" variant="ghost" icon="info" aria-label={label} title={label} /></PopoverTrigger>
    <PopoverContent side="top" align="end" aria-label={label} className="max-w-[90vw] p-0">
      <div className="flex flex-col items-start gap-double py-single">{items.map((item) => <div key={item.id}>{shellFooterNavbarItem(item, locale, false).content}</div>)}</div>
    </PopoverContent>
  </Popover> };
}
