//! Radish is a simple non-comparative sorting algorithm derived from
//! radix sort, but replacing its simple 'insert at hash index' logic
//! with some good old  data structures

use std::marker::PhantomData as Phantom;
use std::slice::Iter;

/// A function mapping a list element of type `T` to a key of type `K`.
/// What's essential is that the resulting key can be cast to a byte
/// slice pointer, which is used to construct the sorted array tree.
pub type Keyer<T, K: AsRef<[u8]>> = fn(&T) -> K;

pub fn sort<T, K: AsRef<[u8]>>(list: &[T], keyer: Keyer<T, K>) -> &[T] {
    let mut tree = Tree::<&[u8], usize, 256>(
        Node {
            leaves: &mut [None; 256],
            value: 0usize,
        },
        Phantom::default(),
    );

    list.iter()
        .map(|el| keyer(el))
        .enumerate()
        .map(|(i, k)| tree.add(k.as_ref(), i))
        .collect::<()>();

    tree.into_iter()
        .map(|i| list[i])
        .collect::<Vec<T>>()
        .as_slice()
}

/// A hashed array tree, where each level N represents the key's Nth
/// most significant byte.
struct Tree<'t, K: AsRef<[u8]>, V, const L: usize>(Node<'t, V, L>, Phantom<K>);

struct Node<'n, V, const L: usize> {
    leaves: &'n mut [Option<Node<'n, V, L>>; L],
    value: V,
}

impl<K: AsRef<[u8]>, V, const L: usize> Tree<'_, K, V, L> {
    fn add(&mut self, key: K, value: V) {
        let ref mut node = self.0;
        let mut kb: Iter<u8> = key.as_ref().iter();

        let off = loop {
            let msb = kb.next().unwrap_or(&0x0);
            match node.leaves[*msb as usize] {
                None => break *msb,
                Some(ref s) => {
                    node = s;
                }
            }
        };

        node.leaves[off as usize] = Some(Node {
            leaves: &mut [None; L],
            value,
        });

        todo!();
    }
}

impl<'t, K: AsRef<[u8]>, V: Copy, const L: usize> IntoIterator for Tree<'t, K, V, L> {
    type Item = V;

    type IntoIter = TreeIterator<'t, K, V, L>;

    fn into_iter(self) -> Self::IntoIter {
        TreeIterator {
            off: &[],
            tree: self,
        }
    }
}

struct TreeIterator<'t, K: AsRef<[u8]>, V: Copy, const L: usize> {
    off: &'t [u8],
    tree: Tree<'t, K, V, L>,
}

impl<K: AsRef<[u8]>, V: Copy, const L: usize> Iterator for TreeIterator<'_, K, V, L> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        let mut offs: Iter<u8> = self.off.iter();
        let mut node = &self.tree.0;

        loop {
            let msb = offs.next().unwrap_or(&0x0);
            match node.leaves[*msb as usize] {
                None => {
                    break;
                }
                Some(ref s) => {
                    node = s;
                }
            }
        }

        Some(node.value)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
