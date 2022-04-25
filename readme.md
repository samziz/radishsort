# radishsort

radishsort is, as the name suggests, a variant of radix sort / counting sort, a non-comparison sort with unrivalled time complexity. It uses the key of each element, as defined by a user-supplied function, to assign it to an index in a pre-allocated fixed-width array.

<img src="https://www.garden.eco/wp-content/uploads/2018/01/Red-radish.svg" width="180px"/>

## context

Counting sort is a non-comparison sort which uses the key of each element to index it into an array. Larger elements get placed at higher indexes. As such, the list is sorted without needing to use a single angle bracket. Unfortunately, counting sort requires an array large enough for the highest possible key, which is an exacting requirement even for the most amenable of use cases.

Radix sort improves on this by trading off space for time, by splitting keys into buckets based on the most significant bit/byte, then performing an auxiliary sort at the end, either radix sort or something else. This significantly reduces space complexity but sacrifices counting sort's excellent time characteristics.

Radish sort circumambulates this dilemma by using a modified hashed array tree to store keys (and offsets into the original array, to further save on space and `malloc`/`alloca` time). Each level matches against each successive (most-significant) byte of the key. If there is only one item matching at that index, then the tree does not extend beyond that level; when a second item is encountered, it splits both of them into child nodes, and so forth.

This amortises the space and time costs of sorting the less significant digits over the far smaller time and space costs of non-pathological inputs. As a result, it accounts for the possibility of fine-grained key differences without either (1) needing to allocate an enormous sparse max-key-sized array, or (b) needing to perform auxiliary sorts over an only-coarsely-partitioned array.

## performance

blah, blah, 'blazing fast', blah, 'see our benchmarks', blah

### parallelism

Radix sort is eminently parallelisable; this is no exception. The `radishsort` lib won't parallelise it for you, but it aims to offer an interface that makes that easy to do. It's also written with a view to *vector*isation, though again we don't explicitly invoke it for you.

To be specific: we structure the code and use the necessary intrinsics so that `rustc` will auto-vectorise the IR if `opt-level` >= 1. You can validate this with a tool like Godbolt, or `llvm-mca`. While each are independent and any combination can be used, I recommend using both vectorisation and parallelisation. The only exception is if your inputs are too few for the overhead of thread spawning (and copying across main memory, etc) to be justified by the parallel speedup.