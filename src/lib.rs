mod devon;
mod utils;
pub use devon::Rex;

pub use client;
pub use http;
pub use middleware;
pub use router;
pub use router_macro;

pub mod prelude {
    pub use crate::devon::*;
    pub use middleware::*;

    pub use client::*;
    pub use http::*;
    pub use router::*;
    pub use router_macro::*;
}
