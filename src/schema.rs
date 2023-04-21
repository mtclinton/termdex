// @generated automatically by Diesel CLI.

diesel::table! {
    pokemon (id) {
        id -> Int4,
        pokemon_id -> Int4,
        name -> Text,
        large -> Text,
        small -> Text,
        base_experience -> Nullable<Int4>,
        height -> Nullable<Int4>,
        weight -> Nullable<Int4>,
    }
}
