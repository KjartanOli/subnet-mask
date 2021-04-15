/*
 * Copyright (C) 2021  Ágústsson, Kjartan Óli <kjartanoli@protonmail.com>, and Benjamínsson, Dagur <dagurbenjamins@protonmail.com>
 * This file is a part of Subnet Mask
 * Subnet Mask is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Subnet Mask is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#![allow(non_snake_case)]
fn main() {
	let matches = clap::App::new("Subnet Mask")
		.version(clap::crate_version!())
		.author("Kjartan Óli Ágústsson <kjartanoli@protonmail.com> and Dagur Benjamínsson <dagurbenjamins@protonmail.com>")
		.about("Calculate the subnet mask required to create a subnet supporting a given number of hosts.")
		.bin_name("subnet-mask")
		.arg(clap::Arg::with_name("ip-version")
				 .short("v")
				 .long("ip-version")
				 .help("Select the IP version to use")
				 .takes_value(true)
				 .possible_values(&["4", "6"])
				 .default_value("4")
				 .number_of_values(1))
		.arg(clap::Arg::with_name("output")
				 .short("o")
				 .long("output")
				 .help("Select the output type")
				 .takes_value(true)
				 .possible_values(&["b", "bin", "binary", "h", "human", "p", "prefix", "a", "all"])
				 .default_value("all")
				 .number_of_values(1))
		.arg(clap::Arg::with_name("HOSTS")
				 .help("The number of hosts required on the subnet")
				 .required(true)
				 .index(1))
		.get_matches();


	let output: OutputType = match matches.value_of("output").unwrap() {
		"b" => OutputType::Binary,
		"bin" => OutputType::Binary,
		"binary" => OutputType::Binary,
		"h" => OutputType::Human,
		"human" => OutputType::Human,
		"p" => OutputType::Prefix,
		"prefix" => OutputType::Prefix,
		"a" => OutputType::All,
		"all" => OutputType::All,
		_ => OutputType::Invalid,
	};

	let ipVersion: u8 = matches.value_of("ip-version").unwrap().parse().expect("Somehow you managed to get an invalid  IP version past our argument validation. We would appreciate it if you could let us know how you did that.");
	let requiredHosts: u32 = matches.value_of("HOSTS").unwrap().parse().expect("HOSTS must be a number.");

	match output {
		OutputType::Binary => {
			println!("{:b}", calculate_subnet_mask(&requiredHosts, &ipVersion));
		},
		OutputType::Human => {
			println!("Not yet implemented");
		},
		OutputType::Prefix => {
			println!("/{}", calculate_prefix(&requiredHosts, &ipVersion));
		},
		OutputType::All => {
			let subnetMask = calculate_subnet_mask(&requiredHosts, &ipVersion);
			let prefix = calculate_prefix(&requiredHosts, &ipVersion);
			println!("Binary: {:b}\nHuman: {}\nPrefix: /{}", subnetMask, "Not yet implemented", prefix);
		},
		OutputType::Invalid => {
			println!("Somehow an invalid argument got past our error checking. It would be greatly appreciated if you could let us know how you did that.");
				std::process::exit(1)
		}
	};
}

enum OutputType {
	Binary,
	Human,
	Prefix,
	All,
	Invalid,
}

fn calculate_subnet_mask(hosts: &u32, version: &u8) -> u32 {
	match version {
		4 => {
			let hostBits: u32 = calculate_host_bits(&hosts, &version);
			return u32::MAX - ((1 << hostBits) - 1);
		},
		6 => {
			println!("IPv6 is not supported yet.");
			std::process::exit(2);
		},
		_ => {
			println!("Somehow an invalid argument got past our error checking. It would be greatly appreciated if you could let us know how you did that.");
			std::process::exit(1)
		}
	}
}

fn calculate_prefix(hosts: &u32, version: &u8) -> u32 {
	match version {
		4 => {
			let hostBits: u32 = calculate_host_bits(&hosts, &version);
			return 32u32 - hostBits;
		},
		6 => {
			println!("IPv6 is not supported yet.");
			std::process::exit(2);
		},
		_ => {
			println!("Somehow an invalid argument got past our error checking. It would be greatly appreciated if you could let us know how you did that.");
				std::process::exit(1)
		}
	}
}

fn calculate_host_bits(hosts: &u32, version: &u8) -> u32 {
		match version {
			4 => {
				return 32u32 - hosts.leading_zeros()
			},
			6 => {
				println!("IPv6 is not supported yet.");
				std::process::exit(2);
			},
			_ => {
				println!("Somehow an invalid argument got past our error checking. It would be greatly appreciated if you could let us know how you did that.");
					std::process::exit(1)
			}
		}
}
