// Generated from nakagin.manifest.json

import type { GraphManifestDocument, KindCatalogBundle } from "./types.js";

export const NAKAGIN_NODE_BALCONY = "Balcony" as const;
export const NAKAGIN_NODE_BASE = "Base" as const;
export const NAKAGIN_NODE_BASE_BLOB = "Base Blob" as const;
export const NAKAGIN_NODE_BRIDGE = "Bridge" as const;
export const NAKAGIN_NODE_CAPITAL = "Capital" as const;
export const NAKAGIN_NODE_CAPSULE = "Capsule" as const;
export const NAKAGIN_NODE_CAPSULE_BACKSLASH = "Capsule Backslash" as const;
export const NAKAGIN_NODE_CAPSULE_J = "Capsule J" as const;
export const NAKAGIN_NODE_CAPSULE_L = "Capsule L" as const;
export const NAKAGIN_NODE_CAPSULE_P = "Capsule P" as const;
export const NAKAGIN_NODE_CAPSULE_Q = "Capsule q" as const;
export const NAKAGIN_NODE_CAPSULE_S = "Capsule S" as const;
export const NAKAGIN_NODE_CAPSULE_SLASH = "Capsule Slash" as const;
export const NAKAGIN_NODE_CAPSULE_WITH_BALCONY_BACKSLASH = "Capsule With Balcony Backslash" as const;
export const NAKAGIN_NODE_CAPSULE_WITH_BALCONY_J = "Capsule With Balcony J" as const;
export const NAKAGIN_NODE_CAPSULE_WITH_BALCONY_L = "Capsule With Balcony L" as const;
export const NAKAGIN_NODE_CAPSULE_WITH_BALCONY_P = "Capsule With Balcony P" as const;
export const NAKAGIN_NODE_CAPSULE_WITH_BALCONY_Q = "Capsule With Balcony Q" as const;
export const NAKAGIN_NODE_CAPSULE_WITH_BALCONY_S = "Capsule With Balcony S" as const;
export const NAKAGIN_NODE_CAPSULE_WITH_BALCONY_SLASH = "Capsule With Balcony Slash" as const;
export const NAKAGIN_NODE_CAPSULE_WITH_BALCONY_Z = "Capsule With Balcony Z" as const;
export const NAKAGIN_NODE_CAPSULE_Z = "Capsule Z" as const;
export const NAKAGIN_NODE_CYLINDRIC_CAPITAL = "Cylindric Capital" as const;
export const NAKAGIN_NODE_CYLINDRIC_FIRST_STOREY_TAMBOUR = "Cylindric First Storey Tambour" as const;
export const NAKAGIN_NODE_CYLINDRIC_LAST_STOREY_TAMBOUR = "Cylindric Last Storey Tambour" as const;
export const NAKAGIN_NODE_CYLINDRIC_SINGLE_STOREY_TAMBOUR = "Cylindric Single Storey Tambour" as const;
export const NAKAGIN_NODE_CYLINDRIC_TAMBOUR = "Cylindric Tambour" as const;
export const NAKAGIN_NODE_ELLIPSOID = "Ellipsoid" as const;
export const NAKAGIN_NODE_FIRST_STOREY_TAMBOUR = "First Storey Tambour" as const;
export const NAKAGIN_NODE_LAST_STOREY_TAMBOUR = "Last Storey Tambour" as const;
export const NAKAGIN_NODE_SINGLE_STOREY_TAMBOUR = "Single Storey Tambour" as const;
export const NAKAGIN_NODE_TAMBOUR = "Tambour" as const;
export const NAKAGIN_NODE_TRAPEZOID = "Trapezoid" as const;
export const NAKAGIN_NODE_TRAPEZOID_CAPSULE_BACKSLASH = "Trapezoid Capsule Backslash" as const;
export const NAKAGIN_NODE_TRAPEZOID_CAPSULE_J = "Trapezoid Capsule J" as const;
export const NAKAGIN_NODE_TRAPEZOID_CAPSULE_L = "Trapezoid Capsule L" as const;
export const NAKAGIN_NODE_TRAPEZOID_CAPSULE_P = "Trapezoid Capsule P" as const;
export const NAKAGIN_NODE_TRAPEZOID_CAPSULE_Q = "Trapezoid Capsule Q" as const;
export const NAKAGIN_NODE_TRAPEZOID_CAPSULE_S = "Trapezoid Capsule S" as const;
export const NAKAGIN_NODE_TRAPEZOID_CAPSULE_SLASH = "Trapezoid Capsule Slash" as const;
export const NAKAGIN_NODE_TRAPEZOID_CAPSULE_Z = "Trapezoid Capsule Z" as const;
export const NAKAGIN_NODE_PIECE = "Piece" as const;

