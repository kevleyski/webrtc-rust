#![warn(rust_2018_idioms)]
#![allow(dead_code)]

use ::dtls::extension::extension_use_srtp::SrtpProtectionProfile;

pub mod api;
pub mod data;
mod error;
pub mod media;
pub mod peer;
pub mod stats;
pub mod util;

pub use error::Error;

#[macro_use]
extern crate lazy_static;

pub(crate) const UNSPECIFIED_STR: &str = "Unspecified";
pub(crate) const SSRC_STR: &str = "ssrc";

/// Equal to UDP MTU
pub(crate) const RECEIVE_MTU: usize = 1460;

/// SIMULCAST_PROBE_COUNT is the amount of RTP Packets
/// that handleUndeclaredSSRC will read and try to dispatch from
/// mid and rid values
pub(crate) const SIMULCAST_PROBE_COUNT: usize = 10;

/// SIMULCAST_MAX_PROBE_ROUTINES is how many active routines can be used to probe
/// If the total amount of incoming SSRCes exceeds this new requests will be ignored
pub(crate) const SIMULCAST_MAX_PROBE_ROUTINES: u64 = 25;

pub(crate) const MEDIA_SECTION_APPLICATION: &str = "application";

pub(crate) const RTP_OUTBOUND_MTU: usize = 1200;

pub(crate) const RTP_PAYLOAD_TYPE_BITMASK: u8 = 0x7F;

pub(crate) fn default_srtp_protection_profiles() -> Vec<SrtpProtectionProfile> {
    vec![
        SrtpProtectionProfile::Srtp_Aead_Aes_128_Gcm,
        SrtpProtectionProfile::Srtp_Aes128_Cm_Hmac_Sha1_80,
    ]
}
