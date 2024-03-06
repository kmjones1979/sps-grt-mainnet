mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const GRT_TRACKED_CONTRACT: [u8; 20] = hex!("c944e90c64b2c07662a292be6244bdf05cda44a7");

fn map_grt_events(blk: &eth::Block, events: &mut contract::Events) {
    events.grt_approvals.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GRT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::grt_contract::events::Approval::match_and_decode(log) {
                        return Some(contract::GrtApproval {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            owner: event.owner,
                            spender: event.spender,
                            value: event.value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.grt_minter_addeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GRT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::grt_contract::events::MinterAdded::match_and_decode(log) {
                        return Some(contract::GrtMinterAdded {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            account: event.account,
                        });
                    }

                    None
                })
        })
        .collect());
    events.grt_minter_removeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GRT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::grt_contract::events::MinterRemoved::match_and_decode(log) {
                        return Some(contract::GrtMinterRemoved {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            account: event.account,
                        });
                    }

                    None
                })
        })
        .collect());
    events.grt_new_ownerships.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GRT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::grt_contract::events::NewOwnership::match_and_decode(log) {
                        return Some(contract::GrtNewOwnership {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            from: event.from,
                            to: event.to,
                        });
                    }

                    None
                })
        })
        .collect());
    events.grt_new_pending_ownerships.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GRT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::grt_contract::events::NewPendingOwnership::match_and_decode(log) {
                        return Some(contract::GrtNewPendingOwnership {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            from: event.from,
                            to: event.to,
                        });
                    }

                    None
                })
        })
        .collect());
    events.grt_transfers.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GRT_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::grt_contract::events::Transfer::match_and_decode(log) {
                        return Some(contract::GrtTransfer {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            from: event.from,
                            to: event.to,
                            value: event.value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
}

fn map_grt_calls(blk: &eth::Block, calls: &mut contract::Calls) {
    calls.grt_call_accept_ownerships.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == GRT_TRACKED_CONTRACT && abi::grt_contract::functions::AcceptOwnership::match_call(call))
                .filter_map(|call| {
                    match abi::grt_contract::functions::AcceptOwnership::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::GrtAcceptOwnershipCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.grt_call_add_minters.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == GRT_TRACKED_CONTRACT && abi::grt_contract::functions::AddMinter::match_call(call))
                .filter_map(|call| {
                    match abi::grt_contract::functions::AddMinter::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::GrtAddMinterCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_account: decoded_call.u_account,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.grt_call_approves.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == GRT_TRACKED_CONTRACT && abi::grt_contract::functions::Approve::match_call(call))
                .filter_map(|call| {
                    match abi::grt_contract::functions::Approve::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::grt_contract::functions::Approve::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::GrtApproveCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                output_param0: output_param0,
                                spender: decoded_call.spender,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.grt_call_burns.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == GRT_TRACKED_CONTRACT && abi::grt_contract::functions::Burn::match_call(call))
                .filter_map(|call| {
                    match abi::grt_contract::functions::Burn::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::GrtBurnCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.grt_call_burn_froms.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == GRT_TRACKED_CONTRACT && abi::grt_contract::functions::BurnFrom::match_call(call))
                .filter_map(|call| {
                    match abi::grt_contract::functions::BurnFrom::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::GrtBurnFromCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                account: decoded_call.account,
                                amount: decoded_call.amount.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.grt_call_decrease_allowances.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == GRT_TRACKED_CONTRACT && abi::grt_contract::functions::DecreaseAllowance::match_call(call))
                .filter_map(|call| {
                    match abi::grt_contract::functions::DecreaseAllowance::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::grt_contract::functions::DecreaseAllowance::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::GrtDecreaseAllowanceCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                output_param0: output_param0,
                                spender: decoded_call.spender,
                                subtracted_value: decoded_call.subtracted_value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.grt_call_increase_allowances.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == GRT_TRACKED_CONTRACT && abi::grt_contract::functions::IncreaseAllowance::match_call(call))
                .filter_map(|call| {
                    match abi::grt_contract::functions::IncreaseAllowance::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::grt_contract::functions::IncreaseAllowance::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::GrtIncreaseAllowanceCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                added_value: decoded_call.added_value.to_string(),
                                output_param0: output_param0,
                                spender: decoded_call.spender,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.grt_call_mints.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == GRT_TRACKED_CONTRACT && abi::grt_contract::functions::Mint::match_call(call))
                .filter_map(|call| {
                    match abi::grt_contract::functions::Mint::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::GrtMintCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_amount: decoded_call.u_amount.to_string(),
                                u_to: decoded_call.u_to,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.grt_call_permits.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == GRT_TRACKED_CONTRACT && abi::grt_contract::functions::Permit::match_call(call))
                .filter_map(|call| {
                    match abi::grt_contract::functions::Permit::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::GrtPermitCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_deadline: decoded_call.u_deadline.to_string(),
                                u_owner: decoded_call.u_owner,
                                u_r: Vec::from(decoded_call.u_r),
                                u_s: Vec::from(decoded_call.u_s),
                                u_spender: decoded_call.u_spender,
                                u_v: decoded_call.u_v.to_u64(),
                                u_value: decoded_call.u_value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.grt_call_remove_minters.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == GRT_TRACKED_CONTRACT && abi::grt_contract::functions::RemoveMinter::match_call(call))
                .filter_map(|call| {
                    match abi::grt_contract::functions::RemoveMinter::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::GrtRemoveMinterCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_account: decoded_call.u_account,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.grt_call_renounce_minters.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == GRT_TRACKED_CONTRACT && abi::grt_contract::functions::RenounceMinter::match_call(call))
                .filter_map(|call| {
                    match abi::grt_contract::functions::RenounceMinter::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::GrtRenounceMinterCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.grt_call_transfers.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == GRT_TRACKED_CONTRACT && abi::grt_contract::functions::Transfer::match_call(call))
                .filter_map(|call| {
                    match abi::grt_contract::functions::Transfer::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::grt_contract::functions::Transfer::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::GrtTransferCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                output_param0: output_param0,
                                recipient: decoded_call.recipient,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.grt_call_transfer_froms.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == GRT_TRACKED_CONTRACT && abi::grt_contract::functions::TransferFrom::match_call(call))
                .filter_map(|call| {
                    match abi::grt_contract::functions::TransferFrom::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::grt_contract::functions::TransferFrom::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::GrtTransferFromCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                output_param0: output_param0,
                                recipient: decoded_call.recipient,
                                sender: decoded_call.sender,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.grt_call_transfer_ownerships.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == GRT_TRACKED_CONTRACT && abi::grt_contract::functions::TransferOwnership::match_call(call))
                .filter_map(|call| {
                    match abi::grt_contract::functions::TransferOwnership::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::GrtTransferOwnershipCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_new_governor: decoded_call.u_new_governor,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
}

fn db_grt_out(events: &contract::Events, tables: &mut DatabaseChangeTables) {
    // Loop over all the abis events to create table changes
    events.grt_approvals.iter().for_each(|evt| {
        tables
            .create_row("grt_approval", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("owner", Hex(&evt.owner).to_string())
            .set("spender", Hex(&evt.spender).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.grt_minter_addeds.iter().for_each(|evt| {
        tables
            .create_row("grt_minter_added", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("account", Hex(&evt.account).to_string());
    });
    events.grt_minter_removeds.iter().for_each(|evt| {
        tables
            .create_row("grt_minter_removed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("account", Hex(&evt.account).to_string());
    });
    events.grt_new_ownerships.iter().for_each(|evt| {
        tables
            .create_row("grt_new_ownership", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string());
    });
    events.grt_new_pending_ownerships.iter().for_each(|evt| {
        tables
            .create_row("grt_new_pending_ownership", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string());
    });
    events.grt_transfers.iter().for_each(|evt| {
        tables
            .create_row("grt_transfer", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
}
fn db_grt_calls_out(calls: &contract::Calls, tables: &mut DatabaseChangeTables) {
    // Loop over all the abis calls to create table changes
    calls.grt_call_accept_ownerships.iter().for_each(|call| {
        tables
            .create_row("grt_call_accept_ownership", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success);
    });
    calls.grt_call_add_minters.iter().for_each(|call| {
        tables
            .create_row("grt_call_add_minter", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("u_account", Hex(&call.u_account).to_string());
    });
    calls.grt_call_approves.iter().for_each(|call| {
        tables
            .create_row("grt_call_approve", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap())
            .set("output_param0", call.output_param0)
            .set("spender", Hex(&call.spender).to_string());
    });
    calls.grt_call_burns.iter().for_each(|call| {
        tables
            .create_row("grt_call_burn", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap());
    });
    calls.grt_call_burn_froms.iter().for_each(|call| {
        tables
            .create_row("grt_call_burn_from", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("account", Hex(&call.account).to_string())
            .set("amount", BigDecimal::from_str(&call.amount).unwrap());
    });
    calls.grt_call_decrease_allowances.iter().for_each(|call| {
        tables
            .create_row("grt_call_decrease_allowance", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("output_param0", call.output_param0)
            .set("spender", Hex(&call.spender).to_string())
            .set("subtracted_value", BigDecimal::from_str(&call.subtracted_value).unwrap());
    });
    calls.grt_call_increase_allowances.iter().for_each(|call| {
        tables
            .create_row("grt_call_increase_allowance", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("added_value", BigDecimal::from_str(&call.added_value).unwrap())
            .set("output_param0", call.output_param0)
            .set("spender", Hex(&call.spender).to_string());
    });
    calls.grt_call_mints.iter().for_each(|call| {
        tables
            .create_row("grt_call_mint", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("u_amount", BigDecimal::from_str(&call.u_amount).unwrap())
            .set("u_to", Hex(&call.u_to).to_string());
    });
    calls.grt_call_permits.iter().for_each(|call| {
        tables
            .create_row("grt_call_permit", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("u_deadline", BigDecimal::from_str(&call.u_deadline).unwrap())
            .set("u_owner", Hex(&call.u_owner).to_string())
            .set("u_r", Hex(&call.u_r).to_string())
            .set("u_s", Hex(&call.u_s).to_string())
            .set("u_spender", Hex(&call.u_spender).to_string())
            .set("u_v", call.u_v)
            .set("u_value", BigDecimal::from_str(&call.u_value).unwrap());
    });
    calls.grt_call_remove_minters.iter().for_each(|call| {
        tables
            .create_row("grt_call_remove_minter", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("u_account", Hex(&call.u_account).to_string());
    });
    calls.grt_call_renounce_minters.iter().for_each(|call| {
        tables
            .create_row("grt_call_renounce_minter", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success);
    });
    calls.grt_call_transfers.iter().for_each(|call| {
        tables
            .create_row("grt_call_transfer", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap())
            .set("output_param0", call.output_param0)
            .set("recipient", Hex(&call.recipient).to_string());
    });
    calls.grt_call_transfer_froms.iter().for_each(|call| {
        tables
            .create_row("grt_call_transfer_from", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap())
            .set("output_param0", call.output_param0)
            .set("recipient", Hex(&call.recipient).to_string())
            .set("sender", Hex(&call.sender).to_string());
    });
    calls.grt_call_transfer_ownerships.iter().for_each(|call| {
        tables
            .create_row("grt_call_transfer_ownership", [("call_tx_hash", call.call_tx_hash.to_string()),("call_ordinal", call.call_ordinal.to_string())])
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("u_new_governor", Hex(&call.u_new_governor).to_string());
    });
}


fn graph_grt_out(events: &contract::Events, tables: &mut EntityChangesTables) {
    // Loop over all the abis events to create table changes
    events.grt_approvals.iter().for_each(|evt| {
        tables
            .create_row("grt_approval", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("owner", Hex(&evt.owner).to_string())
            .set("spender", Hex(&evt.spender).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.grt_minter_addeds.iter().for_each(|evt| {
        tables
            .create_row("grt_minter_added", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("account", Hex(&evt.account).to_string());
    });
    events.grt_minter_removeds.iter().for_each(|evt| {
        tables
            .create_row("grt_minter_removed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("account", Hex(&evt.account).to_string());
    });
    events.grt_new_ownerships.iter().for_each(|evt| {
        tables
            .create_row("grt_new_ownership", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string());
    });
    events.grt_new_pending_ownerships.iter().for_each(|evt| {
        tables
            .create_row("grt_new_pending_ownership", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string());
    });
    events.grt_transfers.iter().for_each(|evt| {
        tables
            .create_row("grt_transfer", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
}
fn graph_grt_calls_out(calls: &contract::Calls, tables: &mut EntityChangesTables) {
    // Loop over all the abis calls to create table changes
    calls.grt_call_accept_ownerships.iter().for_each(|call| {
        tables
            .create_row("grt_call_accept_ownership", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success);
    });
    calls.grt_call_add_minters.iter().for_each(|call| {
        tables
            .create_row("grt_call_add_minter", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("u_account", Hex(&call.u_account).to_string());
    });
    calls.grt_call_approves.iter().for_each(|call| {
        tables
            .create_row("grt_call_approve", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap())
            .set("output_param0", call.output_param0)
            .set("spender", Hex(&call.spender).to_string());
    });
    calls.grt_call_burns.iter().for_each(|call| {
        tables
            .create_row("grt_call_burn", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap());
    });
    calls.grt_call_burn_froms.iter().for_each(|call| {
        tables
            .create_row("grt_call_burn_from", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("account", Hex(&call.account).to_string())
            .set("amount", BigDecimal::from_str(&call.amount).unwrap());
    });
    calls.grt_call_decrease_allowances.iter().for_each(|call| {
        tables
            .create_row("grt_call_decrease_allowance", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("output_param0", call.output_param0)
            .set("spender", Hex(&call.spender).to_string())
            .set("subtracted_value", BigDecimal::from_str(&call.subtracted_value).unwrap());
    });
    calls.grt_call_increase_allowances.iter().for_each(|call| {
        tables
            .create_row("grt_call_increase_allowance", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("added_value", BigDecimal::from_str(&call.added_value).unwrap())
            .set("output_param0", call.output_param0)
            .set("spender", Hex(&call.spender).to_string());
    });
    calls.grt_call_mints.iter().for_each(|call| {
        tables
            .create_row("grt_call_mint", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("u_amount", BigDecimal::from_str(&call.u_amount).unwrap())
            .set("u_to", Hex(&call.u_to).to_string());
    });
    calls.grt_call_permits.iter().for_each(|call| {
        tables
            .create_row("grt_call_permit", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("u_deadline", BigDecimal::from_str(&call.u_deadline).unwrap())
            .set("u_owner", Hex(&call.u_owner).to_string())
            .set("u_r", Hex(&call.u_r).to_string())
            .set("u_s", Hex(&call.u_s).to_string())
            .set("u_spender", Hex(&call.u_spender).to_string())
            .set("u_v", call.u_v)
            .set("u_value", BigDecimal::from_str(&call.u_value).unwrap());
    });
    calls.grt_call_remove_minters.iter().for_each(|call| {
        tables
            .create_row("grt_call_remove_minter", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("u_account", Hex(&call.u_account).to_string());
    });
    calls.grt_call_renounce_minters.iter().for_each(|call| {
        tables
            .create_row("grt_call_renounce_minter", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success);
    });
    calls.grt_call_transfers.iter().for_each(|call| {
        tables
            .create_row("grt_call_transfer", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap())
            .set("output_param0", call.output_param0)
            .set("recipient", Hex(&call.recipient).to_string());
    });
    calls.grt_call_transfer_froms.iter().for_each(|call| {
        tables
            .create_row("grt_call_transfer_from", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("amount", BigDecimal::from_str(&call.amount).unwrap())
            .set("output_param0", call.output_param0)
            .set("recipient", Hex(&call.recipient).to_string())
            .set("sender", Hex(&call.sender).to_string());
    });
    calls.grt_call_transfer_ownerships.iter().for_each(|call| {
        tables
            .create_row("grt_call_transfer_ownership", format!("{}-{}", call.call_tx_hash, call.call_ordinal))
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("u_new_governor", Hex(&call.u_new_governor).to_string());
    });
  }

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_grt_events(&blk, &mut events);
    Ok(events)
}
#[substreams::handlers::map]
fn map_calls(blk: eth::Block) -> Result<contract::Calls, substreams::errors::Error> {
    let mut calls = contract::Calls::default();
    map_grt_calls(&blk, &mut calls);
    Ok(calls)
}

#[substreams::handlers::map]
fn db_out(events: contract::Events, calls: contract::Calls) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = DatabaseChangeTables::new();
    db_grt_out(&events, &mut tables);
    db_grt_calls_out(&calls, &mut tables);
    Ok(tables.to_database_changes())
}

#[substreams::handlers::map]
fn graph_out(events: contract::Events, calls: contract::Calls) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = EntityChangesTables::new();
    graph_grt_out(&events, &mut tables);
    graph_grt_calls_out(&calls, &mut tables);
    Ok(tables.to_entity_changes())
}
