fn main()
{
  // slint_build::compile("ui/main.slint").expect("Failed to compile app-window.slint");
  let config =
    slint_build::CompilerConfiguration::new()
    .with_style("native".into()); // Other options: native, cosmic, cupertino, material, fluent
  slint_build::compile_with_config("ui/main.slint", config).unwrap();
}
