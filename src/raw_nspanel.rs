use cocoa::{
    appkit::NSWindowCollectionBehavior,
    base::{id, nil, BOOL, YES},
};

use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{self, Class, Object, Sel},
    sel, sel_impl, Message,
};
use objc_foundation::INSObject;
use objc_id::{Id, ShareId};
use tauri::{Runtime, WebviewWindow};

extern "C" {
    pub fn object_setClass(obj: id, cls: id) -> id;
}

const CLS_NAME: &str = "NonInteractiveNSPanel";

pub struct NonInteractiveNSPanel;

unsafe impl Sync for NonInteractiveNSPanel {}
unsafe impl Send for NonInteractiveNSPanel {}

impl INSObject for NonInteractiveNSPanel {
    fn class() -> &'static runtime::Class {
        Class::get(CLS_NAME).unwrap_or_else(Self::define_class)
    }
}

impl NonInteractiveNSPanel {
    /// Returns YES to ensure that RawNSPanel can become a key window
    extern "C" fn can_become_key_window(_: &Object, _: Sel) -> BOOL {
        YES // [UPDATED COMMENT] if "NO", the panel becomes a key window only when you click on it (previous behavior)
    }

    extern "C" fn dealloc(this: &mut Object, _cmd: Sel) {
        unsafe {
            let superclass = class!(NSObject);
            let dealloc: extern "C" fn(&mut Object, Sel) =
                msg_send![super(this, superclass), dealloc];
            dealloc(this, _cmd);
        }
    }

    fn define_class() -> &'static Class {
        let mut cls = ClassDecl::new(CLS_NAME, class!(NSPanel))
            .unwrap_or_else(|| panic!("Unable to register {} class", CLS_NAME));

        unsafe {
            cls.add_method(
                sel!(canBecomeKeyWindow),
                Self::can_become_key_window as extern "C" fn(&Object, Sel) -> BOOL,
            );

            cls.add_method(
                sel!(dealloc),
                Self::dealloc as extern "C" fn(&mut Object, Sel), // not needed anymore
            );
        }

