use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;
use serde::Serialize;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "passpack_to_lastpass",
    about = "converting passpack csv of 2020 into "
)]
struct Opt {
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input: PathBuf,
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: PathBuf,
}

// see https://support.logmeininc.com/lastpass/help/how-do-i-import-stored-data-into-lastpass-using-a-generic-csv-file
#[derive(Debug, Serialize)]
struct LastPassRecord<'a> {
    url: &'a str,
    username: &'a str,
    password: &'a str,
    extra: &'a str, // aka notes
    name: &'a str,
    grouping: &'a str,
    fav: u8, // 1 or 0
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let mut rdr = csv::Reader::from_reader(File::open(opt.input)?);
    let mut wtr = csv::Writer::from_writer(File::create(opt.output)?);

    for result in rdr.records() {
        let record = result?;
        let name = &record[0];
        let url = &record[1];
        let password = &record[3];
        let extra = &record[4];
        let grouping = "";
        let username = if !&record[2].is_empty() {
            &record[2]
        } else {
            &record[8]
        };
        let fav = 0;
        wtr.serialize({
            LastPassRecord {
                url,
                username,
                password,
                extra,
                name,
                grouping,
                fav,
            }
        })?;
    }

    Ok(())
}
