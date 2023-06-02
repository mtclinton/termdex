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
        hp -> Nullable<Int4>,
        attack -> Nullable<Int4>,
        defense -> Nullable<Int4>,
        special_attack -> Nullable<Int4>,
        special_defense -> Nullable<Int4>,
        speed -> Nullable<Int4>,
    }
}

diesel::table! {
    pokemon_type (id) {
        id -> Int4,
        pokemon_id -> Int4,
        type_id -> Int4,
    }
}

diesel::table! {
    ptype (id) {
        id -> Int4,
        name -> Text,
        url -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    pokemon,
    pokemon_type,
    ptype,
);
