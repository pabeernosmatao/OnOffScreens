use crate::lang;
use std::process::Command;

pub fn send(title: &str, message: &str) {
    let _ = Command::new("notify-send")
        .arg("-a")
        .arg("OnOffScreens")
        .arg(title)
        .arg(message)
        .status();
}

pub fn profile_applied(name: &str) {
    let strings = lang::strings(lang::current());
    send(
        strings.notification_title,
        &format!("{}: {name}", strings.profile_applied),
    );
}

pub fn profile_failed(message: &str) {
    let strings = lang::strings(lang::current());
    send(strings.profile_failed, message);
}

pub fn reconfigured() {
    let strings = lang::strings(lang::current());
    send(strings.notification_title, strings.reconfigured);
}

pub fn reconfigure_failed(message: &str) {
    let strings = lang::strings(lang::current());
    send(strings.reconfigure_failed, message);
}
