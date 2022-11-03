// This file is autogenerated, do not manually edit.

use zbus::dbus_proxy;

/// Proxy object for `org.freedesktop.login1.Manager`.
#[dbus_proxy(
    interface = "org.freedesktop.login1.Manager",
    gen_blocking = false,
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1"
)]
trait Manager {
    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetSession()) Call interface method `GetSession`.
    #[dbus_proxy(name = "GetSession")]
    fn get_session(
        &self,
        session_id: String,
    ) -> crate::zbus::Result<crate::zvariant::OwnedObjectPath>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetSessionByPID()) Call interface method `GetSessionByPID`.
    #[dbus_proxy(name = "GetSessionByPID")]
    fn get_session_by_pid(&self, pid: u32)
        -> crate::zbus::Result<crate::zvariant::OwnedObjectPath>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetUser()) Call interface method `GetUser`.
    #[dbus_proxy(name = "GetUser")]
    fn get_user(&self, uid: u32) -> crate::zbus::Result<crate::zvariant::OwnedObjectPath>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetUserByPID()) Call interface method `GetUserByPID`.
    #[dbus_proxy(name = "GetUserByPID")]
    fn get_user_by_pid(&self, pid: u32) -> crate::zbus::Result<crate::zvariant::OwnedObjectPath>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetSeat()) Call interface method `GetSeat`.
    #[dbus_proxy(name = "GetSeat")]
    fn get_seat(&self, seat_id: String) -> crate::zbus::Result<crate::zvariant::OwnedObjectPath>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ListSessions()) Call interface method `ListSessions`.
    #[dbus_proxy(name = "ListSessions")]
    fn list_sessions(
        &self,
    ) -> crate::zbus::Result<
        Vec<(
            String,
            u32,
            String,
            String,
            crate::zvariant::OwnedObjectPath,
        )>,
    >;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ListUsers()) Call interface method `ListUsers`.
    #[dbus_proxy(name = "ListUsers")]
    fn list_users(
        &self,
    ) -> crate::zbus::Result<Vec<(u32, String, crate::zvariant::OwnedObjectPath)>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ListSeats()) Call interface method `ListSeats`.
    #[dbus_proxy(name = "ListSeats")]
    fn list_seats(&self) -> crate::zbus::Result<Vec<(String, crate::zvariant::OwnedObjectPath)>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ListInhibitors()) Call interface method `ListInhibitors`.
    #[dbus_proxy(name = "ListInhibitors")]
    fn list_inhibitors(
        &self,
    ) -> crate::zbus::Result<Vec<(String, String, String, String, u32, u32)>>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CreateSession()) Call interface method `CreateSession`.
    #[dbus_proxy(name = "CreateSession")]
    fn create_session(
        &self,
        uid: u32,
        pid: u32,
        service: String,
        typelabel: String,
        class: String,
        desktop: String,
        seat_id: String,
        vtnr: u32,
        tty: String,
        display: String,
        remote: bool,
        remote_user: String,
        remote_host: String,
        properties: Vec<(String, crate::zvariant::OwnedValue)>,
    ) -> crate::zbus::Result<(
        String,
        crate::zvariant::OwnedObjectPath,
        String,
        crate::zvariant::OwnedFd,
        u32,
        String,
        u32,
        bool,
    )>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ReleaseSession()) Call interface method `ReleaseSession`.
    #[dbus_proxy(name = "ReleaseSession")]
    fn release_session(&self, session_id: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ActivateSession()) Call interface method `ActivateSession`.
    #[dbus_proxy(name = "ActivateSession")]
    fn activate_session(&self, session_id: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ActivateSessionOnSeat()) Call interface method `ActivateSessionOnSeat`.
    #[dbus_proxy(name = "ActivateSessionOnSeat")]
    fn activate_session_on_seat(
        &self,
        session_id: String,
        seat_id: String,
    ) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#LockSession()) Call interface method `LockSession`.
    #[dbus_proxy(name = "LockSession")]
    fn lock_session(&self, session_id: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#UnlockSession()) Call interface method `UnlockSession`.
    #[dbus_proxy(name = "UnlockSession")]
    fn unlock_session(&self, session_id: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#LockSessions()) Call interface method `LockSessions`.
    #[dbus_proxy(name = "LockSessions")]
    fn lock_sessions(&self) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#UnlockSessions()) Call interface method `UnlockSessions`.
    #[dbus_proxy(name = "UnlockSessions")]
    fn unlock_sessions(&self) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#KillSession()) Call interface method `KillSession`.
    #[dbus_proxy(name = "KillSession")]
    fn kill_session(
        &self,
        session_id: String,
        who: String,
        signal_number: i32,
    ) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#KillUser()) Call interface method `KillUser`.
    #[dbus_proxy(name = "KillUser")]
    fn kill_user(&self, uid: u32, signal_number: i32) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#TerminateSession()) Call interface method `TerminateSession`.
    #[dbus_proxy(name = "TerminateSession")]
    fn terminate_session(&self, session_id: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#TerminateUser()) Call interface method `TerminateUser`.
    #[dbus_proxy(name = "TerminateUser")]
    fn terminate_user(&self, uid: u32) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#TerminateSeat()) Call interface method `TerminateSeat`.
    #[dbus_proxy(name = "TerminateSeat")]
    fn terminate_seat(&self, seat_id: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetUserLinger()) Call interface method `SetUserLinger`.
    #[dbus_proxy(name = "SetUserLinger")]
    fn set_user_linger(&self, uid: u32, enable: bool, interactive: bool)
        -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#AttachDevice()) Call interface method `AttachDevice`.
    #[dbus_proxy(name = "AttachDevice")]
    fn attach_device(
        &self,
        seat_id: String,
        sysfs_path: String,
        interactive: bool,
    ) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#FlushDevices()) Call interface method `FlushDevices`.
    #[dbus_proxy(name = "FlushDevices")]
    fn flush_devices(&self, interactive: bool) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#PowerOff()) Call interface method `PowerOff`.
    #[dbus_proxy(name = "PowerOff")]
    fn power_off(&self, interactive: bool) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#PowerOffWithFlags()) Call interface method `PowerOffWithFlags`.
    #[dbus_proxy(name = "PowerOffWithFlags")]
    fn power_off_with_flags(&self, flags: u64) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Reboot()) Call interface method `Reboot`.
    #[dbus_proxy(name = "Reboot")]
    fn reboot(&self, interactive: bool) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#RebootWithFlags()) Call interface method `RebootWithFlags`.
    #[dbus_proxy(name = "RebootWithFlags")]
    fn reboot_with_flags(&self, flags: u64) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Halt()) Call interface method `Halt`.
    #[dbus_proxy(name = "Halt")]
    fn halt(&self, interactive: bool) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#HaltWithFlags()) Call interface method `HaltWithFlags`.
    #[dbus_proxy(name = "HaltWithFlags")]
    fn halt_with_flags(&self, flags: u64) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Suspend()) Call interface method `Suspend`.
    #[dbus_proxy(name = "Suspend")]
    fn suspend(&self, interactive: bool) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SuspendWithFlags()) Call interface method `SuspendWithFlags`.
    #[dbus_proxy(name = "SuspendWithFlags")]
    fn suspend_with_flags(&self, flags: u64) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Hibernate()) Call interface method `Hibernate`.
    #[dbus_proxy(name = "Hibernate")]
    fn hibernate(&self, interactive: bool) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#HibernateWithFlags()) Call interface method `HibernateWithFlags`.
    #[dbus_proxy(name = "HibernateWithFlags")]
    fn hibernate_with_flags(&self, flags: u64) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#HybridSleep()) Call interface method `HybridSleep`.
    #[dbus_proxy(name = "HybridSleep")]
    fn hybrid_sleep(&self, interactive: bool) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#HybridSleepWithFlags()) Call interface method `HybridSleepWithFlags`.
    #[dbus_proxy(name = "HybridSleepWithFlags")]
    fn hybrid_sleep_with_flags(&self, flags: u64) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SuspendThenHibernate()) Call interface method `SuspendThenHibernate`.
    #[dbus_proxy(name = "SuspendThenHibernate")]
    fn suspend_then_hibernate(&self, interactive: bool) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SuspendThenHibernateWithFlags()) Call interface method `SuspendThenHibernateWithFlags`.
    #[dbus_proxy(name = "SuspendThenHibernateWithFlags")]
    fn suspend_then_hibernate_with_flags(&self, flags: u64) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CanPowerOff()) Call interface method `CanPowerOff`.
    #[dbus_proxy(name = "CanPowerOff")]
    fn can_power_off(&self) -> crate::zbus::Result<String>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CanReboot()) Call interface method `CanReboot`.
    #[dbus_proxy(name = "CanReboot")]
    fn can_reboot(&self) -> crate::zbus::Result<String>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CanHalt()) Call interface method `CanHalt`.
    #[dbus_proxy(name = "CanHalt")]
    fn can_halt(&self) -> crate::zbus::Result<String>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CanSuspend()) Call interface method `CanSuspend`.
    #[dbus_proxy(name = "CanSuspend")]
    fn can_suspend(&self) -> crate::zbus::Result<String>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CanHibernate()) Call interface method `CanHibernate`.
    #[dbus_proxy(name = "CanHibernate")]
    fn can_hibernate(&self) -> crate::zbus::Result<String>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CanHybridSleep()) Call interface method `CanHybridSleep`.
    #[dbus_proxy(name = "CanHybridSleep")]
    fn can_hybrid_sleep(&self) -> crate::zbus::Result<String>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CanSuspendThenHibernate()) Call interface method `CanSuspendThenHibernate`.
    #[dbus_proxy(name = "CanSuspendThenHibernate")]
    fn can_suspend_then_hibernate(&self) -> crate::zbus::Result<String>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ScheduleShutdown()) Call interface method `ScheduleShutdown`.
    #[dbus_proxy(name = "ScheduleShutdown")]
    fn schedule_shutdown(&self, typelabel: String, usec: u64) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CancelScheduledShutdown()) Call interface method `CancelScheduledShutdown`.
    #[dbus_proxy(name = "CancelScheduledShutdown")]
    fn cancel_scheduled_shutdown(&self) -> crate::zbus::Result<bool>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Inhibit()) Call interface method `Inhibit`.
    #[dbus_proxy(name = "Inhibit")]
    fn inhibit(
        &self,
        what: String,
        who: String,
        why: String,
        mode: String,
    ) -> crate::zbus::Result<crate::zvariant::OwnedFd>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CanRebootParameter()) Call interface method `CanRebootParameter`.
    #[dbus_proxy(name = "CanRebootParameter")]
    fn can_reboot_parameter(&self) -> crate::zbus::Result<String>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetRebootParameter()) Call interface method `SetRebootParameter`.
    #[dbus_proxy(name = "SetRebootParameter")]
    fn set_reboot_parameter(&self, parameter: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CanRebootToFirmwareSetup()) Call interface method `CanRebootToFirmwareSetup`.
    #[dbus_proxy(name = "CanRebootToFirmwareSetup")]
    fn can_reboot_to_firmware_setup(&self) -> crate::zbus::Result<String>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetRebootToFirmwareSetup()) Call interface method `SetRebootToFirmwareSetup`.
    #[dbus_proxy(name = "SetRebootToFirmwareSetup")]
    fn set_reboot_to_firmware_setup(&self, enable: bool) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CanRebootToBootLoaderMenu()) Call interface method `CanRebootToBootLoaderMenu`.
    #[dbus_proxy(name = "CanRebootToBootLoaderMenu")]
    fn can_reboot_to_boot_loader_menu(&self) -> crate::zbus::Result<String>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetRebootToBootLoaderMenu()) Call interface method `SetRebootToBootLoaderMenu`.
    #[dbus_proxy(name = "SetRebootToBootLoaderMenu")]
    fn set_reboot_to_boot_loader_menu(&self, timeout: u64) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#CanRebootToBootLoaderEntry()) Call interface method `CanRebootToBootLoaderEntry`.
    #[dbus_proxy(name = "CanRebootToBootLoaderEntry")]
    fn can_reboot_to_boot_loader_entry(&self) -> crate::zbus::Result<String>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetRebootToBootLoaderEntry()) Call interface method `SetRebootToBootLoaderEntry`.
    #[dbus_proxy(name = "SetRebootToBootLoaderEntry")]
    fn set_reboot_to_boot_loader_entry(&self, boot_loader_entry: String)
        -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetWallMessage()) Call interface method `SetWallMessage`.
    #[dbus_proxy(name = "SetWallMessage")]
    fn set_wall_message(&self, wall_message: String, enable: bool) -> crate::zbus::Result<()>;

    /// Receive `SessionNew` signal.
    #[dbus_proxy(signal, name = "SessionNew")]
    fn session_new(
        &self,
        session_id: String,
        object_path: crate::zvariant::OwnedObjectPath,
    ) -> crate::zbus::Result<()>;

    /// Receive `SessionRemoved` signal.
    #[dbus_proxy(signal, name = "SessionRemoved")]
    fn session_removed(
        &self,
        session_id: String,
        object_path: crate::zvariant::OwnedObjectPath,
    ) -> crate::zbus::Result<()>;

    /// Receive `UserNew` signal.
    #[dbus_proxy(signal, name = "UserNew")]
    fn user_new(
        &self,
        uid: u32,
        object_path: crate::zvariant::OwnedObjectPath,
    ) -> crate::zbus::Result<()>;

    /// Receive `UserRemoved` signal.
    #[dbus_proxy(signal, name = "UserRemoved")]
    fn user_removed(
        &self,
        uid: u32,
        object_path: crate::zvariant::OwnedObjectPath,
    ) -> crate::zbus::Result<()>;

    /// Receive `SeatNew` signal.
    #[dbus_proxy(signal, name = "SeatNew")]
    fn seat_new(
        &self,
        seat_id: String,
        object_path: crate::zvariant::OwnedObjectPath,
    ) -> crate::zbus::Result<()>;

    /// Receive `SeatRemoved` signal.
    #[dbus_proxy(signal, name = "SeatRemoved")]
    fn seat_removed(
        &self,
        seat_id: String,
        object_path: crate::zvariant::OwnedObjectPath,
    ) -> crate::zbus::Result<()>;

    /// Receive `PrepareForShutdown` signal.
    #[dbus_proxy(signal, name = "PrepareForShutdown")]
    fn prepare_for_shutdown(&self, start: bool) -> crate::zbus::Result<()>;

    /// Receive `PrepareForSleep` signal.
    #[dbus_proxy(signal, name = "PrepareForSleep")]
    fn prepare_for_sleep(&self, start: bool) -> crate::zbus::Result<()>;

    /// Get property `NAutoVTs`.
    #[dbus_proxy(property, name = "NAutoVTs")]
    fn n_auto_v_ts(&self) -> crate::zbus::Result<u32>;

    /// Get property `KillOnlyUsers`.
    #[dbus_proxy(property, name = "KillOnlyUsers")]
    fn kill_only_users(&self) -> crate::zbus::Result<Vec<String>>;

    /// Get property `KillExcludeUsers`.
    #[dbus_proxy(property, name = "KillExcludeUsers")]
    fn kill_exclude_users(&self) -> crate::zbus::Result<Vec<String>>;

    /// Get property `KillUserProcesses`.
    #[dbus_proxy(property, name = "KillUserProcesses")]
    fn kill_user_processes(&self) -> crate::zbus::Result<bool>;

    /// Get property `RebootParameter`.
    #[dbus_proxy(property, name = "RebootParameter")]
    fn reboot_parameter(&self) -> crate::zbus::Result<String>;

    /// Get property `RebootToFirmwareSetup`.
    #[dbus_proxy(property, name = "RebootToFirmwareSetup")]
    fn reboot_to_firmware_setup(&self) -> crate::zbus::Result<bool>;

    /// Get property `RebootToBootLoaderMenu`.
    #[dbus_proxy(property, name = "RebootToBootLoaderMenu")]
    fn reboot_to_boot_loader_menu(&self) -> crate::zbus::Result<u64>;

    /// Get property `RebootToBootLoaderEntry`.
    #[dbus_proxy(property, name = "RebootToBootLoaderEntry")]
    fn reboot_to_boot_loader_entry(&self) -> crate::zbus::Result<String>;

    /// Get property `BootLoaderEntries`.
    #[dbus_proxy(property, name = "BootLoaderEntries")]
    fn boot_loader_entries(&self) -> crate::zbus::Result<Vec<String>>;

    /// Get property `IdleHint`.
    #[dbus_proxy(property, name = "IdleHint")]
    fn idle_hint(&self) -> crate::zbus::Result<bool>;

    /// Get property `IdleSinceHint`.
    #[dbus_proxy(property, name = "IdleSinceHint")]
    fn idle_since_hint(&self) -> crate::zbus::Result<u64>;

    /// Get property `IdleSinceHintMonotonic`.
    #[dbus_proxy(property, name = "IdleSinceHintMonotonic")]
    fn idle_since_hint_monotonic(&self) -> crate::zbus::Result<u64>;

    /// Get property `BlockInhibited`.
    #[dbus_proxy(property, name = "BlockInhibited")]
    fn block_inhibited(&self) -> crate::zbus::Result<String>;

    /// Get property `DelayInhibited`.
    #[dbus_proxy(property, name = "DelayInhibited")]
    fn delay_inhibited(&self) -> crate::zbus::Result<String>;

    /// Get property `InhibitDelayMaxUSec`.
    #[dbus_proxy(property, name = "InhibitDelayMaxUSec")]
    fn inhibit_delay_max_u_sec(&self) -> crate::zbus::Result<u64>;

    /// Get property `UserStopDelayUSec`.
    #[dbus_proxy(property, name = "UserStopDelayUSec")]
    fn user_stop_delay_u_sec(&self) -> crate::zbus::Result<u64>;

    /// Get property `HandlePowerKey`.
    #[dbus_proxy(property, name = "HandlePowerKey")]
    fn handle_power_key(&self) -> crate::zbus::Result<String>;

    /// Get property `HandlePowerKeyLongPress`.
    #[dbus_proxy(property, name = "HandlePowerKeyLongPress")]
    fn handle_power_key_long_press(&self) -> crate::zbus::Result<String>;

    /// Get property `HandleRebootKey`.
    #[dbus_proxy(property, name = "HandleRebootKey")]
    fn handle_reboot_key(&self) -> crate::zbus::Result<String>;

    /// Get property `HandleRebootKeyLongPress`.
    #[dbus_proxy(property, name = "HandleRebootKeyLongPress")]
    fn handle_reboot_key_long_press(&self) -> crate::zbus::Result<String>;

    /// Get property `HandleSuspendKey`.
    #[dbus_proxy(property, name = "HandleSuspendKey")]
    fn handle_suspend_key(&self) -> crate::zbus::Result<String>;

    /// Get property `HandleSuspendKeyLongPress`.
    #[dbus_proxy(property, name = "HandleSuspendKeyLongPress")]
    fn handle_suspend_key_long_press(&self) -> crate::zbus::Result<String>;

    /// Get property `HandleHibernateKey`.
    #[dbus_proxy(property, name = "HandleHibernateKey")]
    fn handle_hibernate_key(&self) -> crate::zbus::Result<String>;

    /// Get property `HandleHibernateKeyLongPress`.
    #[dbus_proxy(property, name = "HandleHibernateKeyLongPress")]
    fn handle_hibernate_key_long_press(&self) -> crate::zbus::Result<String>;

    /// Get property `HandleLidSwitch`.
    #[dbus_proxy(property, name = "HandleLidSwitch")]
    fn handle_lid_switch(&self) -> crate::zbus::Result<String>;

    /// Get property `HandleLidSwitchExternalPower`.
    #[dbus_proxy(property, name = "HandleLidSwitchExternalPower")]
    fn handle_lid_switch_external_power(&self) -> crate::zbus::Result<String>;

    /// Get property `HandleLidSwitchDocked`.
    #[dbus_proxy(property, name = "HandleLidSwitchDocked")]
    fn handle_lid_switch_docked(&self) -> crate::zbus::Result<String>;

    /// Get property `HoldoffTimeoutUSec`.
    #[dbus_proxy(property, name = "HoldoffTimeoutUSec")]
    fn holdoff_timeout_u_sec(&self) -> crate::zbus::Result<u64>;

    /// Get property `IdleAction`.
    #[dbus_proxy(property, name = "IdleAction")]
    fn idle_action(&self) -> crate::zbus::Result<String>;

    /// Get property `IdleActionUSec`.
    #[dbus_proxy(property, name = "IdleActionUSec")]
    fn idle_action_u_sec(&self) -> crate::zbus::Result<u64>;

    /// Get property `PreparingForShutdown`.
    #[dbus_proxy(property, name = "PreparingForShutdown")]
    fn preparing_for_shutdown(&self) -> crate::zbus::Result<bool>;

    /// Get property `PreparingForSleep`.
    #[dbus_proxy(property, name = "PreparingForSleep")]
    fn preparing_for_sleep(&self) -> crate::zbus::Result<bool>;

    /// Get property `ScheduledShutdown`.
    #[dbus_proxy(property, name = "ScheduledShutdown")]
    fn scheduled_shutdown(&self) -> crate::zbus::Result<(String, u64)>;

    /// Get property `Docked`.
    #[dbus_proxy(property, name = "Docked")]
    fn docked(&self) -> crate::zbus::Result<bool>;

    /// Get property `LidClosed`.
    #[dbus_proxy(property, name = "LidClosed")]
    fn lid_closed(&self) -> crate::zbus::Result<bool>;

    /// Get property `OnExternalPower`.
    #[dbus_proxy(property, name = "OnExternalPower")]
    fn on_external_power(&self) -> crate::zbus::Result<bool>;

    /// Get property `RemoveIPC`.
    #[dbus_proxy(property, name = "RemoveIPC")]
    fn remove_ipc(&self) -> crate::zbus::Result<bool>;

    /// Get property `RuntimeDirectorySize`.
    #[dbus_proxy(property, name = "RuntimeDirectorySize")]
    fn runtime_directory_size(&self) -> crate::zbus::Result<u64>;

    /// Get property `RuntimeDirectoryInodesMax`.
    #[dbus_proxy(property, name = "RuntimeDirectoryInodesMax")]
    fn runtime_directory_inodes_max(&self) -> crate::zbus::Result<u64>;

    /// Get property `InhibitorsMax`.
    #[dbus_proxy(property, name = "InhibitorsMax")]
    fn inhibitors_max(&self) -> crate::zbus::Result<u64>;

    /// Get property `NCurrentInhibitors`.
    #[dbus_proxy(property, name = "NCurrentInhibitors")]
    fn n_current_inhibitors(&self) -> crate::zbus::Result<u64>;

    /// Get property `SessionsMax`.
    #[dbus_proxy(property, name = "SessionsMax")]
    fn sessions_max(&self) -> crate::zbus::Result<u64>;

    /// Get property `NCurrentSessions`.
    #[dbus_proxy(property, name = "NCurrentSessions")]
    fn n_current_sessions(&self) -> crate::zbus::Result<u64>;

    /// Get property `StopIdleSessionUSec`.
    #[dbus_proxy(property, name = "StopIdleSessionUSec")]
    fn stop_idle_session_u_sec(&self) -> crate::zbus::Result<u64>;
}

