//! Melts csv

use failure::{Error, bail};
use std::io;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let id_vars_range: Vec<_> = opt.id_vars.split("-").collect();
    if id_vars_range.len() != 2 {
        bail!("for now, it's required to specify both ends of id-vars range");
    }
    let id_vars_min: usize = id_vars_range[0].parse()?;
    let id_vars_max: usize = id_vars_range[1].parse()?;

    if id_vars_min != 0 {
        bail!("for now, melt only supports splitting id-vars and value-vars, so min must be 0");
    }

    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut wtr = csv::Writer::from_writer(io::stdout());
    dbg!(id_vars_min);
    dbg!(id_vars_max);

    // get the var members (col names) from header
    let headers = rdr.headers()?.clone();
    let headers: Vec<_> = headers.into_iter().collect();
    let (id_var_header, var_members) = headers.split_at(id_vars_max + 1);

    // header
    let mut new_header = id_var_header.to_vec();
    new_header.push(&opt.var_name);
    new_header.push(&opt.value_name);

    wtr.write_record(new_header)?;

    for result in rdr.byte_records() {
        // creating vecs seems like it should be slow;
        // but these are vecs of references to the bytes in the
        // byterecord, so that may be why it's fast
        //
        // Otherwise, manipulating the ByteRecord itself may not
        // actually be that fast.
        let record = result?;

        let record: Vec<_> = record.iter().collect();
        let (id_vars, value_vars) = record.split_at(id_vars_max + 1);

        let mut out_record = id_vars.to_vec();

        for (value, var_member) in value_vars.iter().zip(var_members) {
            // TODO: don't write if value is blank
            out_record.push(var_member.as_bytes());
            out_record.push(value);

            wtr.write_record(&out_record)?;

            out_record.pop();
            out_record.pop();
        }
        wtr.flush()?;
    }

    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(name="melt")]
struct Opt {
    #[structopt(long="id-vars")]
    #[structopt(help="id vars, specified as a range for now")]
    id_vars: String,
    //#[structopt(long="value-vars")]
    //value_vars: Option<String>,
    #[structopt(long="var_name", default_value="variable")]
    var_name: String,
    #[structopt(long="value-name", default_value="value")]
    value_name: String,

    #[structopt(long="no-header")]
    no_header: bool,
}
