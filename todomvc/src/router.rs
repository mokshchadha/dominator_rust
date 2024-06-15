use dominator::routing;
use web_sys::{Url};

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
            _ => Route::All
        }
    }


    pub fn to_url(&self) -> & 'static str {
        match self {
            Route::Completed => "#/completed",
            Route::Active => "#/active",
            Route::All => "#/"
        }
    }
}

impl Default for Route {
     fn default() -> Self {
        Self::from_url(&routing::url().lock_ref())
    }
}