/** 🧮️ Renders the interpreter's scalar parameter vector shapes for a fixture signature. */
export function coreParameterVector(parameters: readonly string[]): string {
  const i64Start = parameters.indexOf("i64");
  const scalarTail = i64Start >= 0 && parameters.slice(i64Start).every(type => type === "i64");
  const rustTypes = parameters.map(type => `ValueType::${type.toUpperCase()}`);
  if (!scalarTail) return `vec![${rustTypes.join(",")}]`;
  const tail = `vec![ValueType::I64;${parameters.length - i64Start}]`;
  return i64Start === 0 ? tail : `[vec![${rustTypes.slice(0, i64Start).join(",")}],${tail}].concat()`;
}
