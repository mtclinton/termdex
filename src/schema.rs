// @generated automatically by Diesel CLI.

diesel::table! {
    max_stats (id) {
        id -> Int4,
        hp -> Int4,
        attack -> Int4,
        defense -> Int4,
        special_attack -> Int4,
        special_defense -> Int4,
        speed -> Int4,
    }
}

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
        hp -> Int4,
        attack -> Int4,
        defense -> Int4,
        special_attack -> Int4,
        special_defense -> Int4,
        speed -> Int4,
        entry -> Text,
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

diesel::allow_tables_to_appear_in_same_query!(max_stats, pokemon, pokemon_type, ptype,);
