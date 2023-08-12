use serde::{Deserialize, Serialize};
use simplicity::elements;

// Copied from hal-elements
// We don't use hal-elements directly because of different rust-elements versions

/// Known Elements networks.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Network {
    ElementsRegtest,
    Liquid,
}

impl Network {
    #[allow(dead_code)]
    pub fn from_params(params: &'static elements::AddressParams) -> Option<Network> {
        match *params {
            elements::AddressParams::ELEMENTS => Some(Network::ElementsRegtest),
            elements::AddressParams::LIQUID => Some(Network::Liquid),
            _ => None,
        }
    }

    pub fn address_params(self) -> &'static elements::AddressParams {
        match self {
            Network::ElementsRegtest => &elements::AddressParams::ELEMENTS,
            Network::Liquid => &elements::AddressParams::LIQUID,
        }
    }
}

/// Get JSON-able objects that describe the type.
pub trait GetInfo<T: Serialize> {
    /// Get a description of this object given the network of interest.
    fn get_info(&self, network: Network) -> T;
}
