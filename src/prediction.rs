use crate::regression::Modelo;

pub fn prever(modelo: &Modelo, passos: usize) -> Vec<f64> {
    // Gera previsões para os próximos 'passos' períodos.
    let inicio = passos as f64;
    (0..passos)
        .map(|i| modelo.slope * (inicio + i as f64) + modelo.intercept) // Aplica a fórmula da reta.
        .collect()
}
