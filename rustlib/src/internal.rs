pub struct _State {
	pub i: Option<i64>,
	pub f: Option<f64>,
	pub b: Option<bool>,
	pub s: Option<std::string::String>
}

struct _StateList { v: Option<std::vec::Vec<_State>> }
static mut _STATES: _StateList = _StateList { v: None };
static mut _STATE: Option<_State> = None;

static mut _STATE_GET_SUCCESS: bool = false;
static mut _STATE_GET_END: bool = false;
static mut _STATE_GET_POS: usize = 0;
static mut _STATE_CREATING: bool = false;

#[inline(always)]
pub extern fn init() {
	unsafe {
		_STATES.v = Some(std::vec::Vec::new());
	}
}

#[inline(always)]
pub unsafe extern fn push_state(s: _State) -> bool {
	if _STATE_CREATING {
		return false;
	}

	match _STATES.v {
		None => return false,
		Some(ref mut states) => {
			states.push(s);
			return true;
		}
	}
}

/*
 * Create state.
 */

#[inline(always)]
pub unsafe extern fn is_creating_state() -> bool {
	_STATE_CREATING
}

#[inline(always)]
pub unsafe extern fn start_create_state() -> bool {
	if _STATE_CREATING {
		return false;
	}

	_STATE_CREATING = true;
	_STATE = Some(_State { i: None, f: None, b: None, s: None });
	return true;
}

#[inline(always)]
pub unsafe extern fn state_set_int(value: i64) -> bool {
	if !_STATE_CREATING {
		return false;
	}

	match _STATE {
		None => return false,
		Some(ref mut s) => s.i = Some(value)
	}

	return true;
}

#[inline(always)]
pub unsafe extern fn state_set_float(value: f64) -> bool {
	if !_STATE_CREATING {
		return false;
	}

	match _STATE {
		None => return false,
		Some(ref mut s) => s.f = Some(value)
	}

	return true;
}

#[inline(always)]
pub unsafe extern fn state_set_bool(value: bool) -> bool {
	if !_STATE_CREATING {
		return false;
	}

	match _STATE {
		None => return false,
		Some(ref mut s) => s.b = Some(value)
	}

	return true;
}

#[inline(always)]
pub unsafe extern fn state_add_char(value: i32) -> bool {
	if !_STATE_CREATING {
		return false;
	}

	match _STATE {
		None => return false,
		Some(ref mut s) => {
			match s.s {
				None => {
					s.s = Some(String::from(""));
					return state_add_char(value);
				},

				Some(ref mut ss) => {
					match std::char::from_u32(value as u32) {
						None => {
							println!("Invalid symbol");
							return false;
						},
						Some(ref c) => {
							// F*ck yeah.
							ss.push(*c);
							return true;
						}
					}
				}
			}
		}
	}
}

#[inline(always)]
pub unsafe extern fn state_end() -> bool {
	if !_STATE_CREATING {
		return false;
	}

	match _STATE {
		None => return false,
		Some(ref s) => {
			_STATE_CREATING = false;
			push_state(_State { i: s.i, f: s.f, b: s.b, s: s.s.clone() });
			_STATE = None;
			return true;
		}
	}
}

/*
 * Get state info.
 */

#[inline(always)]
pub unsafe extern fn pop_state() -> bool {
	if _STATE_CREATING {
		return false;
	}

	match _STATES.v {
		None => return false,
		Some(ref mut states) => {
			_STATE = states.pop();
			return true;
		}
	}
}

#[inline(always)]
pub unsafe extern fn state_success() -> bool {
	if _STATE_CREATING {
		return false;
	}

	return _STATE_GET_SUCCESS;
}

#[inline(always)]
pub unsafe extern fn state_int() -> i64 {
	if _STATE_CREATING {
		_STATE_GET_SUCCESS = false;
		return 0;
	}

	match _STATE {
		None => {
			_STATE_GET_SUCCESS = false;
			return 0;
		},

		Some(ref mut s) => {
			match s.i {
				None => {
					_STATE_GET_SUCCESS = false;
					return 0;
				},

				Some(ref i) => {
					_STATE_GET_SUCCESS = true;
					return *i;
				}
			}
		}
	}
}

#[inline(always)]
pub unsafe extern fn state_float() -> f64 {
	if _STATE_CREATING {
		_STATE_GET_SUCCESS = false;
		return 0.0;
	}

	match _STATE {
		None => {
			_STATE_GET_SUCCESS = false;
			return 0.0;
		},

		Some(ref mut s) => {
			match s.f {
				None => {
					_STATE_GET_SUCCESS = false;
					return 0.0;
				},

				Some(ref f) => {
					_STATE_GET_SUCCESS = true;
					return *f;
				}
			}
		}
	}
}

#[inline(always)]
pub unsafe extern fn state_bool() -> bool {
	if _STATE_CREATING {
		_STATE_GET_SUCCESS = false;
		return false;
	}

	match _STATE {
		None => {
			_STATE_GET_SUCCESS = false;
			return false;
		},

		Some(ref mut s) => {
			match s.b {
				None => {
					_STATE_GET_SUCCESS = false;
					return false;
				},

				Some(ref b) => {
					_STATE_GET_SUCCESS = true;
					return *b;
				}
			}
		}
	}
}

#[inline(always)]
pub unsafe extern fn state_string() -> i32 {
	if _STATE_CREATING {
		_STATE_GET_SUCCESS = false;
		return 0;
	}

	match _STATE {
		None => {
			_STATE_GET_SUCCESS = false;
			return -1;
		},

		Some(ref mut s) => {
			match s.s {
				None => {
					_STATE_GET_SUCCESS = false;
					return -1;
				},

				Some(ref mut s) => {
					if _STATE_GET_POS == s.len() {
						_STATE_GET_SUCCESS = true;
						_STATE_GET_END = true;
						return -1;
					}

					if _STATE_GET_END {
						_STATE_GET_POS = 0;
						_STATE_GET_END = false;
					}

					_STATE_GET_SUCCESS = true;
					_STATE_GET_POS += 1;

					return s.as_bytes()[_STATE_GET_POS-1] as i32;
				}
			}
		}
	}
}

#[inline(always)]
pub unsafe extern fn state_string_end() -> bool {
	_STATE_GET_END
}
