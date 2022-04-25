use std::marker::PhantomData as Phantom;
use std::result::Result;
use std::slice::Iter;

/// A hashed array tree, where each level N represents the key's Nth
/// most significant byte.
pub(crate) struct Tree<'t, K: AsRef<[u8]>, V, const L: usize>(Node<'t, V, L>, Phantom<K>);

pub(crate) type TreeResult<'r, T = ()> = Result<T, &'r str>;

pub(crate) struct Node<'n, V, const L: usize> {
    leaves: Option<&'n mut [Option<Node<'n, V, L>>; L]>,
    value: Option<V>,
}

/// # Tree implementation

impl<K: AsRef<[u8]>, V, const L: usize> Tree<'_, K, V, L> {
    /// Initialise an empty tree.
    #[inline(always)]
    pub const fn new() -> Self {
        Self(
            Node {
                leaves: None,
                value: None,
            },
            Phantom,
        )
    }

    /// Add a new element to the tree, with a `key` used to determine its
    /// place in the tree, and a `value` being any value of any type `V`.
    /// The tree doesn't care what `value` is, although in the context of
    /// this lib we store in it the item's index in the original array.
    pub fn add(&mut self, key: K, value: V) -> TreeResult<()> {
        let ref mut node = self.0;
        let mut kb: Iter<u8> = key.as_ref().iter();

        let off = loop {
            match (&mut node.leaves, kb.next()) {
                // No more bytes left in key. Return an error.
                (_, None) => return Err("out of values"),
                // This byte's 'bucket' is empty. Initialise and return it.
                (l @ None, Some(&msb)) => {
                    l.replace(&mut [None; L]);
                    break msb;
                }
                // This byte's bucket is non-empty, and...
                (Some(leaves), Some(&msb)) => {
                    // ... the next byte's bucket is ...
                    match unsafe { leaves.get_unchecked_mut(msb as usize) } {
                        // ... empty - return it.
                        None => break msb,
                        // ... non-empty - use it as the next bucket to check.
                        Some(mut s) => {
                            *node = s;
                        }
                    }
                }
            }
        };

        let n = match node.leaves {
            Some(leaves) => {
                node.value.take();
                unsafe { leaves.get_unchecked_mut(off as usize) }.replace(Node {
                    leaves: None,
                    value: Some(value),
                });
            }
            None => {
                node.value.replace(value);
            }
        };

        Ok(())
    }
}

/// # Iterator code

struct TreeIterator<'t, K: AsRef<[u8]>, V: Copy, const L: usize> {
    off: &'t [u8],
    tree: Tree<'t, K, V, L>,
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

impl<K: AsRef<[u8]>, V: Copy, const L: usize> Iterator for TreeIterator<'_, K, V, L> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        let mut offs: Iter<u8> = self.off.iter();
        let mut node = &self.tree.0;

        loop {
            let msb = offs.next().unwrap_or(&0x0);
            match node.leaves.and_then(|l| l.get(*msb as usize)) {
                None | Some(None) => break,
                Some(Some(ref s)) => {
                    node = s;
                }
            }
        }

        node.value
    }
}
