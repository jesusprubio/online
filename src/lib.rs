//! ```rust
//! use online::check;
//!
//! #[async_std::main]
//! async fn main() {
//!     println!("Online? {}", check(None).await.is_ok());
//!     println!("Online (timeout)? {}", check(Some(5)).await.is_ok());
//!     println!("Online (`Result`)? {:?}", check(None).await.unwrap());
//! }
//! ```
#[cfg(any(feature = "async-std-runtime", feature = "tokio-runtime"))]
#[path = "./impls/default.rs"]
mod default;

#[cfg(any(feature = "async-std-runtime", feature = "tokio-runtime"))]
pub use default::check;

#[cfg(feature = "sync")]
#[path = "./impls/sync.rs"]
pub mod sync;