export type NakaginNodeKindId = "Balcony" | "Base" | "Base Blob" | "Bridge" | "Capital" | "Capsule" | "Capsule Backslash" | "Capsule J" | "Capsule L" | "Capsule P" | "Capsule q" | "Capsule S" | "Capsule Slash" | "Capsule With Balcony Backslash" | "Capsule With Balcony J" | "Capsule With Balcony L" | "Capsule With Balcony P" | "Capsule With Balcony Q" | "Capsule With Balcony S" | "Capsule With Balcony Slash" | "Capsule With Balcony Z" | "Capsule Z" | "Cylindric Capital" | "Cylindric First Storey Tambour" | "Cylindric Last Storey Tambour" | "Cylindric Single Storey Tambour" | "Cylindric Tambour" | "Ellipsoid" | "First Storey Tambour" | "Last Storey Tambour" | "Single Storey Tambour" | "Tambour" | "Trapezoid" | "Trapezoid Capsule Backslash" | "Trapezoid Capsule J" | "Trapezoid Capsule L" | "Trapezoid Capsule P" | "Trapezoid Capsule Q" | "Trapezoid Capsule S" | "Trapezoid Capsule Slash" | "Trapezoid Capsule Z" | "Piece";
export const NAKAGIN_NODE_IDS = ["Balcony", "Base", "Base Blob", "Bridge", "Capital", "Capsule", "Capsule Backslash", "Capsule J", "Capsule L", "Capsule P", "Capsule q", "Capsule S", "Capsule Slash", "Capsule With Balcony Backslash", "Capsule With Balcony J", "Capsule With Balcony L", "Capsule With Balcony P", "Capsule With Balcony Q", "Capsule With Balcony S", "Capsule With Balcony Slash", "Capsule With Balcony Z", "Capsule Z", "Cylindric Capital", "Cylindric First Storey Tambour", "Cylindric Last Storey Tambour", "Cylindric Single Storey Tambour", "Cylindric Tambour", "Ellipsoid", "First Storey Tambour", "Last Storey Tambour", "Single Storey Tambour", "Tambour", "Trapezoid", "Trapezoid Capsule Backslash", "Trapezoid Capsule J", "Trapezoid Capsule L", "Trapezoid Capsule P", "Trapezoid Capsule Q", "Trapezoid Capsule S", "Trapezoid Capsule Slash", "Trapezoid Capsule Z", "Piece"] as const satisfies readonly NakaginNodeKindId[];
export const NAKAGIN_EDGE_CONNECTION = "Connection" as const;
export const NAKAGIN_EDGE_EDGE_LINK = "edge.link" as const;

