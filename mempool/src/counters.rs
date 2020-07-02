// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use libra_metrics::{
    register_int_counter, register_int_counter_vec, register_int_gauge_vec, IntCounter,
    IntCounterVec, IntGaugeVec,
};
use once_cell::sync::Lazy;

// Core mempool index labels
pub const PRIORITY_INDEX_LABEL: &str = "priority";
pub const EXPIRATION_TIME_INDEX_LABEL: &str = "expiration";
pub const SYSTEM_TTL_INDEX_LABEL: &str = "system_ttl";
pub const TIMELINE_INDEX_LABEL: &str = "timeline";
pub const PARKING_LOT_INDEX_LABEL: &str = "parking_lot";

/// Counter tracking size of various indices in core mempool
pub static CORE_MEMPOOL_INDEX_SIZE: Lazy<IntGaugeVec> = Lazy::new(|| {
    register_int_gauge_vec!(
        "libra_core_mempool_index_size",
        "Size of a core mempool index",
        &["index"]
    )
    .unwrap()
});

/// Counter of pending network events to Mempool
pub static PENDING_MEMPOOL_NETWORK_EVENTS: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "mempool_pending_network_events",
        "Counters(queued,dequeued,dropped) related to pending network notifications to Mempool",
        &["state"]
    )
    .unwrap()
});

pub static MEMPOOL_SERVICE: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "libra_mempool_service",
        "Number of txns processed by Mempool Service",
        &[
            // type of request through which the transactions were delivered to MempoolService:
            // "get_block", "commit_transactions"
            "req_type",
            // direction of the request: "requested", "returned"
            "req_status",
        ]
    )
    .unwrap()
});

pub static SHARED_MEMPOOL_EVENTS: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "libra_shared_mempool_events",
        "Number of network events received by shared mempool",
        &["event"] // type of event: "new_peer", "lost_peer", "message"
    )
    .unwrap()
});

pub static SHARED_MEMPOOL_TRANSACTIONS_PROCESSED: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "libra_shared_mempool_transactions_processed",
        "Number of transactions received and handled by shared mempool",
        &[
            // state of transaction processing: "received", "success", status code from failed txn processing
            "status", // sender of the txns
            "sender"
        ]
    )
    .unwrap()
});

pub static SHARED_MEMPOOL_TRANSACTION_BROADCAST: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "libra_shared_mempool_transaction_broadcast",
        "Number of transactions broadcasted by shared mempool"
    )
    .unwrap()
});