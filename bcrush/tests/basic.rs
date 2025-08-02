#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bcrush_init() {
        let api = Bcrush::new();
        assert!(api.is_ok());
    }

    #[test]
    fn test_command_queue_submit() {
        let cq = command_queue::CommandQueue::new();
        cq.submit();
        assert!(true); // Dummy test
    }
}