export type NakaginEdgeKindId = "Connection" | "edge.link";
export const NAKAGIN_EDGE_IDS = ["Connection", "edge.link"] as const satisfies readonly NakaginEdgeKindId[];
export const NAKAGIN_PORT_CONNECTOR = "Connector" as const;
export const NAKAGIN_PORT_CORE_CIRCULAR_BOTTOM = "core circular bottom" as const;
export const NAKAGIN_PORT_CORE_CIRCULAR_TOP = "core circular top" as const;
export const NAKAGIN_PORT_CORE_RECTANGULAR_BOTTOM = "core rectangular bottom" as const;
export const NAKAGIN_PORT_CORE_RECTANGULAR_TOP = "core rectangular top" as const;
export const NAKAGIN_PORT_DOOR_CAPSULE_RIGHT = "door capsule right" as const;
export const NAKAGIN_PORT_DOOR_CAPSULE_LEFT = "door capsule left" as const;
export const NAKAGIN_PORT_DOOR_TAMBOUR_LEFT = "door tambour left" as const;
export const NAKAGIN_PORT_DOOR_TAMBOUR_RIGHT = "door tambour right" as const;
export const NAKAGIN_PORT_PLATFORM_RIGHT = "platform right" as const;
export const NAKAGIN_PORT_PLATFORM_LEFT = "platform left" as const;
export const NAKAGIN_PORT_ROOF_CIRCULAR_BOTTOM = "roof circular bottom" as const;
export const NAKAGIN_PORT_ROOF_CIRCULAR_TOP = "roof circular top" as const;
export const NAKAGIN_PORT_ROOF_RECTANGULAR_BOTTOM = "roof rectangular bottom" as const;
export const NAKAGIN_PORT_ROOF_RECTANGULAR_TOP = "roof rectangular top" as const;
export const NAKAGIN_PORT_TAMBOUR_CIRCULAR_BOTTOM = "tambour circular bottom" as const;
export const NAKAGIN_PORT_TAMBOUR_CIRCULAR_TOP = "tambour circular top" as const;
export const NAKAGIN_PORT_TAMBOUR_RECTANGULAR_BOTTOM = "tambour rectangular bottom" as const;
export const NAKAGIN_PORT_TAMBOUR_RECTANGULAR_TOP = "tambour rectangular top" as const;

export type NakaginPortKindId = "Connector" | "core circular bottom" | "core circular top" | "core rectangular bottom" | "core rectangular top" | "door capsule right" | "door capsule left" | "door tambour left" | "door tambour right" | "platform right" | "platform left" | "roof circular bottom" | "roof circular top" | "roof rectangular bottom" | "roof rectangular top" | "tambour circular bottom" | "tambour circular top" | "tambour rectangular bottom" | "tambour rectangular top";
export const NAKAGIN_PORT_IDS = ["Connector", "core circular bottom", "core circular top", "core rectangular bottom", "core rectangular top", "door capsule right", "door capsule left", "door tambour left", "door tambour right", "platform right", "platform left", "roof circular bottom", "roof circular top", "roof rectangular bottom", "roof rectangular top", "tambour circular bottom", "tambour circular top", "tambour rectangular bottom", "tambour rectangular top"] as const satisfies readonly NakaginPortKindId[];
export const NAKAGIN_WIRE_WIRE_LINK = "wire.link" as const;

export type NakaginWireKindId = "wire.link";
export const NAKAGIN_WIRE_IDS = ["wire.link"] as const satisfies readonly NakaginWireKindId[];

