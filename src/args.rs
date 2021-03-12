use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pngme", about = "Hide secret messages in PNG files")]
pub enum Args {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(StructOpt, Debug)]
#[structopt(name = "encode", about = "Inserts a message into a PNG file")]
pub struct EncodeArgs {
    pub file_path: String,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<String>,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "decode", about = "Decodes the specified chunk type")]
pub struct DecodeArgs {
    pub file_path: String,
    pub chunk_type: String,
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "remove",
    about = "Remove a message with the provided chunk type"
)]
pub struct RemoveArgs {
    pub file_path: String,
    pub chunk_type: String,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "print", about = "Print chunks stored in the PNG file")]
pub struct PrintArgs {
    pub file_path: String,
}
