use shopping::Basket;
use shopping::Item;
use std::collections::HashMap;

mod tests {
    use super::*;

    #[test]
    fn should_find_quantity_of_particular_item_a() {
        let mut basket = Basket::new();

        basket.add(Item::ItemA, 5);

        let actual = basket.quantity_of(Item::ItemA);

        assert_eq!(actual, 5);
    }

    #[test]
    fn should_find_quantity_of_particular_item_b() {
        let mut basket = Basket::new();

        basket.add(Item::ItemB, 6);

        let actual = basket.quantity_of(Item::ItemB);

        assert_eq!(actual, 6);
    }

    #[test]
    fn should_add_two_items_of_same_kind() {
        let mut basket = Basket::new();

        basket.add(Item::ItemA, 4);
        basket.add(Item::ItemA, 3);

        let actual = basket.quantity_of(Item::ItemA);

        assert_eq!(actual, 7);
    }

    #[test]
    fn should_calculate_total_price() {
        let mut basket = Basket::from_prices(prices());

        basket.add(Item::ItemA, 1);
        basket.add(Item::ItemB, 2);

        assert_eq!(basket.total(), 60.0);
    }

    #[test]
    fn should_calculate_total_price_with_5_discount_percent() {
        let mut basket = Basket::from_prices(prices());
        basket.add(Item::ItemA, 5);
        basket.add(Item::ItemB, 2);
        basket.add(Item::ItemC, 6);

        assert_eq!(basket.total(), 151.94);
    }

    fn prices() -> HashMap<Item, f32> {
        HashMap::from([
            (Item::ItemA, 10.0),
            (Item::ItemB, 25.0),
            (Item::ItemC, 9.99),
        ])
    }
}
