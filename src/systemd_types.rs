use std::cell::RefCell;
use std::rc::Rc;

use super::*;

#[ derive (Debug, Clone, Eq, PartialEq) ]
pub enum SystemdLoadState {
	Loaded,
	Error,
	Masked,
	Other (String),
}

impl SystemdLoadState {

	pub fn as_str (& self) -> & str {

		match * self {
			SystemdLoadState::Loaded => "loaded",
			SystemdLoadState::Error => "error",
			SystemdLoadState::Masked => "masked",
			SystemdLoadState::Other (ref value) => value,
		}

	}

}

impl <'a> From <& 'a str> for SystemdLoadState {

	fn from (
		string: & str,
	) -> SystemdLoadState {

		match string {
			"loaded" => SystemdLoadState::Loaded,
			"error" => SystemdLoadState::Error,
			"masked" => SystemdLoadState::Masked,
			other => SystemdLoadState::Other (other.to_owned ()),
		}

	}

}

#[ derive (Debug, Clone, Eq, PartialEq) ]
pub enum SystemdActiveState {
	Active,
	Reloading,
	Inactive,
	Failed,
	Activating,
	Deactivating,
	Other (String),
}

impl SystemdActiveState {

	pub fn as_str (& self) -> & str {

		match * self {
			SystemdActiveState::Active => "active",
			SystemdActiveState::Reloading => "reloading",
			SystemdActiveState::Inactive => "inactive",
			SystemdActiveState::Failed => "failed",
			SystemdActiveState::Activating => "activating",
			SystemdActiveState::Deactivating => "deactivating",
			SystemdActiveState::Other (ref value) => value,
		}

	}

}

impl <'a> From <& 'a str> for SystemdActiveState {

	fn from (
		string: & str,
	) -> SystemdActiveState {

		match string {
			"active" => SystemdActiveState::Active,
			"reloading" => SystemdActiveState::Reloading,
			"inactive" => SystemdActiveState::Inactive,
			"failed" => SystemdActiveState::Failed,
			"activating" => SystemdActiveState::Activating,
			"deactivating" => SystemdActiveState::Deactivating,
			other => SystemdActiveState::Other (other.to_owned ()),
		}

	}

}

#[ derive (Debug, Clone, Eq, PartialEq) ]
pub enum SystemdSubState {
	Other (String),
}

impl SystemdSubState {

	pub fn as_str (& self) -> & str {

		match * self {
			SystemdSubState::Other (ref value) => value,
		}

	}

}

impl <'a> From <& 'a str> for SystemdSubState {

	fn from (
		string: & str,
	) -> SystemdSubState {

		match string {
			other => SystemdSubState::Other (other.to_owned ()),
		}

	}

}

#[ derive (Debug, Clone, Eq, PartialEq) ]
pub enum SystemdJobType {
	Start,
	VerifyActive,
	Stop,
	Reload,
	Restart,
	TryRestart,
	ReloadOrStart,
	Other (String),
}

impl <'a> From <& 'a str> for SystemdJobType {

	fn from (
		string: & str,
	) -> SystemdJobType {

		match string {
			"start" => SystemdJobType::Start,
			"verify-active" => SystemdJobType::VerifyActive,
			"stop" => SystemdJobType::Stop,
			"reload" => SystemdJobType::Reload,
			"restart" => SystemdJobType::Restart,
			"try-restart" => SystemdJobType::TryRestart,
			"reload-or-start" => SystemdJobType::ReloadOrStart,
			other => SystemdJobType::Other (other.to_owned ()),
		}

	}

}

// ex: noet ts=4 filetype=rust
