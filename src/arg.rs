use clap::{Parser, Subcommand, command };
use std::ffi::OsString;

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(about = "png parser", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Encode {
        #[arg(required=true, help="File to read from.")]
        file_path: OsString,
        #[arg(required=true, help="A 4-byte chunk type code. For convenience in description and in examining PNG files, type codes are restricted to consist of uppercase and lowercase ASCII letters (A-Z and a-z, or 65-90 and 97-122 decimal).")]
        chunk_type: OsString,
        #[arg(required=true, help="The data bytes appropriate to the chunk type, if any.")]
        message: OsString,
        #[arg()]
        output_file: Option<OsString>,
    },
		Remove {
			#[arg(required=true, help="File to read from.")]
			file_path: OsString,
			#[arg(long, short, help="remove the chunk type")]
			chunk_type: OsString,
			#[arg()]
			output_file: Option<OsString>,
		},
		Create {
			#[arg(required=true, help="File path to create.")]
			file_path: OsString,
		},
		Print {
			#[arg(required=true, help="File to read from.")]
			file_path: OsString,
			#[arg(long, short, help="print the full data at index")]
			index: Option<OsString>,
			#[arg(long, short, help="print the full data with chunk type")]
			chunk_type: Option<OsString>,
			#[arg(long="string", short='s', help="print the full data as string")]
			as_string: bool,
		}
}