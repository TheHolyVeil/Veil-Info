use sysinfo::System;

slint::include_modules!();

fn format_uptime(secs: u64) -> String {
    let hours = secs / 3600;
    let mins = (secs % 3600) / 60;
    format!("{hours}h {mins}m")
}

fn main() -> Result<(), slint::PlatformError> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let os_name = System::name().unwrap_or_else(|| "VeilOS".into());
    let kernel = System::kernel_version().unwrap_or_else(|| "unknown".into());
    let hostname = System::host_name().unwrap_or_else(|| "unknown".into());
    let uptime = format_uptime(System::uptime());

    // Package count: VeilOS is Arch-based, so shell out to pacman.
    // Kept simple here; swap for a direct query against the pacman db
    // if startup latency ever matters.
    let pkg_count = std::process::Command::new("pacman")
        .args(["-Qq"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).lines().count().to_string())
        .unwrap_or_else(|| "-".into());

    // TODO: replace with a real query against veil-cli / veil-compositor's
    // IPC socket once that's exposed. Placeholder values for now so the
    // page renders something during UI development.
    let session_mode = "windowed".to_string();
    let encode_tier = "kitty".to_string();
    let output_count = "1".to_string();
    let compositor_version = "dev".to_string();

    let ui = AppWindow::new()?;
    ui.set_os_name(os_name.into());
    ui.set_kernel(kernel.into());
    ui.set_hostname(hostname.into());
    ui.set_uptime(uptime.into());
    ui.set_pkg_count(pkg_count.into());

    ui.set_session_mode(session_mode.into());
    ui.set_encode_tier(encode_tier.into());
    ui.set_output_count(output_count.into());
    ui.set_compositor_version(compositor_version.into());

    ui.run()
}
