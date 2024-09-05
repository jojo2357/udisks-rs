//! # DBus interface proxy for: `org.freedesktop.UDisks2.Drive`
//!
//! This code was generated by `zbus-xmlgen` `4.0.0` from DBus introspection data.
//! Source: `org.freedesktop.UDisks2.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus2.github.io/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus::{proxy, zvariant::OwnedValue};

use crate::error;

/// Rotational rate of a drive.
#[derive(Debug, Default, PartialEq, Eq)]
pub enum RotationRate {
    /// The drive is known to be rotating media but rotation rate isn't known.
    Unknown,
    /// The drive is known to be non-rotating media.
    #[default]
    NonRotating,
    /// The rotation rate in rounds per minute.
    Rotating(i32),
}

impl TryFrom<OwnedValue> for RotationRate {
    type Error = <i32 as TryFrom<OwnedValue>>::Error;

    fn try_from(v: OwnedValue) -> Result<Self, Self::Error> {
        Ok(match v.try_into()? {
            -1 => RotationRate::Unknown,
            0 => RotationRate::NonRotating,
            v => RotationRate::Rotating(v),
        })
    }
}

#[proxy(
    interface = "org.freedesktop.UDisks2.Drive",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/Drive"
)]
trait Drive {
    /// Eject method
    fn eject(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// PowerOff method
    fn power_off(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// SetConfiguration method
    fn set_configuration(
        &self,
        value: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// CanPowerOff property
    #[zbus(property)]
    fn can_power_off(&self) -> error::Result<bool>;

    /// Configuration property
    #[zbus(property)]
    fn configuration(
        &self,
    ) -> error::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// ConnectionBus property
    #[zbus(property)]
    fn connection_bus(&self) -> error::Result<String>;

    /// Ejectable property
    #[zbus(property)]
    fn ejectable(&self) -> error::Result<bool>;

    /// Id property
    #[zbus(property)]
    fn id(&self) -> error::Result<String>;

    /// Media property
    #[zbus(property)]
    fn media(&self) -> error::Result<String>;

    /// MediaAvailable property
    #[zbus(property)]
    fn media_available(&self) -> error::Result<bool>;

    /// MediaChangeDetected property
    #[zbus(property)]
    fn media_change_detected(&self) -> error::Result<bool>;

    /// MediaCompatibility property
    #[zbus(property)]
    fn media_compatibility(&self) -> error::Result<Vec<String>>;

    /// MediaRemovable property
    #[zbus(property)]
    fn media_removable(&self) -> error::Result<bool>;

    /// Model property
    #[zbus(property)]
    fn model(&self) -> error::Result<String>;

    /// Optical property
    #[zbus(property)]
    fn optical(&self) -> error::Result<bool>;

    /// OpticalBlank property
    #[zbus(property)]
    fn optical_blank(&self) -> error::Result<bool>;

    /// OpticalNumAudioTracks property
    #[zbus(property)]
    fn optical_num_audio_tracks(&self) -> error::Result<u32>;

    /// OpticalNumDataTracks property
    #[zbus(property)]
    fn optical_num_data_tracks(&self) -> error::Result<u32>;

    /// OpticalNumSessions property
    #[zbus(property)]
    fn optical_num_sessions(&self) -> error::Result<u32>;

    /// OpticalNumTracks property
    #[zbus(property)]
    fn optical_num_tracks(&self) -> error::Result<u32>;

    /// Removable property
    #[zbus(property)]
    fn removable(&self) -> error::Result<bool>;

    /// Revision property
    #[zbus(property)]
    fn revision(&self) -> error::Result<String>;

    /// Rotational rate of the drive.
    #[zbus(property)]
    fn rotation_rate(&self) -> error::Result<RotationRate>;

    /// Seat property
    #[zbus(property)]
    fn seat(&self) -> error::Result<String>;

    /// Serial property
    #[zbus(property)]
    fn serial(&self) -> error::Result<String>;

    /// SiblingId property
    #[zbus(property)]
    fn sibling_id(&self) -> error::Result<String>;

    /// Size property
    #[zbus(property)]
    fn size(&self) -> error::Result<u64>;

    /// SortKey property
    #[zbus(property)]
    fn sort_key(&self) -> error::Result<String>;

    /// TimeDetected property
    #[zbus(property)]
    fn time_detected(&self) -> error::Result<u64>;

    /// TimeMediaDetected property
    #[zbus(property)]
    fn time_media_detected(&self) -> error::Result<u64>;

    /// Vendor property
    #[zbus(property)]
    fn vendor(&self) -> error::Result<String>;

    /// WWN property
    #[zbus(property, name = "WWN")]
    fn wwn(&self) -> error::Result<String>;
}
