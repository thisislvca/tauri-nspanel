# Demo
[tauri-nspanel](https://github.com/ahkohd/tauri-nspanel) plugin converts a standard Tauri [WebviewWindow](https://docs.rs/tauri/2.1.1/tauri/webview/struct.WebviewWindow.html) ([NSWindow](https://developer.apple.com/documentation/appkit/nswindow/)) to [NSPanel](https://developer.apple.com/documentation/appkit/nspanel/) that can display over fullscreen window.

To run the demo:
```bash
pnpm install

pnpm tauri dev
```

# What you should know

## Remove Window Decorations
Configure the window by setting `decorations` and `fullscreen` to false:

[tauri-config.json](https://github.com/ahkohd/tauri-nspanel/blob/754f9b0fdf39511a839280b6b9a418ff51630acc/examples/fullscreen/src-tauri/tauri.conf.json#L52)

```json
{
    "decorations": false,
    "fullscreen": false
}
```

## Set Activation Policy (Optional)
Set the app's activation policy to auxiliary during startup; this prevents the app icon from appearing in the dock:

[main.rs](https://github.com/ahkohd/tauri-nspanel/blob/be8ba6c71e03cd115536bbb74eccc42df3d52ba6/examples/fullscreen/src-tauri/src/main.rs#L19)

```rust
    .setup(|app| {
      // Set activation poicy to Accessory to prevent the app icon from showing on the dock
      app.set_activation_policy(tauri::ActivationPolicy::Accessory);

      init(app.app_handle());

      Ok(())
    })
```

## Set Window Level
Raise the panel to floating window level:

[main.rs](https://github.com/ahkohd/tauri-nspanel/blob/be8ba6c71e03cd115536bbb74eccc42df3d52ba6/examples/fullscreen/src-tauri/src/main.rs#L58)

```rust
  // Set the window to float level
  #[allow(non_upper_case_globals)]
  const NSFloatWindowLevel: i32 = 4;

  panel.set_level(NSFloatWindowLevel);
```
You can set to other levels as long as it is above the normal window level, for example, set the panel above the main menu window level:
```rust
  use cocoa::appkit::NSMainMenuWindowLevel;

  // this level is recommend for a spotlight panel
  panel.set_level(NSMainMenuWindowLevel + 1);
```

## Prevent Panel From Activating The Application
It's required to prevent the panel from activating the application to display over a fullscreen window:

[main.rs](https://github.com/ahkohd/tauri-nspanel/blob/be8ba6c71e03cd115536bbb74eccc42df3d52ba6/examples/fullscreen/src-tauri/src/main.rs#L63)

```rust
  #[allow(non_upper_case_globals)]
  const NSWindowStyleMaskNonActivatingPanel: i32 = 1 << 7;
  // Ensures the panel cannot activate the app
  panel.set_style_mask(NSWindowStyleMaskNonActivatingPanel);
```
## Set Window Collection Behaviour
To display the panel over a fullscreen window, we need to ensure it can join all spaces and be in the same space as the fullscreen window:

[main.rs](https://github.com/ahkohd/tauri-nspanel/blob/be8ba6c71e03cd115536bbb74eccc42df3d52ba6/examples/fullscreen/src-tauri/src/main.rs#L68)

```rust
  // Allows the panel to:
  // - display on the same space as the full screen window
  // - join all spaces
  panel.set_collection_behaviour(
    NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary |
    NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
  );
```

## Make The Panel Resizeable
To make the panel resizable, append the appropriate window style mask to the panel:

[main.rs](https://github.com/ahkohd/tauri-nspanel/blob/be8ba6c71e03cd115536bbb74eccc42df3d52ba6/examples/fullscreen/src-tauri/src/main.rs#L63)

```rust
  #[allow(non_upper_case_globals)]
  const NSWindowStyleMaskNonActivatingPanel: i32 = 1 << 7;
  #[allow(non_upper_case_globals)]
  const NSResizableWindowMask: i32 = 1 << 3;
  
  panel.set_style_mask(NSWindowStyleMaskNonActivatingPanel + NSResizableWindowMask);
```
## Add A Drag Region (Optional)
To make the panel dragable, setup a drag region:

[index.html](https://github.com/ahkohd/tauri-nspanel/blob/c2d3dd072fdb40d9fdaf5267eeb967e314b1151a/examples/fullscreen/public/index.html#L33)

```html
<div data-tauri-drag-region>drag me</div>
```

Add the permission to allow dragging: 

[migrated.json](https://github.com/ahkohd/tauri-nspanel/blob/c2d3dd072fdb40d9fdaf5267eeb967e314b1151a/examples/fullscreen/src-tauri/capabilities/migrated.json#L10)

```json
{
  "permissions": [
    "core:window:allow-start-dragging",
  ]
}
```
Now that the panel can be displayed over fullscreen windows, it cannot become fullscreen or be maximised. Therefore, avoid calling `{panel, window}.maximize()` or `{panel, window}.fullscreen()` on this panel, as it will result in a crash.

Due to the use of the drag region, the standard macOS behavior is that double-clicking on the drag region maximizes the window. As mentioned earlier, we can no longer call maximize on this panel, so we need to set permission to disable the maximize toggle.

[migrated.json](https://github.com/ahkohd/tauri-nspanel/blob/c2d3dd072fdb40d9fdaf5267eeb967e314b1151a/examples/fullscreen/src-tauri/capabilities/migrated.json#L11)

```json
{
  "permissions": [
    "core:window:deny-internal-toggle-maximize"
  ]
}

```
