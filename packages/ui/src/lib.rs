use dioxus::prelude::*;

mod hero;
pub use hero::Hero;

mod navbar;
pub use navbar::Navbar;

mod features;
pub use features::Features;

mod footer;
pub use footer::Footer;

pub const GLOBAL_CSS: Asset = asset!("/assets/styling/global.css");
