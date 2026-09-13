import tailwindcss from "@tailwindcss/postcss";

export type OwnedPostcssConfiguration = Readonly<{
  plugins: Readonly<Record<string, Readonly<Record<string, never>>>>;
}>;

/** 🎨️ Declares the React styling transform without coupling callers to PostCSS types. */
export const uiPostcssConfiguration: OwnedPostcssConfiguration = {
  plugins: {
    "@tailwindcss/postcss": {},
  },
};

/** 🧵️ Creates the explicit plugin array required by Vite's inline PostCSS interface. */
export function uiPostcssInlinePlugins(): readonly unknown[] {
  return [tailwindcss()];
}

export default uiPostcssConfiguration;