/// Proxy object for `org.freedesktop.login1.Seat`.
#[dbus_proxy(
    interface = "org.freedesktop.login1.Seat",
    gen_blocking = false,
    default_service = "org.freedesktop.login1"
)]
trait Seat {
    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Terminate()) Call interface method `Terminate`.
    #[dbus_proxy(name = "Terminate")]
    fn terminate(&self) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ActivateSession()) Call interface method `ActivateSession`.
    #[dbus_proxy(name = "ActivateSession")]
    fn activate_session(&self, session_id: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SwitchTo()) Call interface method `SwitchTo`.
    #[dbus_proxy(name = "SwitchTo")]
    fn switch_to(&self, vtnr: u32) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SwitchToNext()) Call interface method `SwitchToNext`.
    #[dbus_proxy(name = "SwitchToNext")]
    fn switch_to_next(&self) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SwitchToPrevious()) Call interface method `SwitchToPrevious`.
    #[dbus_proxy(name = "SwitchToPrevious")]
    fn switch_to_previous(&self) -> crate::zbus::Result<()>;

    /// Get property `Id`.
    #[dbus_proxy(property, name = "Id")]
    fn id(&self) -> crate::zbus::Result<String>;

    /// Get property `ActiveSession`.
    #[dbus_proxy(property, name = "ActiveSession")]
    fn active_session(&self) -> crate::zbus::Result<(String, crate::zvariant::OwnedObjectPath)>;

    /// Get property `CanTTY`.
    #[dbus_proxy(property, name = "CanTTY")]
    fn can_tty(&self) -> crate::zbus::Result<bool>;

    /// Get property `CanGraphical`.
    #[dbus_proxy(property, name = "CanGraphical")]
    fn can_graphical(&self) -> crate::zbus::Result<bool>;

    /// Get property `Sessions`.
    #[dbus_proxy(property, name = "Sessions")]
    fn sessions(&self) -> crate::zbus::Result<Vec<(String, crate::zvariant::OwnedObjectPath)>>;

    /// Get property `IdleHint`.
    #[dbus_proxy(property, name = "IdleHint")]
    fn idle_hint(&self) -> crate::zbus::Result<bool>;

    /// Get property `IdleSinceHint`.
    #[dbus_proxy(property, name = "IdleSinceHint")]
    fn idle_since_hint(&self) -> crate::zbus::Result<u64>;

    /// Get property `IdleSinceHintMonotonic`.
    #[dbus_proxy(property, name = "IdleSinceHintMonotonic")]
    fn idle_since_hint_monotonic(&self) -> crate::zbus::Result<u64>;
}

