#![deny(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
#![cfg_attr(test, deny(warnings))]

//! Library to send messages to slack rooms
//! supports entire messaging API, including attachments and fields
//! also support for built-in colors as well as any hex colors

pub use crate::attachment::{Action, Attachment, AttachmentBuilder, Field, Section};
pub use crate::error::{Error, Result};
pub use crate::hex_ext::{HexColor, SlackColor};
pub use crate::payload::{Parse, Payload, PayloadBuilder};
pub use crate::slack::{Slack, SlackLink, SlackText, SlackTextContent, SlackTime, SlackUserLink};

#[macro_use]
mod macros;
mod attachment;
mod error;
mod helper;
mod hex_ext;
mod payload;
mod slack;
