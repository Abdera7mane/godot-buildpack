use crate::layers::GodotLayer;
use libcnb::build::{BuildContext, BuildResult, BuildResultBuilder};
use libcnb::data::build_plan::BuildPlanBuilder;
use libcnb::data::launch::{LaunchBuilder, ProcessBuilder};
use libcnb::data::{layer_name, process_type};
use libcnb::detect::{DetectContext, DetectResult, DetectResultBuilder};
use libcnb::generic::GenericPlatform;
use libcnb::{buildpack_main, Buildpack};
use serde::Deserialize;
use util::{DownloadError, UnzipError};

mod layers;
mod util;

pub(crate) struct GodotBuildpack;

impl Buildpack for GodotBuildpack {
    type Platform = GenericPlatform;
    type Metadata = GodotBuildpackMetadata;
    type Error = GodotBuildpackError;

    fn detect(&self, context: DetectContext<Self>) -> libcnb::Result<DetectResult, Self::Error> {
        if context.app_dir.join("project.godot").exists()
        || context.app_dir.join("config.godot").exists() {
            DetectResultBuilder::pass()
                .build_plan(
                    BuildPlanBuilder::new()
                        .provides("godot")
                        .requires("godot")
                        .build()
                )
                .build()
        } else {
            DetectResultBuilder::fail().build()
        }
    }

    fn build(&self, context: BuildContext<Self>) -> libcnb::Result<BuildResult, Self::Error> {
        println!("---> Godot Buildpack");

        context.handle_layer(layer_name!("godot"), GodotLayer)?;

        BuildResultBuilder::new()
        .launch(
            LaunchBuilder::new()
            .process(
                ProcessBuilder::new(process_type!("worker"), "godot")
                    .args(vec!["--path", "."])
                    .build(),
            )
            .build(),
        )
        .build()
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct GodotBuildpackMetadata {
    pub godot_url: String,
    pub godot_version: String,
    pub godot_tag: String,
    pub godot_mono: bool,
}


#[derive(Debug)]
pub(crate) enum GodotBuildpackError {
    GodotDownloadError(DownloadError),
    GodotUnzipError(UnzipError),
    TempFileCreateError(std::io::Error),
    SymbolicLinkError(std::io::Error),
    InvalidConfig

}

buildpack_main!(GodotBuildpack);
