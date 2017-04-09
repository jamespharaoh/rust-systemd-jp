use std::cell::RefCell;
use std::rc::Rc;

use super::*;
use super::systemd_dbus::*;

type ListUnitTuple <'a> = (
	& 'a str,
	& 'a str,
	& 'a str,
	& 'a str,
	& 'a str,
	& 'a str,
	DbusPath <'a>,
	u32,
	& 'a str,
	DbusPath <'a>,
);

pub struct SystemdManager {
	connection: Rc <RefCell <SystemdConnection>>,
}

impl SystemdManager {

	pub fn new (
		connection: Rc <RefCell <SystemdConnection>>,
	) -> SystemdManager {

		SystemdManager {
			connection: connection,
		}

	}

	pub fn list_units (
		& self,
	) -> Result <Vec <SystemdUnit>, String> {

		let connection =
			self.connection.borrow ();

		let dbus_request =
			DbusMessage::new_method_call (
				"org.freedesktop.systemd1",
				"/org/freedesktop/systemd1",
				"org.freedesktop.systemd1.Manager",
				"ListUnits",
			).unwrap ();

		let dbus_response =
			connection.dbus_connection ().send_with_reply_and_block (
				dbus_request,
				2000,
			).unwrap ();

		let systemd_units: Vec <ListUnitTuple> =
			dbus_response.get1 ().unwrap ();

		Ok (systemd_units.into_iter ().map (|systemd_unit| {

			let (
				unit_name,
				unit_description,
				unit_load_state,
				unit_active_state,
				unit_sub_state,
				unit_following_unit,
				unit_object_path,
				unit_job_id,
				unit_job_type,
				unit_job_object_path,
			) = systemd_unit;

			SystemdUnit::new (
				self.connection.clone (),
				unit_name.to_string (),
				unit_description.to_string (),
				SystemdLoadState::from (unit_load_state),
				SystemdActiveState::from (unit_active_state),
				SystemdSubState::from (unit_sub_state),
				if unit_following_unit != "" {
					Some (unit_following_unit.to_owned ())
				} else { None },
				unit_object_path.to_string (),
				unit_job_id,
				SystemdJobType::from (unit_job_type),
				unit_job_object_path.to_string (),
			)

		}).collect ())

	}

}

// ex: noet ts=4 filetype=rust
