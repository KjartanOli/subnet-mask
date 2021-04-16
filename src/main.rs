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
				 .help("Select the output type from dotted binary, dotted decimal, prefix notation, machine readable binary, and all")
				 .takes_value(true)
				 .possible_values(&["b", "bin", "binary", "d", "dotted", "p", "prefix", "m", "machine", "a", "all"])
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
		"d" => OutputType::DottedDecimal,
		"dotted" => OutputType::DottedDecimal,
		"p" => OutputType::Prefix,
		"prefix" => OutputType::Prefix,
		"m" => OutputType::Machine,
		"machine" => OutputType::Machine,
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
		OutputType::DottedDecimal => {
			println!("{}", format_dotted_decimal(&calculate_mask_octets(calculate_subnet_mask(&requiredHosts, &ipVersion))))
		},
		OutputType::Prefix => {
			println!("/{}", calculate_prefix(&requiredHosts, &ipVersion));
		},
		OutputType::Machine => {
			println!("{:b}", calculate_subnet_mask(&requiredHosts, &ipVersion));
		},
		OutputType::All => {
			let subnetMask = calculate_subnet_mask(&requiredHosts, &ipVersion);
			let prefix = calculate_prefix(&requiredHosts, &ipVersion);
			let octets = calculate_mask_octets(subnetMask);
			println!("Dotted Binary: {}\nDotted Decimal: {}\nPrefix: /{}\nMachine Binary: {:b}", format_dotted_binary(&octets), format_dotted_decimal(&octets), prefix, subnetMask);
		},
		OutputType::Invalid => {
			println!("Somehow an invalid argument got past our error checking. It would be greatly appreciated if you could let us know how you did that.");
				std::process::exit(1)
		}
	};
}

enum OutputType {
	Binary,
	DottedDecimal,
	Prefix,
	Machine,
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

fn calculate_mask_octets(mut mask: u32) -> [u32; 4] {
	let mut octets: [u32; 4] = [0, 0, 0, 0];

	for i in (0..4).rev() {
		octets[i] = mask & 0xFF;
		mask >>= 8u8;
	}

	octets
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

fn format_dotted_decimal(octets: &[u32; 4]) -> String {
	format!("{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3])
}

fn format_dotted_binary(octets: &[u32; 4]) -> String {
	format!("{:08b}.{:08b}.{:08b}.{:08b}", octets[0], octets[1], octets[2], octets[3])
}
