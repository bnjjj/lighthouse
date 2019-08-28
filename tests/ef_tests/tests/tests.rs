use ef_tests::*;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use types::{
    Attestation, AttestationData, AttestationDataAndCustodyBit, AttesterSlashing, BeaconBlock,
    BeaconBlockBody, BeaconBlockHeader, BeaconState, Checkpoint, CompactCommittee, Crosslink,
    Deposit, DepositData, Eth1Data, Fork, HistoricalBatch, IndexedAttestation, MainnetEthSpec,
    MinimalEthSpec, PendingAttestation, ProposerSlashing, Transfer, Validator, VoluntaryExit,
};
use walkdir::WalkDir;

fn yaml_files_in_test_dir(dir: &Path) -> Vec<PathBuf> {
    let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("eth2.0-spec-tests")
        .join("tests")
        .join("general")
        .join("phase0")
        .join(dir);

    assert!(
        base_path.exists(),
        format!(
            "Unable to locate {:?}. Did you init git submodules?",
            base_path
        )
    );

    let mut paths: Vec<PathBuf> = WalkDir::new(base_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|entry| {
            if entry.file_type().is_file() {
                match entry.file_name().to_str() {
                    Some(f) if f.ends_with(".yaml") => Some(entry.path().to_path_buf()),
                    Some(f) if f.ends_with(".yml") => Some(entry.path().to_path_buf()),
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect();

    // Reverse the file order. Assuming files come in lexicographical order, executing tests in
    // reverse means we get the "minimal" tests before the "mainnet" tests. This makes life easier
    // for debugging.
    paths.reverse();
    paths
}

#[test]
#[cfg(feature = "fake_crypto")]
fn ssz_generic() {
    yaml_files_in_test_dir(&Path::new("ssz_generic"))
        .into_par_iter()
        .for_each(|file| {
            Doc::assert_tests_pass(file);
        });
}

#[test]
#[cfg(feature = "fake_crypto")]
fn ssz_static() {
    yaml_files_in_test_dir(&Path::new("ssz_static"))
        .into_par_iter()
        .for_each(|file| {
            Doc::assert_tests_pass(file);
        });
}

#[test]
fn shuffling() {
    yaml_files_in_test_dir(&Path::new("shuffling").join("core"))
        .into_par_iter()
        .for_each(|file| {
            Doc::assert_tests_pass(file);
        });
}

#[test]
fn operations_deposit() {
    yaml_files_in_test_dir(&Path::new("operations").join("deposit"))
        .into_par_iter()
        .for_each(|file| {
            Doc::assert_tests_pass(file);
        });
}

#[test]
fn operations_transfer() {
    yaml_files_in_test_dir(&Path::new("operations").join("transfer"))
        .into_par_iter()
        .rev()
        .for_each(|file| {
            Doc::assert_tests_pass(file);
        });
}

#[test]
fn operations_exit() {
    yaml_files_in_test_dir(&Path::new("operations").join("voluntary_exit"))
        .into_par_iter()
        .for_each(|file| {
            Doc::assert_tests_pass(file);
        });
}

#[test]
fn operations_proposer_slashing() {
    yaml_files_in_test_dir(&Path::new("operations").join("proposer_slashing"))
        .into_par_iter()
        .for_each(|file| {
            Doc::assert_tests_pass(file);
        });
}

#[test]
fn operations_attester_slashing() {
    yaml_files_in_test_dir(&Path::new("operations").join("attester_slashing"))
        .into_par_iter()
        .for_each(|file| {
            Doc::assert_tests_pass(file);
        });
}

#[test]
fn operations_attestation() {
    yaml_files_in_test_dir(&Path::new("operations").join("attestation"))
        .into_par_iter()
        .for_each(|file| {
            Doc::assert_tests_pass(file);
        });
}

#[test]
fn operations_block_header() {
    yaml_files_in_test_dir(&Path::new("operations").join("block_header"))
        .into_par_iter()
        .for_each(|file| {
            Doc::assert_tests_pass(file);
        });
}

#[test]
fn sanity_blocks() {
    yaml_files_in_test_dir(&Path::new("sanity").join("blocks"))
        .into_par_iter()
        .for_each(|file| {
            Doc::assert_tests_pass(file);
        });
}

#[test]
fn sanity_slots() {
    yaml_files_in_test_dir(&Path::new("sanity").join("slots"))
        .into_par_iter()
        .for_each(|file| {
            Doc::assert_tests_pass(file);
        });
}

#[test]
#[cfg(not(feature = "fake_crypto"))]
fn bls() {
    yaml_files_in_test_dir(&Path::new("bls"))
        .into_iter()
        .for_each(|file| {
            Doc::assert_tests_pass(file);
        });
}

#[test]
#[cfg(not(feature = "fake_crypto"))]
fn bls_aggregate_pubkeys() {
    BlsAggregatePubkeysHandler::run();
}

#[test]
#[cfg(not(feature = "fake_crypto"))]
fn bls_aggregate_sigs() {
    BlsAggregateSigsHandler::run();
}

#[test]
#[cfg(not(feature = "fake_crypto"))]
fn bls_msg_hash_g2_compressed() {
    BlsG2CompressedHandler::run();
}

#[test]
#[cfg(not(feature = "fake_crypto"))]
fn bls_priv_to_pub() {
    BlsPrivToPubHandler::run();
}

#[test]
#[cfg(not(feature = "fake_crypto"))]
fn bls_sign_msg() {
    BlsSignMsgHandler::run();
}

macro_rules! ssz_static_test {
    // Signed-root
    ($test_name:ident, $typ:ident$(<$generics:tt>)?, SR) => {
        ssz_static_test!($test_name, SszStaticSRHandler, $typ$(<$generics>)?);
    };
    // Non-signed root
    ($test_name:ident, $typ:ident$(<$generics:tt>)?) => {
        ssz_static_test!($test_name, SszStaticHandler, $typ$(<$generics>)?);
    };
    // Generic
    ($test_name:ident, $handler:ident, $typ:ident<_>) => {
        ssz_static_test!(
            $test_name, $handler, {
                ($typ<MinimalEthSpec>, MinimalEthSpec),
                ($typ<MainnetEthSpec>, MainnetEthSpec)
            }
        );
    };
    // Non-generic
    ($test_name:ident, $handler:ident, $typ:ident) => {
        ssz_static_test!(
            $test_name, $handler, {
                ($typ, MinimalEthSpec),
                ($typ, MainnetEthSpec)
            }
        );
    };
    // Base case
    ($test_name:ident, $handler:ident, { $(($typ:ty, $spec:ident)),+ }) => {
        #[test]
        #[cfg(feature = "fake_crypto")]
        fn $test_name() {
            $(
                $handler::<$typ, $spec>::run();
            )+
        }
    };
}

ssz_static_test!(ssz_static_attestation, Attestation<_>, SR);
ssz_static_test!(ssz_static_attestation_data, AttestationData);
ssz_static_test!(
    ssz_static_attestation_data_and_custody_bit,
    AttestationDataAndCustodyBit
);
ssz_static_test!(ssz_static_attester_slashing, AttesterSlashing<_>);
ssz_static_test!(ssz_static_beacon_block, BeaconBlock<_>, SR);
ssz_static_test!(ssz_static_beacon_block_body, BeaconBlockBody<_>);
ssz_static_test!(ssz_static_beacon_block_header, BeaconBlockHeader, SR);
ssz_static_test!(ssz_static_beacon_state, BeaconState<_>);
ssz_static_test!(ssz_static_checkpoint, Checkpoint);
ssz_static_test!(ssz_static_compact_committee, CompactCommittee<_>);
ssz_static_test!(ssz_static_crosslink, Crosslink);
ssz_static_test!(ssz_static_deposit, Deposit);
ssz_static_test!(ssz_static_deposit_data, DepositData, SR);
ssz_static_test!(ssz_static_eth1_data, Eth1Data);
ssz_static_test!(ssz_static_fork, Fork);
ssz_static_test!(ssz_static_historical_batch, HistoricalBatch<_>);
ssz_static_test!(ssz_static_indexed_attestation, IndexedAttestation<_>, SR);
ssz_static_test!(ssz_static_pending_attestation, PendingAttestation<_>);
ssz_static_test!(ssz_static_proposer_slashing, ProposerSlashing);
ssz_static_test!(ssz_static_transfer, Transfer, SR);
ssz_static_test!(ssz_static_validator, Validator);
ssz_static_test!(ssz_static_voluntary_exit, VoluntaryExit, SR);

#[test]
fn epoch_processing_justification_and_finalization() {
    yaml_files_in_test_dir(&Path::new("epoch_processing").join("justification_and_finalization"))
        .into_par_iter()
        .for_each(|file| {
            Doc::assert_tests_pass(file);
        });
}

#[test]
fn epoch_processing_crosslinks() {
    yaml_files_in_test_dir(&Path::new("epoch_processing").join("crosslinks"))
        .into_par_iter()
        .for_each(|file| {
            Doc::assert_tests_pass(file);
        });
}

#[test]
fn epoch_processing_registry_updates() {
    yaml_files_in_test_dir(&Path::new("epoch_processing").join("registry_updates"))
        .into_par_iter()
        .for_each(|file| {
            Doc::assert_tests_pass(file);
        });
}

#[test]
fn epoch_processing_slashings() {
    yaml_files_in_test_dir(&Path::new("epoch_processing").join("slashings"))
        .into_par_iter()
        .for_each(|file| {
            Doc::assert_tests_pass(file);
        });
}

#[test]
fn epoch_processing_final_updates() {
    yaml_files_in_test_dir(&Path::new("epoch_processing").join("final_updates"))
        .into_par_iter()
        .for_each(|file| {
            Doc::assert_tests_pass(file);
        });
}

#[test]
fn genesis_initialization() {
    yaml_files_in_test_dir(&Path::new("genesis").join("initialization"))
        .into_par_iter()
        .for_each(|file| {
            Doc::assert_tests_pass(file);
        });
}

#[test]
fn genesis_validity() {
    yaml_files_in_test_dir(&Path::new("genesis").join("validity"))
        .into_par_iter()
        .for_each(|file| {
            Doc::assert_tests_pass(file);
        });
}
