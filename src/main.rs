use evet::date::{EventDate, TimezoneData};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "evet")]
struct Opt {
    #[structopt(short, long)]
    message: String,

    #[structopt(short, long)]
    date: String,

    #[structopt(short, long)]
    timezone: Vec<String>,

    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,
}

fn main() {
    let opt = Opt::from_args();
    match EventDate::new(opt.date.clone(), opt.timezone.clone()) {
        Ok(d) => pretty_print(opt.message, d.get_dates_by_timezones()),
        Err(e) => eprintln!("{}", e),
    }
}

fn pretty_print(message: String, timezones: Vec<TimezoneData>) {
    println!("---");
    println!("{}", message);
    println!("");
    for tz_data in timezones {
        println!("{}", tz_data.to_string());
    }
    println!("---");
}
