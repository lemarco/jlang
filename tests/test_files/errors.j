module errors {
    // Invalid number
    let bad_num = 42.42.42

    // Unterminated string
    let bad_string = "unterminated

    // Invalid character
    let bad_char = @

    // Missing closing brace
    type Incomplete => {
        field: Number
}