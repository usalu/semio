/** 📏️ The part of d3-format the print-size oracle uses: `format(specifier)(value)`. */
declare module "d3-format" {
  export function format(specifier: string): (value: number) => string;
}
