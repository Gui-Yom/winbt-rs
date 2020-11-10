fn main() {
    winrt::build!(
        types
            windows::foundation::*
            windows::foundation::collections::*
            windows::devices::bluetooth::*
            windows::devices::enumeration::*
    );
}
