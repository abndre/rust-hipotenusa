mod calculo_hipotenusa;

fn main() {
    println!("Hello, world! Hipotenusa");
    let cateto_a: f64= 3.0;
    let cateto_b: f64= 4.0;

    let hipotenusa: f64 = calculo_hipotenusa::calcular_hipotenusa(cateto_a, cateto_b);
    println!("Cateto a: {} e Cateto b: {}", cateto_a, cateto_b);
    println!("A hipotenusa é: {}", hipotenusa);
}
