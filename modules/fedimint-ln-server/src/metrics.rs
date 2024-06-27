use fedimint_metrics::prometheus::{
    register_histogram_vec_with_registry, register_int_counter_with_registry,
};
use fedimint_metrics::{
    histogram_opts, opts, HistogramVec, IntCounter, AMOUNTS_BUCKETS_SATS, REGISTRY,
};
use once_cell::sync::Lazy;

pub static LN_INCOMING_OFFER: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter_with_registry!(
        opts!("ln_incoming_offer_total", "Incoming payment offer"),
        REGISTRY
    )
    .unwrap()
});
pub static LN_CANCEL_OUTGOING_CONTRACTS: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter_with_registry!(
        opts!(
            "ln_canceled_outgoing_contract_total",
            "Canceled outgoing contract"
        ),
        REGISTRY
    )
    .unwrap()
});
pub static LN_FUNDED_CONTRACT_SATS: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec_with_registry!(
        histogram_opts!(
            "ln_funded_contract_sats",
            "Funded (with outgoing or incoming direction) contract amount in sats",
            AMOUNTS_BUCKETS_SATS.clone()
        ),
        &["direction"],
        REGISTRY
    )
    .unwrap()
});
