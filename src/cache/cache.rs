use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use crate::cache::trait_cache::TraitCache;

/// Structure qui représente un cache LRU
///
/// Permets de créer un cache avec une taille fixe (donnée en paramètre lors de la création)
/// Lorsque le cache atteint sa capacité maximale, les éléments les plus anciens
/// sont retirés pour faire de la place aux nouveaux éléments
///
pub struct Cache<K, V>
{
    size: usize,
    cache_content: HashMap<K, V>,
    cache_order: VecDeque<K>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Créé un cache d'une taille donnée en paramètre
    ///
    /// # Arguments
    /// - `size` : La taille maximale du cache
    ///
    /// # Exemples
    ///
    /// ```
    /// use hashmap_cache::cache::Cache;
    ///
    /// let cache : Cache<&str, String> = Cache::new(3);
    /// ```
    pub fn new(size: usize) -> Self {
        Self {
            size,
            cache_content: HashMap::new(),
            cache_order: VecDeque::new(),
        }
    }
}

impl<K, V> TraitCache<K, V> for Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Ajoute une clé et sa valeur associée dans le cache
    ///
    /// # Arguments
    /// - `key` : La clé à insérer
    /// - `value` : La valeur associée à la clé
    /// # Exemples
    ///
    /// ```
    /// use hashmap_cache::cache::Cache;
    /// use hashmap_cache::cache::trait_cache::TraitCache;
    ///
    /// let mut cache: Cache<&str, String> = Cache::new(2);
    ///
    /// cache.put("A", String::from("value_a")); // [A]
    /// cache.put("B", String::from("value_b")); // [A,B]
    /// cache.put("C", String::from("value_c")); // [B,C] ("A" est supprimé car la taille du cache est de 2)
    /// ```
    fn put(&mut self, key: K, value: V) {
        if self.cache_content.contains_key(&key) {
            // Met à jour la valeur
            self.cache_content.insert(key.clone(), value);
            self.move_key_end_cache(&key);
        } else {
            if self.cache_order.len() >= self.size {
                // Enlève la clé la plus ancienne
                if let Some(cle_supprime) = self.cache_order.pop_front() {
                    self.cache_content.remove(&cle_supprime);
                }
            }
            // Ajoute la nouvelle pair de clé-valeur
            self.cache_order.push_back(key.clone());
            self.cache_content.insert(key, value);
        }
    }

    /// Retourne la valeur V de la clé K
    /// et place l'élément à la fin du cache (élément le plus récent)
    ///
    /// # Arguments
    /// - `key` : La clé dont on veut obtenir la valeur
    ///
    /// # Return
    /// - `Option<&V>` : La valeur associée si la clé existe sinon `None`
    ///
    /// # Exemples
    ///
    /// ```
    /// use hashmap_cache::cache::Cache;
    /// use hashmap_cache::cache::trait_cache::TraitCache;
    ///
    /// let mut cache = Cache::new(3);
    ///
    /// cache.put("A", String::from("value_a")); // [A]
    /// cache.put("B", String::from("value_b")); // [A,B]
    /// cache.put("C", String::from("value_c")); // [A,B,C]
    ///
    /// assert_eq!(cache.get("A"), Some(&String::from("value_a"))); // [B,C,A]
    /// assert_eq!(cache.get("X"), None); // "X" n'est pas dans le cache
    /// ```
    fn get(&mut self, key: K) -> Option<&V> {
        if self.cache_content.contains_key(&key) {
            self.move_key_end_cache(&key);
            self.cache_content.get(&key)
        } else {
            None
        }
    }

    /// Déplace une clé à la fin du cache
    ///
    /// # Arguments
    /// - `key` : La clé que l'on veut déplacer
    ///
    /// # Exemples
    ///
    /// ```
    /// use hashmap_cache::cache::Cache;
    /// use hashmap_cache::cache::trait_cache::TraitCache;
    ///
    /// let mut cache = Cache::new(3);
    ///
    /// cache.put("A", String::from("value_a")); // [A]
    /// cache.put("B", String::from("value_b")); // [A,B]
    /// cache.put("C", String::from("value_c")); // [A,B,C]
    ///
    /// cache.move_key_end_cache(&"A"); // [B,C,A]
    /// ```
    fn move_key_end_cache(&mut self, key: &K) {
        self.cache_order.retain(|k| k != key);
        self.cache_order.push_back(key.clone());
    }
}