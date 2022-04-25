//! Radish is a simple non-comparative sorting algorithm derived from
//! radix sort, but replacing its simple 'insert at hash index' logic
//! with some good old  data structures

mod tree;

use tree::Tree;

/// A function mapping a list element of type `T` to a key of type `K`.
/// What's essential is that the resulting key can be cast to a byte
/// slice pointer, which is used to construct the sorted array tree.
pub type Keyer<T, K: AsRef<[u8]>> = fn(&T) -> K;

pub fn sort<T, K: AsRef<[u8]>>(list: &[T], keyer: Keyer<T, K>) -> &[T] {
    let mut tree = Tree::<&[u8], usize, 256>::new();

    list.iter()
        .map(|el| keyer(el))
        .enumerate()
        .filter_map(|(i, k)| {
            tree.add(k.as_ref(), i)
                .map_err(|e| eprintln!("failed to add item to tree with error: '{}'; exiting;", e))
                .ok()
        })
        .collect::<()>();

    tree.into_iter()
        .map(|i| list[i])
        .collect::<Vec<T>>()
        .as_slice()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let inlist = ["pluto", "earth", "saturn", "mars", "uranus"];
        let sorted = crate::sort(&inlist, |&el| el);
        assert_ne!(sorted, inlist);
        assert_eq!(sorted, ["earth", "mars", "pluto", "saturn", "uranus"]);
    }
}
