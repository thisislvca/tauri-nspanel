mod macros;
pub mod raw_nspanel;

use std::{collections::HashMap, sync::Mutex};

use cocoa::base::id;
use objc_id::ShareId;
use raw_nspanel::NonInteractiveNSPanel;
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

pub type NonInteractivePanel = ShareId<NonInteractiveNSPanel>;

#[derive(Default)]
pub struct Store {
    panels: HashMap<String, ShareId<NonInteractiveNSPanel>>,
}

pub struct NonInteractivePanelManager(pub Mutex<Store>);

impl Default for NonInteractivePanelManager {
    fn default() -> Self {
        Self(Mutex::new(Store::default()))
    }
}

pub trait ManagerExt<R: Runtime> {
    fn get_webview_panel(&self, label: &str) -> Result<ShareId<NonInteractiveNSPanel>, Error>;
}

#[derive(Debug)]
pub enum Error {
    PanelNotFound,
}

impl<R: Runtime, T: Manager<R>> ManagerExt<R> for T {
    fn get_webview_panel(&self, label: &str) -> Result<ShareId<NonInteractiveNSPanel>, Error> {
        let manager = self.state::<self::NonInteractivePanelManager>();
        let manager = manager.0.lock().unwrap();

        match manager.panels.get(label) {
            Some(panel) => Ok(panel.clone()),
            None => Err(Error::PanelNotFound),
        }
    }
}

#[derive(Default)]
pub struct WebviewPanelConfig {
    pub delegate: Option<id>,
}

pub trait WebviewWindowExt<R: Runtime> {
    fn to_panel(&self) -> tauri::Result<ShareId<NonInteractiveNSPanel>>;
}

impl<R: Runtime> WebviewWindowExt<R> for WebviewWindow<R> {
    fn to_panel(&self) -> tauri::Result<ShareId<NonInteractiveNSPanel>> {
        let panel = NonInteractiveNSPanel::from_window(self.to_owned());
        let shared_panel = panel.share();
        let manager = self.state::<self::NonInteractivePanelManager>();

        manager
            .0
            .lock()
            .unwrap()
            .panels
            .insert(self.label().into(), shared_panel.clone());

        Ok(shared_panel)
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("nspanel")
        .setup(|app, _api| {
            app.manage(self::NonInteractivePanelManager::default());

            Ok(())
        })
        .build()
}
