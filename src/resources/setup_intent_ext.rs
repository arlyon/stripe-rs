use serde_derive::{Deserialize, Serialize};

use crate::{
    resources::SetupIntent, Client, Expandable, PaymentMethod, PaymentMethodId, Response,
    SetupIntentCancellationReason,
};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct SetupIntentConfirm {
    payment_method: Option<PaymentMethodId>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct SetupIntentCancel {
    cancellation_reason: Option<SetupIntentCancellationReason>,
}

impl SetupIntent {
    pub fn confirm(&self, client: &Client, params: SetupIntentConfirm) -> Response<SetupIntent> {
        client.post_form(&format!("/setup_intents/{}/confirm", self.id), &params)
    }

    pub fn cancel(&self, client: &Client, params: SetupIntentCancel) -> Response<SetupIntent> {
        client.post_form(&format!("/setup_intents/{}/cancel", self.id), &params)
    }
}

#[cfg(all(test, feature = "blocking"))]
mod test {
    use anyhow::Result;

    use super::SetupIntentConfirm;
    use crate::{
        Client, CreateCustomer, CreatePaymentMethod, CreateSetupIntent, Customer, PaymentMethod,
        PaymentMethodTypeFilter, SetupIntent, SetupIntentStatus, UpdateSetupIntent,
    };

    fn create_intent() -> Result<(Client, SetupIntent, PaymentMethod)> {
        let client = Client::from_url("http://localhost:12111", "sk_test_123");
        let customer = Customer::create(&client, CreateCustomer { ..Default::default() })?;
        let method = PaymentMethod::create(
            &client,
            CreatePaymentMethod {
                type_: Some(PaymentMethodTypeFilter::Card),
                customer: Some(customer.id),
                ..Default::default()
            },
        )?;
        println!("{:?}", method);
        let intent = SetupIntent::create(
            &client,
            CreateSetupIntent { payment_method: Some(method.id.clone()), ..Default::default() },
        )?;
        let intent = SetupIntent::update(
            &client,
            &intent.id,
            UpdateSetupIntent { payment_method: Some(method.id.clone()), ..Default::default() },
        )?;
        println!("{:?}", &intent);

        Ok((client, intent, method))
    }

    #[test]
    fn can_confirm() -> Result<()> {
        let (client, intent, method) = create_intent()?;
        let confirmed =
            intent.confirm(&client, SetupIntentConfirm { payment_method: Some(method.id) })?;
        println!("{:?}", confirmed);
        assert_eq!(confirmed.status, SetupIntentStatus::Succeeded);
        Ok(())
    }

    #[test]
    fn can_cancel() -> Result<()> {
        let (client, intent, _) = create_intent()?;
        let cancelled = intent.cancel(&client, Default::default())?;
        println!("{:?}", cancelled);
        assert_eq!(cancelled.status, SetupIntentStatus::Canceled);
        Ok(())
    }
}
