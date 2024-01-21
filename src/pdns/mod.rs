/// Implement PowerDNS Remote Backend ABI for Unix sockets.
/// https://doc.powerdns.com/authoritative/backends/remote.html#api
///
mod query;
pub use query::Query;
mod reply;
pub use reply::Reply;
