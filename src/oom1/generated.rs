// This file is autogenerated, do not manually edit.

use zbus::dbus_proxy;

/// Proxy object for `org.freedesktop.oom1.Manager`.
#[dbus_proxy(
    interface = "org.freedesktop.oom1.Manager",
    gen_blocking = false,
    default_service = "org.freedesktop.oom1",
    default_path = "/org/freedesktop/oom1"
)]
trait Manager {
    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#DumpByFileDescriptor()) Call interface method `DumpByFileDescriptor`.
    #[dbus_proxy(name = "DumpByFileDescriptor")]
    fn dump_by_file_descriptor(&self) -> crate::zbus::Result<crate::zvariant::OwnedFd>;
}