        cls.register()
    }

    pub fn show(&self) {
        self.make_first_responder(Some(self.content_view()));
        self.order_front_regardless();
        self.make_key_window(); // technically not needed, not sure why it's here. This would make the panel key when showing it
                                // meaning the keyboard focus would move to this panel even though the mouse hasn't entered it yet (aka the user hasn't acknowledged the panel yet)
                                // comment this out if creating window at runtime instead of during app launch (I'm currently creating the win as hidden and showing it only when needed)
    }

    pub fn is_visible(&self) -> bool {
        let flag: BOOL = unsafe { msg_send![self, isVisible] };
        flag == YES
    }

    pub fn is_floating_panel(&self) -> bool {
        let flag: BOOL = unsafe { msg_send![self, isFloatingPanel] };
        flag == YES
    }

    pub fn make_key_window(&self) {
        let _: () = unsafe { msg_send![self, makeKeyWindow] };
    }

    pub fn resign_key_window(&self) {
        let _: () = unsafe { msg_send![self, resignKeyWindow] };
    }

    pub fn make_key_and_order_front(&self, sender: Option<id>) {
        let _: () = unsafe { msg_send![self, makeKeyAndOrderFront: sender.unwrap_or(nil)] };
    }

    pub fn order_front_regardless(&self) {
        let _: () = unsafe { msg_send![self, orderFrontRegardless] };
    }

    pub fn order_out(&self, sender: Option<id>) {
        let _: () = unsafe { msg_send![self, orderOut: sender.unwrap_or(nil)] };
    }

    pub fn content_view(&self) -> id {
        unsafe { msg_send![self, contentView] }
    }

    pub fn make_first_responder(&self, sender: Option<id>) {
        if let Some(responder) = sender {
            let _: () = unsafe { msg_send![self, makeFirstResponder: responder] };
        } else {
            let _: () = unsafe { msg_send![self, makeFirstResponder: self] };
        }
    }

    pub fn set_level(&self, level: i32) {
        let _: () = unsafe { msg_send![self, setLevel: level] };
    }

    pub fn set_alpha_value(&self, value: f64) {
        let _: () = unsafe { msg_send![self, setAlphaValue: value] };
    }

    pub fn set_content_size(&self, width: f64, height: f64) {
        let _: () = unsafe { msg_send![self, setContentSize: (width, height)] };
    }

    pub fn set_style_mask(&self, style_mask: i32) {
        let _: () = unsafe { msg_send![self, setStyleMask: style_mask] };
    }

    pub fn set_collection_behaviour(&self, behaviour: NSWindowCollectionBehavior) {
        let _: () = unsafe { msg_send![self, setCollectionBehavior: behaviour] };
    }

    pub fn set_delegate<T>(&self, delegate: Id<T>) {
        let _: () = unsafe { msg_send![self, setDelegate: delegate] };
    }

    pub fn set_floating_panel(&self, value: bool) {
        let _: () = unsafe { msg_send![self, setFloatingPanel: value] };
    }

    pub fn set_accepts_mouse_moved_events(&self, value: bool) {
        let _: () = unsafe { msg_send![self, setAcceptsMouseMovedEvents: value] };
    }

    pub fn set_ignore_mouse_events(&self, value: bool) {
        let _: () = unsafe { msg_send![self, setIgnoresMouseEvents: value] };
    }

    pub fn set_hides_on_deactivate(&self, value: bool) {
        let _: () = unsafe { msg_send![self, setHidesOnDeactivate: value] };
    }

    pub fn activate(&self) {
        // Configure panel for interaction
        self.set_becomes_key_only_if_needed(true);
        self.set_works_when_modal(true);
        self.set_hides_on_deactivate(false);

        // Make the window visible and activated with higher window level
        self.set_level(3); // NSFloatingWindowLevel - alternative to set_floating_panel(true)... not sure which to use, nothing changes rn
                           // also this does the same as order_front_regardless, so one of the two could be removed
        self.order_front_regardless();
        self.make_key_and_order_front(None);
        // self.set_floating_panel(true); // maybe this is needed instead? not sure
        self.make_key_window(); // same as the above make_key_window() comment

        // Set first responder to ensure focus
        self.make_first_responder(Some(self.content_view()));
    }

    pub fn set_moveable_by_window_background(&self, value: bool) {
        let _: () = unsafe { msg_send![self, setMovableByWindowBackground: value] };
    }

    pub fn set_becomes_key_only_if_needed(&self, value: bool) {
        let _: () = unsafe { msg_send![self, setBecomesKeyOnlyIfNeeded: value] };
    }

    pub fn set_works_when_modal(&self, value: bool) {
        let _: () = unsafe { msg_send![self, setWorksWhenModal: value] };
    }

    pub fn set_opaque(&self, value: bool) {
        let _: () = unsafe { msg_send![self, setOpaque: value] };
    }

    pub fn set_has_shadow(&self, value: bool) {
        let _: () = unsafe { msg_send![self, setHasShadow: value] };
    }

    pub fn set_released_when_closed(&self, value: bool) {
        let _: () = unsafe { msg_send![self, setReleasedWhenClosed: value] };
    }

    pub fn close(&self) {
        let _: () = unsafe { msg_send![self, close] };
    }

    // not sure when the heck this is needed
    pub fn handle(&mut self) -> ShareId<Self> {
        unsafe { ShareId::from_ptr(self as *mut Self) }
    }

    /// Create an NSPanel from a Tauri Webview Window
    pub fn from_window<R: Runtime>(window: WebviewWindow<R>) -> Id<Self> {
        let nswindow: id = window.ns_window().unwrap() as _;
        let nspanel_class: id = unsafe { msg_send![Self::class(), class] };
        unsafe {
            object_setClass(nswindow, nspanel_class);
            let panel = Id::from_retained_ptr(nswindow as *mut NonInteractiveNSPanel);

            // Configure panel to maintain focus - do this immediately
            panel.set_hides_on_deactivate(false);
            panel.set_works_when_modal(true);

            // Set to floating window level for better focus retention
            panel.set_level(3); // NSFloatingWindowLevel = 3

            // panel.make_key_window(); // Make it the key window initially - not needed rn, see comments above

            panel
        }
    }
}

unsafe impl Message for NonInteractiveNSPanel {}
