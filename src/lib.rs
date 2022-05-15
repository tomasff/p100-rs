use uuid::Uuid;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

struct P100 {
    ip_address: String,
    terminal_uuid: Uuid,
}