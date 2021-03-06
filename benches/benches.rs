#![feature(test)]

extern crate test;

use ordnung::Map;
use std::collections::HashMap;
use test::{black_box, Bencher};
// use ahash::AHashMap;
use fnv::FnvHashMap;
use rustc_hash::FxHashMap;

type AHashMap<K, V> = HashMap<K, V, ahash::RandomState>;

// Bunch of keys taken from twitter JSON benchmark
static KEYS: &[&str] = &[
    "statuses",
    "metadata",
    "result_type",
    "recent",
    "iso_language_code",
    "created_at",
    "id",
    "id_str",
    "text",
    "source",
    "truncated",
    "in_reply_to_status_id",
    "in_reply_to_status_id_str",
    "in_reply_to_user_id",
    "in_reply_to_user_id_str",
    "in_reply_to_screen_name",
    "user",
    "id",
    "id_str",
    "name",
    "screen_name",
    "location",
    "description",
    "url",
    "entities",
    "description",
    "urls",
    "protected",
    "followers_count",
    "friends_count",
    "listed_count",
    "created_at",
    "favourites_count",
    "utc_offset",
    "time_zone",
    "geo_enabled",
    "verified",
    "statuses_count",
    "lang",
    "contributors_enabled",
    "is_translator",
    "is_translation_enabled",
    "profile_background_color",
    "profile_background_image_url",
    "profile_background_image_url_https",
    "profile_background_tile",
    "profile_image_url",
    "profile_image_url_https",
    "profile_banner_url",
    "profile_link_color",
    "profile_sidebar_border_color",
    "profile_sidebar_fill_color",
    "profile_text_color",
    "profile_use_background_image",
    "default_profile",
    "default_profile_image",
    "following",
    "follow_request_sent",
    "notifications",
    "geo",
    "coordinates",
    "place",
    "contributors",
    "retweet_count",
    "favorite_count",
    "entities",
    "hashtags",
    "symbols",
    "urls",
    "user_mentions",
    "screen_name",
    "name",
    "id",
    "id_str",
    "indices",
    "favorited",
    "retweeted",
    "lang",
    "foo",
    "bar",
];

macro_rules! bench_all {
    ($cap:literal) => {
        use super::*;

        #[bench]
        #[allow(non_snake_case)]
        fn create__ordnung(b: &mut Bencher) {
            b.iter(|| {
                let mut map = Map::new();

                for key in KEYS[..$cap].iter().copied() {
                    map.insert(key, key);
                }

                black_box(map);
            });
        }

        #[bench]
        fn create_ahash_hashmap(b: &mut Bencher) {
            b.iter(|| {
                let mut map: AHashMap<&str, &str> = AHashMap::default();

                for key in KEYS[..$cap].iter().copied() {
                    map.insert(key, key);
                }

                black_box(map);
            });
        }

        #[bench]
        fn create_fnv_hashmap(b: &mut Bencher) {
            b.iter(|| {
                let mut map = FnvHashMap::default();

                for key in KEYS[..$cap].iter().copied() {
                    map.insert(key, key);
                }

                black_box(map);
            });
        }

        #[bench]
        fn create_rustc_hashmap(b: &mut Bencher) {
            b.iter(|| {
                let mut map = FxHashMap::default();

                for key in KEYS[..$cap].iter().copied() {
                    map.insert(key, key);
                }

                black_box(map);
            });
        }

        #[bench]
        fn create_std_hashmap(b: &mut Bencher) {
            b.iter(|| {
                let mut map = HashMap::new();

                for key in KEYS[..$cap].iter().copied() {
                    map.insert(key, key);
                }

                black_box(map);
            });
        }

        #[bench]
        #[allow(non_snake_case)]
        fn prealloc_create__ordnung(b: &mut Bencher) {
            b.iter(|| {
                let mut map = Map::with_capacity($cap);

                for key in KEYS[..$cap].iter().copied() {
                    map.insert(key, key);
                }

                black_box(map);
            });
        }

        #[bench]
        fn prealloc_create_ahash_hashmap(b: &mut Bencher) {
            b.iter(|| {
                let mut map: AHashMap<&str, &str> = AHashMap::default();
                map.reserve($cap);

                for key in KEYS[..$cap].iter().copied() {
                    map.insert(key, key);
                }

                black_box(map);
            });
        }

        #[bench]
        fn prealloc_create_fnv_hashmap(b: &mut Bencher) {
            b.iter(|| {
                let mut map = FnvHashMap::default();
                map.reserve($cap);

                for key in KEYS[..$cap].iter().copied() {
                    map.insert(key, key);
                }

                black_box(map);
            });
        }

        #[bench]
        fn prealloc_create_rustc_hashmap(b: &mut Bencher) {
            b.iter(|| {
                let mut map = FxHashMap::default();
                map.reserve($cap);

                for key in KEYS[..$cap].iter().copied() {
                    map.insert(key, key);
                }

                black_box(map);
            });
        }

        #[bench]
        fn prealloc_create_std_hashmap(b: &mut Bencher) {
            b.iter(|| {
                let mut map = HashMap::with_capacity($cap);

                for key in KEYS[..$cap].iter().copied() {
                    map.insert(key, key);
                }

                black_box(map);
            });
        }

        const INDEXING_OPS: usize = 80;

        #[bench]
        #[allow(non_snake_case)]
        fn x_index__ordnung(b: &mut Bencher) {
            let mut map = Map::new();

            for key in KEYS[..$cap].iter().copied() {
                map.insert(key, key);
            }

            b.iter(|| {
                for i in 0..INDEXING_OPS {
                    black_box(map.get(KEYS[i % $cap]));
                }
            });
        }

        #[bench]
        fn x_index_ahash_hashmap(b: &mut Bencher) {
            let mut map: AHashMap<&str, &str> = AHashMap::default();

            for key in KEYS[..$cap].iter().copied() {
                map.insert(key, key);
            }

            b.iter(|| {
                for i in 0..INDEXING_OPS {
                    black_box(map.get(KEYS[i % $cap]));
                }
            });
        }

        #[bench]
        fn x_index_fnv_hashmap(b: &mut Bencher) {
            let mut map = FnvHashMap::default();

            for key in KEYS[..$cap].iter().copied() {
                map.insert(key, key);
            }

            b.iter(|| {
                for i in 0..INDEXING_OPS {
                    black_box(map.get(KEYS[i % $cap]));
                }
            });
        }

        #[bench]
        fn x_index_rustc_hashmap(b: &mut Bencher) {
            let mut map = FxHashMap::default();

            for key in KEYS[..$cap].iter().copied() {
                map.insert(key, key);
            }

            b.iter(|| {
                for i in 0..INDEXING_OPS {
                    black_box(map.get(KEYS[i % $cap]));
                }
            });
        }

        #[bench]
        fn x_index_std_hashmap(b: &mut Bencher) {
            let mut map = HashMap::new();

            for key in KEYS[..$cap].iter().copied() {
                map.insert(key, key);
            }

            b.iter(|| {
                for i in 0..INDEXING_OPS {
                    black_box(map.get(KEYS[i % $cap]));
                }
            });
        }
    };
}

mod bench80 {
    bench_all!(80);
}
mod bench40 {
    bench_all!(40);
}
mod bench20 {
    bench_all!(20);
}
mod bench10 {
    bench_all!(10);
}
mod bench05 {
    bench_all!(5);
}
