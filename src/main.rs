use clap::{ Parser};
use std::{
    fmt::Display, fs::File, io::{BufRead, BufReader, Error,}, path::PathBuf
};

/* Do not modify this */
// CLI args parser
#[derive(Parser, Debug)]
pub struct Args {
    // Csv file read path
    #[arg(short, long)]
    pub read_path: PathBuf,
    // Sub command for handling data in csv file
    #[clap(subcommand)]
    pub command: Command,
}

/* Do not modify this */
// Command and args prser
#[derive(Parser, Debug)]
pub enum Command {
    // Display entire file
    Display,
    // Modify a row/field
    Replace {
        #[clap(short, long)]
        row: usize,

        #[clap(short, long)]
        col: usize,

        // the new item to put into csv file
        #[clap(short, long)]
        data: String,
    },
}


#[allow(dead_code)]
#[derive(Debug, Default)]
struct CSVFile {
    data: Vec<Vec<String>>,
    rows: usize,
    cols: usize,
}

pub trait CSVFileReader {
    fn read(&mut self, file_path: PathBuf) -> Result<Vec<String>, Error>;

    fn write(&mut self, file_path: PathBuf, row: usize, col: usize, data: String) -> Result<(), Error>;
}

impl CSVFileReader for CSVFile {
    fn read(&mut self, file_path: PathBuf) -> Result<Vec<String>, Error> {
        let file = File::open(file_path)?;
        let buff = BufReader::new(file);
        let mut file_content = Vec::new();

        for (_index, line) in buff.lines().enumerate() {
            file_content.push(line.expect("expected line"));
        }

        Ok(file_content)
    }

    fn write(&mut self, file_path: PathBuf, row: usize, col: usize, data: String) -> Result<(), Error> {
        // read file if it exists
        let mut file = File::open(file_path)?;
        //  file.write(data.as_bytes()).expect("error");
        let buff = BufReader::new(file);


        // go through file lines
        for (index, line) in buff.lines().enumerate() {
            // check if the row and col to be replaced is in current line
            let mut current_line: Vec<String> = Vec::new();
            // check if current row = row to be replaced
            if index == row {
                // store current line
                current_line.push(line.expect("expected line"));

                // split line(string)
                
                // replace the word in column
                // use a replace function to replace word in a specific column
                let mut count = 0; 
                // use a replace function
                // for mut i in current_line {
                //     if count == col {
                //         i = data.clone();
                //     }
                //     count += 1;
                //     // current_line.push(i);
                // }
                
            // write to vector
                
            }
        }
        // write to file

        Ok(())
    }
}

// implement display trait to aid easy printing
impl Display for CSVFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

fn main() {
    // Reading CLI args
    let args = Args::parse();

    // create CSVFile instance and read file into it
    let mut csv = CSVFile::default();
    let read_csv = csv.read(args.read_path);

    // match and execute command
    match args.command {
        Command::Display => println!("read {:?}", read_csv),
        Command::Replace { row, col, data } => println!("--Replace and write to file--"),
    }
}
