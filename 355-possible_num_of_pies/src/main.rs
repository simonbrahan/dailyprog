use std::env;

type Ingredients = [usize; 5];

fn ingredients_from_comma_separated(list: &str) -> Ingredients {
    let parts: Vec<usize> = list
        .split(",")
        .map(
            |ingredient_count|
                ingredient_count
                    .parse::<usize>()
                    .expect("Ingredient input must be five comma separated positive integers eg 10,0,4,3,2")
        )
        .collect();

    return [parts[0], parts[1], parts[2], parts[3], parts[4]];
}

fn num_can_bake(available: &Ingredients, recipe: &Ingredients) -> usize {
    return available.iter().zip(recipe.iter()).map(
        |(available_count, recipe_requirement)| {
            if recipe_requirement == &0 {
                return usize::max_value();
            }

            return (available_count / recipe_requirement) as usize;
        }
    ).min().unwrap();
}

fn main() {
    let input = &env::args()
        .nth(1)
        .expect("Ingredient input must be five comma separated positive integers eg 10,0,4,3,2");

    let starting_ingredients = ingredients_from_comma_separated(input);
    let pumpkin_pie = ingredients_from_comma_separated("1,0,3,4,3");
    let apple_pie = ingredients_from_comma_separated("0,1,4,3,2");

    println!("{}", num_can_bake(&starting_ingredients, &pumpkin_pie));
    println!("{}", num_can_bake(&starting_ingredients, &apple_pie));
}
