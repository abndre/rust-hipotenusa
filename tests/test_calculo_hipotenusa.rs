// Arquivo: tests/test_calculo_hipotenusa.rs

mod calculo_hipotenusa;


#[test]
fn test_calculo_hipotenusa() {
    assert_eq!(calculo_hipotenusa::calcular_hipotenusa(3.0, 4.0), 5.0);
    assert_eq!(calculo_hipotenusa::calcular_hipotenusa(5.0, 12.0), 13.0);
    assert_eq!(calculo_hipotenusa::calcular_hipotenusa(8.0, 15.0), 17.0);
    // Adicione mais casos de teste conforme necessário
}
