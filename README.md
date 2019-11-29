# semantic-db

An event is data. Events have unique handles. Events are queryable by handle.

```rust
use crate::{Event, Handle, query};

pub fn main() {
    let handle = Handle::new();
    let event: Event = query(handle);
}
```