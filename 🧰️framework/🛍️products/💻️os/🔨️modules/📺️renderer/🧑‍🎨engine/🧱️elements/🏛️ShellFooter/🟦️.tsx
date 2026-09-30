import { Fragment } from "react";
import type { ShellFooterItem, ShellLocale } from "@semio-tech/framework";
import type { NavbarItem } from "@semio-tech/ui-react";

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
