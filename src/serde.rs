use serde::Deserialize;

use crate::prelude::{ProductMaterial, Count, Price};

#[derive(Deserialize)]
pub(crate) struct ProductMaterialDef {
    pub(crate) init_bought: Count,
    pub(crate) kind: String,
    pub(crate) base_price: Price,
    pub(crate) growth: f64,
    pub(crate) unlocked: bool
}

impl From<ProductMaterialDef> for ProductMaterial {
    fn from(product: ProductMaterialDef) -> Self {
        Self::new(
            product.init_bought,
            product.kind,
            product.base_price,
            product.growth,
            product.unlocked
        )
    }
}