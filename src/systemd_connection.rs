use super::systemd_dbus::*;

pub struct SystemdConnection {
	dbus_connection: DbusConnection,
}

impl SystemdConnection {

	pub fn new (
	) -> Result <SystemdConnection, String> {

		let dbus_connection =
			DbusConnection::get_private (
				DbusBusType::System,
			).unwrap ();

		Ok (SystemdConnection {
			dbus_connection: dbus_connection,
		})

	}

	pub fn dbus_connection (& self) -> & DbusConnection {
		& self.dbus_connection
	}

}

// ex: noet ts=4 filetype=rust
