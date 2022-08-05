use clap::Parser;
use std::{
    env,
    fs::{self, create_dir_all},
    io,
    os::unix::prelude::PermissionsExt,
    path::Path,
};

use crate::{
    utils::{download_with_progress, get_data_dir},
    Commands,
};

#[derive(Parser)]
pub struct CliCommand {
    /// Version to install
    /// ex. 0.1.6 or latest
    #[clap(value_parser)]
    version: String,

    /// Install even though already installed
    #[clap(short, long)]
    reinstall: bool,
}

pub async fn match_and_run(commands: &Commands) {
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match commands {
        Commands::Install(cmd) => run(cmd).await,
        _ => (),
    };
}

async fn run(cmd: &CliCommand) {
    println!("Searching for a bun version containing {} 🔎", cmd.version);
    let octocrab = octocrab::instance();
    let releases = octocrab
        .repos("oven-sh", "bun")
        .releases()
        .list()
        .per_page(100)
        .send()
        .await
        .expect("Error getting bun releases");

    let release = match &*cmd.version {
        "latest" => releases
            .items
            .iter()
            .find(|x| x.tag_name.contains("bun-v"))
            .expect("Couldn't find latest release"),
        _ => releases
            .items
            .iter()
            .find(|x| x.tag_name.contains(&cmd.version))
            .unwrap_or_else(|| panic!("Couldn't find a release containing {}",
                cmd.version)),
    };

    println!("Found {} ✔", release.tag_name);

    let outdir = Path::new(&get_data_dir()).join(format!("versions/{}", release.tag_name));
    let outpath = outdir.join("bun");

    if outpath.exists() && !cmd.reinstall {
        println!(
            "This version of bun is already installed. Run with `--reinstall` flag to reinstall"
        );
        return;
    } else if outpath.exists() {
        fs::remove_dir(&outdir).unwrap();
    }

    let bun_os_name = match env::consts::OS {
        "linux" => "linux",
        "macos" => "darwin",
        _ => todo!("Platform {} not implemented yet", env::consts::OS),
    };
    let bun_arch = match env::consts::ARCH {
        "x86_64" => "x64",
        _ => env::consts::ARCH,
    };

    let download_link = &release
        .assets
        .iter()
        .find(|x| x.name == format!("bun-{}-{}.zip", bun_os_name, bun_arch))
        .expect("Couldn't find release compatible with your system")
        .browser_download_url
        .as_str();

    let zip_buf = download_with_progress(download_link).await;

    let zip_reader = std::io::Cursor::new(zip_buf);

    let mut zip = zip::ZipArchive::new(zip_reader).unwrap();

    println!("Extracting...");

    // Find bun file in zip and extract it to outdir
    let mut successful = false;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        if file.name().ends_with("/bun") {
            successful = true;
            // Make sure dir is created
            create_dir_all(&outdir)
                .expect("Installation failed, couldn't create install directory");

            // Copy file from zip to install dir
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();

            // Make file executable
            fs::set_permissions(&outpath, fs::Permissions::from_mode(0o755)).unwrap();
        }
    }

    if !successful {
        panic!("Couldn't find bun executable in zip")
    }

    println!("Installed bun {}", release.tag_name);
}
