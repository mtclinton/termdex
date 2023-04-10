// @generated automatically by Diesel CLI.

diesel::table! {
    pokemon (id) {
        id -> Int4,
        pokemon_id -> Int4,
        name -> Text,
        large -> Nullable<Text>,
        small -> Nullable<Text>,
    }
}
