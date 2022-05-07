pub const APPLICATION_JSON: &str = "application/json";
pub const APPLICATION_TOML: &str = "application/toml";
pub const APPLICATION_MSGPACK: &str = "application/msgpack";

pub fn is_valid(t: &str) -> bool
{
 match t
 {
  APPLICATION_JSON | APPLICATION_TOML | APPLICATION_MSGPACK => true,
  _ => false
 }
}
