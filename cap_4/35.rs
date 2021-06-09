struct Sacola<T> {
    item: Option<T>,
}

fn main() {
    let i32_sacola = Sacola::<i32> { item: None };
    if i32_sacola.item.is_none() {
        println!("não há nada na sacola")
    } else {
        println!("tem alguma coisa na sacola")
    }

    let i32_sacola = Sacola::<i32> { item: Some(32) };
    if i32_sacola.item.is_none() {
        println!("não há nada na sacola")
    } else {
        println!("tem alguma coisa na sacola")
    }

    match i32_sacola.item {
        Some(v) => println!("encontrei {}, na sacola",),
        None => println!("não há nada na sacola"),
    }
}
