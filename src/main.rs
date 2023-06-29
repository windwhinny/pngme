use crate::arg::{Cli, Commands};
use crate::png::Png;
use std::error::Error;
use std::os::unix::prelude::OsStringExt;
use clap::{Parser};

mod png;
mod chunk;
mod chunk_type;
mod error;
mod arg;

fn main() -> Result<(), Box<dyn Error>> {
	let cli = Cli::parse();
	match cli.command {
		Commands::Encode {
			file_path,
			chunk_type,
			message,
			output_file } => {
				let file_path = file_path.to_str().expect("filepath not readable").to_owned();
				let chunk_type = chunk_type.to_str().expect("chunk_type not readable").to_owned();
				let message = message.into_vec();
				let output_file = output_file
					.and_then(|f| Some(f.to_str().unwrap().to_owned()))
					.unwrap_or_else(|| file_path.to_owned());

				let mut png = Png::from_file(&file_path)?;

				png.upinsert(&chunk_type, message)?;

				png.to_file(&output_file)?;
			},
			Commands::Print { file_path , index , as_string, chunk_type} => {
				let file_path = file_path.to_str().expect("filepath not readable");
				let png = Png::from_file(file_path)?;

				let chunk = if let Some(index) = index {
					let index: usize = index.to_str().expect("index not readable").parse()?;
					Some(&png.chunks()[index])
				} else if let Some(chunk_type) = chunk_type {
					let chunk_type = chunk_type.to_str().expect("chunk_type not readable");
					png.chunk_by_type(chunk_type)
				} else {
					None
				};

				if let Some(chunk) = chunk {
					if as_string {
						println!("{}", chunk.data_as_string()?);
					} else {
						println!("{:?}", chunk.data());
					}
				} else {
					let names: Vec<&str> = png.chunks().into_iter().map(|chunk| chunk.chunk_type().to_str()).collect();
					println!("{}", names.join(" "));
				}
			},
			Commands::Create { file_path } => {
				let file_path = file_path.to_str().expect("filepath not readable");	
				let png = Png::new();
				png.to_file(file_path)?;
			},
			Commands::Remove { file_path, chunk_type, output_file } => {
				let file_path = file_path.to_str().expect("filepath not readable");	
				let chunk_type = chunk_type.to_str().expect("chunk_type not readable");
				let output_file = output_file
					.and_then(|f| Some(f.to_str().unwrap().to_owned()))
					.unwrap_or_else(|| file_path.to_owned());

				let mut png = Png::from_file(file_path)?;
				png.remove_chunk(chunk_type)?;
				png.to_file(&output_file)?;
			}
	};
	Ok(())
}