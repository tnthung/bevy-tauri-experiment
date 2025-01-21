mod tauri;

use tap::Tap;
use bevy::prelude::*;


#[cfg_attr(mobile, ::tauri::mobile_entry_point)]
pub fn run() {
  App::new()
    .add_plugins(DefaultPlugins)
    .tap_borrow_mut(tauri::register)
    .run();
}
