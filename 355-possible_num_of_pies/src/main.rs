use std::env;

#[derive(Debug)]
struct Ingredients {
    pumpkin: usize,
    apples: usize,
    eggs: usize,
    milk: usize,
    sugar: usize
}

impl Ingredients {
    fn from_cs(list: &str) -> Ingredients {
        let ingredient_counts: Vec<usize> = list
            .split(",")
            .map(
                |ingredient_count|
                    ingredient_count
                        .parse::<usize>()
                        .expect("Ingredient input must be five comma separated positive integers eg 10,0,4,3,2")
            )
            .collect();

        return Ingredients {
            pumpkin: ingredient_counts[0],
            apples: ingredient_counts[1],
            eggs: ingredient_counts[2],
            milk: ingredient_counts[3],
            sugar: ingredient_counts[4]
        }
    }
}

fn main() {
    let input = &env::args()
        .nth(1)
        .expect("Ingredient input must be five comma separated positive integers eg 10,0,4,3,2");

    let starting_ingredients = Ingredients::from_cs(input);
    let pumpkin_pie = Ingredients::from_cs("1,0,3,4,3");
    let apple_pie = Ingredients::from_cs("0,1,4,3,2");
}
