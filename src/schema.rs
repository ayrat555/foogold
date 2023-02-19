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

diesel::table! {
    found_addresses (id) {
        id -> Int8,
        address -> Nullable<Text>,
        derivation_path -> Nullable<Text>,
        mnemonic -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    addresses,
    blocks,
    found_addresses,
);
