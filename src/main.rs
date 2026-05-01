// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::needless_return)]

use crate::bridge::BridgeBackend;

pub(crate) mod bridge;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError>
{
  let main_window = MainWindow::new()?;

  // NOTE: this is one way for interaction between the front- and backend. There are other options tho.
  let bridge = main_window.global::<BridgeUI>();
  let bridge = BridgeBackend::new(&bridge);
  let bridge = main_window.global::<BridgeUI>();
  bridge.on_operation(|value|
  {
    println!("operation input: {value}");
    return value * 2;
  });
  bridge.set_value(bridge.invoke_operation(42));

  bridge.invoke_operation(bridge.get_value());

  main_window.run()
}