/// Proxy object for `org.freedesktop.login1.User`.
#[dbus_proxy(
    interface = "org.freedesktop.login1.User",
    gen_blocking = false,
    default_service = "org.freedesktop.login1"
)]
trait User {
    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Terminate()) Call interface method `Terminate`.
    #[dbus_proxy(name = "Terminate")]
    fn terminate(&self) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Kill()) Call interface method `Kill`.
    #[dbus_proxy(name = "Kill")]
    fn kill(&self, signal_number: i32) -> crate::zbus::Result<()>;

    /// Get property `UID`.
    #[dbus_proxy(property, name = "UID")]
    fn uid(&self) -> crate::zbus::Result<u32>;

    /// Get property `GID`.
    #[dbus_proxy(property, name = "GID")]
    fn gid(&self) -> crate::zbus::Result<u32>;

    /// Get property `Name`.
    #[dbus_proxy(property, name = "Name")]
    fn name(&self) -> crate::zbus::Result<String>;

    /// Get property `Timestamp`.
    #[dbus_proxy(property, name = "Timestamp")]
    fn timestamp(&self) -> crate::zbus::Result<u64>;

    /// Get property `TimestampMonotonic`.
    #[dbus_proxy(property, name = "TimestampMonotonic")]
    fn timestamp_monotonic(&self) -> crate::zbus::Result<u64>;

    /// Get property `RuntimePath`.
    #[dbus_proxy(property, name = "RuntimePath")]
    fn runtime_path(&self) -> crate::zbus::Result<String>;

    /// Get property `Service`.
    #[dbus_proxy(property, name = "Service")]
    fn service(&self) -> crate::zbus::Result<String>;

    /// Get property `Slice`.
    #[dbus_proxy(property, name = "Slice")]
    fn slice(&self) -> crate::zbus::Result<String>;

    /// Get property `Display`.
    #[dbus_proxy(property, name = "Display")]
    fn display(&self) -> crate::zbus::Result<(String, crate::zvariant::OwnedObjectPath)>;

    /// Get property `State`.
    #[dbus_proxy(property, name = "State")]
    fn state(&self) -> crate::zbus::Result<String>;

    /// Get property `Sessions`.
    #[dbus_proxy(property, name = "Sessions")]
    fn sessions(&self) -> crate::zbus::Result<Vec<(String, crate::zvariant::OwnedObjectPath)>>;

    /// Get property `IdleHint`.
    #[dbus_proxy(property, name = "IdleHint")]
    fn idle_hint(&self) -> crate::zbus::Result<bool>;

    /// Get property `IdleSinceHint`.
    #[dbus_proxy(property, name = "IdleSinceHint")]
    fn idle_since_hint(&self) -> crate::zbus::Result<u64>;

    /// Get property `IdleSinceHintMonotonic`.
    #[dbus_proxy(property, name = "IdleSinceHintMonotonic")]
    fn idle_since_hint_monotonic(&self) -> crate::zbus::Result<u64>;

    /// Get property `Linger`.
    #[dbus_proxy(property, name = "Linger")]
    fn linger(&self) -> crate::zbus::Result<bool>;
}

