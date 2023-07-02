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

    #[structopt(short, long)]
    local: Option<String>,
}

fn main() {
    let opt = Opt::from_args();
    match EventDate::new(opt.date.clone(), opt.local, opt.timezone.clone()) {
        Ok(d) => pretty_print(opt.message, d.get_dates_by_timezones()),
        Err(e) => eprintln!("{}", e),
    }
}

fn pretty_print(message: String, timezones: Vec<TimezoneData>) {
    println!("---");
    println!("{}\n", message);
    for tz_data in timezones {
        println!("{}", tz_data);
    }
    println!("---");
}
