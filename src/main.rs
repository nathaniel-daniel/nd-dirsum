use anyhow::bail;
use anyhow::Context;
use sha2::Digest;
use sha2::Sha256;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, argh::FromArgs)]
#[argh(description = "generate a sum for a directory")]
struct Options {
    #[argh(positional)]
    path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let options: Options = argh::from_env();

    let mut hasher = Sha256::new();

    let dir_iter = WalkDir::new(&options.path)
        .contents_first(false)
        .follow_links(false)
        .follow_root_links(false)
        .sort_by_file_name();
    for dir_entry in dir_iter {
        let dir_entry = dir_entry?;
        let file_type = dir_entry.file_type();
        let path = dir_entry.path();

        if file_type.is_dir() {
            hasher.update([0x00]);
        } else if file_type.is_file() {
            hasher.update([0x01]);
        } else if file_type.is_symlink() {
            bail!("symlinks are currently not supported");
        }

        let relative_path = path
            .strip_prefix(&options.path)
            .context("failed to strip root directory prefix")?;

        hash_path(&mut hasher, relative_path)?;

        if file_type.is_file() {
            let mut file = File::open(path)?;
            std::io::copy(&mut file, &mut hasher)?;
        }
    }

    let hash = hasher.finalize();
    let hex_hash = base16ct::lower::encode_string(&hash);

    println!("{hex_hash}");

    Ok(())
}

fn hash_path(hasher: &mut Sha256, path: &Path) -> anyhow::Result<()> {
    let path = path
        .to_str()
        .context("non-unicode paths are currently not supported")?;
    hasher.update(path);

    Ok(())
}
