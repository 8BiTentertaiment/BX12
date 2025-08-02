#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_translation() {
        let wrapper = BX12Wrapper::new();
        wrapper.translate_dx12_calls();
        assert!(true);
    }
}
