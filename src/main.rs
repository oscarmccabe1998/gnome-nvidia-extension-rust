// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/*
 * Name:
 * main.rs
 *
 * Description:
 * Rust Implementation of Nvidia Gnome Extension
 *
 * Made:
 * 12/09/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */

// Modules
mod custom_button;
//use custom_button::CustomButton;
mod subprocess;

use std::ffi::OsStr;

// Imports
use gtk::prelude::*;
use gtk::{
    /* Libraries */ gio,
    /* Application */ Application, ApplicationWindow,
    /* Widgets */ Button,
};
//use std::env;
//use std::path::Path;
//use libappindicator::{
//    /* SysTray */ AppIndicator, AppIndicatorStatus
//};

// Constants
const APP_ID: &str = "org.gtk_rs.NvidiaExtensionRust";

// Main Function
fn main() {
    gtk::init().expect("Failed to initialise gtk");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

// Build Function
fn build_ui(app: &Application) {
    // Button Child 1
    let button1 = Button::builder()
        .label("Open Settings")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    // Connect to "clicked" signal of `button`
    button1.connect_clicked(move |_| {
        println!("Trying to open settings!"); //DEBUG

        // Ideally we should grab if nvidia-settings 'failed' somehow or exited normally
        match subprocess::exec_check(&[OsStr::new("nvidia-settings")], None::<&gio::Cancellable>) {
            Ok(_x) => {
                println!("........yay"); //DEBUG
            }
            Err(_y) => {
                println!("........fak"); //DEBUG
            }
        };
    });
    // Button Child 2
    let button2 = Button::builder()
        .label("Get GPU Names")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    // Connect to "clicked" signal of `button`
    button2.connect_clicked(move |_| {
        println!("Trying to open settings!"); //DEBUG

        // Ideally we should grab if nvidia-settings 'failed' somehow or exited normally
        match subprocess::exec_communicate(
            &[
                OsStr::new("nvidia-settings"),
                OsStr::new("-q"),
                OsStr::new("GpuUUID"),
                OsStr::new("-t"),
            ],
            None,
            None::<&gio::Cancellable>,
        ) {
            Ok(_x) => {
                println!("........yay"); //DEBUG
            }
            Err(_y) => {
                println!("........fak"); //DEBUG
            }
        };
    });

    // Menu Child
    let menu = gio::Menu::new();
    let item = gio::Menu::new();
    item.append(Some("Utilisation"), Some("app.util"));
    item.append(Some("Temperature"), Some("app.temp"));
    item.append(Some("Memory Usage"), Some("app.memo"));
    item.append(Some("Fan Speed"), Some("app.fans"));
    menu.append_submenu(Some("Item 1"), &item);
    menu.append(Some("SMI"), Some("app.smi"));
    menu.append(Some("Settings"), Some("app.settings"));
    app.set_menubar(Some(&menu));

    // App Indicator
    //let mut indicator = AppIndicator::new("Nvidia App", "");
    //indicator.set_status(AppIndicatorStatus::Active);
    //let icon_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources");
    //indicator.set_icon_theme_path(icon_path.to_str().unwrap());
    //indicator.set_icon_full("rust-logo", "icon");
    //indicator.set_menu(&mut menu);

    // Create Parent window
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Nvidia App"));
    window.set_default_size(400, 400);
    window.set_show_menubar(true);

    // Add children to window
    window.set_child(Some(&button1));
    //window.set_child(Some(&button2));

    // Present window
    window.show();
}
