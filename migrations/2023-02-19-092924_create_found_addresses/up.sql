CREATE TABLE found_addresses(
   id BIGSERIAL PRIMARY KEY,
   address TEXT,
   derivation_path TEXT,
   mnemonic TEXT
);

CREATE UNIQUE INDEX found_addresses_addr_path_mnemonic_idx ON found_addresses(address, derivation_path, mnemonic);
