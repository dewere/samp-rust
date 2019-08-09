mod internal;

#[no_mangle]
pub extern fn start() {
  internal::init();

  unsafe {
    internal::push_state(internal::_State {
      i: None,
      f: None,
      b: Some(true),
      s: None
    });
  }

  println!("Hello from library.");
}

/*
 * BRIDGE TO INTERNAL API.
 * MANUAL USAGE IS NOT RECOMMEND.
 */

#[no_mangle]
pub unsafe extern fn push_state(s: internal::_State) -> bool {
  return internal::push_state(s);
}

#[no_mangle]
pub unsafe extern fn is_creating_state() -> bool {
  return internal::is_creating_state();
}

#[no_mangle]
pub unsafe extern fn start_create_state() -> bool {
  return internal::start_create_state();
}

#[no_mangle]
pub unsafe extern fn state_set_int(value: i64) -> bool {
  return internal::state_set_int(value);
}

#[no_mangle]
pub unsafe extern fn state_set_float(value: f64) -> bool {
  return internal::state_set_float(value);
}

#[no_mangle]
pub unsafe extern fn state_set_bool(value: bool) -> bool {
  return internal::state_set_bool(value);
}

#[no_mangle]
pub unsafe extern fn state_add_char(value: i32) -> bool {
  return internal::state_add_char(value);
}

#[no_mangle]
pub unsafe extern fn state_end() -> bool {
  return internal::state_end();
}

#[no_mangle]
pub unsafe extern fn pop_state() -> bool {
  return internal::pop_state();
}

#[no_mangle]
pub unsafe extern fn state_success() -> bool {
  return internal::state_success();
}

#[no_mangle]
pub unsafe extern fn state_int() -> i64 {
  return internal::state_int();
}

#[no_mangle]
pub unsafe extern fn state_float() -> f64 {
  return internal::state_float();
}

#[no_mangle]
pub unsafe extern fn state_bool() -> bool {
  return internal::state_bool();
}

#[no_mangle]
pub unsafe extern fn state_string() -> i32 {
  return internal::state_string();
}

#[no_mangle]
pub unsafe extern fn state_string_end() -> bool {
  return internal::state_string_end();
}
