use std::error::Error;

use rmcl::mc_manifest::McManifest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mc_manifest =
        reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest_v2.json")
            .await?
            .json::<McManifest>()
            .await?;


    for version in mc_manifest.versions.iter() {
        println!("{}", version.id)
    }

    Ok(())
}
