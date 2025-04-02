pub mod ping;

use pavex::blueprint::{Blueprint, router::GET};
use pavex::f;

pub fn register(bp: &mut Blueprint) {
    bp.route(GET, "/api/ping", f!(self::ping::get));
}
