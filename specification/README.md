# Specification pattern

Long ago around the time when I used this pattern first, I implemented it in Python, mostly for myself.
Since then I and my fellow developers have used it a few times.
Here is the Gist: https://gist.github.com/palankai/f73a18ce06751ab8f245

Recently I started working with Rust, and I wanted to implement this pattern.

This is not the only way or the right way to implement it, this is just one way of doing it.

If I start again, I'd probably implement then and, or, invert, xor as a trait.
I don't like the dyn in there, I'd probably choose enum as specifications...


