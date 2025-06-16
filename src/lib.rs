mod macros;
pub mod raw_nspanel;

use std::{cell::RefCell, collections::HashMap, sync::Mutex};

use cocoa::base::id;
use objc2::{rc::Retained, MainThreadMarker};

use objc2::MainThreadOnly;

use objc2_app_kit::NSPanel;
use objc_id::ShareId;
use raw_nspanel::RawNSPanel;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime, WebviewWindow,
};

pub extern crate block;
pub extern crate cocoa;
pub extern crate objc;
pub extern crate objc_foundation;
pub extern crate objc_id;
pub extern crate tauri;
type Panel = Retained<NSPanel>;

struct Store {
    panels: HashMap<String, Panel>,
}

thread_local! {
    static PANEL_STORE: RefCell<Store> = RefCell::new(Store { panels: HashMap::new() });
}

pub trait ManagerExt<R: Runtime> {
    fn get_webview_panel(&self, label: &str) -> Result<Retained<NSPanel>, Error>;
}

#[derive(Debug)]
pub enum Error {
    PanelNotFound,
}

impl<R: Runtime, T: Manager<R>> ManagerExt<R> for T {
    fn get_webview_panel(&self, label: &str) -> Result<Retained<NSPanel>, Error> {
        PANEL_STORE.with(|store| {
            store
                .borrow_mut()
                .panels
                .get(label)
                .cloned()
                .ok_or(Error::PanelNotFound)
        })
    }
}

#[derive(Default)]
pub struct WebviewPanelConfig {
    pub delegate: Option<id>,
}

pub trait WebviewWindowExt<R: Runtime> {
    fn to_panel(&self) -> tauri::Result<Retained<NSPanel>>;
}

impl<R: Runtime> WebviewWindowExt<R> for WebviewWindow<R> {
    fn to_panel(&self) -> tauri::Result<Retained<NSPanel>> {
        let panel = NSPanel::from_window(self.to_owned());
        let shared_panel = panel.share();

        PANEL_STORE.with(|store| {
            store
                .borrow_mut()
                .panels
                .insert(self.label().into(), shared_panel.clone());
        });

        Ok(shared_panel)
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("nspanel")
        .setup(|app, _api| {
            let marker = MainThreadMarker::from(app);
            app.manage(MainThreadOnly::new_with_marker(Store::default(), marker));

            Ok(())
        })
        .build()
}
