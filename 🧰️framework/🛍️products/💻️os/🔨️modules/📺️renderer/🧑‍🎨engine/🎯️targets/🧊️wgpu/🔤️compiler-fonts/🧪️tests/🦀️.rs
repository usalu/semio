use super::*;
use semio_framework_canvas::{resolve_icon_kind, FontRole, ResolvedIcon};

fn manifest_anchor() -> Vec<PathBuf> {
    vec![PathBuf::from(env!("CARGO_MANIFEST_DIR"))]
}

#[test]
fn repository_font_directory_is_found_above_the_crate_and_holds_every_role() {
    let directory = compiler_font_directory(None, &manifest_anchor()).expect("repository fonts above the crate manifest");
    for role in FontRole::ALL {
        assert!(directory.join(role.asset_file_name()).is_file(), "{role:?}");
    }
}

#[test]
fn an_explicit_directory_wins_and_a_missing_one_is_refused_not_replaced() {
    let found = compiler_font_directory(None, &manifest_anchor()).unwrap();
    assert_eq!(compiler_font_directory(Some(&found), &[]).unwrap(), found);
    let missing = std::env::temp_dir().join(format!("semio-no-compiler-fonts-{}", std::process::id()));
    let refusal = compiler_font_directory(Some(&missing), &manifest_anchor()).unwrap_err();
    assert!(refusal.contains(COMPILER_FONTS_ENV), "{refusal}");
    assert!(compiler_font_directory(None, &[missing]).is_err());
}

#[test]
fn math_emoji_and_text_icons_resolve_after_the_native_install() {
    let directory = compiler_font_directory(None, &manifest_anchor()).unwrap();
    install_compiler_fonts_from_directory(&directory).expect("install from the repository font directory");
    install_compiler_fonts_from_directory(&directory).expect("a second install in the same process stays installed");
    for (encoded, marker) in [("math:x^2", "<path"), ("emoji:🚀", "<image"), ("text:Hi", "<path")] {
        match resolve_icon_kind(encoded, |_| None) {
            ResolvedIcon::SvgPlain(svg) => assert!(svg.contains(marker), "{encoded}: {}", &svg[..svg.len().min(200)]),
            other => panic!("{encoded}: unexpected {other:?}"),
        }
    }
}
