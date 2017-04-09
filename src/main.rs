extern crate dbus;

mod systemd;

use std::cell::RefCell;
use std::env;
use std::rc::Rc;

use systemd::*;

fn main () {

	let arguments: Vec <String> =
		env::args ().skip (1).collect ();

	main_real ().unwrap_or_else (|error| {

		println! (
			"Error: {}",
			error);

	});

}

fn main_real (
) -> Result <(), String> {

	let systemd_connection =
		Rc::new (RefCell::new (
			SystemdConnection::new () ?,
		));

	let systemd_manager =
		SystemdManager::new (
			systemd_connection);

	list_services (
		& systemd_manager);

	Ok (())

}

fn list_services (
	systemd_manager: & SystemdManager,
) -> Result <(), String> {

	let systemd_units =
		systemd_manager.list_units () ?;

	for systemd_unit in systemd_units {

		println! (
			"Unit: {}",
			systemd_unit.name ());

	}

	Ok (())

}

// ex: noet ts=4 filetype=rust
