//! 🗣️ S Home launcher app — locale × terminology label set (constitutional: ui/Terminology).

use semio_framework_plugin::app_labels;

//#region 🔖️Terminology
app_labels! {
    /// 🗣️ Complete UI label set for the Home launcher; one field per label makes every locale×terminology combination compile-checked.
    pub struct SHomeLabels {
        vfs_empty_message: native_en "No studios yet. Create one from the navbar.", native_de "Noch keine Studios vorhanden. Erstelle eines über die Navigationsleiste.",
            reuse_en "No studios yet. Create one from the navbar.", reuse_de "Noch keine Studios vorhanden. Erstelle eines über die Navigationsleiste.";
        window_main: native_en "Studios", native_de "Studios", reuse_en "Studios", reuse_de "Studios";
    }
}
//#endregion 🔖️Terminology
