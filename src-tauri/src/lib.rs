/*
 *   Copyright (c) 2023 Nazmul Idris
 *   All rights reserved.
 *
 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
 */

use std::error::Error;
use tauri::{
    menu::{IconMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu},
    Manager,
};
use tauri::{App, Icon, Wry};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// More info on system tray:
/// - <https://tauri.app/v1/guides/features/system-tray/#updating-tray-icon> List of tauri
///
/// 2.0.0-alpha.11 breaking changes section:
/// - <https://github.com/tauri-apps/tauri/pull/7270>
fn setup(app: &mut App<Wry>) -> Result<(), Box<dyn Error>> {
    let handle = app.handle();

    let menu = Menu::with_items(
        handle,
        &[
            &IconMenuItem::new(
                handle,
                "With Icon",
                true,
                Some(Icon::File("icons/icon.png".into())),
                None,
            ),
            &Submenu::with_items(
                handle,
                "Sub",
                true,
                &[&MenuItem::new(handle, "Hello", true, None)],
            )?,
            &PredefinedMenuItem::quit(handle, Some("Quit")),
        ],
    )?;

    // More info: https://github.com/tauri-apps/tauri/issues/7648
    app.manage(menu.clone());

    tauri::tray::TrayIconBuilder::new()
        .icon(Icon::File("icons/icon.png".into()))
        .menu(&menu)
        .on_menu_event(|app_handle, menu_event| {
            println!("menu event: {:?}", menu_event);
            match menu_event.id().as_ref() {
                "quit" => {
                    std::process::exit(0);
                }
                "close" => {
                    app_handle
                        .get_window("main")
                        .unwrap()
                        .emit("close", ())
                        .unwrap();
                }
                _ => {}
            }
        })
        .build(handle)?;

    Ok(())
}
