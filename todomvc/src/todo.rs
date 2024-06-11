use dominator::{clone, events, html, with_node, Dom};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use web_sys::HtmlInputElement;

use crate::util::trim;
