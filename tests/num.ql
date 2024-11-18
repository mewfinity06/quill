module num.ql;

use io::print as print;

public func main : None {
    
    mut a: Int = 0;
    const b: ^Int = a;

    a += 6;

    print("A: <>\n"); // should print `A: 6`
}