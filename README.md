# ring_buf 
A basic ring buffer implementation in Rust, guided by https://en.wikipedia.org/wiki/Circular_buffer

This is just a small learning project, initially designed to get more experience with `unsafe` and use raw pointers to implement this, I discovered `vec.set_len()` and `vec.to_boxed_slice()` which seemed sufficient for the task.

Took a lot of pointers from https://github.com/Lolirofle/fixed_circular_buffer/blob/master/src/lib.rs, particularly the `PhantomData` pattern and the idea of Boxing the slice. I think this would be just as good backed with a simple `Vec`, but I wanted to explore alternative techniques.

# Next Steps

- Implement From<Iterator>
- Provide an iterator over the ring_buf
