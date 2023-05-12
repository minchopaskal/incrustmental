use std::time::Duration;

use serde::Deserialize;

use crate::prelude::{Count, Price, ProductMaterial, Timer};

#[derive(Deserialize)]
pub(crate) struct ProductMaterialDef {
    init_bought: Count,
    kind: String,
    base_price: Price,
    growth: f64,
    unlocked: bool,
}

impl From<ProductMaterialDef> for ProductMaterial {
    fn from(product: ProductMaterialDef) -> Self {
        Self::new(
            product.init_bought,
            product.kind,
            product.base_price,
            product.growth,
            product.unlocked,
        )
    }
}

#[derive(Deserialize)]
pub(crate) struct TimerDef(u64);

impl From<TimerDef> for Timer {
    fn from(timer_def: TimerDef) -> Self {
        Self::new(Duration::from_secs(timer_def.0))
    }
}
