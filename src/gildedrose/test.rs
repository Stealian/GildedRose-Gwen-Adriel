use super::{Item, GildedRose};

// +5 Dexterity Vest
// Aged Brie
// Elixir of the Mongoose
// Sulfuras, Hand of Ragnaros
// Backstage passes to a TAFKAL80ETC concert

#[test]
pub fn sellin_always_decrease_each_day() {
    let items = vec![Item::new(String::from("+5 Dexterity Vest"), 4, 0),
                     Item::new(String::from("+5 Dexterity Vest"), -4, 0)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();

   assert_eq!(3, rose.items[0].sell_in); 
   assert_eq!(-5, rose.items[1].sell_in); 
}

#[test]
pub fn quality_decrease_by_1_each_day_before_peremption() {
    let items = vec![Item::new(String::from("+5 Dexterity Vest"), 4, 1)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();

    assert_eq!(0, rose.items[0].quality);
}

#[test]
pub fn quality_decrease_by_2_each_day_after_peremption() {
    let items = vec![Item::new(String::from("+5 Dexterity Vest"), -4, 1)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();

    assert_eq!(0, rose.items[0].quality);
}

#[test]
pub fn quality_cannot_be_negative() {
    let items = vec![Item::new(String::from("+5 Dexterity Vest"), 4, 0),
                     Item::new(String::from("+5 Dexterity Vest"), -4, 1)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();

    assert_eq!(0, rose.items[0].quality);
    assert_eq!(0, rose.items[1].quality);
}

#[test]
pub fn aged_brie_quality_increase_by_time() {
    let items = vec![Item::new(String::from("Aged Brie"), 4, 0)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();

    assert_eq!(1, rose.items[0].quality);
}

#[test]
pub fn quality_cannot_be_more_than_fifty() {
    let items = vec![Item::new(String::from("Aged Brie"), 4, 50),
                     Item::new(String::from("Backstage passes to a TAFKAL80ETC concert"), 4, 50)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();

    assert_eq!(50, rose.items[0].quality);
    assert_eq!(50, rose.items[1].quality);
}

#[test]
pub fn sulfuras_quality_never_decrease() {
    let items = vec![Item::new(String::from("Sulfuras, Hand of Ragnaros"), 0, 80)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();

    assert_eq!(80, rose.items[0].quality);
}

#[test]
pub fn backstage_passes_increase_by_2_10_days_before_peremption() {
    let items = vec![Item::new(String::from("Backstage passes to a TAFKAL80ETC concert"), 10, 4),
                     Item::new(String::from("Backstage passes to a TAFKAL80ETC concert"), 6, 4)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();

    assert_eq!(6, rose.items[0].quality);
    assert_eq!(6, rose.items[1].quality);
}

#[test]
pub fn backstage_passes_increase_by_3_5_days_before_peremption() {
    let items = vec![Item::new(String::from("Backstage passes to a TAFKAL80ETC concert"), 5, 4),
                     Item::new(String::from("Backstage passes to a TAFKAL80ETC concert"), 1, 4)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();

    assert_eq!(7, rose.items[0].quality);
    assert_eq!(7, rose.items[1].quality);
}

#[test]
pub fn backstage_passes_lose_quality_after_concert() {
    let items = vec![Item::new(String::from("Backstage passes to a TAFKAL80ETC concert"), 0, 50)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();

    assert_eq!(0, rose.items[0].quality);
}
