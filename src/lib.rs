pub mod cache;

pub use cache::Cache;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::trait_cache::TraitCache;


    #[test]
    fn test_lru_cache() {
        let mut cache = Cache::new(3); // Taille de 3
        cache.put("A", String::from("value_a"));
        cache.put("B", String::from("value_b"));
        cache.put("C", String::from("value_c"));
        cache.put("D", String::from("value_d"));
        // Premier élément moins récemment utilisé et dernier le plus récent
        // Cache == [B, C, D]

        let my_value = cache.get("A");
        assert_eq!(my_value, None);
        let my_value = cache.get("D");
        assert_eq!(my_value, Some(&String::from("value_d")));
        // Cache == [B, C, D]

        let my_value = cache.get("B");
        assert_eq!(my_value, Some(&String::from("value_b")));
        // Cache == [C, D, B]

        let my_value = cache.get("C");
        assert_eq!(my_value, Some(&String::from("value_c")));
        // Cache == [D, B, C]

        let my_value = cache.get("X");
        assert_eq!(my_value, None);
        // Cache == [D, B, C]

        cache.put("A", String::from("value_a"));
        // Cache == [B, C, A]

        cache.put("X", String::from("value_x"));
        // Cache == [C, A, X]

        let my_value = cache.get("B");
        assert_eq!(my_value, None);
        // Cache == [C, A, X]

        let my_value = cache.get("D");
        // Cache == [C, A, X]
        assert_eq!(my_value, None);
    }
}