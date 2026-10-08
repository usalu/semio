meta:
  id: stdio_jpg_mutation
  endian: le
seq:
  - id: format
    contents: [1]
  - id: tag
    type: u1
    valid:
      min: 1
      max: 5
  - id: payload
    type:
      switch-on: tag
      cases:
        1: change_jfif_header
        2: insert_other_segment
        3: remove_other_segment
        4: replace_pixels
        5: replace_image
types:
  varint:
    seq:
      - id: groups
        type: u1
        repeat: until
        repeat-until: _ & 128 == 0
    instances:
      value:
        value: '(groups.size > 0 ? (groups[0] & 127).as<u8> << 0 : 0) | (groups.size > 1 ? (groups[1] & 127).as<u8> << 7 : 0) | (groups.size > 2 ? (groups[2] & 127).as<u8> << 14 : 0) | (groups.size > 3 ? (groups[3] & 127).as<u8> << 21 : 0) | (groups.size > 4 ? (groups[4] & 127).as<u8> << 28 : 0) | (groups.size > 5 ? (groups[5] & 127).as<u8> << 35 : 0) | (groups.size > 6 ? (groups[6] & 127).as<u8> << 42 : 0) | (groups.size > 7 ? (groups[7] & 127).as<u8> << 49 : 0) | (groups.size > 8 ? (groups[8] & 127).as<u8> << 56 : 0) | (groups.size > 9 ? (groups[9] & 127).as<u8> << 63 : 0)'
  blob:
    seq:
      - id: length
        type: varint
      - id: data
        size: length.value
  version:
    seq:
      - id: major
        type: u1
      - id: minor
        type: u1
  thumbnail:
    seq:
      - id: width
        type: u1
      - id: height
        type: u1
      - id: rgb_data
        type: blob
  thumbnail_change:
    seq:
      - id: presence
        type: u1
        valid:
          max: 1
      - id: thumbnail
        type: thumbnail
        if: presence == 1
  segment:
    seq:
      - id: marker
        type: u1
      - id: data
        type: blob
  change_jfif_header:
    seq:
      - id: version
        type: version
      - id: density_units
        type: u1
        valid:
          max: 2
      - id: x_density
        type: varint
      - id: y_density
        type: varint
      - id: thumbnail
        type: thumbnail_change
  insert_other_segment:
    seq:
      - id: index
        type: varint
      - id: segment
        type: segment
  remove_other_segment:
    seq:
      - id: index
        type: varint
  replace_pixels:
    seq:
      - id: pixels
        type: blob
  replace_image:
    seq:
      - id: width
        type: varint
      - id: height
        type: varint
      - id: pixels
        type: blob
      - id: jfif_version
        type: version
      - id: jfif_density_units
        type: u1
        valid:
          max: 2
      - id: jfif_x_density
        type: varint
      - id: jfif_y_density
        type: varint
      - id: jfif_thumbnail
        type: thumbnail_change
      - id: segment_count
        type: varint
      - id: other_segments
        type: segment
        repeat: expr
        repeat-expr: segment_count.value