/// Proxy object for `org.freedesktop.login1.Session`.
#[dbus_proxy(
    interface = "org.freedesktop.login1.Session",
    gen_blocking = false,
    default_service = "org.freedesktop.login1"
)]
trait Session {
    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Terminate()) Call interface method `Terminate`.
    #[dbus_proxy(name = "Terminate")]
    fn terminate(&self) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Activate()) Call interface method `Activate`.
    #[dbus_proxy(name = "Activate")]
    fn activate(&self) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Lock()) Call interface method `Lock`.
    #[dbus_proxy(name = "Lock")]
    fn lock(&self) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Unlock()) Call interface method `Unlock`.
    #[dbus_proxy(name = "Unlock")]
    fn unlock(&self) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetIdleHint()) Call interface method `SetIdleHint`.
    #[dbus_proxy(name = "SetIdleHint")]
    fn set_idle_hint(&self, idle: bool) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetLockedHint()) Call interface method `SetLockedHint`.
    #[dbus_proxy(name = "SetLockedHint")]
    fn set_locked_hint(&self, locked: bool) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Kill()) Call interface method `Kill`.
    #[dbus_proxy(name = "Kill")]
    fn kill(&self, who: String, signal_number: i32) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#TakeControl()) Call interface method `TakeControl`.
    #[dbus_proxy(name = "TakeControl")]
    fn take_control(&self, force: bool) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ReleaseControl()) Call interface method `ReleaseControl`.
    #[dbus_proxy(name = "ReleaseControl")]
    fn release_control(&self) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetType()) Call interface method `SetType`.
    #[dbus_proxy(name = "SetType")]
    fn set_type(&self, typelabel: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetDisplay()) Call interface method `SetDisplay`.
    #[dbus_proxy(name = "SetDisplay")]
    fn set_display(&self, display: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#TakeDevice()) Call interface method `TakeDevice`.
    #[dbus_proxy(name = "TakeDevice")]
    fn take_device(
        &self,
        major: u32,
        minor: u32,
    ) -> crate::zbus::Result<(crate::zvariant::OwnedFd, bool)>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ReleaseDevice()) Call interface method `ReleaseDevice`.
    #[dbus_proxy(name = "ReleaseDevice")]
    fn release_device(&self, major: u32, minor: u32) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#PauseDeviceComplete()) Call interface method `PauseDeviceComplete`.
    #[dbus_proxy(name = "PauseDeviceComplete")]
    fn pause_device_complete(&self, major: u32, minor: u32) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetBrightness()) Call interface method `SetBrightness`.
    #[dbus_proxy(name = "SetBrightness")]
    fn set_brightness(
        &self,
        subsystem: String,
        name: String,
        brightness: u32,
    ) -> crate::zbus::Result<()>;

    /// Receive `PauseDevice` signal.
    #[dbus_proxy(signal, name = "PauseDevice")]
    fn pause_device(&self, major: u32, minor: u32, typelabel: String) -> crate::zbus::Result<()>;

    /// Receive `ResumeDevice` signal.
    #[dbus_proxy(signal, name = "ResumeDevice")]
    fn resume_device(
        &self,
        major: u32,
        minor: u32,
        fd: crate::zvariant::OwnedFd,
    ) -> crate::zbus::Result<()>;

    /// Receive `Lock` signal.
    #[dbus_proxy(signal, name = "Lock")]
    fn lock(&self) -> crate::zbus::Result<()>;

    /// Receive `Unlock` signal.
    #[dbus_proxy(signal, name = "Unlock")]
    fn unlock(&self) -> crate::zbus::Result<()>;

    /// Get property `Id`.
    #[dbus_proxy(property, name = "Id")]
    fn id(&self) -> crate::zbus::Result<String>;

    /// Get property `User`.
    #[dbus_proxy(property, name = "User")]
    fn user(&self) -> crate::zbus::Result<(u32, crate::zvariant::OwnedObjectPath)>;

    /// Get property `Name`.
    #[dbus_proxy(property, name = "Name")]
    fn name(&self) -> crate::zbus::Result<String>;

    /// Get property `Timestamp`.
    #[dbus_proxy(property, name = "Timestamp")]
    fn timestamp(&self) -> crate::zbus::Result<u64>;

    /// Get property `TimestampMonotonic`.
    #[dbus_proxy(property, name = "TimestampMonotonic")]
    fn timestamp_monotonic(&self) -> crate::zbus::Result<u64>;

    /// Get property `VTNr`.
    #[dbus_proxy(property, name = "VTNr")]
    fn vt_nr(&self) -> crate::zbus::Result<u32>;

    /// Get property `Seat`.
    #[dbus_proxy(property, name = "Seat")]
    fn seat(&self) -> crate::zbus::Result<(String, crate::zvariant::OwnedObjectPath)>;

    /// Get property `TTY`.
    #[dbus_proxy(property, name = "TTY")]
    fn tty(&self) -> crate::zbus::Result<String>;

    /// Get property `Display`.
    #[dbus_proxy(property, name = "Display")]
    fn display(&self) -> crate::zbus::Result<String>;

    /// Get property `Remote`.
    #[dbus_proxy(property, name = "Remote")]
    fn remote(&self) -> crate::zbus::Result<bool>;

    /// Get property `RemoteHost`.
    #[dbus_proxy(property, name = "RemoteHost")]
    fn remote_host(&self) -> crate::zbus::Result<String>;

    /// Get property `RemoteUser`.
    #[dbus_proxy(property, name = "RemoteUser")]
    fn remote_user(&self) -> crate::zbus::Result<String>;

    /// Get property `Service`.
    #[dbus_proxy(property, name = "Service")]
    fn service(&self) -> crate::zbus::Result<String>;

    /// Get property `Desktop`.
    #[dbus_proxy(property, name = "Desktop")]
    fn desktop(&self) -> crate::zbus::Result<String>;

    /// Get property `Scope`.
    #[dbus_proxy(property, name = "Scope")]
    fn scope(&self) -> crate::zbus::Result<String>;

    /// Get property `Leader`.
    #[dbus_proxy(property, name = "Leader")]
    fn leader(&self) -> crate::zbus::Result<u32>;

    /// Get property `Audit`.
    #[dbus_proxy(property, name = "Audit")]
    fn audit(&self) -> crate::zbus::Result<u32>;

    /// Get property `Class`.
    #[dbus_proxy(property, name = "Class")]
    fn class(&self) -> crate::zbus::Result<String>;

    /// Get property `Active`.
    #[dbus_proxy(property, name = "Active")]
    fn active(&self) -> crate::zbus::Result<bool>;

    /// Get property `State`.
    #[dbus_proxy(property, name = "State")]
    fn state(&self) -> crate::zbus::Result<String>;

    /// Get property `IdleHint`.
    #[dbus_proxy(property, name = "IdleHint")]
    fn idle_hint(&self) -> crate::zbus::Result<bool>;

    /// Get property `IdleSinceHint`.
    #[dbus_proxy(property, name = "IdleSinceHint")]
    fn idle_since_hint(&self) -> crate::zbus::Result<u64>;

    /// Get property `IdleSinceHintMonotonic`.
    #[dbus_proxy(property, name = "IdleSinceHintMonotonic")]
    fn idle_since_hint_monotonic(&self) -> crate::zbus::Result<u64>;

    /// Get property `LockedHint`.
    #[dbus_proxy(property, name = "LockedHint")]
    fn locked_hint(&self) -> crate::zbus::Result<bool>;
}
