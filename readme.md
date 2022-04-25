# radishsort

radishsort is, as the name suggests, a variant of radix sort.

## context

Radix sort is a non-comparison sort which uses the key of each element to index it into an array. Larger elements get placed at higher indexes. As such, the list is sorted without needing to use a single angle bracket.

### non-comparison sorts

Classic radix sort with the right kinds of input is almost incomparable (😉) in performance: linear in time, almost linear in space. But lists with small, well-bounded integer keys are rare. For lists not meeting those conditions, allocating a sparse array is the result, and the space overhead of doing so can be infeasible.

This variant avoids the sparse array problem by exploiting maths. We need a solution for mapping almost any hashable key to a small range of values, preserving its order. In other words, `[1283, 90013, 331]` should become, say, `[29, 43, 18]`.

## performance

blah, blah, 'blazing fast', blah, 'see our benchmarks', blah

### parallelism

Radix sort is eminently parallelisable; this is no exception. The `radishsort` lib won't parallelise it for you, but it aims to offer an interface that makes that easy to do. It's also written with a view to *vector*isation, though again we don't explicitly invoke it for you.

To be specific: we structure the code and use the necessary intrinsics so that `rustc` will auto-vectorise the IR if `opt-level` >= 1. You can validate this with a tool like Godbolt, or `llvm-mca`. While each are independent and any combination can be used, I recommend using both vectorisation and parallelisation. The only exception is if your ar