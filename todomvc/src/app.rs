use dominator::{clone, events, html, link, routing, text_signals, with_node, Dom, EvenOptions};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVec, SignalVecExt};
use serde_derive::{Deserialize, Serialize};
use std::cell::Cell;
use std::sync::Arc;
use web_sys::{HtmlInputElement, Url};

use crate::todo::Todo;
use crate::util::{local_storage, trim};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Active,
    Completed,
    All,
}

impl Route {
    pub fn from_url(url: &str) -> Self {
        let url = Url::new(&url).unwrap();
        match url.hash().as_str() {
            "#/active" => Route::Active,
            "#/completed" => Route::Completed,
            _ => Route::All,
        }
    }

    pub fn to_url(&self) -> &'static str {
        match self {
            Route::Active => "#/active",
            Route::Complted => "#/completed",
            Route::All => "#/",
        }
    }
}
