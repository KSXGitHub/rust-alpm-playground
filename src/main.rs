use alpm::{Alpm, Package};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "rust-alpm-playground", rename_all = "kebab")]
struct Args {
    /// Package name
    #[structopt(name = "PACKAGE")]
    pub package: String,
}

fn find_package<'a>(alpm: &'a Alpm, name: &'a str) -> Option<Package<'a>> {
    alpm.localdb()
        .pkgs()
        .into_iter()
        .find(|pkg| pkg.name() == name)
}

/// # Experiment
///
/// Run this script before and after installing &lt;PACKAGE&gt;
///
/// ## Result
///
/// ### Before installing &lt;PACKAGE&gt;
///
/// `None`
///
/// ### After installing &lt;PACKAGE&gt;
///
/// `Some(Package { ... })`
fn main() {
    let args = Args::from_args();
    let alpm = Alpm::new("/", "/var/lib/pacman").expect("init alpm");
    dbg!(find_package(&alpm, &args.package));
}
