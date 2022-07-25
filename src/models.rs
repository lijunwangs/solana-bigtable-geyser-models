pub mod generated {
    include!(concat!(
        env!("OUT_DIR"),
        "/solana.geyser.plugin.confirmed_block.rs"
    ));
}

pub mod tx_by_addr {
    include!(concat!(
        env!("OUT_DIR"),
        "/solana.geyser.plugin.transaction_by_addr.rs"
    ));
}

pub mod accounts {
    include!(concat!(env!("OUT_DIR"), "/solana.geyser.plugin.account.rs"));
}

pub mod slots {
    include!(concat!(env!("OUT_DIR"), "/solana.geyser.plugin.slot.rs"));
}
