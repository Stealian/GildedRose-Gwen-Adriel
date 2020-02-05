use std::string;
use std::vec;

pub struct Item {
    pub name: string::String,
    pub sell_in: i32,
    pub quality: i32
}

impl Item {
    pub fn new(name: String, sell_in: i32, quality: i32) -> Item {
        Item {name: name, sell_in: sell_in, quality: quality}
    }
}

pub struct GildedRose {
    pub items: vec::Vec<Item>
}

impl GildedRose {
    pub fn new(items: vec::Vec<Item>) -> GildedRose {
        GildedRose {items: items}
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            if item.name == "Aged Brie" {
                item.quality += 1;
            } else if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                if item.sell_in > 10 {
                    item.quality += 1;
                } else if item.sell_in > 5 {
                    item.quality += 2;
                } else if item.sell_in > 0 {
                    item.quality += 3;
                } else {
                    item.quality = 0;
                }
            } else if item.name == "Sulfuras, Hand of Ragnaros" {
                item.sell_in -= 1;
                break;
            } else {
                item.quality -= 1;
                if item.sell_in < 0 {
                    item.quality -= 1;
                }
            }
            item.sell_in -= 1;
            if item.quality > 50 {
                item.quality = 50;
            }
            if item.quality < 0 {
                item.quality = 0;
            }
        }
    }
}

#[cfg(test)]
    mod test;
