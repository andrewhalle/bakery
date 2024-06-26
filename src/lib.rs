use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A collection of items that is priced together.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cart {
    pub parcels: Vec<Parcel>,
}

/// A `Parcel` is a number of items grouped together. (ex. "8 cookies")
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Parcel {
    pub item: Item,
    pub count: u32,
}

/// A good with a price.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: u64,
    name: String,
    // TODO: Not money as float.
    price: f64,
    bulk_pricing: Option<BulkPrice>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkPrice {
    amount: u32,
    // TODO: Not money as float.
    total_price: f64,
}

/// A change that can be applied to the price of a parcel.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Sale {
    /// Replace the bulk price (if there is one) for the parcel's item.
    Bulk(BulkPrice),
    /// Reduce the total price of the parcel by this percentage.
    PercentOff(f64),
    /// For the purposes of calculating the price, reduce the number of items by this factor.
    NForOne(u32),
}

impl Cart {
    /// Calculate the total price of all parcels in the cart.
    pub fn price(&self, sales: &HashMap<u64, Sale>) -> f64 {
        self.parcels
            .iter()
            .map(|parcel| {
                let item = &parcel.item;
                let sale = sales.get(&item.id);
                parcel.price(sale)
            })
            .sum()
    }
}

impl Parcel {
    /// Calculate the price of this parcel.
    pub fn price(&self, sale: Option<&Sale>) -> f64 {
        let mut count = if let Some(Sale::NForOne(n)) = sale {
            self.count / n
        } else {
            self.count
        };
        let mut total = 0.0;
        let sale_bulk_pricing = if let Some(Sale::Bulk(bulk_pricing)) = sale {
            Some(bulk_pricing)
        } else {
            None
        };
        let bulk_price = sale_bulk_pricing.or(self.item.bulk_pricing.as_ref());
        if let Some(bulk_pricing) = bulk_price {
            let number_of_groups = count / bulk_pricing.amount;
            count -= number_of_groups * bulk_pricing.amount;
            total += (number_of_groups as f64) * bulk_pricing.total_price;
        }
        total += (count as f64) * self.item.price;

        if let Some(Sale::PercentOff(percent_off)) = sale {
            total *= 1. - percent_off;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seven_cookies() {
        let cart = Cart {
            parcels: vec![Parcel {
                count: 7,
                item: Item {
                    id: 1,
                    name: String::from("cookies"),
                    price: 1.25,
                    bulk_pricing: Some(BulkPrice {
                        amount: 6,
                        total_price: 6.0,
                    }),
                },
            }],
        };
        assert_eq!(cart.price(&HashMap::new()), 7.25);
    }

    #[test]
    fn multi_item_one() {
        let cart = Cart {
            parcels: vec![
                Parcel {
                    item: Item {
                        id: 1,
                        name: String::from("cookies"),
                        price: 1.25,
                        bulk_pricing: Some(BulkPrice {
                            amount: 6,
                            total_price: 6.0,
                        }),
                    },
                    count: 1,
                },
                Parcel {
                    item: Item {
                        id: 2,
                        name: String::from("Brownies"),
                        price: 2.0,
                        bulk_pricing: Some(BulkPrice {
                            amount: 4,
                            total_price: 7.0,
                        }),
                    },
                    count: 4,
                },
                Parcel {
                    item: Item {
                        id: 3,
                        name: String::from("Cheesecake"),
                        price: 8.0,
                        bulk_pricing: None,
                    },
                    count: 1,
                },
            ],
        };

        assert_eq!(cart.price(&HashMap::new()), 16.25);
    }

    #[test]
    fn multi_item_two() {
        let cart = Cart {
            parcels: vec![
                Parcel {
                    item: Item {
                        id: 1,
                        name: String::from("cookies"),
                        price: 1.25,
                        bulk_pricing: Some(BulkPrice {
                            amount: 6,
                            total_price: 6.0,
                        }),
                    },
                    count: 1,
                },
                Parcel {
                    item: Item {
                        id: 2,
                        name: String::from("Brownies"),
                        price: 2.0,
                        bulk_pricing: Some(BulkPrice {
                            amount: 4,
                            total_price: 7.0,
                        }),
                    },
                    count: 1,
                },
                Parcel {
                    item: Item {
                        id: 3,
                        name: String::from("Cheesecake"),
                        price: 8.0,
                        bulk_pricing: None,
                    },
                    count: 1,
                },
                Parcel {
                    item: Item {
                        id: 4,
                        name: String::from("Donuts"),
                        price: 0.5,
                        bulk_pricing: None,
                    },
                    count: 2,
                },
            ],
        };

        assert_eq!(cart.price(&HashMap::new()), 12.25);
    }

    #[test]
    fn eight_cookies() {
        let cart = Cart {
            parcels: vec![Parcel {
                count: 8,
                item: Item {
                    id: 1,
                    name: String::from("cookies"),
                    price: 1.25,
                    bulk_pricing: Some(BulkPrice {
                        amount: 6,
                        total_price: 6.0,
                    }),
                },
            }],
        };
        assert_eq!(cart.price(&HashMap::new()), 8.50);
    }

    #[test]
    fn sale() {
        let cart = Cart {
            parcels: vec![
                Parcel {
                    item: Item {
                        id: 1,
                        name: String::from("cookies"),
                        price: 1.25,
                        bulk_pricing: Some(BulkPrice {
                            amount: 6,
                            total_price: 6.0,
                        }),
                    },
                    count: 8,
                },
                Parcel {
                    item: Item {
                        id: 2,
                        name: String::from("Cheesecakes"),
                        price: 8.0,
                        bulk_pricing: None,
                    },
                    count: 4,
                },
            ],
        };
        let sales: HashMap<u64, Sale> = [
            (
                1,
                Sale::Bulk(BulkPrice {
                    amount: 8,
                    total_price: 6.0,
                }),
            ),
            (2, Sale::PercentOff(0.25)),
        ]
        .into_iter()
        .collect();
        assert_eq!(cart.price(&sales), 30.0);
    }

    #[test]
    fn n_for_one_sale() {
        let cart = Cart {
            parcels: vec![Parcel {
                item: Item {
                    id: 1,
                    name: String::from("cookies"),
                    price: 1.25,
                    bulk_pricing: Some(BulkPrice {
                        amount: 6,
                        total_price: 6.0,
                    }),
                },
                count: 12,
            }],
        };
        let sales: HashMap<u64, Sale> = [(1, Sale::NForOne(2))].into_iter().collect();
        assert_eq!(cart.price(&sales), 6.0);
    }
}
