struct Event;
struct Handle;
struct Query;
struct HandleGenerator;

impl HandleGenerator {
    pub fn create(&self) -> Handle {
        Handle
    }
}

fn query(handle: Handle) -> Event {
    Event
}
fn custom_query(query: Query) -> Vec<Event> {
    vec![]
}

fn make_query() -> Query {
    Query
}

fn do_work() {
    let generator = HandleGenerator;
    let handle = generator.create();
    let event: Event = query(handle);
    let query: Query = make_query();
    let events = custom_query(query);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
