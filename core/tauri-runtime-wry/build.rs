// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// creates a cfg alias if `has_feature` is true.
// `alias` must be a snake case string.
fn alias(alias: &str, has_feature: bool) {
  if has_feature {
    println!("cargo:rustc-cfg={alias}");
  }
}

fn main() {
  let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
  let mobile = target_os == "ios" || target_os == "android";
  alias("desktop", !mobile);
  alias("mobile", mobile);

  cfg_aliases::cfg_aliases! {
    // Platforms
    android: { target_os = "android" },
    macos: { target_os = "macos" },
    ios: { target_os = "ios" },
    windows: { target_os = "windows" },
    apple: { any(target_os = "ios", target_os = "macos") },
    linux: { all(unix, not(apple), not(android)) },
    // Backends
    servo: { all(feature = "servo", any(linux, macos)) },
  }
}
