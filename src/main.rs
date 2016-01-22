fn main() {
    let mut taljare = 3;

    println!("{}", 2);

    loop {
        let mut namnare = 3;
        let mut is_prim = 2;

        while namnare < taljare {
            if taljare % namnare == 0 {
                is_prim += 1;

                if is_prim > 2 {
                    namnare = taljare
                }
            }

            namnare += 2;
        }

        if is_prim == 2 {
            println!("{}", taljare);
        }

        taljare += 2;
    }
}
