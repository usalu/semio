//! 🧪️ Laws of the runtime asset port: the catalog against its shared fixture and the files on disk, loader order, refusal of malformed packs and fonts, and one install serving every consumer.

use super::testkit::{install_repository_assets, repository_root};
use super::*;
use serde_json::Value;

const RUNTIME_ASSET_FIXTURE: &str = include_str!("🧫️fixtures/🔣️.json");

fn fixture() -> Value {
    serde_json::from_str(RUNTIME_ASSET_FIXTURE).expect("runtime asset fixture")
}

#[test]
fn the_asset_catalog_matches_the_shared_fixture_and_the_files_on_disk() {
    let document = fixture();
    let channels = document["channels"].as_array().expect("channels");
    let rows = document["assets"].as_array().expect("assets");
    assert_eq!(rows.len(), RuntimeAsset::ALL.len());
    for (asset, row) in RuntimeAsset::ALL.iter().zip(rows) {
        assert_eq!(asset.file_name(), row["file"].as_str().expect("file"));
        assert_eq!(asset.channel(), row["channel"].as_str().expect("channel"));
        let channel = channels.iter().find(|channel| channel["id"] == row["channel"]).expect("asset channel");
        assert_eq!(asset.repository_directory(), channel["directory"].as_str().expect("directory"));
        let bytes = fs::read(repository_root().join(asset.repository_directory()).join(asset.file_name())).expect("asset file");
        if let Some(expected) = row["bytes"].as_u64() {
            assert_eq!(bytes.len() as u64, expected, "{}", asset.file_name());
        }
        if let Some(tag) = row["sfntTag"].as_str() {
            let head: String = bytes.iter().take(4).map(|byte| format!("{byte:02x}")).collect();
            assert_eq!(head, tag, "{}", asset.file_name());
        }
    }
}

#[test]
fn compiler_font_roles_keep_their_file_names_inside_the_catalog() {
    for role in FontRole::ALL {
        assert_eq!(RuntimeAsset::CompilerFont(role).file_name(), role.asset_file_name());
    }
    let names: Vec<&str> = RuntimeAsset::ALL.iter().map(|asset| asset.file_name()).collect();
    let mut distinct = names.clone();
    distinct.sort_unstable();
    distinct.dedup();
    assert_eq!(distinct.len(), names.len());
}

#[test]
fn try_from_loader_asks_every_asset_in_order_and_stops_at_the_first_failure() {
    let mut asked = Vec::new();
    let set = RuntimeAssets::try_from_loader(|asset| {
        asked.push(asset);
        Ok::<&[u8], ()>(&[])
    })
    .expect("every asset loads");
    assert_eq!(asked, RuntimeAsset::ALL.to_vec());
    assert!(set.get(RuntimeAsset::IconPack).is_empty());
    let mut asked = Vec::new();
    let failure = RuntimeAssets::try_from_loader(|asset| -> Result<&'static [u8], RuntimeAsset> {
        asked.push(asset);
        if asset == RuntimeAsset::LabelFont {
            Err(asset)
        } else {
            Ok(&[])
        }
    })
    .unwrap_err();
    assert_eq!(failure, RuntimeAsset::LabelFont);
    assert_eq!(asked.len(), 6);
}

#[test]
fn a_staged_folder_holds_the_same_bytes_as_the_repository_under_channel_directories() {
    let staged = std::env::temp_dir().join(format!("semio-staged-runtime-assets-{}", std::process::id()));
    for asset in RuntimeAsset::ALL {
        let directory = staged.join(asset.channel());
        fs::create_dir_all(&directory).expect("channel directory");
        fs::copy(repository_root().join(asset.repository_directory()).join(asset.file_name()), directory.join(asset.file_name())).expect("staged copy");
    }
    let (from_staged, from_repository) = (RuntimeAssets::from_staged_root(&staged).expect("staged assets"), RuntimeAssets::from_root(&repository_root()).expect("repository assets"));
    for asset in RuntimeAsset::ALL {
        assert_eq!(from_staged.get(asset), from_repository.get(asset), "{asset:?}");
    }
    from_staged.validate().expect("staged assets validate");
    fs::remove_file(staged.join(RuntimeAsset::IconPack.channel()).join(RuntimeAsset::IconPack.file_name())).expect("remove pack");
    let failure = RuntimeAssets::from_staged_root(&staged).unwrap_err();
    assert_eq!(failure.kind(), io::ErrorKind::NotFound);
    assert!(failure.to_string().contains(RuntimeAsset::IconPack.file_name()), "{failure}");
    fs::remove_dir_all(&staged).expect("cleanup");
}