export const NAKAGIN_MANIFEST_DOCUMENT = {
  "schema": "manifest",
  "id": "nakagin",
  "name": "Nakagin Capsule Tower",
  "axes": {
    "portModel": "ported",
    "directedness": "directed"
  },
  "nodeKinds": [
    {
      "id": "Balcony",
      "name": "Balcony",
      "ports": [],
      "presentation": {}
    },
    {
      "id": "Base",
      "name": "Base",
      "ports": [
        "core rectangular bottom"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "core rectangular bottom",
            "angle": -2.3561944901923453,
            "radius": 3
          },
          {
            "handleKind": "core rectangular bottom",
            "angle": -0.7853981633974483,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Base Blob",
      "name": "Base Blob",
      "ports": [
        "core circular bottom"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "core circular bottom",
            "angle": -2.3561944901923453,
            "radius": 3
          },
          {
            "handleKind": "core circular bottom",
            "angle": -0.7853981633974483,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Bridge",
      "name": "Bridge",
      "ports": [
        "platform right",
        "platform left"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "platform right",
            "angle": 0,
            "radius": 3
          },
          {
            "handleKind": "platform left",
            "angle": 3.141592653589793,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Capital",
      "name": "Capital",
      "ports": [
        "roof rectangular top"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "roof rectangular top",
            "angle": -1.5707963267948966,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Capsule",
      "name": "Capsule",
      "ports": [],
      "presentation": {}
    },
    {
      "id": "Capsule Backslash",
      "name": "Capsule Backslash",
      "ports": [
        "door capsule right"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule right",
            "angle": -1.5707963267948966,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Capsule J",
      "name": "Capsule J",
      "ports": [
        "door capsule right"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule right",
            "angle": -1.5707963267948966,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Capsule L",
      "name": "Capsule L",
      "ports": [
        "door capsule right"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule right",
            "angle": -1.5707963267948966,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Capsule P",
      "name": "Capsule P",
      "ports": [
        "door capsule right"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule right",
            "angle": -1.5707963267948966,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Capsule q",
      "name": "Capsule q",
      "ports": [
        "door capsule right"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule right",
            "angle": -0.450225596260715,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Capsule S",
      "name": "Capsule S",
      "ports": [
        "door capsule right"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule right",
            "angle": -1.5707963267948966,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Capsule Slash",
      "name": "Capsule Slash",
      "ports": [
        "door capsule right"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule right",
            "angle": -1.5707963267948966,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Capsule With Balcony Backslash",
      "name": "Capsule With Balcony Backslash",
      "ports": [
        "door capsule right"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule right",
            "angle": -0.21109333322274654,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Capsule With Balcony J",
      "name": "Capsule With Balcony J",
      "ports": [
        "door capsule right"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule right",
            "angle": 0.805003494254653,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Capsule With Balcony L",
      "name": "Capsule With Balcony L",
      "ports": [
        "door capsule left"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule left",
            "angle": -0.805003494254653,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Capsule With Balcony P",
      "name": "Capsule With Balcony P",
      "ports": [
        "door capsule right"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule right",
            "angle": 0.21109333322274654,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Capsule With Balcony Q",
      "name": "Capsule With Balcony Q",
      "ports": [
        "door capsule right"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule right",
            "angle": -0.21109333322274654,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Capsule With Balcony S",
      "name": "Capsule With Balcony S",
      "ports": [
        "door capsule right"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule right",
            "angle": 0.805003494254653,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Capsule With Balcony Slash",
      "name": "Capsule With Balcony Slash",
      "ports": [
        "door capsule right"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule right",
            "angle": 0.21109333322274654,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Capsule With Balcony Z",
      "name": "Capsule With Balcony Z",
      "ports": [
        "door capsule left"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule left",
            "angle": -0.805003494254653,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Capsule Z",
      "name": "Capsule Z",
      "ports": [
        "door capsule left"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule left",
            "angle": -0.805003494254653,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Cylindric Capital",
      "name": "Cylindric Capital",
      "ports": [
        "roof circular top"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "roof circular top",
            "angle": -1.5707963267948966,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Cylindric First Storey Tambour",
      "name": "Cylindric First Storey Tambour",
      "ports": [
        "core circular top",
        "tambour circular top",
        "door tambour right",
        "door tambour left"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "core circular top",
            "angle": -3.141592653589793,
            "radius": 3
          },
          {
            "handleKind": "tambour circular top",
            "angle": -3.141592653589793,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": -1.8121518132334324,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": -2.9002371671512575,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": 2.9002371671512575,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": 1.8121518132334324,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": 1.329440840356361,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": 0.24135548643853572,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": -0.24135548643853572,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": -1.329440840356361,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Cylindric Last Storey Tambour",
      "name": "Cylindric Last Storey Tambour",
      "ports": [
        "tambour circular bottom",
        "roof circular bottom",
        "door tambour right",
        "door tambour left"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "tambour circular bottom",
            "angle": -3.141592653589793,
            "radius": 3
          },
          {
            "handleKind": "roof circular bottom",
            "angle": -3.141592653589793,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": -1.8121518132334324,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": -2.9002371671512575,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": 2.9002371671512575,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": 1.8121518132334324,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": 1.329440840356361,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": 0.24135548643853572,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": -0.24135548643853572,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": -1.329440840356361,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Cylindric Single Storey Tambour",
      "name": "Cylindric Single Storey Tambour",
      "ports": [
        "core circular top",
        "roof circular bottom",
        "door tambour right",
        "door tambour left"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "core circular top",
            "angle": -3.141592653589793,
            "radius": 3
          },
          {
            "handleKind": "roof circular bottom",
            "angle": -3.141592653589793,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": -1.8121518132334324,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": -2.9002371671512575,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": 2.9002371671512575,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": 1.8121518132334324,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": 1.329440840356361,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": 0.24135548643853572,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": -0.24135548643853572,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": -1.329440840356361,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Cylindric Tambour",
      "name": "Cylindric Tambour",
      "ports": [
        "tambour circular bottom",
        "tambour circular top",
        "door tambour right",
        "door tambour left"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "tambour circular bottom",
            "angle": 1.5707963267948966,
            "radius": 3
          },
          {
            "handleKind": "tambour circular top",
            "angle": -1.5707963267948966,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": -0.31415926535897953,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": -1.256637061435917,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": -1.884955592153876,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": -2.8274333882308142,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": 2.827433388230814,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": 1.8849555921538763,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": 1.2566370614359168,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": 0.31415926535897953,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Ellipsoid",
      "name": "Ellipsoid",
      "ports": [],
      "presentation": {}
    },
    {
      "id": "First Storey Tambour",
      "name": "First Storey Tambour",
      "ports": [
        "core rectangular top",
        "tambour rectangular top",
        "door tambour right",
        "door tambour left"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "core rectangular top",
            "angle": 1.5707963267948966,
            "radius": 3
          },
          {
            "handleKind": "tambour rectangular top",
            "angle": -1.5707963267948966,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": -0.31415926535897953,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": -1.256637061435917,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": -1.884955592153876,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": -2.8274333882308142,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": 2.827433388230814,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": 1.8849555921538763,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": 1.2566370614359168,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": 0.31415926535897953,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Last Storey Tambour",
      "name": "Last Storey Tambour",
      "ports": [
        "tambour rectangular bottom",
        "roof rectangular bottom",
        "door tambour right",
        "door tambour left"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "tambour rectangular bottom",
            "angle": 1.5707963267948966,
            "radius": 3
          },
          {
            "handleKind": "roof rectangular bottom",
            "angle": -1.5707963267948966,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": -0.31415926535897953,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": -1.256637061435917,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": -1.884955592153876,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": -2.8274333882308142,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": 2.827433388230814,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": 1.8849555921538763,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": 1.2566370614359168,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": 0.31415926535897953,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Single Storey Tambour",
      "name": "Single Storey Tambour",
      "ports": [
        "core circular top",
        "roof rectangular bottom",
        "door tambour right",
        "door tambour left"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "core circular top",
            "angle": -3.141592653589793,
            "radius": 3
          },
          {
            "handleKind": "roof rectangular bottom",
            "angle": -3.141592653589793,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": -1.887082454707051,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": -2.8253065256776386,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": 2.8253065256776386,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": 1.887082454707051,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": 1.254510198882742,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": 0.3162861279121545,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": -0.3162861279121545,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": -1.254510198882742,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Tambour",
      "name": "Tambour",
      "ports": [
        "tambour rectangular bottom",
        "tambour rectangular top",
        "door tambour right",
        "door tambour left"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "tambour rectangular bottom",
            "angle": 1.5707963267948966,
            "radius": 3
          },
          {
            "handleKind": "tambour rectangular top",
            "angle": -1.5707963267948966,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": -0.31415926535897953,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": -1.256637061435917,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": -1.884955592153876,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": -2.8274333882308142,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": 2.827433388230814,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": 1.8849555921538763,
            "radius": 3
          },
          {
            "handleKind": "door tambour right",
            "angle": 1.2566370614359168,
            "radius": 3
          },
          {
            "handleKind": "door tambour left",
            "angle": 0.31415926535897953,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Trapezoid",
      "name": "Trapezoid",
      "ports": [],
      "presentation": {}
    },
    {
      "id": "Trapezoid Capsule Backslash",
      "name": "Trapezoid Capsule Backslash",
      "ports": [
        "door capsule right"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule right",
            "angle": -1.5707963267948966,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Trapezoid Capsule J",
      "name": "Trapezoid Capsule J",
      "ports": [
        "door capsule right"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule right",
            "angle": -1.5707963267948966,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Trapezoid Capsule L",
      "name": "Trapezoid Capsule L",
      "ports": [
        "door capsule left"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule left",
            "angle": 1.5707963267948966,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Trapezoid Capsule P",
      "name": "Trapezoid Capsule P",
      "ports": [
        "door capsule right"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule right",
            "angle": 1.5707963267948966,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Trapezoid Capsule Q",
      "name": "Trapezoid Capsule Q",
      "ports": [
        "door capsule right"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule right",
            "angle": -1.5707963267948966,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Trapezoid Capsule S",
      "name": "Trapezoid Capsule S",
      "ports": [
        "door capsule right"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule right",
            "angle": -1.5707963267948966,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Trapezoid Capsule Slash",
      "name": "Trapezoid Capsule Slash",
      "ports": [
        "door capsule right"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule right",
            "angle": 1.5707963267948966,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Trapezoid Capsule Z",
      "name": "Trapezoid Capsule Z",
      "ports": [
        "door capsule left"
      ],
      "presentation": {
        "handles": [
          {
            "handleKind": "door capsule left",
            "angle": 1.5707963267948966,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "Piece",
      "name": "Piece",
      "ports": [
        "Connector"
      ],
      "properties": [
        {
          "name": "position",
          "kind": "data",
          "valueType": "object"
        },
        {
          "name": "label",
          "kind": "data",
          "valueType": "string"
        },
        {
          "name": "tier",
          "kind": "data",
          "valueType": "number"
        },
        {
          "name": "flatPosition",
          "kind": "derived",
          "valueType": "object",
          "expr": "flatFromConnections"
        }
      ]
    }
  ],
  "edgeKinds": [
    {
      "id": "Connection",
      "name": "Connection",
      "properties": [
        {
          "name": "gap",
          "kind": "data",
          "valueType": "number"
        },
        {
          "name": "rotation",
          "kind": "data",
          "valueType": "number"
        },
        {
          "name": "tilt",
          "kind": "data",
          "valueType": "number"
        },
        {
          "name": "rise",
          "kind": "data",
          "valueType": "number"
        },
        {
          "name": "turn",
          "kind": "data",
          "valueType": "number"
        },
        {
          "name": "shift",
          "kind": "data",
          "valueType": "number"
        },
        {
          "name": "u",
          "kind": "data",
          "valueType": "number"
        },
        {
          "name": "v",
          "kind": "data",
          "valueType": "number"
        }
      ]
    },
    {
      "id": "edge.link",
      "name": "Link",
      "presentation": {
        "id": "edge.link",
        "name": "Link"
      }
    }
  ],
  "portKinds": [
    {
      "id": "Connector",
      "name": "Connector",
      "direction": "out",
      "properties": []
    },
    {
      "id": "core circular bottom",
      "name": "core circular bottom",
      "presentation": {
        "color": "hsl(206 52% 48%)",
        "defaultWireKind": "wire.link"
      }
    },
    {
      "id": "core circular top",
      "name": "core circular top",
      "presentation": {
        "color": "hsl(290 52% 48%)",
        "defaultWireKind": "wire.link"
      }
    },
    {
      "id": "core rectangular bottom",
      "name": "core rectangular bottom",
      "presentation": {
        "color": "hsl(55 52% 48%)",
        "defaultWireKind": "wire.link"
      }
    },
    {
      "id": "core rectangular top",
      "name": "core rectangular top",
      "presentation": {
        "color": "hsl(37 52% 48%)",
        "defaultWireKind": "wire.link"
      }
    },
    {
      "id": "door capsule right",
      "name": "door capsule right",
      "presentation": {
        "color": "hsl(124 52% 48%)",
        "defaultWireKind": "wire.link"
      }
    },
    {
      "id": "door capsule left",
      "name": "door capsule left",
      "presentation": {
        "color": "hsl(239 52% 48%)",
        "defaultWireKind": "wire.link"
      }
    },
    {
      "id": "door tambour left",
      "name": "door tambour left",
      "presentation": {
        "color": "hsl(344 52% 48%)",
        "defaultWireKind": "wire.link"
      }
    },
    {
      "id": "door tambour right",
      "name": "door tambour right",
      "presentation": {
        "color": "hsl(91 52% 48%)",
        "defaultWireKind": "wire.link"
      }
    },
    {
      "id": "platform right",
      "name": "platform right",
      "presentation": {
        "color": "hsl(169 52% 48%)",
        "defaultWireKind": "wire.link"
      }
    },
    {
      "id": "platform left",
      "name": "platform left",
      "presentation": {
        "color": "hsl(215 52% 48%)",
        "defaultWireKind": "wire.link"
      }
    },
    {
      "id": "roof circular bottom",
      "name": "roof circular bottom",
      "presentation": {
        "color": "hsl(277 52% 48%)",
        "defaultWireKind": "wire.link"
      }
    },
    {
      "id": "roof circular top",
      "name": "roof circular top",
      "presentation": {
        "color": "hsl(215 52% 48%)",
        "defaultWireKind": "wire.link"
      }
    },
    {
      "id": "roof rectangular bottom",
      "name": "roof rectangular bottom",
      "presentation": {
        "color": "hsl(108 52% 48%)",
        "defaultWireKind": "wire.link"
      }
    },
    {
      "id": "roof rectangular top",
      "name": "roof rectangular top",
      "presentation": {
        "color": "hsl(100 52% 48%)",
        "defaultWireKind": "wire.link"
      }
    },
    {
      "id": "tambour circular bottom",
      "name": "tambour circular bottom",
      "presentation": {
        "color": "hsl(231 52% 48%)",
        "defaultWireKind": "wire.link"
      }
    },
    {
      "id": "tambour circular top",
      "name": "tambour circular top",
      "presentation": {
        "color": "hsl(156 52% 48%)",
        "defaultWireKind": "wire.link"
      }
    },
    {
      "id": "tambour rectangular bottom",
      "name": "tambour rectangular bottom",
      "presentation": {
        "color": "hsl(223 52% 48%)",
        "defaultWireKind": "wire.link"
      }
    },
    {
      "id": "tambour rectangular top",
      "name": "tambour rectangular top",
      "presentation": {
        "color": "hsl(108 52% 48%)",
        "defaultWireKind": "wire.link"
      }
    }
  ],
  "wireKinds": [
    {
      "id": "wire.link",
      "name": "Link",
      "presentation": {
        "defaultEdgeKind": "edge.link"
      }
    }
  ],
  "edgeTips": []
} as const satisfies GraphManifestDocument;

export function nakaginManifestCatalogBundle(): KindCatalogBundle {
  const doc = NAKAGIN_MANIFEST_DOCUMENT;
  return {
    handles: doc.portKinds?.map((row) => ({
      id: row.id,
      name: row.name ?? row.id,
      color: String((row.presentation as { color?: string })?.color ?? "hsl(215 52% 48%)"),
      defaultWireKind: (row.presentation as { defaultWireKind?: string })?.defaultWireKind,
    })),
    wires: doc.wireKinds?.map((row) => ({
      id: row.id,
      name: row.name ?? row.id,
      defaultEdgeKind: (row.presentation as { defaultEdgeKind?: string })?.defaultEdgeKind,
    })),
    nodes: doc.nodeKinds?.map((row) => ({
      id: row.id,
      name: row.name ?? row.id,
      color: (row.presentation as { color?: string })?.color,
      stroke: (row.presentation as { stroke?: string })?.stroke,
      icon: (row.presentation as { icon?: string })?.icon,
      handles: (row.presentation as { handles?: readonly { handleKind: string; angle: number; radius?: number }[] })?.handles,
    })),
    edges: doc.edgeKinds?.map((row) => ({
      id: row.id,
      name: row.name ?? row.id,
      color: (row.presentation as { color?: string })?.color,
      stroke: (row.presentation as { stroke?: string | number })?.stroke as string | undefined,
      pattern: (row.presentation as { pattern?: string })?.pattern,
      shape: (row.presentation as { shape?: "bezier" | "line" })?.shape,
      sourceTip: (row.presentation as { sourceTip?: string })?.sourceTip,
      targetTip: (row.presentation as { targetTip?: string })?.targetTip,
      directed: (row.presentation as { directed?: boolean })?.directed,
    })),
    edgeTips: doc.edgeTips as KindCatalogBundle["edgeTips"],
  };
}
