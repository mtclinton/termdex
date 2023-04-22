// @generated automatically by Diesel CLI.

diesel::table! {
    pokemon (id) {
        id -> Int4,
        pokemon_id -> Int4,
        name -> Text,
        large -> Text,
        small -> Text,
        base_experience -> Int4,
        height -> Int4,
        weight -> Int4,
    }
}
