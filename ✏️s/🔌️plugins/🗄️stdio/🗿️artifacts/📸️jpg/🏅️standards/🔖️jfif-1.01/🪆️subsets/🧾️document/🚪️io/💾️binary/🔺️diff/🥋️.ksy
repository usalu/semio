meta:
  id: stdio_jpg_diff
  endian: le
doc: Typed JPEG semantic content diff; native SOF and table observations are I/O facts.
seq:
  - id: format
    contents: [1]
  - id: flags
    type: u2
    valid:
      max: 511
  - id: width
    type: varint
    if: flags & 1 != 0
  - id: height
    type: varint
    if: flags & 2 != 0
  - id: pixels
    type: blob
    if: flags & 4 != 0
  - id: jfif_version
    type: version
    if: flags & 8 != 0
  - id: jfif_density_units
    type: u1
    if: flags & 16 != 0
    valid:
      max: 2
  - id: jfif_x_density
    type: varint
    if: flags & 32 != 0
  - id: jfif_y_density
    type: varint
    if: flags & 64 != 0
  - id: jfif_thumbnail
    type: thumbnail_change
    if: flags & 128 != 0
  - id: other_segments
    type: segment_collection_diff
    if: flags & 256 != 0
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
  segment_diff:
    seq:
      - id: marker_present
        type: u1
        valid:
          max: 1
      - id: marker
        type: u1
        if: marker_present == 1
      - id: data_present
        type: u1
        valid:
          max: 1
      - id: data
        type: blob
        if: data_present == 1
  modified_segment:
    seq:
      - id: index
        type: varint
      - id: diff
        type: segment_diff
  added_segment:
    seq:
      - id: index
        type: varint
      - id: item
        type: segment
  segment_collection_diff:
    seq:
      - id: removed_count
        type: varint
      - id: removed
        type: varint
        repeat: expr
        repeat-expr: removed_count.value
      - id: modified_count
        type: varint
      - id: modified
        type: modified_segment
        repeat: expr
        repeat-expr: modified_count.value
      - id: added_count
        type: varint
      - id: added
        type: added_segment
        repeat: expr
        repeat-expr: added_count.value
