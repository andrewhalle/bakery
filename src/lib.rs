/// A collection of items that is priced together.
pub struct Cart {
    parcels: Vec<Parcel>,
}

/// A `Parcel` is a number of items grouped together. (ex. "8 cookies")
pub struct Parcel {
    item: Item,
    count: u32,
}

/// A good with a price.
pub struct Item {
    _id: u64,
    _name: String,
    // TODO: Not money as float.
    price: f64,
    bulk_pricing: Option<BulkPrice>,
}

pub struct BulkPrice {
    amount: u32,
    // TODO: Not money as float.
    total_price: f64,
}

impl Cart {
    /// Calculate the total price of all parcels in the cart.
    pub fn price(&self) -> f64 {
        self.parcels.iter().map(Parcel::price).sum()
    }
}

impl Parcel {
    /// Calculate the price of this parcel.
    pub fn price(&self) -> f64 {
        let mut count = self.count;
        let mut total = 0.0;
        if let Some(bulk_pricing) = &self.item.bulk_pricing {
            let number_of_groups = self.count / bulk_pricing.amount;
            count -= number_of_groups * bulk_pricing.amount;
            total += (number_of_groups as f64) * bulk_pricing.total_price;
        }
        total += (count as f64) * self.item.price;
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
                    _id: 1,
                    _name: String::from("cookies"),
                    price: 1.25,
                    bulk_pricing: Some(BulkPrice {
                        amount: 6,
                        total_price: 6.0,
                    }),
                },
            }],
        };
        assert_eq!(cart.price(), 7.25);
    }

    #[test]
    fn multi_item_one() {
        let cart = Cart {
            parcels: vec![
                Parcel {
                    item: Item {
                        _id: 1,
                        _name: String::from("cookies"),
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
                        _id: 2,
                        _name: String::from("Brownies"),
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
                        _id: 3,
                        _name: String::from("Cheesecake"),
                        price: 8.0,
                        bulk_pricing: None,
                    },
                    count: 1,
                },
            ],
        };

        assert_eq!(cart.price(), 16.25);
    }

    #[test]
    fn multi_item_two() {
        let cart = Cart {
            parcels: vec![
                Parcel {
                    item: Item {
                        _id: 1,
                        _name: String::from("cookies"),
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
                        _id: 2,
                        _name: String::from("Brownies"),
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
                        _id: 3,
                        _name: String::from("Cheesecake"),
                        price: 8.0,
                        bulk_pricing: None,
                    },
                    count: 1,
                },
                Parcel {
                    item: Item {
                        _id: 4,
                        _name: String::from("Donuts"),
                        price: 0.5,
                        bulk_pricing: None,
                    },
                    count: 2,
                },
            ],
        };

        assert_eq!(cart.price(), 12.25);
    }

    #[test]
    fn eight_cookies() {
        let cart = Cart {
            parcels: vec![Parcel {
                count: 8,
                item: Item {
                    _id: 1,
                    _name: String::from("cookies"),
                    price: 1.25,
                    bulk_pricing: Some(BulkPrice {
                        amount: 6,
                        total_price: 6.0,
                    }),
                },
            }],
        };
        assert_eq!(cart.price(), 8.50);
    }
}
