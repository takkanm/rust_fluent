pub mod tcp;

#[cfg(test)]
mod test {
    use tcp;
    use std::collections::HashMap;

    #[test]
    fn tcp_write() {
        let mut object: HashMap<String, String> = HashMap::new();
        object.insert("key".to_string(), "value".to_string());

        let fluentd = tcp::Fluentd {
            address: "0.0.0.0".to_string(),
            port: 24224,
            tag: "foo".to_string(),
        };

        fluentd.write(object);
    }
}
