use hashmap_cache::Cache;

mod cache;

fn main() {
    let mut cache = Cache::new(3);
    cache.put("A", String::from("value_a"));
    cache.put("B", String::from("value_b"));
    cache.put("C", String::from("value_c"));
    cache.put("D", String::from("value_d"));

    cache.get("B");

    dbg!(cache.get("B"));
}