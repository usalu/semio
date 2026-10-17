//! 🧪️ The native host side of the runtime asset port: repository root resolution, explicit-root refusal, and icons resolving after the native install.

use super::*;
use semio_framework_canvas::{resolve_icon_kind, ResolvedIcon};
use semio_framework_test_fixtures::manifest_dir;

fn manifest_anchor() -> Vec<PathBuf> {
    vec![manifest_dir!()]
}

#[test]
fn the_repository_root_is_found_above_the_crate_and_holds_every_asset_directory() {
    let RuntimeAssetRoot::Repository(root) = runtime_asset_root(None, None, &manifest_anchor()).expect("repository above the crate manifest") else { panic!("an anchor inside the repository resolves a repository root") };
    for asset in RuntimeAsset::ALL {
        assert!(root.join(asset.repository_directory()).join(asset.file_name()).is_file(), "{asset:?}");
    }
}

#[test]
fn an_explicit_root_wins_and_a_missing_one_is_refused_not_replaced() {
    let found = runtime_asset_root(None, None, &manifest_anchor()).unwrap();
    let RuntimeAssetRoot::Repository(path) = &found else { panic!("repository root") };
    assert_eq!(runtime_asset_root(Some(path.as_path()), None, &[]).unwrap(), found);
    let missing = std::env::temp_dir().join(format!("semio-no-runtime-assets-{}", std::process::id()));
    let refusal = runtime_asset_root(Some(missing.as_path()), None, &manifest_anchor()).unwrap_err();
    assert!(refusal.contains(RUNTIME_ASSETS_ENV), "{refusal}");
    assert!(runtime_asset_root(None, None, &[missing]).is_err());
}

#[test]
fn a_staged_folder_beside_the_executable_is_the_default_and_an_explicit_root_still_overrides_it() {
    let install = std::env::temp_dir().join(format!("semio-packaged-install-{}", std::process::id()));
    let RuntimeAssetRoot::Repository(repository) = runtime_asset_root(None, None, &manifest_anchor()).unwrap() else { panic!("repository root") };
    let staged = install.join(STAGED_DIRECTORY);
    for asset in RuntimeAsset::ALL {
        std::fs::create_dir_all(staged.join(asset.channel())).expect("channel directory");
        std::fs::copy(repository.join(asset.repository_directory()).join(asset.file_name()), staged.join(asset.channel()).join(asset.file_name())).expect("staged copy");
    }
    let executable = install.join("semio");
    assert_eq!(runtime_asset_root(None, Some(executable.as_path()), &manifest_anchor()).unwrap(), RuntimeAssetRoot::Staged(staged.clone()), "a staged folder beside the executable beats the repository above the anchors");
    assert_eq!(runtime_asset_root(Some(repository.as_path()), Some(executable.as_path()), &[]).unwrap(), RuntimeAssetRoot::Repository(repository.clone()), "the override wins over the staged folder");
    assert_eq!(runtime_asset_root(None, Some(install.join("elsewhere").join("semio").as_path()), &manifest_anchor()).unwrap(), RuntimeAssetRoot::Repository(repository), "no staged folder beside the executable falls back to the repository");
    install_runtime_assets_from_root(&RuntimeAssetRoot::Staged(staged)).expect("install from the staged folder");
    assert!(install_runtime_assets_from_root(&RuntimeAssetRoot::Staged(install.join("missing"))).is_err());
    std::fs::remove_dir_all(&install).expect("cleanup");
}

#[test]
fn icons_resolve_after_the_native_install() {
    let root = runtime_asset_root(None, None, &manifest_anchor()).unwrap();
    install_runtime_assets_from_root(&root).expect("install from the repository");
    install_runtime_assets_from_root(&root).expect("a second install in the same process stays installed");
    for (encoded, marker) in [("math:x^2", "<path"), ("emoji:🚀", "<image"), ("text:Hi", "<path"), (":plus:", "<svg"), (":capsule_J:", "<svg")] {
        match resolve_icon_kind(encoded, |_| None) {
            ResolvedIcon::SvgPlain(svg) | ResolvedIcon::SvgThemed(svg) => assert!(svg.contains(marker), "{encoded}: {}", &svg[..svg.len().min(200)]),
            other => panic!("{encoded}: unexpected {other:?}"),
        }
    }
}
