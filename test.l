func main(a: u32, b: u32): u32 {
    let c: u32 = a;

    if a > b {
        a = b;
        b = c;
        c = a;
    };

    while c < b {
        c = c + 1;
    };
    return 0;
}
