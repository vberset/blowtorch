// This file is part of blowtorch - Burn. Efficiently.
//
// blowtorch is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// blowtorch is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with blowtorch. If not, see <http://www.gnu.org/licenses/>.

extern crate clap;
extern crate indicatif;
extern crate acetylene;

use std::thread;
use std::sync::mpsc::channel;

use clap::{Arg, App, AppSettings, SubCommand, ArgMatches};

use indicatif::{ProgressBar, ProgressStyle};

use acetylene::{get_device_list, device_path, burn_image, BurnConfig, BurnSetting, Progress};

fn main() {
    let settings = [
        AppSettings::UnifiedHelpMessage,
        AppSettings::SubcommandRequiredElseHelp,
    ];

    let matches = App::new("blowtorch")
        .version("0.1")
        .settings(&settings)
        .subcommand(SubCommand::with_name("devices")
            .about("list available devices"))
        .subcommand(SubCommand::with_name("burn")
            .about("burn desired image to the specified device")
            .arg(Arg::with_name("VERIFY")
                .long("verify")
                .help("verify the integrity of the write data"))
            .arg(Arg::with_name("DEVICE")
                .long("device")
                .short("d")
                .takes_value(true))
            .arg(Arg::with_name("IMAGE")
                .required(true)
                .takes_value(true)))
        .get_matches();

    match matches.subcommand() {
        ("burn", Some(submatches)) => burn(submatches),
        ("devices", Some(submatches)) => devices(submatches),
        _ => unreachable!(),
    }
}

fn devices(_submatches: &ArgMatches) {
    for device in get_device_list() {
        println!("{} : {} ({} MB)", device.name, device.path, device.mbytes);
    }
}

fn burn(submatches: &ArgMatches) {
    let devices = get_device_list();
    let mut settings = Vec::new();

    if submatches.is_present("VERIFY") {
        settings.push(BurnSetting::Verify);
    }

    let device = submatches.value_of("DEVICE").unwrap().to_owned();
    let image = submatches.value_of("IMAGE").unwrap().to_owned();

    let device = device_path(&devices, &device).unwrap_or_else(|| {
        std::process::abort();
    });

    let config = BurnConfig {
        device,
        image,
        settings,
    };

    let (tx, rx) = channel();

    thread::spawn(move || {
        burn_image(config, tx);
    });

    let bar = ProgressBar::new_spinner();
    bar.set_style(
        ProgressStyle::default_bar()
           .template("{spinner:.green} [{elapsed_precise}] [{bar:40}] {bytes}/{total_bytes} ({eta})")
    );

    for progress in rx.iter() {
        match progress {
            Progress::Start {total} => {
                bar.set_length(total);
                bar.set_position(0);
            },
            Progress::Progress {count, total: _} => {
                let pc = count;
                bar.set_position(pc);
            },
            Progress::End {digest: _} => {
                bar.finish_with_message("burned!");
                break;
            },
            Progress::Error => {
                println!("An error occured");

                break;
            }
        }
    }
}