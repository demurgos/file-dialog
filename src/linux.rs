use detect_desktop_environment::DesktopEnvironment;

fn is_qt_preferred() -> bool {
  match DesktopEnvironment::detect() {
    DesktopEnvironment::Kde => true,
    DesktopEnvironment::Lxqt => true,
    _ => false,
  }
}
