# Summary

Reusable compose UI elements and Storybook source for shared interface primitives used by `framework/renderer/react` and other framework-facing packages.

### Specs

- The `ui` bundle owns the shared element source formerly embedded in `compose/js/sketchpad`.
- Storybook configuration, stories, and static output for shared elements live in this bundle.
- `framework/renderer/react` (e.g. `os-shell.tsx`, `ui-interpreter.tsx`, the component hosts) and other current consumers such as `cad/renderer/js` and `infinite/world/r3f` import `@semio-tech/ui-react` instead of defining shared element primitives locally.
