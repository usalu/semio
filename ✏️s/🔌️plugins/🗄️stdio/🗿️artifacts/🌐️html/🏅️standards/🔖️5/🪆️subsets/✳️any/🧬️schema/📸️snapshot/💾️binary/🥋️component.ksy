meta:
  id: stdio_html_snapshot
  endian: le
doc: |
  `pack` (binary) form of a `stdio.html` snapshot: the `semio_format` envelope (magic/header/
  footer, see the sibling `📡️component.protocol.semio`) wrapping a `payload` segment whose bytes
  are the canonical UTF-8 HTML5 document text -- the exact same grammar as
  `../📝️text/📖️component.grammar.semio`'s `document` production. Not an opaque blob: the payload's
  own internal structure is defined there, not here -- this leaf's concern is only the envelope
  framing this codec adds around it.
seq:
  - id: payload
    size-eos: true
    doc: UTF-8 HTML5 text, see the text-facet grammar for its structure.
