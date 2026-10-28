# STEP Alphabet Contract

The nine pages are the ISO 8859 parts 1–9 selected by `\PA\` through `\PI\`. A `\S\` payload is one graphic ASCII character in U+0020–U+007E. Adding 128 addresses the 95 positions U+00A0–U+00FE in the selected byte alphabet. Null entries are undefined byte positions and must cause a typed refusal. Alphabet selection resets for each string.

`data.json` pins each official Unicode mapping URL and SHA-256. `data.rs` contains the same owned scalar rows for native use. `cases.json` is the complete 855-case language-neutral oracle; it includes apostrophe and reverse-solidus payloads without inventing another escaping layer. The retained ticket input `step-alphabet-data.py` records the reproducible source acquisition. Mapping data is distributed under the associated [Unicode License](LICENSE.txt).

The mapping for page G is ISO 8859-7:2003, including its current Euro, drachma and Greek ypogegrammeni positions. No earlier Greek mapping is retained. Sources: [Part 21 Edition 3, clauses 5.2, 5.6 and 6.4.3](https://steptools.com/stds/step/IS_final_p21e3.html), and the exact Unicode URLs recorded beside each page. This alphabet contract does not assert complete physical-file or MVD conformance.
