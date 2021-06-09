// https://tourofrust.com/33_pt-br.html
struct Sacola<T> {
    item: T,
}

fn main() {
    // Nota: usando tipos genéricos aqui nós criamos tipos em tempo de compilação
    // fazendo o nosso código ficar maior. O turbofish nos permite ser explícitos.
    let i32_sacola = Sacola::<i32> { item: 42 };
    let bool_sacola = Sacola::<bool> { item: true };

    // O Rust pode inferir os tipos para genéricos também!
    let float_sacola = Sacola { item: 3.14 };

    // Nota: nunca coloque uma sacola dentro da outra na vida real
    let sacola_na_sacola = Sacola {
        item: Sacola { item: "boom!" },
    };

    println!(
        "{} {} {} {}",
        i32_sacola.item, bool_sacola.item, float_sacola.item, sacola_na_sacola.item.item
    );
}