#[test]
fn from_root_names_the_first_missing_asset_file() {
    let missing = std::env::temp_dir().join(format!("semio-no-runtime-assets-{}", std::process::id()));
    let failure = RuntimeAssets::from_root(&missing).unwrap_err();
    assert_eq!(failure.kind(), io::ErrorKind::NotFound);
    assert!(failure.to_string().contains(RuntimeAsset::CompilerFont(FontRole::Math).file_name()), "{failure}");
}

#[test]
fn an_icon_pack_of_another_version_or_shape_is_refused() {
    assert!(matches!(IconPack::parse(b"not json"), Err(IconPackError::Malformed(_))));
    assert!(matches!(IconPack::parse(br#"{"version":1,"themed":{},"plain":{},"extra":1}"#), Err(IconPackError::Malformed(_))));
    assert!(matches!(IconPack::parse(br#"{"version":1,"themed":{"a":1},"plain":{}}"#), Err(IconPackError::Malformed(_))));
    assert_eq!(IconPack::parse(br#"{"version":2,"themed":{},"plain":{}}"#).unwrap_err(), IconPackError::Version(2));
    let pack = IconPack::parse(br#"{"version":1,"themed":{"a":"<svg/>"},"plain":{"b":"<svg id=\"b\"/>"}}"#).expect("a version 1 pack");
    assert_eq!((pack.themed("a"), pack.plain("a"), pack.themed("b"), pack.plain("b")), (Some("<svg/>"), None, None, Some("<svg id=\"b\"/>")));
    assert_eq!((pack.themed_len(), pack.plain_len()), (1, 1));
}

#[test]
fn the_catalog_ids_come_in_byte_order_whatever_the_document_order_was() {
    let pack = IconPack::parse(br#"{"version":1,"themed":{"z":"<svg/>"},"plain":{"b":"<svg/>","a-b":"<svg/>","a":"<svg/>","B":"<svg/>"}}"#).expect("a version 1 pack");
    assert_eq!(pack.plain_ids(), vec!["B", "a", "a-b", "b"]);
    assert_eq!(IconPack::parse(br#"{"version":1,"themed":{"a":"<svg/>"},"plain":{}}"#).expect("an empty catalog").plain_ids(), Vec::<&str>::new());
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[test]
fn unparseable_assets_are_refused_by_validation_before_anything_is_installed() {
    let real = RuntimeAssets::from_root(&repository_root()).expect("runtime assets must be readable from the repository");
    real.validate().expect("the repository assets validate");
    let garbage: &'static [u8] = b"not a font";
    assert_eq!(RuntimeAssets { fonts: FontSet { serif: garbage, ..real.fonts }, ..real }.validate(), Err(RuntimeAssetsInstallError::UnparseableFont { role: FontRole::Serif }));
    let refused = RuntimeAssets { icon_pack: b"{", ..real }.validate().unwrap_err();
    assert!(matches!(refused, RuntimeAssetsInstallError::UnparseableIconPack { .. }), "{refused}");
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[test]
fn installed_assets_serve_every_consumer_and_a_second_install_is_refused() {
    install_repository_assets();
    let root = repository_root();
    assert!(compiler_fonts().is_some());
    assert_eq!(emoji_font().expect("emoji face"), fs::read(root.join(RuntimeAsset::CompilerFont(FontRole::Emoji).repository_directory()).join(RuntimeAsset::CompilerFont(FontRole::Emoji).file_name())).unwrap().as_slice());
    assert_eq!(label_font().expect("label face").len(), 26772);
    let pack = icon_pack().expect("icon pack");
    let ids = pack.plain_ids();
    assert_eq!(ids.len(), pack.plain_len());
    assert!(ids.windows(2).all(|pair| pair[0] < pair[1]), "the catalog ids are strictly ascending");
    let document = fixture();
    let samples = &document["iconPackSamples"];
    for row in samples["rows"].as_array().expect("sample rows") {
        let id = row["id"].as_str().expect("id");
        let source = fs::read_to_string(root.join(samples["sourceRoot"].as_str().expect("source root")).join(row["source"].as_str().expect("source"))).expect("sample source");
        let (themed, plain) = (pack.themed(id), pack.plain(id));
        match row["kind"].as_str().expect("kind") {
            "themed" => assert_eq!((themed, plain), (Some(source.as_str()), None), "{id}"),
            "plain" => assert_eq!((themed, plain), (None, Some(source.as_str())), "{id}"),
            other => panic!("unknown sample kind {other}"),
        }
    }
    let again = RuntimeAssets::from_root(&root).expect("runtime assets must be readable from the repository");
    assert_eq!(install_runtime_assets(again), Err(RuntimeAssetsInstallError::AlreadyInstalled));
}
