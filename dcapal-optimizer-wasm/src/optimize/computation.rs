use nalgebra::{DMatrix, DVector};
use rust_decimal::Decimal;

extern crate nalgebra as na;

use na::{Dyn, OMatrix, U1};


//to retrieve n I should iterate over the assets and remove those with current_weight > target_weight

fn void() {
    let mut dm = DMatrix::<f64>::zeros(2, 2);
    dm.row_mut(0)
        .copy_from_slice(&[0.8 * 102.72 - 102.72, 27.99 * 0.8]);
    dm.row_mut(1)
        .copy_from_slice(&[102.72 * 0.1, 0.1 * 27.99 - 27.99]);

    let mut b = DVector::<f64>::zeros(2);
    b.copy_from_slice(&[
        1.00 * 102.72 - 0.8 * 1.00 * 102.72 - 0.8 * 5.00 * 27.99 - 0.8 * 300.00 * 4.56,
        5.00 * 27.99 - 0.1 * 300.00 * 4.56 - 0.1 * 1.00 * 102.72 - 0.1 * 5.00 * 27.99,
    ]);

    let decomp = dm.lu();
    let x = decomp.solve(&b).expect("Linear resolution failed.");
    println!("Solution: {:?}", x);
}

pub struct ProblemAsset {
    pub symbol: String,
    pub qty: f64,
    pub price: f64,
    pub current_weight: f64,
    pub target_weight: f64,
}

fn main() {
    let etf1 = ProblemAsset {
        symbol: "AAPL".to_string(),
        qty: 1.00,
        price: 102.72,
        current_weight: 0.064,
        target_weight: 0.8,
    };
    let etf2 = ProblemAsset {
        symbol: "MSFT".to_string(),
        qty: 5.00,
        price: 27.99,
        current_weight: 0.087,
        target_weight: 0.1,
    };
    let etf3 = ProblemAsset {
        symbol: "AMZN".to_string(),
        qty: 300.00,
        price: 4.56,
        current_weight: 0.849,
        target_weight: 0.1,
    };
    let sum = compute_sum(&vec![etf1, etf2, etf3]);

    let decomp = sum.0.lu();
    let x = decomp.solve(&sum.1).expect("Linear resolution failed.");
    println!("Solution: {:?}", x);
}

pub fn compute_sum(pa: &Vec<ProblemAsset>) -> (OMatrix<f64, Dyn, Dyn>, OMatrix<f64, Dyn, U1>) {
    let filtered = pa.iter().filter(|a| a.current_weight < a.target_weight).collect::<Vec<_>>();
    let n = filtered.len();
    let mut dm = DMatrix::<f64>::zeros(n, n);
    let mut b = DVector::<f64>::zeros(n);

    for (i, outer_a) in filtered.iter().enumerate() {
        b[i] = outer_a.price * outer_a.qty;
        for (j, pa) in pa.iter().enumerate() {
            if pa.current_weight < pa.target_weight {
                if i == j {
                    dm[(i, j)] = pa.price * pa.target_weight - pa.price;
                } else {
                    dm[(i, j)] = pa.price * outer_a.target_weight;
                }
            }
            b[i] -= pa.price * pa.qty * outer_a.target_weight;
        }
    }
    println!("dm: {:?}", dm);
    println!("b: {:?}", b);
    return (dm, b);
}