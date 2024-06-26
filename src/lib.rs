pub mod lib {
    pub mod broadcast;
    pub mod echo;
    pub mod generate;
    pub mod gossip;
    pub mod message;
}

pub use lib::broadcast;
pub use lib::echo;
pub use lib::generate;
pub use lib::gossip;
pub use lib::message;
