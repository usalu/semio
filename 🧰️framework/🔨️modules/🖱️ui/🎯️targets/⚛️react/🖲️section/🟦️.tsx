import type { UiLabel } from "../../../🧱️elements/🎗️UiLabel/🟦️.tsx";
import * as React from "react";

// Collapsible section container with heading and specificity.
// Consumers MUST provide a heading string.

/**
 * Props interface for the Section component.
 **/
export interface SectionProps {
  id?: string;
  title?: UiLabel;
  children: React.ReactNode;
  className?: string;
}

/**
 **/
export const Section: React.FC<SectionProps> = ({ id, title, children, className = "" }) => {
  return (
    <section id={id} className={`mb-8 ${className}`} aria-labelledby={id && title ? `${id}.title` : undefined}>
      {title && (
        <h2 className="text-2xl font-semibold mb-4" id={id ? `${id}.title` : undefined}>
          {title}
        </h2>
      )}
      <div>{children}</div>
    </section>
  );
};
