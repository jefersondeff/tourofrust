enum Item {
    Inventario(String),
    // None representa a ausência de um item
    None,
}

struct Sacola {
    item: Item,
}
