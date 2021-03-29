//! resources module
//!
//! This module exposes various elements of the
//! stripe api depending on the features exposed.
//!
//! Some of these modules are hand-written, and
//! some are generated.

mod currency;
mod generated;
mod placeholders;
mod types;

#[path = "resources"]
mod core {
    pub mod balance_transaction_ext;
    pub mod charge_ext;
    pub mod customer_ext;
    pub mod payment_intent_ext;
    pub mod payment_source;
    pub mod payout_ext;
    pub mod placeholders;
    pub mod setup_intent_ext;
    pub mod token_ext;
}

#[path = "resources"]
mod payment {
    pub mod bank_account_ext;
    pub mod card;
    pub mod payment_method_ext;
    pub mod source_ext;
}

#[cfg(feature = "events")]
mod webhook_events;

#[path = "resources"]
#[cfg(feature = "billing")]
mod billing {
    pub mod invoice_ext;
    pub mod line_item_ext;
    pub mod subscription_ext;
}

#[path = "resources"]
#[cfg(feature = "fraud")]
mod fraud {
    pub mod review_ext;
}

#[path = "resources"]
#[cfg(feature = "issuing")]
mod issuing {
    pub mod issuing_authorization_ext;
    pub mod issuing_card_ext;
    pub mod issuing_dispute_ext;
    pub mod issuing_merchant_data;
    pub mod issuing_transaction_ext;
}

#[path = "resources"]
#[cfg(feature = "orders")]
mod orders {
    pub mod order_ext;
}

#[path = "resources"]
#[cfg(feature = "webhook-endpoints")]
mod webhook_endpoints {
    pub mod webhook_endpoint_ext;
}

#[rustfmt::skip]
pub use {
    currency::*,
    types::*,

    self::core::{
        balance_transaction_ext::*,
        charge_ext::*,
        customer_ext::*,
        payment_intent_ext::*,
        payment_source::*,
        placeholders::*,
        payout_ext::*,
        setup_intent_ext::*,
        token_ext::*,
    },
    generated::core::{
        balance::*,
        balance_transaction::*,
        charge::*,
        customer::*,
        dispute::*,
        file::*,
        file_link::*,
        mandate::*,
        payment_intent::*,
        payout::*,
        platform_tax_fee::*,
        product::*,
        refund::*,
        reserve_transaction::*,
        setup_attempt::*,
        setup_intent::*,
        tax_deducted_at_source::*,
        token::*,
    },

    payment::{
        bank_account_ext::*,
        card::*,
        payment_method_ext::*,
        source_ext::*
    },
    generated::payment::{
        alipay_account::*,
        card::*,
        bank_account::*,
        payment_method::*,
        source::*
    },
};

#[rustfmt::skip]
#[cfg(feature = "events")]
pub use {
    webhook_events::*,
    generated::event::*,
};

#[rustfmt::skip]
#[cfg(feature = "checkout")]
pub use {
    generated::checkout::{
        checkout_session::*,
        item::*
    },
};

#[rustfmt::skip]
#[cfg(feature = "billing")]
pub use {
    billing::{
        invoice_ext::*,
        line_item_ext::*,
        subscription_ext::*
    },
    generated::billing::{
        coupon::*,
        discount::*,
        invoice::*,
        invoiceitem::*,
        line_item::*,
        plan::*,
        price::*,
        promotion_code::*,
        subscription::*,
        subscription_item::*,
        subscription_schedule::*,
        tax_id::*,
        tax_rate::*,
    },
};

#[rustfmt::skip]
#[cfg(feature = "connect")]
pub use {
    generated::connect::{
        account_link::*,
        account::*,
        application::*,
        application_fee::*,
        connect_collection_transfer::*,
        fee_refund::*,
        person::*,
        recipient::*,
        topup::*,
        transfer::*,
        transfer_reversal::*,
    }
};

#[rustfmt::skip]
#[cfg(feature = "fraud")]
pub use {
    fraud::review_ext::*,
    generated::fraud::review::*
};

#[rustfmt::skip]
#[cfg(feature = "issuing")]
pub use {
    issuing::{
        issuing_authorization_ext::*,
        issuing_card_ext::*,
        issuing_dispute_ext::*,
        issuing_merchant_data::*,
        issuing_transaction_ext::*,
    },
    generated::issuing::{
        issuing_authorization::*,
        issuing_card::*,
        issuing_cardholder::*,
        issuing_dispute::*,
        issuing_transaction::*,
    },
};

#[rustfmt::skip]
#[cfg(feature = "orders")]
pub use {
    orders::order_ext::*,
    generated::orders::{
        order::*,
        order_item::*,
        order_return::*,
        sku::*
    },
};

#[rustfmt::skip]
#[cfg(feature = "sigma")]
pub use {
    generated::scheduled_query_run::*,
};

#[rustfmt::skip]
#[cfg(feature = "webhook-endpoints")]
pub use {
    webhook_endpoints::webhook_endpoint_ext::*,
    generated::webhook_endpoints::webhook_endpoint::*, 
};

#[cfg(not(feature = "full"))]
pub use generated::placeholders::*;

#[cfg(not(feature = "account"))]
#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct CompanyParams {
    #[serde(default)]
    pub metadata: crate::params::Metadata,
}

#[cfg(not(feature = "account"))]
#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct PersonParams {
    #[serde(default)]
    pub metadata: crate::params::Metadata,
}
