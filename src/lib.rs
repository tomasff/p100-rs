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

impl P100 {
    fn handshake(&self) {
        todo!()
    }

    fn login(&self) {
        todo!()
    }

    fn set_state(&self, on: bool) {
        todo!()
    }
}