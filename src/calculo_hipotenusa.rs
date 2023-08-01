// Arquivo: src/calculo_hipotenusa.rs

pub fn calcular_hipotenusa(cateto_a: f64, cateto_b: f64) -> f64 {
    (cateto_a.powi(2) + cateto_b.powi(2)).sqrt()
}
