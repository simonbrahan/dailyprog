use std::env;

type Ingredients = [usize; 5];

fn ingredients_from_comma_separated(list: &str) -> Ingredients {
    let parts: Vec<usize> = list.split(",")
        .map(|ingredient_count| {
            ingredient_count.parse::<usize>().expect(
                "Ingredient input must be five comma separated positive integers eg 10,0,4,3,2",
            )
        })
        .collect();

    return ingredients_from_vec(parts);
}

fn ingredients_from_vec(vec: Vec<usize>) -> Ingredients {
    [vec[0], vec[1], vec[2], vec[3], vec[4]]
}

fn num_can_bake(available: &Ingredients, recipe: &Ingredients) -> usize {
    return available
        .iter()
        .zip(recipe.iter())
        .map(|(available_count, recipe_requirement)| {
            if recipe_requirement == &0 {
                return usize::max_value();
            }

            return (available_count / recipe_requirement) as usize;
        })
        .min()
        .unwrap();
}

fn remaining_after_baking(available: &Ingredients, recipe: &Ingredients, num_to_bake: &usize) -> Ingredients {
    let ingredients_used = recipe.iter().map(|ingredient_count| ingredient_count * num_to_bake);

    let ingredients_remaining: Vec<usize> = ingredients_used.zip(available)
        .map(|(count_used, count_available)| {
            return count_available - count_used;
        })
        .collect();

    return ingredients_from_vec(ingredients_remaining);
}

fn main() {
    let input = &env::args()
        .nth(1)
        .expect("Ingredient input must be five comma separated positive integers eg 10,0,4,3,2");

    let starting_ingredients = ingredients_from_comma_separated(input);
    let pumpkin_pie = ingredients_from_comma_separated("1,0,3,4,3");
    let apple_pie = ingredients_from_comma_separated("0,1,4,3,2");

    remaining_after_baking(&starting_ingredients, &apple_pie, &2);
}
