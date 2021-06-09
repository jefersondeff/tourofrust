fn faz_alguma_coisa_que_pode_falhar(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("este não é o numero correto"))
    }
}

fn main() {
    let result = faz_alguma_coisa_que_pode_falhar(12);
    match result {
        Ok(v) => println!("Encontrei, {}", v),
        Err(e) => println!("Error, {}", e),
    }
}
