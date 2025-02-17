use anyhow::bail;
use anyhow::Context;
use sha2::Digest;
use sha2::Sha256;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

const DIR_MARKER: u8 = 0x00;
const FILE_MARKER: u8 = 0x01;

#[derive(Debug, argh::FromArgs)]
#[argh(description = "generate a sum for a directory")]
struct Options {
    #[argh(positional, default = "PathBuf::from(\".\")")]
    path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let options: Options = argh::from_env();

    match std::fs::symlink_metadata(&options.path) {
        Ok(metadata) => {
            let file_type = metadata.file_type();

            if file_type.is_dir() {
                // Pass
            } else if file_type.is_file() {
                bail!("the input path is a file");
            } else if file_type.is_symlink() {
                bail!("symlinks are currently not supported");
            } else {
                bail!("unknown file type");
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            bail!(
                "the directory at \"{}\" does not exist",
                options.path.display()
            );
        }
        Err(error) => {
            return Err(error).with_context(|| {
                format!("failed to get metadata for \"{}\"", options.path.display())
            });
        }
    }

    let mut hasher = Sha256::new();

    let dir_iter = WalkDir::new(&options.path)
        .contents_first(false)
        .follow_links(false)
        .follow_root_links(false)
        .sort_by_file_name();
    for dir_entry in dir_iter {
        let dir_entry = dir_entry.context("failed to get directory entry")?;
        let file_type = dir_entry.file_type();
        let path = dir_entry.path();

        if file_type.is_dir() {
            hasher.update([DIR_MARKER]);
        } else if file_type.is_file() {
            hasher.update([FILE_MARKER]);
        } else if file_type.is_symlink() {
            bail!("symlinks are currently not supported");
        } else {
            bail!("unknown file type");
        }

        let relative_path = path
            .strip_prefix(&options.path)
            .context("failed to strip root directory prefix")?;

        hash_path(&mut hasher, relative_path)?;

        if file_type.is_file() {
            let mut file = File::open(path)
                .with_context(|| format!("failed to open \"{}\"", path.display()))?;
            std::io::copy(&mut file, &mut hasher)?;
        }
    }

    let hash = hasher.finalize();
    let hex_hash = base16ct::lower::encode_string(&hash);

    println!("{hex_hash}");

    Ok(())
}

fn hash_path(hasher: &mut Sha256, path: &Path) -> anyhow::Result<()> {
    for component in path.components() {
        let component = component
            .as_os_str()
            .to_str()
            .context("non-unicode paths are currently not supported")?;
        hasher.update([0x00]);
        hasher.update(component);
    }

    Ok(())
}
