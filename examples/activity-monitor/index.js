import init from "./crates/app-renderer/pkg/activity_monitor_app_renderer.js";

async function main() {
  await init("./crates/app-renderer/pkg/activity_monitor_app_renderer_bg.wasm");
}

main();
