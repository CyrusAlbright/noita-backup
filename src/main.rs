use std::path::PathBuf;

use clap::Parser;

#[derive(clap::Parser, Debug)]
struct Args {
	command: Command,
}

#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq)]
enum Command {
	Save,
	Load,
	Delete,
}

fn main() -> anyhow::Result<()> {
	let args = Args::parse();

	let command = args.command;

	let local_low_path = PathBuf::from(
		windows::Storage::UserDataPaths::GetDefault()?
			.LocalAppDataLow()?
			.to_string(),
	);

	let save_path = {
		let mut path = local_low_path.clone();
		path.push("Nolla_Games_Noita");
		path.push("save00");
		path
	};

	let backups_path = {
		let mut path = local_low_path;
		path.push("noita-backup");
		path
	};

	assert!(save_path.exists());

	std::fs::create_dir_all(&backups_path)?;

	let options = fs_extra::dir::CopyOptions::new().overwrite(true);

	match command {
		Command::Save => {
			fs_extra::dir::copy(save_path, backups_path, &options)?;
		}
		Command::Load => {
			fs_extra::dir::copy(backups_path, save_path, &options)?;
		}
		Command::Delete => {
			for entry in std::fs::read_dir(backups_path)? {
				let entry = entry?;
				if entry.metadata()?.is_dir() {
					std::fs::remove_dir_all(entry.path())?;
				} else {
					std::fs::remove_file(entry.path())?;
				}
			}
		}
	}

	Ok(())
}
