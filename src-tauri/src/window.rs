use std::sync::OnceLock;

use bevy::prelude::*;
use bevy::winit::WinitWindows;
use bevy::window::{EnabledButtons, PrimaryWindow, WindowResized};
use wry::raw_window_handle::{HasWindowHandle, RawWindowHandle};


static TAURI: OnceLock<tauri::AppHandle> = OnceLock::new();


pub fn register(app: &mut App) {{
  app
    .add_systems(Startup, setup)
    .add_systems(PreUpdate, update);
};}


fn setup(
  winit_windows: NonSend<WinitWindows>,
  window_query: Single<(Entity, Mut<Window>), With<PrimaryWindow>>,
) {
  let (entity, mut window) = window_query.into_inner();

  window.resizable = false;

  window.enabled_buttons = EnabledButtons {
    maximize: false,
    ..Default::default()
  };

  let Some(window) = winit_windows.get_window(entity)
    else { println!("no window"); return; };

  let RawWindowHandle::Win32(handle) = window.window_handle().unwrap().as_raw()
    else { println!("not windows window"); return; };

  let size = window.inner_size();
  let hwnd = handle.hwnd;

  std::thread::spawn(move || {
    tauri::Builder::default()
      .plugin(tauri_plugin_opener::init())
      .any_thread()
      .invoke_handler(tauri::generate_handler![])
      .setup(move |app| {
        use tauri::*;

        TAURI.set(app.handle().to_owned()).unwrap();

        WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
          .parent_raw(unsafe { std::mem::transmute(hwnd) })
          .additional_browser_args("--disable-gpu")
          .position(0.0, 0.0)
          .decorations(false)
          .shadow(false)
          .transparent(true)
          .skip_taskbar(true)
          .resizable(false)
          .closable(false)
          // .visible(false)
          .build()?
          .set_size(size)?;

        Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
  });
}


#[allow(dead_code)]
pub fn get_window() -> Option<tauri::WebviewWindow> {
  use tauri::Manager;
  TAURI.get()?.get_webview_window("main")
}


fn update(wq: Option<Single<Entity, With<PrimaryWindow>>>, mut ev: EventReader<WindowResized>) {
  let Some(wq) = wq else { return; };

  let w = wq.into_inner();

  for e in ev.read() {
    if w != e.window {
      continue;
    }

    if let Some(window) = get_window() {
      window.set_size(tauri::PhysicalSize {
        width : e.width,
        height: e.height,
      }).ok();
    }
  }
}
