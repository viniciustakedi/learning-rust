fn recursividade (n: u32) -> u32 {
    if n <= 0 {
        return n;
    }

    recursividade(n - 1)
}