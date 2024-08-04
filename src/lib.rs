use std::collections::HashMap;

type Quantity = u8;
type Money = f32;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Item {
    ItemA,
    ItemB,
    ItemC,
}
pub struct Basket {
    prices: HashMap<Item, Money>,
    data: HashMap<Item, Quantity>,
}

impl Basket {
    pub fn add(&mut self, item: Item, qty: Quantity) {
        self.data
            .entry(item)
            .and_modify(|cqty| *cqty += qty)
            .or_insert(qty);
    }

    pub fn quantity_of(&self, item: Item) -> Quantity {
        *self.data.get(&item).unwrap_or(&0)
    }

    pub fn from_prices(prices: HashMap<Item, Money>) -> Basket {
        Basket {
            prices: prices,
            data: HashMap::new(),
        }
    }

    fn unit_price(&self, item: &Item) -> Money {
        *self.prices.get(item).unwrap_or(&0.0)
    }

    fn round(money: Money) -> Money {
        (money * 100.0).round() / 100.0
    }

    pub fn total(&self) -> Money {
        let total = *&self.data.iter()
            .map(|(item, qty)| *qty as f32 * self.unit_price(&item))
            .sum::<f32>();

        if total > 100.0 {
            Self::round(total * 0.95)
        } else {
            Self::round(total)
        }

    }




    pub fn new() -> Basket {
        Basket {
            prices: HashMap::new(),
            data: HashMap::new(),
        }
    }
}
