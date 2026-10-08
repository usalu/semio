//! 🧭️ The details-pane vocabulary of the jpg editors (document and baseline share one mutation set): which snapshot pointer raises which concrete kind.
//! A JFIF header field raises the header setter carrying the other header fields it keeps; a pixel raster edit raises the raster setter; a segment insert or remove raises its positional kind; any other edit inside the image replaces the image.

use semio_s_artifact_stdio_contract::editing::{Carried, EditRules, EntityRule, InsertRule, RemoveRule};

const KEEP_DENSITY_UNITS: Carried = Carried { payload: "densityUnits", pointer: "/image/jfifDensityUnits" };
const KEEP_X_DENSITY: Carried = Carried { payload: "xDensity", pointer: "/image/jfifXDensity" };
const KEEP_Y_DENSITY: Carried = Carried { payload: "yDensity", pointer: "/image/jfifYDensity" };
const KEEP_THUMBNAIL: Carried = Carried { payload: "thumbnail", pointer: "/image/jfifThumbnail" };
const KEEP_VERSION: Carried = Carried { payload: "version", pointer: "/image/jfifVersion" };

/// 📚 Longest template wins: the JFIF fields and the raster leave the generic image replacement.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/image", "replace-image", "image"),
        EntityRule::new("/image/pixels", "replace-pixels", "pixels"),
        EntityRule::new("/image/jfifVersion", "change-jfif-header", "version").carrying(&[KEEP_DENSITY_UNITS, KEEP_X_DENSITY, KEEP_Y_DENSITY, KEEP_THUMBNAIL]),
        EntityRule::new("/image/jfifDensityUnits", "change-jfif-header", "densityUnits").carrying(&[KEEP_VERSION, KEEP_X_DENSITY, KEEP_Y_DENSITY, KEEP_THUMBNAIL]),
        EntityRule::new("/image/jfifXDensity", "change-jfif-header", "xDensity").carrying(&[KEEP_VERSION, KEEP_DENSITY_UNITS, KEEP_Y_DENSITY, KEEP_THUMBNAIL]),
        EntityRule::new("/image/jfifYDensity", "change-jfif-header", "yDensity").carrying(&[KEEP_VERSION, KEEP_DENSITY_UNITS, KEEP_X_DENSITY, KEEP_THUMBNAIL]),
        EntityRule::new("/image/jfifThumbnail", "change-jfif-header", "thumbnail").carrying(&[KEEP_VERSION, KEEP_DENSITY_UNITS, KEEP_X_DENSITY, KEEP_Y_DENSITY]),
    ],
    inserts: &[InsertRule::new("/image/otherSegments", "insert-other-segment", "segment").at("index")],
    removes: &[RemoveRule::by_index("/image/otherSegments", "remove-other-segment", "index")],
};
