# Demo
This demo uses the [tauri-nspanel](https://github.com/ahkohd/tauri-nspanel) crate to convert a standard Tauri [WebviewWindow](https://docs.rs/tauri/2.1.1/tauri/webview/struct.WebviewWindow.html) ([NSWindow](https://developer.apple.com/documentation/appkit/nswindow/)) to [NSPanel](https://developer.apple.com/documentation/appkit/nspanel/) that can display over fullscreen window.

```bash
pnpm install

pnpm tauri dev
```

# What you should know

## Remove Window Decorations
Configure the window, set `decorations` and `fullscreen` to `false`:
```json
{
    "decorations": false,
    "fullscreen": false
}

```

## Set Activation Policy (optional)
Set the app's activation policy during startup to auxiliary, this prevents the app icon from showing on the dock.
```rust
    .setup(|app| {
      // Set activation poicy to Accessory to prevent the app icon from showing on the dock
      app.set_activation_policy(tauri::ActivationPolicy::Accessory);

      init(app.app_handle());

      Ok(())
    })
```

## Set Window Level
Raise the panel to the floating window level:
```rust
  // Set the window to float level
  #[allow(non_upper_case_globals)]
  const NSFloatWindowLevel: i32 = 4;
  panel.set_level(NSFloatWindowLevel);
```
You can configure other levels, such as setting the panel above the main menu window level, as long as it is above the normal window level.

## Prevent Panel From Activating The Application
It's important for the panel to activate the application; this is required for the panel to display over other fullscreen windows: 
```rust
  #[allow(non_upper_case_globals)]
  const NSWindowStyleMaskNonActivatingPanel: i32 = 1 << 7;
  // Ensures the panel cannot activate the app
  panel.set_style_mask(NSWindowStyleMaskNonActivatingPanel);
```
## Set Window Collection Behaviour
To make the panel display over fullscreen window we need to make sure it can join all spaces and be on the same space as a full screen window:
```rust
  // Allows the panel to:
  // - display on the same space as the full screen window
  // - join all spaces
  panel.set_collection_behaviour(
    NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary |
    NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
  );
```

### Make the Panel Resizeable
To make the panel resizeable and the resizable window append the resizeable window style mask:
```rust
  #[allow(non_upper_case_globals)]
  const NSWindowStyleMaskNonActivatingPanel: i32 = 1 << 7;
  #[allow(non_upper_case_globals)]
  const NSResizableWindowMask: i32 = 1 << 3;
  
  panel.set_style_mask(NSWindowStyleMaskNonActivatingPanel + NSResizableWindowMask);
```
## Add A Drag Region (optional)
To make the panel dragable, add drag region:
```html
<div data-tauri-drag-region>drag me</div>
```
Add the permission to allow dragging: 
```json
{
  "permissions": [
    "core:window:allow-start-dragging",
  ]
}
```
Now that the panel can be displayed over fullscreen windows, it cannot become fullscreen or be maximised. Therefore, avoid calling `{panel, window}.maximize()` or `{panel, window}.fullscreen()` on this panel, as it will result in a crash.

Due to the use of the drag region, the standard macOS behavior is that double-clicking on the drag region maximizes the window. As mentioned earlier, we can no longer call maximize on this panel, so we need to set permission to disable the maximize toggle.
```json
{
  "permissions": [
    "core:window:deny-internal-toggle-maximize"
  ]
}

```
