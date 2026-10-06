meta:
  id: pdf_14_mutations
  endian: le
seq:
  - id: format
    contents: [1]
  - id: tag
    type: u1
    valid: { min: 0, max: 4 }
  - id: payload
    type:
      switch-on: tag
      cases:
        0: insert_page
        1: remove_page
        2: move_page
        3: resize_page
        4: replace_page_text
types:
  text:
    seq:
      - id: length
        type: u8
      - id: value
        type: str
        encoding: UTF-8
        size: length
  page:
    seq:
      - id: width
        type: f8
      - id: height
        type: f8
      - id: text
        type: text
  insert_page:
    seq:
      - id: index
        type: u8
      - id: page
        type: page
  remove_page:
    seq:
      - id: index
        type: u8
  move_page:
    seq:
      - id: from
        type: u8
      - id: to
        type: u8
  resize_page:
    seq:
      - id: index
        type: u8
      - id: width
        type: f8
      - id: height
        type: f8
  replace_page_text:
    seq:
      - id: index
        type: u8
      - id: text
        type: text
