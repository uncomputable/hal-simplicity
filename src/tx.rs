use std::io::Write;

use elements::confidential;
use elements::hex::ToHex;
use serde::{Deserialize, Serialize};
use simplicity::elements;

use crate::encode;
use crate::util::{GetInfo, Network};

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct TransactionInfo {
    txid: elements::Txid,
    wtxid: elements::Wtxid,
    hash: elements::Wtxid,
    size: usize,
    weight: usize,
    vsize: usize,
    version: u32,
    locktime: elements::LockTime,
    inputs: Vec<InputInfo>,
    outputs: Vec<OutputInfo>,
}

impl GetInfo<TransactionInfo> for elements::Transaction {
    fn get_info(&self, network: Network) -> TransactionInfo {
        TransactionInfo {
            txid: self.txid(),
            wtxid: self.wtxid(),
            hash: self.wtxid(),
            size: self.size(),
            weight: self.weight(),
            vsize: (self.weight() + 4 - 1) / 4,
            version: self.version,
            locktime: self.lock_time,
            inputs: self.input.iter().map(|i| i.get_info(network)).collect(),
            outputs: self.output.iter().map(|o| o.get_info(network)).collect(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct InputInfo {
    prevout: OutpointInfo,
    sequence: elements::Sequence,
    witness: InputWitnessInfo,
}

impl GetInfo<InputInfo> for elements::TxIn {
    fn get_info(&self, _network: Network) -> InputInfo {
        InputInfo {
            prevout: self.previous_output.get_info(_network),
            sequence: self.sequence,
            witness: self.witness.get_info(_network),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct OutpointInfo {
    txid: elements::Txid,
    vout: u32,
}

impl GetInfo<OutpointInfo> for elements::OutPoint {
    fn get_info(&self, _network: Network) -> OutpointInfo {
        OutpointInfo {
            txid: self.txid,
            vout: self.vout,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct InputWitnessInfo {
    stack: Vec<String>,
    is_key_spend: bool,
    script_spend: Option<ScriptSpendInfo>,
}

impl GetInfo<InputWitnessInfo> for elements::TxInWitness {
    fn get_info(&self, _network: Network) -> InputWitnessInfo {
        let stack = self.script_witness.iter().map(|x| x.to_hex()).collect();

        InputWitnessInfo {
            stack,
            is_key_spend: is_key_spend(&self.script_witness),
            script_spend: ScriptSpendWitness::new(&self.script_witness)
                .map(|x| x.get_info(_network)),
        }
    }
}

fn is_key_spend(script_witness: &[Vec<u8>]) -> bool {
    script_witness.len() == 2 && script_witness[0] == [1] && script_witness[1].len() == 32
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct ScriptSpendInfo {
    internal_key: String,
    merkle_path: Vec<String>,
    leaf_version: String,
    script_inputs: Vec<String>,
    script: String,
    simplicity: Option<SimplicitySpendInfo>,
}

struct ScriptSpendWitness<'a> {
    script_inputs: &'a [Vec<u8>],
    script: &'a [u8],
    control_block: elements::taproot::ControlBlock,
}

impl<'a> ScriptSpendWitness<'a> {
    pub fn new(script_witness: &'a [Vec<u8>]) -> Option<Self> {
        if script_witness.len() < 2 {
            return None;
        }

        let control_block =
            elements::taproot::ControlBlock::from_slice(script_witness.last().unwrap()).ok()?;
        let script_bytes = script_witness.get(script_witness.len() - 2).unwrap();
        let script_input_bytes = &script_witness[0..script_witness.len() - 2];

        Some(Self {
            script_inputs: script_input_bytes,
            script: script_bytes,
            control_block,
        })
    }
}

impl<'a> GetInfo<ScriptSpendInfo> for ScriptSpendWitness<'a> {
    fn get_info(&self, _network: Network) -> ScriptSpendInfo {
        let merkle_path: Vec<_> = self
            .control_block
            .merkle_branch
            .as_inner()
            .iter()
            .map(|h| h.to_hex())
            .collect();

        ScriptSpendInfo {
            internal_key: self.control_block.internal_key.to_hex(),
            merkle_path,
            leaf_version: self.control_block.leaf_version.as_u8().to_hex(),
            script_inputs: self.script_inputs.iter().map(|i| i.to_hex()).collect(),
            script: self.script.to_hex(),
            simplicity: self.get_simplicity_spend_info(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct SimplicitySpendInfo {
    program: String,
    cmr: String,
}

impl<'a> ScriptSpendWitness<'a> {
    pub fn is_simplicity_spend(&self) -> bool {
        self.control_block.leaf_version.as_u8() == 0xbe && self.script_inputs.len() == 1
    }

    pub fn get_simplicity_spend_info(&self) -> Option<SimplicitySpendInfo> {
        if !self.is_simplicity_spend() {
            return None;
        }

        let program_and_witness_bytes = &self.script_inputs[0];
        let cmr_bytes = self.script;

        // FIXME: Does this work with trailing padding?
        let base64 = encode::encode_base64(|w| w.write(program_and_witness_bytes)).ok()?;
        let cmr = cmr_bytes.to_hex();

        Some(SimplicitySpendInfo {
            program: base64,
            cmr,
        })
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct OutputInfo {
    script_pub_key: OutputScriptInfo,
    value: Option<u64>,
    asset: Option<String>,
    is_fee: bool,
}

impl GetInfo<OutputInfo> for elements::TxOut {
    fn get_info(&self, network: Network) -> OutputInfo {
        let value = if let confidential::Value::Explicit(n) = self.value {
            Some(n)
        } else {
            None
        };
        let asset = if let confidential::Asset::Explicit(asset_id) = self.asset {
            Some(asset_id.to_hex())
        } else {
            None
        };

        OutputInfo {
            script_pub_key: self.script_pubkey.get_info(network),
            value,
            asset,
            is_fee: self.is_fee(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct OutputScriptInfo {
    hex: String,
    asm: String,
    #[serde(rename = "type")]
    type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<elements::Address>,
}

impl GetInfo<OutputScriptInfo> for elements::Script {
    fn get_info(&self, network: Network) -> OutputScriptInfo {
        let type_ = if self.is_p2pk() {
            "p2pk"
        } else if self.is_p2pkh() {
            "p2pkh"
        } else if self.is_op_return() {
            "opreturn"
        } else if self.is_p2sh() {
            "p2sh"
        } else if self.is_v0_p2wpkh() {
            "p2wpkh"
        } else if self.is_v0_p2wsh() {
            "p2wsh"
        } else if self.is_v1_p2tr() {
            "p2tr"
        } else {
            "unknown"
        }
        .to_owned();
        let address = elements::Address::from_script(self, None, network.address_params());

        OutputScriptInfo {
            hex: self.to_hex(),
            asm: self.asm(),
            type_,
            address,
        }
    }
}
