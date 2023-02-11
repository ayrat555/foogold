// @generated automatically by Diesel CLI.

diesel::table! {
    addresses (address) {
        address -> Text,
    }
}

diesel::table! {
    blocks (block_number) {
        block_number -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    addresses,
    blocks,
);
