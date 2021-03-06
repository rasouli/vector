#![macro_use]

mod ceil;
mod contains;
mod del;
mod downcase;
mod ends_with;
mod floor;
mod format_number;
mod format_timestamp;
mod r#match;
mod md5;
mod now;
mod only_fields;
mod parse_duration;
mod parse_json;
mod parse_syslog;
mod parse_timestamp;
mod parse_url;
mod replace;
mod round;
mod sha1;
mod sha2;
mod sha3;
mod slice;
mod split;
mod starts_with;
mod strip_ansi_escape_codes;
mod strip_whitespace;
mod to_bool;
mod to_float;
mod to_int;
mod to_string;
mod to_timestamp;
mod tokenize;
mod truncate;
mod upcase;
mod uuid_v4;

pub use self::md5::Md5;
pub use self::sha1::Sha1;
pub use self::sha2::Sha2;
pub use self::sha3::Sha3;
pub use ceil::Ceil;
pub use contains::Contains;
pub use del::Del;
pub use downcase::Downcase;
pub use ends_with::EndsWith;
pub use floor::Floor;
pub use format_number::FormatNumber;
pub use format_timestamp::FormatTimestamp;
pub use now::Now;
pub use only_fields::OnlyFields;
pub use parse_duration::ParseDuration;
pub use parse_json::ParseJson;
pub use parse_syslog::ParseSyslog;
pub use parse_timestamp::ParseTimestamp;
pub use parse_url::ParseUrl;
pub use r#match::Match;
pub use replace::Replace;
pub use round::Round;
pub use slice::Slice;
pub use split::Split;
pub use starts_with::StartsWith;
pub use strip_ansi_escape_codes::StripAnsiEscapeCodes;
pub use strip_whitespace::StripWhitespace;
pub use to_bool::ToBool;
pub use to_float::ToFloat;
pub use to_int::ToInt;
pub use to_string::ToString;
pub use to_timestamp::ToTimestamp;
pub use tokenize::Tokenize;
pub use truncate::Truncate;
pub use upcase::Upcase;
pub use uuid_v4::UuidV4;

use remap::{Result, Value};

fn convert_value_or_default(
    value: Result<Value>,
    default: Option<Result<Value>>,
    convert: impl Fn(Value) -> Result<Value> + Clone,
) -> Result<Value> {
    value
        .and_then(convert.clone())
        .or_else(|err| default.ok_or(err)?.and_then(|value| convert(value)))
}

fn is_scalar_value(value: &Value) -> bool {
    use Value::*;

    match value {
        Integer(_) | Float(_) | Bytes(_) | Boolean(_) | Null => true,
        Timestamp(_) | Map(_) | Array(_) => false,
    }
}

/// Rounds the given number to the given precision.
/// Takes a function parameter so the exact rounding function (ceil, floor or round)
/// can be specified.
fn round_to_precision<F>(num: f64, precision: i64, fun: F) -> f64
where
    F: Fn(f64) -> f64,
{
    let multiplier = 10_f64.powf(precision as f64);
    fun(num * multiplier as f64) / multiplier
}
