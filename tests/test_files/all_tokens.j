module tokens {
    // Testing every possible token type
    type AllTokens => {
        // Keywords
        module type const let

        // Types
        Number String Boolean

        // Symbols
        { } ( ) : => = . ,

        // Literals
        numberLit: 42,
        floatLit: 3.14,
        stringLit: "Hello",
        boolLit: true,
        boolLit2: false
    }
}