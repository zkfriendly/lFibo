
use halo2_proofs::plonk::{Advice, Column};
use halo2_proofs::poly::Rotation;
use halo2_proofs::{arithmetic::FieldExt, plonk::*};
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct FiboConfig {
    pub advice: [Column<Advice>; 3],
    pub selector: Selector,
}

pub struct FiboChip<F: FieldExt> {
    config: FiboConfig,
    _marker: std::marker::PhantomData<F>,
}

impl<F: FieldExt> FiboChip<F> {
    pub fn construct(config: FiboConfig) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }
    pub fn configure(meta: &mut ConstraintSystem<F>, advice: [Column<Advice>; 3]) -> FiboConfig {
        let col_a = advice[0];
        let col_b = advice[1];
        let col_c = advice[2];
        let selector = meta.selector();

        meta.enable_equality(col_a);
        meta.enable_equality(col_b);
        meta.enable_equality(col_c);

        ///
        /// col_a  | col_b  | col_c | selector
        ///   a       b         c       s
        meta.create_gate("add", |meta| {
            let s = meta.query_selector(selector);
            let a = meta.query_advice(col_a, Rotation::cur());
            let b = meta.query_advice(col_b, Rotation::cur());
            let c = meta.query_advice(col_c, Rotation::cur());
            vec![s * (a + b - c)]
        });

        FiboConfig {
            advice: [col_a, col_b, col_c],
            selector,
        }
    }
}
