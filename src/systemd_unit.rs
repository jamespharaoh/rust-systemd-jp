use std::cell::RefCell;
use std::rc::Rc;

use super::*;

pub struct SystemdUnit {

	connection: Rc <RefCell <SystemdConnection>>,

	name: String,
	description: String,
	load_state: SystemdLoadState,
	active_state: SystemdActiveState,
	sub_state: SystemdSubState,
	following_unit: Option <String>,
	unit_object_path: String,
	job_id: u32,
	job_type: SystemdJobType,
	job_object_path: String,

}

impl SystemdUnit {

	pub fn new (
		connection: Rc <RefCell <SystemdConnection>>,
		name: String,
		description: String,
		load_state: SystemdLoadState,
		active_state: SystemdActiveState,
		sub_state: SystemdSubState,
		following_unit: Option <String>,
		unit_object_path: String,
		job_id: u32,
		job_type: SystemdJobType,
		job_object_path: String,
	) -> SystemdUnit {

		SystemdUnit {
			connection: connection,
			name: name,
			description: description,
			load_state: load_state,
			active_state: active_state,
			sub_state: sub_state,
			following_unit: following_unit,
			unit_object_path: unit_object_path,
			job_id: job_id,
			job_type: job_type,
			job_object_path: job_object_path,
		}

	}

	pub fn name (& self) -> & str {
		& self.name
	}

	pub fn description (& self) -> & str {
		& self.name
	}

	pub fn load_state (& self) -> & SystemdLoadState {
		& self.load_state
	}

	pub fn active_state (& self) -> & SystemdActiveState {
		& self.active_state
	}

	pub fn sub_state (& self) -> & SystemdSubState {
		& self.sub_state
	}

}

// ex: noet ts=4 filetype=rust
