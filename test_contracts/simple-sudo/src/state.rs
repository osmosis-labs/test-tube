use cw_storage_plus::Map;

pub const RANDOM_DATA: Map<&'_ str, String> = Map::new("state");
