use crate::{GodotBuildpack, GodotBuildpackError};
use crate::util::{GodotConfig, download, get_download_url, unzip};
use libcnb::build::BuildContext;
use libcnb::data::layer_content_metadata::LayerTypes;
use libcnb::generic::GenericMetadata;
use libcnb::layer::{Layer, LayerResult, LayerResultBuilder};
use libcnb::layer_env::{LayerEnv, ModificationBehavior, Scope};
use tempfile::NamedTempFile;
use std::path::Path;
use std::os::unix::fs::symlink;


pub(crate) struct GodotLayer;

impl Layer for GodotLayer {
    type Buildpack = GodotBuildpack;
    type Metadata = GenericMetadata;

    fn types(&self) -> LayerTypes {
        LayerTypes {
            build: true,
            launch: true,
            cache: false,
        }
    }

    fn create(
        &self,
        context: &BuildContext<Self::Buildpack>,
        layer_path: &Path,
    ) -> Result<LayerResult<Self::Metadata>, GodotBuildpackError> {
        println!("---> Download and install Godot");
        
        let bin_path = &layer_path.join("bin");

        let metadata = &context.buildpack_descriptor.metadata;
        let config = GodotConfig::load(metadata);
        if !config.is_valid() {
            config.print_error();
            return Err(GodotBuildpackError::InvalidConfig);
        }

        let godot_url = &metadata.godot_url;
        let godot_version = config.version.unwrap();
        let godot_tag = config.tag.unwrap();
        let godot_mono = config.mono.unwrap_or_default();

        println!("---> Godot version: {}.{}", godot_version, godot_tag);
        println!("---> Godot Mono: {}", godot_mono);

        let headless_zip = NamedTempFile::new()
            .map_err(GodotBuildpackError::TempFileCreateError)?;
        
        let server_zip = NamedTempFile::new()
            .map_err(GodotBuildpackError::TempFileCreateError)?;

        download(
            &get_download_url(
                godot_url,
                &godot_version,
                &godot_tag,
                godot_mono,
                "linux_headless.64"
            ),
            headless_zip.path()
        ).map_err(GodotBuildpackError::GodotDownloadError)?;

        download(
            &get_download_url(
                godot_url,
                &godot_version,
                &godot_tag,
                godot_mono,
                "linux_server.64"
            ),
            server_zip.path()
        ).map_err(GodotBuildpackError::GodotDownloadError)?;

        unzip(headless_zip.path(), &bin_path, "godot_headless")
            .map_err(GodotBuildpackError::GodotUnzipError)?;
        
        unzip(server_zip.path(), &bin_path, "godot_server")
            .map_err(GodotBuildpackError::GodotUnzipError)?;

        symlink("godot_server", bin_path.join("godot"))
            .map_err(GodotBuildpackError::SymbolicLinkError)?;

        symlink("godot_server", bin_path.join("godot_s"))
            .map_err(GodotBuildpackError::SymbolicLinkError)?;
            
        symlink("godot_headless", bin_path.join("godot_h"))
            .map_err(GodotBuildpackError::SymbolicLinkError)?;
        

        LayerResultBuilder::new(GenericMetadata::default())
            .env(
                LayerEnv::new()
                    .chainable_insert(
                        Scope::All,
                        ModificationBehavior::Default,
                        "PATH",
                        bin_path
                    )
                    .chainable_insert(
                        Scope::All,
                        ModificationBehavior::Default,
                        "LD_LIBRARY_PATH",
                        bin_path
                    )
            )
            .build()
    }
}
