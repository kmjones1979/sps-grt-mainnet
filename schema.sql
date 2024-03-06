CREATE TABLE IF NOT EXISTS grt_approval (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "owner" VARCHAR(40),
    "spender" VARCHAR(40),
    "value" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS grt_minter_added (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "account" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS grt_minter_removed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "account" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS grt_new_ownership (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "from" VARCHAR(40),
    "to" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS grt_new_pending_ownership (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "from" VARCHAR(40),
    "to" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS grt_transfer (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "from" VARCHAR(40),
    "to" VARCHAR(40),
    "value" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS grt_call_accept_ownership (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS grt_call_add_minter (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "u_account" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS grt_call_approve (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "amount" DECIMAL,
    "output_param0" BOOL,
    "spender" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS grt_call_burn (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "amount" DECIMAL,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS grt_call_burn_from (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "account" VARCHAR(40),
    "amount" DECIMAL,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS grt_call_decrease_allowance (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "output_param0" BOOL,
    "spender" VARCHAR(40),
    "subtracted_value" DECIMAL,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS grt_call_increase_allowance (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "added_value" DECIMAL,
    "output_param0" BOOL,
    "spender" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS grt_call_mint (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "u_amount" DECIMAL,
    "u_to" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS grt_call_permit (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "u_deadline" DECIMAL,
    "u_owner" VARCHAR(40),
    "u_r" TEXT,
    "u_s" TEXT,
    "u_spender" VARCHAR(40),
    "u_v" INT,
    "u_value" DECIMAL,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS grt_call_remove_minter (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "u_account" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS grt_call_renounce_minter (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS grt_call_transfer (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "amount" DECIMAL,
    "output_param0" BOOL,
    "recipient" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS grt_call_transfer_from (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "amount" DECIMAL,
    "output_param0" BOOL,
    "recipient" VARCHAR(40),
    "sender" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS grt_call_transfer_ownership (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "u_new_governor" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);


