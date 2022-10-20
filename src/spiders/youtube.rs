/*
    For extending the somafm datastructure:

        Vec: [
            PlaylistItem {
                played_at: "20:59:36",
                artist: "Afterlife",
                song: "5th & Avenida",
                album: "CafŽ Del Mar Vol. 4",
            },
            ...etc.
        ]

    use TryFrom to convert into the youtube datastructure.
    With the above input, the end result should print looking like:

    Vec: [
        YPlaylistItem {
            playlist_item: PlaylistItem {
                played_at: "20:59:36",
                artist: "Afterlife",
                song: "5th & Avenida",
                album: "CafŽ Del Mar Vol. 4",
            },
            video: "https://www.youtube.com/watch?v=sjqLVcPd18E",
        },
        ...etc
    ]

    Learn to impl TryFrom()... it’s super handy

    impl<’a, T> TryFrom<Wrapper<Obj<’a, T>>> for Jbo<’a, T> {

        type Error = anyhow::Error;

        fn try_from(pass_by_value: Wrapper<Obj<’a, T>>) -> Result<Self, Self::Error> {
            Ok(Self)
        }
    }

    Also, learn to use .map(), and fold() to use for iteration in-place of for loops:
    difference between the two is that if you are mapping onto an object, you need to
    declare it outside of the map() call, whereas fold() will create a new object and
    map over it all in one call.

    See test_02 and test_03 below for examples of how all of the above works.

    try replacing TupleVec<i32> with just Vec<i32> and TupleHash<String, u32> with HashMap<String, u32>,
    what error do you get from the compiler?

    Learn more about the error here: https://github.com/Ixrec/rust-orphan-rules/issues/1
*/

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct TupleVec<T>(Vec<T>);

#[derive(Debug, Clone)]
pub struct TupleHash<K, V>(HashMap<K, V>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        println!("Hello youtube test!");

        assert!(true)
    }

    #[test]
    fn test_02_try_from_with_map() {
        use std::collections::HashMap;

        let input = HashMap::<&str, i32>::from([
            ("a", 1),
            ("b", 2),
            ("c", 3),
            ("d", 4),
            ("e", 5),
            ("f", 6),
            ("g", 7),
            ("h", 8),
        ]);

        impl TryFrom<HashMap<&str, i32>> for TupleVec<i32> {
            type Error = anyhow::Error;

            fn try_from(inp: HashMap<&str, i32>) -> Result<Self, Self::Error> {
                let converted = inp
                    .iter()
                    .map(|(key, val)| {
                        let res = val.clone();
                        Ok(res)
                    })
                    .collect::<Result<Vec<i32>, Self::Error>>()?;

                let res = TupleVec(converted);

                Ok(res)
            }
        }

        let res = TupleVec::<i32>::try_from(input).unwrap();
        println!("Result: {:#?}", res);

        assert!(true)
    }

    #[test]
    fn test_03_try_from_with_fold() {
        use std::collections::HashMap;

        let input = HashMap::<&str, i32>::from([
            ("a", 1),
            ("b", 2),
            ("c", 3),
            ("d", 4),
            ("e", 5),
            ("f", 6),
            ("g", 7),
            ("h", 8),
        ]);

        impl TryFrom<HashMap<&str, i32>> for TupleHash<String, u32> {
            type Error = anyhow::Error;

            fn try_from(inp: HashMap<&str, i32>) -> Result<Self, Self::Error> {
                let converted =
                    inp.iter()
                        .fold(HashMap::<String, u32>::new(), |mut accum, (key, val)| {
                            let k_ = key.to_string();
                            let v_ = val.to_owned() as u32;
                            accum.insert(k_, v_);
                            accum
                        });

                let res = TupleHash(converted);

                Ok(res)
            }
        }

        let res = TupleHash::<String, u32>::try_from(input).unwrap();
        println!("Result: {:#?}", res);

        assert!(true)
    }
}
