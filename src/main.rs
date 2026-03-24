// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError>
{
  let main_window = MainWindow::new()?;

  // NOTE: this is one way for interaction between the front- and backend. There are other options tho.
  let bridge = main_window.global::<Bridge>();
  bridge.on_operation(|val|
  {
    println!("operation input: {val}");
    val * 2
  });
  bridge.set_value(42);

  bridge.invoke_operation(bridge.get_value());

  main_window.run()
}
