pub mod tcp;

#[cfg(test)]
mod test {
    use tcp;
    use std::collections::HashMap;

    #[test]
    fn tcp_write() {
        let mut object = HashMap::new();
        object.insert("key", "value");

        let fluentd = tcp::Fluentd {
            address: "0.0.0.0:24224",
            tag: "foo",
        };

        let _ = fluentd.write(&object);
    }
}
